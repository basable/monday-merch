# design

Owns each company's brand assets and the proofs consultants prepare for quotes.

Design: `docs/01-enterprise-merch/02-design.md`. Code:
`nanoservices/design/` (read its `AGENTS.md` first).

## Owns

- Table `brand_profile` (`company_id`, `colours`, `fonts`, `guidelines`, `updated_at`).
- Table `brand_asset` (`id`, `company_id`, `owner_identity_id`, `kind`, `filename`, `object_key`, `mime_type`, `size_bytes`, `created_at`, `confirmed_at`, `deleted_at`).
- Table `proof` (`id`, `subject_name`, `line_ref`, `version`, `preview_key`, `artwork_key`, `print_spec`, `created_by`, `created_at`, `approved_at`, `approved_by`, `source_design_version_id`).
- Table `design_document` (`id`, `company_id`, `owner_identity_id`, `product_id`, `variant_id`, `name`, `draft`, `schema_version`, `revision`, `archived_at`, `created_at`, `updated_at`).
- Table `design_version` (`id`, `design_id`, `version`, `document`, `schema_version`, `preview_keys`, `created_by`, `created_at`).
- Table `design_template` (`id`, `slug`, `title`, `kind`, `product_id`, `document`, `schema_version`, `preview_key`, `published_at`, `sort_order`).
- Config type `studio_font`.
- External call `delete_object` on object_storage: `idempotent`.
- Schedule `purge_orphan_uploads` every 24h.

## Messages

- Handles: `GetBrandProfileRequest`, `UpdateBrandProfileRequest`, `ListBrandAssetsRequest`, `CreateAssetUploadRequest`, `ConfirmAssetUploadRequest`, `DeleteBrandAssetRequest`, `CreateProofRequest`, `ListProofsRequest`, `ApproveProofsRequest`, `GetApprovedArtworkRequest`, `CreateDesignRequest`, `GetDesignRequest`, `ListDesignsRequest`, `SaveDesignDraftRequest`, `SaveDesignVersionRequest`, `ArchiveDesignRequest`, `CreatePreviewUploadRequest`, `ListDesignTemplatesRequest`, `GetDesignTemplateRequest`, `UpsertDesignTemplateRequest`, `ResolveDesignVersionRequest`, `AdoptDesignsRequest`.
- Sends: `ResolveMemberRequest`, `AuditEvent`.

## Directive sections that bind

§2 always; §6 for each adapter; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_get_brand_profile_request` in `src/handlers.rs`.
- [ ] `handle_update_brand_profile_request` in `src/handlers.rs`.
- [ ] `handle_list_brand_assets_request` in `src/handlers.rs`.
- [ ] `handle_create_asset_upload_request` in `src/handlers.rs`.
- [ ] `handle_confirm_asset_upload_request` in `src/handlers.rs`.
- [ ] `handle_delete_brand_asset_request` in `src/handlers.rs`.
- [ ] `handle_create_proof_request` in `src/handlers.rs`.
- [ ] `handle_list_proofs_request` in `src/handlers.rs`.
- [ ] `handle_approve_proofs_request` in `src/handlers.rs`.
- [ ] `handle_get_approved_artwork_request` in `src/handlers.rs`.
- [ ] `handle_create_design_request` in `src/handlers.rs`.
- [ ] `handle_get_design_request` in `src/handlers.rs`.
- [ ] `handle_list_designs_request` in `src/handlers.rs`.
- [ ] `handle_save_design_draft_request` in `src/handlers.rs`.
- [ ] `handle_save_design_version_request` in `src/handlers.rs`.
- [ ] `handle_archive_design_request` in `src/handlers.rs`.
- [ ] `handle_create_preview_upload_request` in `src/handlers.rs`.
- [ ] `handle_list_design_templates_request` in `src/handlers.rs`.
- [ ] `handle_get_design_template_request` in `src/handlers.rs`.
- [ ] `handle_upsert_design_template_request` in `src/handlers.rs`.
- [ ] `handle_resolve_design_version_request` in `src/handlers.rs`.
- [ ] `handle_adopt_designs_request` in `src/handlers.rs`.
- [ ] Table `brand_profile`: the queries in `repository.rs`.
- [ ] Table `brand_asset`: the queries in `repository.rs`.
- [ ] Table `proof`: the queries in `repository.rs`.
- [ ] Table `design_document`: the queries in `repository.rs`.
- [ ] Table `design_version`: the queries in `repository.rs`.
- [ ] Table `design_template`: the queries in `repository.rs`.
- [ ] `send` for `delete_object` in `effects.rs`, the real client in `provider.rs`; the audit stays green.
- [ ] `tick_purge_orphan_uploads` in `worker.rs`.
- [ ] `DesignService` field mapping in `api/src/services/design.rs` and the messages in `proto/design/v1/design.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.
