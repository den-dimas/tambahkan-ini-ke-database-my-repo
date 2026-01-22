use tambahkan_ini_ke_database_my_api::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    serve().await
}
