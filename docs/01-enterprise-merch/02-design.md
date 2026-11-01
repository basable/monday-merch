# 02: design

The Design Studio, brand assets and proofs. No lifecycle: saving a design is a request, nothing polls or drives another system on its behalf, and a proof's wait for a decision is already tracked by the quote. So this is a plain executor (Directive §1). Directive §2 binds every table, §6 the call, §7 every message.

**The user included the Design Studio.** Customers design in the browser, and the editor is why `project.frontend` is `react`. All editing and rendering happens in the browser, because media processing does not belong in this binary.

## How the Studio fits the order flow

**A Studio design does not go straight to production.** It is the customer's input to a quote: a designer checks it and produces the print-ready `proof`, and signing the quote approves that proof, as for any other order. The rest of the system therefore works the same whether a design came from the Studio or from a designer. Self-serve checkout remains deferred.

## What the Studio does

Text, images and shapes, with colour, size and position. Front, back, sleeve and label views, taken from the product's print areas. Cut guides and safe areas for print products. Starting from a template or a blank design. Brand colours, fonts and logos from the company's profile. A comparison of print methods with the cost of each, and a live price as size and quantity change, both from `PriceLines`.

File import in the core editor is SVG, PNG and JPG, which a browser can draw. PDF, AI and PSD files are still accepted as uploads for a designer to work from.

Asked separately (see `00-scope.md`): brand import from a name or website, 3D preview, Shuffle, Vectorize with AI, Figma and Canva import.

## Tables

**`design_document`** is one design being worked on: `company_id`, `owner_identity_id`, `product_id`, `variant_id`, `name`, `draft`, `schema_version`, `revision`, `archived_at`, `created_at`, `updated_at`.

**`design_version`** is an immutable snapshot, taken on an explicit save or when a design joins a quote: `design_id`, `version`, `document`, `schema_version`, `preview_keys`, `created_by`, `created_at`.

**`design_template`** is a starting point: `slug`, `title`, `kind` (`apparel`, `card`, `poster`, `flyer`), `product_id`, `document`, `schema_version`, `preview_key`, `published_at`, `sort_order`. It is a plain table, not a config catalog, because designers publish templates from the Studio and should not need an engineer and a deploy. Templates must be the competitor's own work.

**`brand_profile`** is one row per company (`company_id`): `colours`, `fonts`, `guidelines`, `updated_at`.

**`brand_asset`** is an uploaded logo, font or image: `company_id`, `owner_identity_id`, `kind`, `filename`, `object_key`, `mime_type`, `size_bytes`, `created_at`, `confirmed_at`, `deleted_at`. A row is created with the presigned URL and confirmed once a HEAD shows the object exists and fits the limits.

