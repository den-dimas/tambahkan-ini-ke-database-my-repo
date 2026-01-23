use reqwest;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;
use tokio::net::TcpListener;
use tokio::sync::Barrier;

mod common;

#[tokio::test]
async fn stress_test_rate_limiting() {
    let (app, _pool) = common::setup_app().await;

    // Spawn the server on a random port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    let server_handle = tokio::spawn(async move {
        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        .unwrap();
    });

    // Give it a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let num_requests = 50;
    let barrier = Arc::new(Barrier::new(num_requests));
    let mut handles = vec![];

    let start_all = Instant::now();

    for _ in 0..num_requests {
        let client = client.clone();
        let barrier = Arc::clone(&barrier);
        let url = format!("http://{}/api/v1/people/search?q=test", addr);

        let handle = tokio::spawn(async move {
            barrier.wait().await;
            let start = Instant::now();
            let response = client.get(url).send().await.unwrap();
            let duration = start.elapsed();
            (response.status(), duration)
        });
        handles.push(handle);
    }

    let mut statuses = vec![];
    let mut durations = vec![];

    for handle in handles {
        let (status, duration) = handle.await.unwrap();
        statuses.push(status);
        durations.push(duration);
    }

    let total_duration = start_all.elapsed();
    let avg_duration = durations.iter().sum::<std::time::Duration>() / num_requests as u32;

    println!("\n--- Stress Test Results ---");
    println!(
        "Total duration for {} requests: {:?}",
        num_requests, total_duration
    );
    println!("Average response time: {:?}", avg_duration);

    let rate_limited = statuses
        .iter()
        .filter(|&&s| s == reqwest::StatusCode::TOO_MANY_REQUESTS)
        .count();
    let unauthorized = statuses
        .iter()
        .filter(|&&s| s == reqwest::StatusCode::UNAUTHORIZED)
        .count();

    println!("Rate limited (429): {}", rate_limited);
    println!("Unauthorized (401): {}", unauthorized);
    println!("---------------------------\n");

    // With 50 requests at once, and a burst of 5, we expect many 429s.
    assert!(
        rate_limited > 0,
        "Expected at least some requests to be rate limited (429)"
    );

    // Clean up
    server_handle.abort();
}
