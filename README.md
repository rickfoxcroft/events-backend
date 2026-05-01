# Event Venue Backend

Backend for an event venue search and booking platform, built with Rust and Cloudflare Workers.

## Workflow: Contract-First & Database-First

This project uses a "Bridge" pattern to separate API contracts from database storage:

1. **Contract (`api.tsp`)**: Defined using [TypeSpec](https://typespec.io/). This generates the `RustDTOs`.
2. **Storage (`schema.sql`)**: Defined using SQL for Cloudflare D1. This generates the `RustEntities`.
3. **The Bridge**: In `src/models/dtos.rs`, we implement `impl From<Entity> for DTO` to map database models to API responses.

## Features

- **Cloudflare Workers**: Serverless backend.
- **Rust**: High performance and safety.
- **D1 Database**: Cloudflare's serverless SQL database.
- **TypeSpec**: API contract definition.

## Prerequisites

- [mise](https://mise.jdx.dev/): The project uses `mise` to automatically manage tool versions (Rust, Node, Wrangler, etc.).

## Local Development

1. **Install tools and dependencies**:

    ```bash
    mise install
    ```

2. **Generate code from API contract**:

    ```bash
    mise run codegen
    ```

3. **Run tests**:

    ```bash
    mise run test
    ```

4. **Run the Worker locally**:

    ```bash
    mise run dev
    ```

## Database Migrations (D1)

To manage your database schema and migrations:

1. **Apply Migrations Locally**:

    ```bash
    mise run db:migrate:local
    ```

2. **Apply Migrations to Production**:

    ```bash
    mise run db:migrate:remote
    ```

> **Note on `wrangler types`**: While `npx wrangler types` is great for generating TypeScript definitions from your bindings, in this Rust project, we manually define our `Entities` in `src/models/entities.rs` to match the D1 schema, ensuring full control over Rust's type system and serialization.

## Project Structure

- `api.tsp`: API contract (TypeSpec).
- `schema.sql`: Initial database schema.
- `src/models/entities.rs`: Database models (Entities).
- `src/models/dtos.rs`: API Data Transfer Objects (DTOs) and conversion logic.
- `src/lib.rs`: Worker entry point and routing.

## License

Proprietary. Copyright (c) 2026 Rick Foxcroft. All rights reserved. See `LICENSE` for more details.
