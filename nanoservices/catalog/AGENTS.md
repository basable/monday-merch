# catalog

Owns products, variants, list prices and decoration options, and renders the public shop pages.

Rendered by the scaffold from `docs/01-enterprise-merch/manifest.yaml`.
The Directive sections that bind this nanoservice: §2 (table ownership), §7
(cross-nanoservice rules for every message it sends), §10 (this file and
`flows.md` must stay true).

## What it owns

Plain tables (schema `nano_catalog`): `product`, `variant`, `price_tier`, `decoration_option`, `product_image`, `supplier_offer`.

Config catalogs: `category` (`config/base/category.json`), `collection` (`config/base/collection.json`), `brand` (`config/base/brand.json`), `colour` (`config/base/colour.json`), `print_method` (`config/base/print_method.json`), `supplier` (`config/base/supplier.json`), `pricing_policy` (`config/base/pricing_policy.json`).

## Files

| File | Responsibility |
|---|---|
| `src/lib.rs` | The `Catalog` struct, its constructor (pools, providers, calls), the schema marker, and its `basable_app::Component` impl: `loops()` lists every worker and ticker it runs (none for a plain executor). |
| `src/handlers.rs` | `impl CatalogHandler<R>`: one `handle_<snake_message>` per handled message. Intent only — `create` / `update_spec` / `mark_deleted` / `nudge`; never a status write. |
| `src/repository.rs` / `src/model.rs` | Typed sqlx queries over this nanoservice's pool; the row structs. |
| `src/config.rs` | The config type declarations, their binders, and `register`, which `app/src/main.rs` calls before the seed loads; the handlers read the catalogs through the `config` repository the constructor takes. |
| `tests/integration.rs` | Integration tests on the testkit (a database per test). |
| `flows.md` | State machines, sequences, edge cases. |

## Messages

Handles: `ListProductsRequest` → `ProductPage`, `GetProductRequest` → `Product`, `ListCategoriesRequest` → `CategoryList`, `PriceLinesRequest` → `PricedLines`, `UpsertProductRequest` → `Product`, `UpsertVariantRequest` → `Variant`, `SetPriceTiersRequest` → `Product`, `SetDecorationOptionsRequest` → `Product`, `SetSupplierOffersRequest` → `SupplierOffers`, `CreateImageUploadRequest` → `ImageUpload`, `ArchiveProductRequest` → `Product`, `GetSupplierOffersRequest` → `SupplierOffers`, `GetCustomsDataRequest` → `CustomsData`, `MatchSupplierRequest` → `SupplierMatch`, `RenderShopPageRequest` → `ShopPage`.
Sends: `ResolveMemberRequest`, `AuditEvent`.

API: `CatalogService` in `api/src/services/catalog.rs` (ListProducts, GetProduct, ListCategories, PriceLines, UpsertProduct, UpsertVariant, SetPriceTiers, SetDecorationOptions, SetSupplierOffers, CreateImageUpload, ArchiveProduct).
