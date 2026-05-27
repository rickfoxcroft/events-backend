# Project Instructions (GEMINI.md)

This file contains repository-specific instructions, architecture rules, and workflows for the Event App project.

## Project Overview

- **Stack**: Rust, Cloudflare Workers, D1 Database, TypeSpec (for API contracts).
- **Pattern**: Bridge pattern separating API DTOs (generated from TypeSpec) from Database Entities (SQL-first).

## Architecture Rules

- Always use `impl From<Entity> for DTO` in `src/models/dtos.rs` for data mapping.
- Maintain strict type safety; avoid `any` or unnecessary `unwrap()` calls in Rust.
- Follow the "Contract-First" approach: update `api.tsp` before modifying DTOs or Frontend types.
- **Frontend Validation**: Use auto-generated Zod schemas (from `src/types/api-zod.ts`) at API boundaries to ensure runtime parity with the backend.

## UI & Styling (Open Design)

... (rest of the section)

## Local Development & Cloudflare Images

This project uses real Cloudflare Images for local development to ensure parity with production. All images uploaded locally are automatically prefixed with `dev-` to keep them separate from production assets.

### Setup Requirements

To run the backend locally or execute integration tests that involve images, you must provide real Cloudflare credentials in `packages/backend/.dev.vars`:

```bash
CF_ACCOUNT_ID="your-account-id"
CF_IMAGES_API_TOKEN="your-api-token"
CF_IMAGES_ACCOUNT_HASH="your-account-hash"
```

The frontend application requires the backend API URL to be configured in `packages/web/.env.local` (local) or your deployment environment (production). This is validated at runtime via Zod in `packages/web/src/config.ts`:

```bash
PUBLIC_API_URL="http://localhost:8787"
```

_Note: The frontend will throw a Zod validation error if `PUBLIC_API_URL` is missing or invalid._

- **Unit Tests**: Use `MockImageStorage` and do not require external credentials.
- **Integration Tests**: Run against the local worker and **require** real Cloudflare credentials for any scenario involving image uploads.

## Available Skills

The following specialized skills are configured for this repository. Activate them when performing related tasks:

- **rust-best-practices**: Use this for any Rust-related development, refactoring, or performance optimization.
  - _Activation_: `activate_skill(name="rust-best-practices")`
- **cloudflare-workers-testing**: Use this when writing or fixing integration tests for Workers, D1, or KV.
  - _Activation_: `activate_skill(name="cloudflare-workers-testing")`
- **seo-checklist**: Use this to review public-facing page structure and metadata.
  - _Activation_: `activate_skill(name="seo-checklist")`
- **skill-open-design**: Use this when designing UI components, implementing animations, or auditing the visual polish of the Event App website.
  - _Activation_: `activate_skill(name="skill-open-design")`

## Workflows

- **Codegen**: Run `mise run codegen` after changing `api.tsp`. This generates:
  - Rust DTOs
  - Frontend TypeScript types (`src/types/api.d.ts`)
  - **Frontend Zod schemas** (`src/types/api-zod.ts`) via a custom template (`zod-template.hbs`) to ensure zero runtime dependencies other than Zod.
- **Database**: Use `mise run db:migrate:local` for schema changes.
- **Testing**: Run `mise run test` to verify logic across the stack.
