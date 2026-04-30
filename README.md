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

## Project Structure
- `api.tsp`: API contract (TypeSpec).
- `schema.sql`: Database schema (D1).
- `src/models/entities.rs`: Database models (Entities).
- `src/models/dtos.rs`: API Data Transfer Objects (DTOs) and conversion logic.
- `src/lib.rs`: Worker entry point and routing.
