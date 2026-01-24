# Docker Deployment Guide

This guide explains how to deploy the application using Docker and Docker Compose.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Architecture Overview](#architecture-overview)
- [Configuration](#configuration)
- [Deployment Steps](#deployment-steps)
- [Managing the Application](#managing-the-application)
- [Database Migrations](#database-migrations)
- [Connecting to Existing PostgreSQL](#connecting-to-existing-postgresql)
- [Troubleshooting](#troubleshooting)
- [Production Considerations](#production-considerations)

## Prerequisites

Before deploying, ensure you have the following installed:

- **Docker**: Version 20.10 or higher ([Install Docker](https://docs.docker.com/get-docker/))
- **Docker Compose**: Version 2.0 or higher (included with Docker Desktop)
- **Git**: For cloning the repository

Verify installations:

```bash
docker --version
docker-compose --version
```

## Architecture Overview

The application consists of three main services:

- **Frontend** (Port 12000): Vue 3 application built with Bun, served by Bun
- **Backend** (Port 16000): Rust Axum API server
- **PostgreSQL** (Port 5432): Database server (can use existing instance)

```
┌─────────────────┐
│   Frontend      │
│   (Bun:12000)   │
└────────┬────────┘
         │
         ▼
┌─────────────────┐      ┌──────────────┐
│    Backend      │─────▶│  PostgreSQL  │
│  (Rust:16000)   │      │   (5432)     │
└─────────────────┘      └──────────────┘
```

## Configuration

### Step 1: Environment Variables

Create a `.env` file in the project root based on the template:

```bash
cp .env.docker .env
```

### Step 2: Edit Configuration

Open the `.env` file and update the following values:

#### Required Configuration

**JWT Secret** (⚠️ Critical for security):

```bash
# Generate a strong secret:
openssl rand -base64 32

# Then update in .env:
JWT_SECRET=your-generated-secret-here
```

**Database Configuration**:

```env
DB_URL=postgresql://postgres:yourpassword@postgres:5432/mydb
DATABASE_URL=postgresql://postgres:yourpassword@postgres:5432/mydb
POSTGRES_PASSWORD=yourpassword
```

**CORS Origins**:

```env
# For development:
CORS_ORIGINS=http://localhost:12000

# For production (replace with your domain):
CORS_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
```

#### Cloudflare R2 Configuration

Get your R2 credentials from the [Cloudflare Dashboard](https://dash.cloudflare.com/):

```env
R2_ACCESS_KEY_ID=your-r2-access-key-id
R2_SECRET_ACCESS_KEY=your-r2-secret-access-key
R2_ENDPOINT=https://your-account-id.r2.cloudflarestorage.com
R2_BUCKET_NAME=your-bucket-name
R2_PUBLIC_DOMAIN=https://your-public-domain.com
```

## Deployment Steps

### Quick Start

1. **Clone the repository** (if not already done):

   ```bash
   git clone <repository-url>
   cd tambahkan-ini-ke-database-my-repo
   ```

2. **Configure environment** (see [Configuration](#configuration) above):

   ```bash
   cp .env.docker .env
   # Edit .env with your values
   ```

3. **Build and start all services**:

   ```bash
   docker-compose up -d
   ```

4. **Check service health**:

   ```bash
   docker-compose ps
   ```

5. **View logs** (optional):

   ```bash
   docker-compose logs -f
   ```

6. **Access the application**:
   - Frontend: http://localhost:12000
   - Backend API: http://localhost:16000/api/v1
   - Health check: http://localhost:16000/health

### Step-by-Step Deployment

#### Step 1: Build Images

Build the Docker images without starting containers:

```bash
# Build all services
docker-compose build

# Or build specific services
docker-compose build frontend
docker-compose build backend
```

**Note**: The first build will take several minutes as it downloads dependencies and compiles the Rust backend.

#### Step 2: Start Services

Start all services in detached mode:

```bash
docker-compose up -d
```

Watch the startup process:

```bash
docker-compose logs -f
```

Press `Ctrl+C` to stop watching logs (services continue running).

#### Step 3: Verify Deployment

Check that all services are healthy:

```bash
docker-compose ps
```

Expected output:

```
NAME            IMAGE                    STATUS          PORTS
app-backend     api-backend              Up (healthy)    0.0.0.0:16000->16000/tcp
app-frontend    api-frontend             Up (healthy)    0.0.0.0:12000->12000/tcp
app-postgres    postgres:16-alpine       Up (healthy)    0.0.0.0:5432->5432/tcp
```

Test endpoints:

```bash
# Health check
curl http://localhost:16000/health

# Frontend
curl http://localhost:12000/health

# API endpoint (should return 401 if not authenticated)
curl http://localhost:16000/api/v1/people
```

## Managing the Application

### Start Services

```bash
# Start all services
docker-compose up -d

# Start specific service
docker-compose up -d backend
```

### Stop Services

```bash
# Stop all services (keeps containers)
docker-compose stop

# Stop specific service
docker-compose stop backend
```

### Restart Services

```bash
# Restart all services
docker-compose restart

# Restart specific service
docker-compose restart backend
```

### Shutdown and Remove

```bash
# Stop and remove containers (preserves volumes)
docker-compose down

# Stop, remove containers AND volumes (⚠️ deletes database data)
docker-compose down -v
```

### View Logs

```bash
# All services (follow mode)
docker-compose logs -f

# Specific service
docker-compose logs -f backend

# Last 100 lines
docker-compose logs --tail=100 backend

# Since specific time
docker-compose logs --since 1h backend
```

### Update Application

After code changes:

```bash
# Rebuild and restart
docker-compose up -d --build

# Or rebuild specific service
docker-compose up -d --build backend
```

## Database Migrations

Migrations run automatically on backend startup. To run manually:

```bash
# Execute inside backend container
docker-compose exec backend sqlx migrate run --source /app/migrations

# Check migration status
docker-compose exec backend sqlx migrate info --source /app/migrations
```

### Rollback Migration

```bash
# Revert last migration
docker-compose exec backend sqlx migrate revert --source /app/migrations
```

## Connecting to Existing PostgreSQL

If you already have a PostgreSQL container running, follow these steps:

### Option 1: External Network

1. **Edit `docker-compose.yml`**:

   Comment out the `postgres` service and update the networks section:

   ```yaml
   networks:
     app-network:
       external: true
       name: your-existing-postgres-network-name
   ```

2. **Update `.env`**:

   ```env
   DB_URL=postgresql://user:password@your-postgres-hostname:5432/dbname
   DATABASE_URL=postgresql://user:password@your-postgres-hostname:5432/dbname
   ```

3. **Remove postgres from depends_on** in the backend service.

### Option 2: Connect via Host

1. **Edit `docker-compose.yml`** - Comment out the `postgres` service.

2. **Update `.env`**:

   ```env
   # For connection to host machine
   DB_URL=postgresql://user:password@host.docker.internal:5432/dbname
   DATABASE_URL=postgresql://user:password@host.docker.internal:5432/dbname
   ```

## Troubleshooting

### Port Already in Use

**Error**: `Bind for 0.0.0.0:12000 failed: port is already allocated`

**Solution**: Change the port mapping in `docker-compose.yml`:

```yaml
ports:
  - '12001:12000' # Maps host port 12001 to container port 12000
```

### Database Connection Failed

**Error**: `Failed to connect to the database`

**Solutions**:

1. Check PostgreSQL is running: `docker-compose ps postgres`
2. Verify credentials in `.env` file
3. Check database logs: `docker-compose logs postgres`
4. Ensure database is healthy: `docker-compose exec postgres pg_isready -U postgres`

### Build Failures

**Frontend build fails**:

```bash
# Clear build cache and rebuild
docker-compose build --no-cache frontend
```

**Backend build fails** (Rust compilation):

```bash
# Clear build cache
docker-compose build --no-cache backend

# Check for disk space
df -h
```

### Container Crashes

View logs to identify the issue:

```bash
docker-compose logs backend
```

Common issues:

- Missing environment variables
- Invalid R2 credentials
- Port conflicts
- Insufficient memory

### Health Check Failures

If health checks continuously fail:

```bash
# Check if service is responding
docker-compose exec backend curl -f http://localhost:16000/health

# Check service logs
docker-compose logs backend

# Temporarily disable health check (not recommended for production)
# Comment out the healthcheck section in docker-compose.yml
```

### Migration Errors

If migrations fail on startup:

```bash
# Run migrations manually with verbose output
docker-compose exec backend sqlx migrate run --source /app/migrations

# Check migration history
docker-compose exec backend sqlx migrate info --source /app/migrations

# Connect to database directly
docker-compose exec postgres psql -U postgres -d mydb
```

## Production Considerations

### Security

1. **Environment Variables**:
   - Never commit `.env` to version control
   - Use strong, unique secrets for `JWT_SECRET`
   - Use complex database passwords

2. **CORS Configuration**:

   ```env
   CORS_ORIGINS=https://yourdomain.com
   ```

3. **HTTPS**:
   - Use a reverse proxy (Nginx, Traefik, Caddy) for SSL/TLS
   - Configure SSL certificates (Let's Encrypt)

4. **Resource Limits**:
   Add resource constraints to `docker-compose.yml`:
   ```yaml
   deploy:
     resources:
       limits:
         cpus: '1'
         memory: 1G
       reservations:
         cpus: '0.5'
         memory: 512M
   ```

### Performance

1. **Database Optimization**:
   - Use connection pooling (already configured)
   - Regular backups
   - Monitor query performance

2. **Caching**:
   - Backend already has Moka cache configured
   - Consider adding Redis for distributed caching

3. **Monitoring**:

   ```bash
   # Monitor resource usage
   docker stats

   # Monitor logs
   docker-compose logs -f --tail=100
   ```

### Backups

**Database Backup**:

```bash
# Create backup
docker-compose exec postgres pg_dump -U postgres mydb > backup_$(date +%Y%m%d_%H%M%S).sql

# Restore backup
docker-compose exec -T postgres psql -U postgres mydb < backup.sql
```

**Volume Backup**:

```bash
# Backup postgres data volume
docker run --rm \
  -v tambahkan-ini-ke-database-my-repo_postgres-data:/data \
  -v $(pwd):/backup \
  alpine tar czf /backup/postgres-data-backup.tar.gz -C /data .
```

### Scaling

For horizontal scaling, consider:

- Load balancer in front of multiple backend instances
- Shared PostgreSQL instance
- Shared cache (Redis)
- Container orchestration (Kubernetes, Docker Swarm)

## Additional Resources

- [Docker Documentation](https://docs.docker.com/)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [PostgreSQL Docker Hub](https://hub.docker.com/_/postgres)
- [Nginx Docker Hub](https://hub.docker.com/_/nginx)

## Support

For issues or questions:

1. Check the [Troubleshooting](#troubleshooting) section
2. Review application logs: `docker-compose logs`
3. Check Docker daemon logs: `docker logs`
4. Verify system resources: `docker system df`
