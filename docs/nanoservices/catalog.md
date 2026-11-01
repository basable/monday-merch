# catalog

Owns products, variants, list prices and decoration options, and renders the public shop pages.

Design: `docs/01-enterprise-merch/02-catalog.md`. Code:
`nanoservices/catalog/` (read its `AGENTS.md` first).

## Owns

- Table `product` (`id`, `slug`, `name`, `description`, `category_slug`, `brand_slug`, `collections`, `attributes`, `impact_note`, `sustainability_labels`, `min_qty`, `production_days`, `customisable`, `best_selling`, `is_new`, `weight_grams`, `hs_code`, `origin_country`, `published_at`, `archived_at`, `search_text`).
- Table `variant` (`id`, `product_id`, `colour_slug`, `sku`, `sizes`, `archived_at`).
- Table `price_tier` (`id`, `product_id`, `currency`, `min_qty`, `unit_minor`).
- Table `decoration_option` (`id`, `product_id`, `print_method_slug`, `areas`, `currency`, `setup_minor`, `per_unit_minor`).
- Table `product_image` (`id`, `product_id`, `variant_id`, `object_key`, `alt_text`, `position`, `view`).
- Table `supplier_offer` (`id`, `product_id`, `supplier_slug`, `supplier_sku`, `currency`, `cost_minor`, `lead_days`, `preferred`).
- Config type `category`.
- Config type `collection`.
- Config type `brand`.
- Config type `colour`.
- Config type `print_method`.
- Config type `supplier`.
- Config type `pricing_policy`.

## Messages

- Handles: `ListProductsRequest`, `GetProductRequest`, `ListCategoriesRequest`, `PriceLinesRequest`, `UpsertProductRequest`, `UpsertVariantRequest`, `SetPriceTiersRequest`, `SetDecorationOptionsRequest`, `SetSupplierOffersRequest`, `CreateImageUploadRequest`, `ArchiveProductRequest`, `GetSupplierOffersRequest`, `GetCustomsDataRequest`, `MatchSupplierRequest`, `RenderShopPageRequest`.
- Sends: `ResolveMemberRequest`, `AuditEvent`.

## Directive sections that bind

§2 always; §7 for each message sent; §10 for the docs.

## TODO, in order

- [ ] `handle_list_products_request` in `src/handlers.rs`.
- [ ] `handle_get_product_request` in `src/handlers.rs`.
- [ ] `handle_list_categories_request` in `src/handlers.rs`.
- [ ] `handle_price_lines_request` in `src/handlers.rs`.
- [ ] `handle_upsert_product_request` in `src/handlers.rs`.
- [ ] `handle_upsert_variant_request` in `src/handlers.rs`.
- [ ] `handle_set_price_tiers_request` in `src/handlers.rs`.
- [ ] `handle_set_decoration_options_request` in `src/handlers.rs`.
- [ ] `handle_set_supplier_offers_request` in `src/handlers.rs`.
- [ ] `handle_create_image_upload_request` in `src/handlers.rs`.
- [ ] `handle_archive_product_request` in `src/handlers.rs`.
- [ ] `handle_get_supplier_offers_request` in `src/handlers.rs`.
- [ ] `handle_get_customs_data_request` in `src/handlers.rs`.
- [ ] `handle_match_supplier_request` in `src/handlers.rs`.
- [ ] `handle_render_shop_page_request` in `src/handlers.rs`.
- [ ] Table `product`: the queries in `repository.rs`.
- [ ] Table `variant`: the queries in `repository.rs`.
- [ ] Table `price_tier`: the queries in `repository.rs`.
- [ ] Table `decoration_option`: the queries in `repository.rs`.
- [ ] Table `product_image`: the queries in `repository.rs`.
- [ ] Table `supplier_offer`: the queries in `repository.rs`.
- [ ] `CatalogService` field mapping in `api/src/services/catalog.rs` and the messages in `proto/catalog/v1/catalog.proto`.
- [ ] The message structs in `crates/messages/src/lib.rs`.
- [ ] `flows.md` and this file kept true.