**`proof`** is one version of a design for one quote line: `subject_name` (the caller's name, `quote:<id>`), `line_ref`, `version`, `preview_key`, `artwork_key`, `print_spec` (method, area, colours, size), `created_by`, `created_at`, `approved_at`, `approved_by`, `source_design_version_id` (the Studio version it came from, if any).

## The design document

`draft` and `document` hold **this project's own JSON schema**, versioned by `schema_version`, not a canvas library's internal format. Layers are positioned per print area in physical millimetres, so a design survives a library change and can be read by a designer. Each file stays under 1 MiB, because images are references to assets and are never embedded.

## Invariants

- **Autosave is a compare-and-set.** `SaveDesignDraft` names the `revision` it started from and is refused when stale, so two tabs cannot silently overwrite each other. It does not create versions, which keeps autosave cheap.
- A `design_version` never changes.
- **The document is data.** The server validates it against the schema, and it may reference only the caller's assets, the company's assets or public files.
- A proof version never changes. A correction is a new `version`.
- `approved_at` is one-way. `ApproveProofsRequest` names exact versions and is idempotent, so a retry changes nothing.
- A quote can only be signed if each line has a proof. `order` checks this before signing.
- Customers see proofs through `order` (`GetQuote` authorises, then calls `ListProofsRequest`). This nanoservice therefore never decides who may see a quote.

## Who can use it

Anyone can open the Studio, read templates and products, and design. Those reads are public, and an anonymous draft stays in the browser. **Saving requires signing in** with a verified email, so strangers cannot use the buckets as free storage.

A signed-in person with no company yet owns their designs through `owner_identity_id`. Within a company, members see the company's designs. When a signature creates the company, `AdoptDesignsRequest` moves that person's designs and assets to it, idempotently.

"Add to quote" with a design sends `ResolveDesignVersionRequest` from `order`, which checks ownership, takes a snapshot and returns a reference for the quote line.

## Uploads and buckets

**Private bucket:** `design/assets/<owner>/...`, `design/proofs/<subject>/...`, `design/previews/<design>/...`. The browser uploads with a presigned PUT moved onto `S3_PRIVATE_URL`, and reads with a short-lived presigned GET.

**Public bucket:** `studio/` for template previews and font files from `studio_font`. Fonts must be licensed for this use, and the catalog records each licence.

Previews are rendered by the browser's canvas and uploaded through `CreatePreviewUpload`. Images drawn on that canvas load over CORS, or the canvas could not export them. The platform allows the environment's own hostnames, so check this again when a custom domain is bound.

Accepted upload types: SVG, PDF, AI, PSD, PNG, JPG, capped at 100 MiB. These files come from strangers and designers open them on their own machines, so a download link is presigned with `Content-Disposition: attachment`. Uploaded SVG is only ever drawn through an image element or canvas, where its scripts cannot run, and is never opened as a document on the site's origin.

## External calls

**`delete_object`** on `object_storage`, strategy `idempotent`: deleting an absent object succeeds, so a retry is safe. The simulator models delete-then-delete and ack loss. Presigning sends nothing and HEAD is a read, so neither is an adapter (Directive §6).

## Schedules

`purge_orphan_uploads` runs every 24h. It finds `brand_asset` rows older than 24h with no `confirmed_at`, deletes the object and then the row. It reads the table and never lists the bucket, because bucket listings cost more.

## Config types

`studio_font`. See `04-config.md`.

## Messages and API

`DesignService`, public: `ListDesignTemplates`, `GetDesignTemplate`. Signed in: `CreateDesign`, `GetDesign`, `ListDesigns`, `SaveDesignDraft`, `SaveDesignVersion`, `ArchiveDesign`, `CreatePreviewUpload`, `GetBrandProfile`, `UpdateBrandProfile`, `ListBrandAssets`, `CreateAssetUpload`, `ConfirmAssetUpload`, `DeleteBrandAsset`. Staff: `UpsertDesignTemplate`, `CreateProof`, `ListProofs`.

Internal, from `order`: `ListProofsRequest`, `ApproveProofsRequest`, `GetApprovedArtworkRequest` (presigned artwork links for a purchase order, valid 7 days), `ResolveDesignVersionRequest`, `AdoptDesignsRequest`.

## Binding Directive sections

§1, §2 (tables), §6 (`delete_object`), §7 (messages).

## Tests

- `effecttest` audit for `delete_object` in `tests/effects_audit.rs`.
- A stale `SaveDesignDraft` is refused, and a current one advances `revision`.
- A document over the size cap, with an unknown schema version, or referencing another company's asset is refused.
- A person with no company can save, and `AdoptDesignsRequest` twice moves everything once.
- Company A cannot read, change or quote company B's designs or assets.
- `ResolveDesignVersionRequest` refuses a design the caller does not own.
- `ConfirmAssetUpload` refuses a missing object, one over the limit and a wrong type.
- `ApproveProofsRequest` twice leaves one approval, and a proof version cannot be overwritten.
- `purge_orphan_uploads` keeps confirmed assets and removes only stale unconfirmed ones.
- Browser: open a product, add text, autosave, reload, compare print methods, add to quote. The preview canvas exports with images from the buckets.
