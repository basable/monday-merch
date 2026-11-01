# design

Owns each company's brand assets and the proofs consultants prepare for quotes.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §6 (external-effect admission — it owns adapters), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

Plain tables (schema `nano_design`): `brand_profile`, `brand_asset`, `proof`, `design_document`, `design_version`, `design_template`.

Config catalogs: `studio_font` (`config/base/studio_font.json`).

| External call | System | Strategy | Irreversible |
|---|---|---|---|
| `delete_object` | object_storage | `idempotent` | no |

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Design` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl DesignHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/effects.rs` | One `Call` per external call with its strategy literal; `send` is the only dispatch path. |
| `src/provider.rs` / `src/simulator.rs` | The provider trait, the HTTP client (a parsed 4xx is `SendError::Refused`), the noop; the in-memory simulator the audit and the tests drive. |
| `tests/effects_audit.rs` | One `audit!` per adapter — the audit index. Green on day one; the first thing a real provider must keep green. |
| `src/worker.rs` | The ticker workers (`purge_orphan_uploads` every 24h), listed in this nanoservice's `loops()` (`src/lib.rs`), started by the app and joined on shutdown. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `GetBrandProfileRequest` → `BrandProfile`, `UpdateBrandProfileRequest` → `BrandProfile`, `ListBrandAssetsRequest` → `BrandAssetList`, `CreateAssetUploadRequest` → `AssetUpload`, `ConfirmAssetUploadRequest` → `BrandAsset`, `DeleteBrandAssetRequest` → `BrandAsset`, `CreateProofRequest` → `Proof`, `ListProofsRequest` → `ProofList`, `ApproveProofsRequest` → `ProofApproval`, `GetApprovedArtworkRequest` → `ArtworkBundle`, `CreateDesignRequest` → `DesignDocument`, `GetDesignRequest` → `DesignDocument`, `ListDesignsRequest` → `DesignList`, `SaveDesignDraftRequest` → `DesignDocument`, `SaveDesignVersionRequest` → `DesignVersion`, `ArchiveDesignRequest` → `DesignDocument`, `CreatePreviewUploadRequest` → `PreviewUpload`, `ListDesignTemplatesRequest` → `DesignTemplateList`, `GetDesignTemplateRequest` → `DesignTemplate`, `UpsertDesignTemplateRequest` → `DesignTemplate`, `ResolveDesignVersionRequest` → `DesignVersionRef`, `AdoptDesignsRequest` → `DesignAdoption`.
Sends: `ResolveMemberRequest`, `AuditEvent`.

API: `DesignService` in `api/src/services/design.rs` (GetBrandProfile, UpdateBrandProfile, ListBrandAssets, CreateAssetUpload, ConfirmAssetUpload, DeleteBrandAsset, CreateProof, ListProofs, CreateDesign, GetDesign, ListDesigns, SaveDesignDraft, SaveDesignVersion, ArchiveDesign, CreatePreviewUpload, ListDesignTemplates, GetDesignTemplate, UpsertDesignTemplate).
