# Project Instructions (GEMINI.md)

This file contains repository-specific instructions, architecture rules, and workflows for the Event App project.

## Project Overview

- **Stack**: Rust, Cloudflare Workers, D1 Database, TypeSpec (for API contracts).
- **Pattern**: Bridge pattern separating API DTOs (generated from TypeSpec) from Database Entities (SQL-first).

## Architecture Rules

- Always use `impl From<Entity> for DTO` in `src/models/dtos.rs` for data mapping.
- Maintain strict type safety; avoid `any` or unnecessary `unwrap()` calls in Rust.
- Follow the "Contract-First" approach: update `api.tsp` before modifying DTOs.

## Available Skills

The following specialized skills are configured for this repository. Activate them when performing related tasks:

- **rust-best-practices**: Use this for any Rust-related development, refactoring, or performance optimization.
  - _Activation_: `activate_skill(name="rust-best-practices")`
- **cloudflare-workers-testing**: Use this when writing or fixing integration tests for Workers, D1, or KV.
  - _Activation_: `activate_skill(name="cloudflare-workers-testing")`
- **seo-checklist**: Use this to review public-facing page structure and metadata.
  - _Activation_: `activate_skill(name="seo-checklist")`

## Workflows

- **Codegen**: Run `mise run codegen` after changing `api.tsp`.
- **Database**: Use `mise run db:migrate:local` for schema changes.
- **Testing**: Run `mise run test` to verify logic across the stack.
