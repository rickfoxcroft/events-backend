# Event Venue Backend

Backend for an event venue search and booking platform, built with Rust and Cloudflare Workers.

## Workflow: Contract-First & Database-First
This project uses a "Bridge" pattern to separate API contracts from database storage:

1.  **Contract (`api.tsp`)**: Defined using [TypeSpec](https://typespec.io/). This generates the `RustDTOs`.
2.  **Storage (`schema.sql`)**: Defined using SQL for Cloudflare D1. This generates the `RustEntities`.
3.  **The Bridge**: In `src/models/dtos.rs`, we implement `impl From<Entity> for DTO` to map database models to API responses.

## Features
- **Cloudflare Workers**: Serverless backend.
- **Rust**: High performance and safety.
- **D1 Database**: Cloudflare's serverless SQL database.
- **TypeSpec**: API contract definition.

## Prerequisites
- [Rust](https://www.rust-lang.org/)
- [Node.js & npm](https://nodejs.org/)
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/install-and-update/): `npm install -g wrangler`
- [TypeSpec Compiler](https://typespec.io/docs/installation): `npm install -g @typespec/compiler`
- `worker-build`: `cargo install worker-build`

## Local Development
1. **Install Rust dependencies**:
   ```bash
   cargo build
   ```
2. **Run the Worker locally**:
   ```bash
   npx wrangler dev
   ```

## Database Migrations (D1)
To manage your database schema and migrations:

1.  **Initialize Migrations**:
    ```bash
    npx wrangler d1 migrations create event-app-db initial_schema
    ```
2.  **Apply Migrations Locally**:
    ```bash
    npx wrangler d1 migrations apply event-app-db --local
    ```
3.  **Apply Migrations to Production**:
    ```bash
    npx wrangler d1 migrations apply event-app-db --remote
    ```

> **Note on `wrangler types`**: While `npx wrangler types` is great for generating TypeScript definitions from your bindings, in this Rust project, we manually define our `Entities` in `src/models/entities.rs` to match the D1 schema, ensuring full control over Rust's type system and serialization.

## Project Structure
- `api.tsp`: API contract (TypeSpec).
- `schema.sql`: Initial database schema.
- `src/models/entities.rs`: Database models (Entities).
- `src/models/dtos.rs`: API Data Transfer Objects (DTOs) and conversion logic.
- `src/lib.rs`: Worker entry point and routing.
