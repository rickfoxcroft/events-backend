# Event Venue Backend

Backend for an event venue search and booking platform, built with Rust and Cloudflare Workers.

## Features
- **Cloudflare Workers**: Serverless backend.
- **Rust**: High performance and safety.
- **D1 Database**: Cloudflare's serverless SQL database.
- **OpenAPI**: API documentation in `models/`.

## Prerequisites
- [Rust](https://www.rust-lang.org/)
- [Wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- `worker-build`: `cargo install worker-build`

## Development
1. Install dependencies:
   ```bash
   cargo build
   ```
2. Run locally:
   ```bash
   wrangler dev
   ```

## Database Setup
1. Create a D1 database:
   ```bash
   wrangler d1 create event-app-db
   ```
2. Update `wrangler.toml` with the `database_id`.
3. Apply migrations:
   ```bash
   wrangler d1 execute event-app-db --file=schema.sql
   ```

## Deployment
Automated via GitHub Actions on push to `main`.
Ensure the following secrets are set in your GitHub repository:
- `CLOUDFLARE_API_TOKEN`
- `CLOUDFLARE_ACCOUNT_ID`
