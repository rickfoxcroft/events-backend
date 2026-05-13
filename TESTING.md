# Testing Strategy

This project uses a multi-layered testing strategy to ensure the reliability and correctness of the Event Venue Backend.

## 1. Unit Tests (Rust)

Unit tests focus on individual components, such as services and adapters, using in-memory mocks.

*   **Location**: Defined within the source files (e.g., `src/services/venue.rs`).
*   **Mocks**: Uses `MockVenueRepository` and `MockImageStorage`.
*   **Run**:
    ```bash
    mise run test
    ```

## 2. End-to-End (E2E) Tests (Cucumber)

Acceptance tests defined in Gherkin that verify business requirements by making real HTTP requests against a running instance of the application.

*   **Features**: `features/*.feature`
*   **Step Definitions**: `tests/*.rs`
*   **Requirements**: Requires the local dev server to be running (see below).
*   **Run**:
    ```bash
    # 1. Start dev server in one terminal
    ENVIRONMENT=local mise run dev

    # 2. Run tests in another terminal
    cargo test --test venue_fetching
    cargo test --test venue_listing
    ```
*   **Configuration**: You can point the tests to a different URL using the `TEST_API_URL` environment variable (defaults to `http://127.0.0.1:8787`).

## 3. Local Development (Miniflare)

Test the full application stack locally using Cloudflare's emulation engine.

### Prerequisites

Ensure you have initialized your local database:
```bash
mise run db:migrate:local
```

### Running the Dev Server

Start the worker in "local" mode to enable offline features (like local image storage):
```bash
ENVIRONMENT=local mise run dev
```

### Offline Image Uploads

When `ENVIRONMENT=local` is set, the worker uses `LocalImageStorage`. This adapter:
1.  Returns local upload URLs (e.g., `http://localhost:8787/local-storage/upload/...`).
2.  Allows you to test the full "Upload then Attach" workflow without a Cloudflare API token.

## 4. Continuous Integration (CI)

Our GitHub Actions workflow (`.github/workflows/ci-cd.yml`) automatically runs the following checks on every PR:

1.  **Setup**: Installs dependencies.
2.  **Codegen Check**: Verifies that `src/models/dtos.rs` is in sync with `api.tsp`.
3.  **Linting**: Runs `cargo clippy` and `cargo fmt`.
4.  **Tests**: 
    *   Runs unit tests.
    *   Starts a local Miniflare instance and runs the Cucumber E2E tests against it.

---

### Pro-Tip: Manual Testing with Bruno

For manual exploration, use the [Bruno](https://www.usebruno.com/) collection in the `bruno/` directory. It is pre-configured with `Local` and `Production` environments.
