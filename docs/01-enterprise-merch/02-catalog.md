# 02: catalog

What can be bought and at what list price, and the public shop pages. No lifecycle: staff edit products and readers read them, so this is a plain executor (Directive §1). Prices here are a price list, not money owed, so they stay plain columns (Directive §11). Directive §2 binds every table, §7 every message.

## Tables

**`product`** carries `slug` (unique, used in URLs), `name`, `description`, `category_slug`, `brand_slug`, `collections`, `attributes` (materials, fabric weight, fit and so on, as JSON because they differ per category), `impact_note`, `sustainability_labels`, `min_qty`, `production_days`, `customisable`, `best_selling`, `is_new`, and the facts `published_at` and `archived_at`: a product is public once published and until archived, and nothing drives it between them. `search_text` backs Postgres full-text search through a GIN index added in a follow-up migration. `weight_grams`, `hs_code` and `origin_country` exist for labels and customs.

**`variant`** is one colour of a product: `product_id`, `colour_slug`, `sku` (unique), `sizes`, `archived_at`.

**`price_tier`** is one quantity break: `product_id`, `currency`, `min_qty`, `unit_minor`. Amounts are integer minor units.

**`decoration_option`** is one way to decorate: `product_id`, `print_method_slug`, `areas`, `currency`, `setup_minor`, `per_unit_minor`. Each entry in `areas` is what the Studio draws: the area's name (such as front or chest left), its view, its rectangle on that view's image, its physical size in millimetres, and for print products the bleed and safe margins.

**`product_image`** points at the public bucket: `product_id`, `variant_id`, `object_key`, `alt_text`, `position`, and `view` (`front`, `back`, `sleeve`, `label`) for the images the Studio draws on.

**`supplier_offer`** is who makes it at what cost: `product_id`, `supplier_slug`, `supplier_sku`, `currency`, `cost_minor`, `lead_days`, `preferred`. **Internal only.** Cost must never reach a public response.

## Invariants

- `PriceLinesRequest` is the only pricing logic. The quote form's estimate and a quote's starting lines use the same code.
- The price of a line is the unit price of the highest tier whose `min_qty` is at or below the quantity, plus decoration per unit, plus setup, plus the express fee from `pricing_policy` when the timeline is express. A quantity below `min_qty` is rejected.
- Website prices are a reference. A consultant can change prices on a quote version, and the signed version is what counts.
- A public response type has no cost field, so cost cannot leak through a forgotten filter. `GetSupplierOffersRequest` is internal, and `SetSupplierOffers` requires a staff role.

## The public shop pages

`/shop/*` is **server-rendered by this binary**, not served from the bucket. The reasons:

- Catalogue pages must be real documents for search engines, and they change whenever staff edit a product.
- The bucket serves only the deployed commit's files, and the platform forbids writing into its `site/` folder. A product edit could only appear after a new deploy.
- The site artifact is limited to 5,000 files, which a large catalogue with its images could exceed.

The HTTPRoute sends `/shop` to the binary alongside `/api` (a deviation recorded in `05-deployment.md`). A raw route in the `api` crate makes one `RenderShopPageRequest` send, and this nanoservice answers with `ShopPage` (status, ETag, HTML). Pages: category lists, product pages, collections, brands, `sitemap.xml`. Templates compile into the binary, and product pages carry structured price data. Responses send the two required security headers.

The basket lives in the browser until "Request quote", so the catalogue holds no carts and the server re-prices every line on submit. Filters and "Quick Add" are small scripts over `ListProducts` and `PriceLines`.

## Buckets

Product images go in the public bucket under `catalog/`, uploaded by staff through a presigned PUT from `CreateImageUpload` and linked at `$S3_PUBLIC_URL/<key>`. No image processing happens on the server, so staff upload web-ready files. Supplier photos need usage rights.

## Config types

`category`, `collection`, `brand`, `colour`, `print_method`, `supplier`, `pricing_policy`. See `04-config.md`.

## Messages and API

`CatalogService`: `ListProducts`, `GetProduct`, `ListCategories` and `PriceLines` are public. `UpsertProduct`, `UpsertVariant`, `SetPriceTiers`, `SetDecorationOptions`, `SetSupplierOffers`, `CreateImageUpload` and `ArchiveProduct` require a staff role.

Internal: `PriceLinesRequest` and `GetSupplierOffersRequest` (`order`), `GetCustomsDataRequest` (`fulfilment`), `MatchSupplierRequest` (`inbox`, returning the supplier for a sending domain and its bank accounts on file), `RenderShopPageRequest` (`api` raw route).

## Binding Directive sections

§1 (no lifecycle), §2 (tables), §7 (messages), §11 (price list is not ledger money).

## Tests

- Tier selection at every boundary, with decoration, setup and express, as a table-driven unit test using the observed example (15 at €19.46, 250 at €17.03).
- A quantity below `min_qty` is rejected.
- Serialise every public response and assert no cost field exists.
- A product page renders with a 200, its title, price data and security headers. An archived product answers 404, and an unknown slug answers 404 from the binary.
- Filters combine correctly, and search finds a product by name and by material.
