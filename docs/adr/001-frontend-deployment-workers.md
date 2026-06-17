# ADR 001: Migrate Frontend Deployment from Cloudflare Pages to Cloudflare Workers (with Assets)

## Status
Accepted

## Context
The Event App frontend is built using Astro (v6+) configured for Server-Side Rendering (SSR) (`output: 'server'`) with the `@astrojs/cloudflare` adapter. 

Historically, Cloudflare offered two distinct hosting models:
1. **Cloudflare Pages**: Focused on static site hosting and Git integration, with dynamic features enabled via Pages Functions.
2. **Cloudflare Workers**: Focused on general-purpose serverless compute.

During deployment, we encountered persistent 404 errors and "No Functions Configured" errors because our deployment pipeline used `npx wrangler pages deploy dist`, which uploaded our build output folder (`dist`) as a static Pages site. Since Astro v6's Cloudflare adapter outputs serverless entrypoints (`dist/server/`) and assets (`dist/client/`) optimized for Workers, deploying via Pages ignored the server-side code completely.

Cloudflare is currently converging both platforms into a unified developer platform: **Workers with Assets**.

## Decision
We will migrate the frontend deployment from Cloudflare Pages to **Cloudflare Workers (with Assets)**.

This decision entails:
* Updating our local and CI/CD deployment tasks to use `npx wrangler deploy --config dist/server/wrangler.json` instead of `npx wrangler pages deploy`.
* Managing environment variables (like `PUBLIC_API_URL`) at build-time in GitHub Actions to ensure they are statically compiled into the client bundle.
* Using the Workers subdomain `https://event-app-web.warwickfoxcroft.workers.dev` (or a custom domain bound to the Worker) instead of the `pages.dev` subdomain.

## Comparison (Workers vs. Pages)

| Metric / Feature | Cloudflare Pages (Legacy) | Cloudflare Workers with Assets (Unified) |
| :--- | :--- | :--- |
| **Astro v6+ Alignment** | Deprecated / Legacy wrappers | **Recommended primary target** |
| **Local Parity** | Different runtime wrapper environment | Exact same `workerd` runtime as production |
| **Static + SSR Integration** | Binds assets to Pages Functions | Native unified config (`wrangler.json`) |
| **Feature Access** | Limited/wrapped access to KV, D1, etc. | Direct native bindings for all Cloudflare services |
| **Deployment CI/CD** | Automatic zero-config Git integration | Driven via custom GitHub Actions workflow (`wrangler deploy`) |

## Consequences
* **URL Change**: The application is now served from the Workers subdomain: `https://event-app-web.warwickfoxcroft.workers.dev/`. Any DNS or custom domain configurations must be updated to route traffic to the Worker instead of the Pages project.
* **CD Pipeline**: GitHub Actions now handles the deployment of both static assets and server code in a single unified step. We must ensure the `CLOUDFLARE_API_TOKEN` secret in GitHub has the necessary `Workers Scripts: Edit` permissions.
