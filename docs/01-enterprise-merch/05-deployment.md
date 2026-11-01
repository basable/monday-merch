# 05: Deployment

## Sizing

Requests against the 2 vCPU, 4 Gi, 10 Gi namespace bundle:

| Workload | Count | CPU each | Memory each | Total CPU | Total memory |
| --- | --- | --- | --- | --- | --- |
| App | 2 | 250m | 512Mi | 500m | 1024Mi |
| Postgres (CNPG, `instances: 1`) | 1 | 500m | 1024Mi | 500m | 1024Mi |
| CNPG operator | 1 | 100m | 256Mi | 100m | 256Mi |
| Kratos public and admin | 2 | 100m | 128Mi | 200m | 256Mi |
| **Total** | | | | **1300m** | **2560Mi** |

That leaves 700m and about 1.5 Gi for migration Jobs and rollouts, which run briefly. These are starting values to correct from `query_metrics` in preview. The next size up is 4 vCPU and 8 Gi, since memory must be 2 Gi per vCPU. Postgres gets 5 Gi of the 10 Gi storage, and all files go in buckets.

**Billing:** this project's free-tier window ended on 2026-10-11, so compute is billed from the first deploy, and the organisation has a payment method on file. Object storage is billed separately. Check the current rates before the first deploy. Plan commits deploy nothing.

## `.basable/config.yaml`

Rendered by the scaffold, then extended with the blocks below.

```yaml
apiVersion: basable.com/v1
kind: TenantConfig
spec:
  environments:
    preview:
      virtual-cluster: { cpu: "2000m", memory: "4Gi", storage: "10Gi", maxPods: 30 }
      objectStorage: {}
    live:
      virtual-cluster: { cpu: "2000m", memory: "4Gi", storage: "10Gi", maxPods: 30 }
      objectStorage: {}
      alerts:
        email: { enabled: true }
```

`objectStorage` is needed in both environments, for the site, for files and for backups.

## Buckets

Both buckets are shared by key prefix, with one owner per prefix.

| Bucket | Prefix | Owner | Contents |
| --- | --- | --- | --- |
| Public | site (platform-managed) | Platform | The frontend. Never written by the app. |
| Public | `catalog/` | `catalog` | Product images |
| Public | `studio/` | `design` | Template previews and fonts |
| Private | `design/` | `design` | Brand assets, proofs, artwork |
| Private | `billing/invoices/` | `billing` | Invoice and credit note PDFs |
| Private | `inbox/` | `inbox` | Received emails and attachments |
| Private | `cnpg/` | Platform backup | Postgres base backups and WAL |

## Secrets

| Secret | Kind | Notes |
| --- | --- | --- |
| `app-db-app`, Kratos database secret | CNPG-minted | Never supplied by hand |
| `object-storage` | Platform-minted | Read with `envFrom` |
| Kratos cookie and cipher secrets | Generated in the cluster | Not user secrets |
| `KRATOS_COURIER_SMTP_URI` | **User, in `app-secrets`** | Required before `basableLogin` is turned off, or nobody can sign in |
| `RESEND_API_KEY` | **User** | Sends mail and reads received mail |
| `RESEND_WEBHOOK_SECRET` | **User** | Verifies the inbound webhook |
| `ANTHROPIC_API_KEY` | **User** | Document extraction |
| `STRIPE_SECRET_KEY` | **User** | A restricted key (`rk_`). A separate Stripe sandbox for preview. |
| `STRIPE_WEBHOOK_SECRET` | **User** | Per environment |
| `CARRIER_API_KEY`, `CARRIER_WEBHOOK_SECRET` | **User** | Named once the provider is chosen in round 3 |

Each is read with its own `secretKeyRef`. When a reference is added, the deploy waits until its value is entered through `request_secrets`, so **references are added with the code that needs them, not before**. That keeps the first preview from waiting on keys. The values never go in the repository or in chat.

## Database

One CNPG cluster. One application database with one schema and role per stateful nanoservice (`account`, `catalog`, `design`, `order`, `fulfilment`, `billing`, `notifier` for its config, `inbox`, plus the framework's), and a separate `kratos` database. Migrations run as a dbmate-format Job in an earlier apply wave. The app verifies the migration head at boot.

**Backups are required before real data exists.** Not covered by the scaffold, so add later via the agent: `backup.barmanObjectStore` on the Cluster, into the private bucket, retention `14d`, plus a daily `ScheduledBackup`. Run a restore drill before launch, because an untested backup is not a backup. Invoices have long legal retention, so an accounting export is the long-term record, and backups are for disaster recovery.

## Deviations from the standard shape

1. **`/shop` is routed to the binary** beside `/api`, through the same HTTPRoute, which still never claims `/`. Reason: catalogue pages must be real documents that change without a deploy, and the bucket serves only the deployed commit's files (`02-catalog.md`). No nginx container is added. The route and raw handler are added by the agent after the scaffold, and the binary sends the two required security headers on these pages.

Standard otherwise: two replicas, frontend from the public bucket, CNPG, Kratos, tickers as workers, webhooks as raw routes.

## Before customers arrive

- Turn `basableLogin` off, with `KRATOS_COURIER_SMTP_URI` set (`01-architecture.md`).
- Replace the placeholder product name, and bind a custom domain.
- Backups running and a restore tested.
- Anthropic and Resend processing terms confirmed for this data.

## Bucket-hosting limits to design around

- An unknown path answers the root `index.html` with status 200. The React apps therefore use hash routes. Unknown marketing URLs return the homepage instead of a 404, which is acceptable at launch. If search quality suffers, nginx is the documented fix.
- No redirects or custom headers on bucket-served pages.
- 16 MiB zipped, 64 MiB and 5,000 files unpacked for the site. Large images go in the bucket under `catalog/`.

## Environments

- `dev` → preview, `main` → live. Preview sleeps after 30 minutes without a deploy. While asleep, the static pages still answer but `/shop`, `/api` and webhooks do not. A Stripe or carrier event missed in preview is caught by the pass's polling after wake-up.
- Promotion to live only through `promote_to_live`, when asked.
- A custom domain is bound once the product has a name. Hostnames come from `TENANT_DOMAIN_PLACEHOLDER`, so it needs no repository change.

## Data protection

Before enterprise customers sign a data processing agreement:

- **Confirm where data lives.** Region selection is managed by the platform, so ask Basable to confirm the EU region and its subprocessors. Recipients' home addresses are personal data under GDPR.
- Retention is implemented through `purge_recipient_pii` and `purge_shipment_addresses`, and erased data persists in backups for up to the 14-day retention, which the privacy policy must state.
- Resend, Stripe and Anthropic become subprocessors. Emails and invoices go to Anthropic for extraction.

## CI and registry

GitHub variant: the scaffold renders `.github/workflows/` with lint, Bazel build and push to `registry.basable.com/abandon-above-trade/monday-merch`, then renders `k8s/` with built digests and uploads `rendered-manifests` and the `static` site artifact. `paths-ignore: ["docs/**"]` keeps plan commits from building. The repository's `basable-builder.yaml` is never touched.

## Not covered by the scaffold

Later, via the agent: Stripe, Resend, Anthropic and carrier integrations (the adapters and clients), the `/shop` route and templates, database backups, the Kratos SMTP courier setup, PDF rendering. WebSockets are not needed, since dashboards refresh on demand.
