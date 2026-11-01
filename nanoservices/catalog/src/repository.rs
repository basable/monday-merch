//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_catalog`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Catalog;

impl Catalog {
    // --- product ---

    pub async fn insert_product(&self, row: &model::Product) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO product (id, slug, name, description, category_slug, brand_slug, collections, attributes, impact_note, sustainability_labels, min_qty, production_days, customisable, best_selling, is_new, weight_grams, hs_code, origin_country, published_at, archived_at, search_text) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)",
        )
        .bind(row.id)
        .bind(&row.slug)
        .bind(&row.name)
        .bind(&row.description)
        .bind(&row.category_slug)
        .bind(&row.brand_slug)
        .bind(&row.collections)
        .bind(&row.attributes)
        .bind(&row.impact_note)
        .bind(&row.sustainability_labels)
        .bind(row.min_qty)
        .bind(row.production_days)
        .bind(row.customisable)
        .bind(row.best_selling)
        .bind(row.is_new)
        .bind(row.weight_grams)
        .bind(&row.hs_code)
        .bind(&row.origin_country)
        .bind(&row.published_at)
        .bind(&row.archived_at)
        .bind(&row.search_text)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_product(&self, id: Uuid) -> Result<Option<model::Product>, sqlx::Error> {
        sqlx::query_as::<_, model::Product>("SELECT id, slug, name, description, category_slug, brand_slug, collections, attributes, impact_note, sustainability_labels, min_qty, production_days, customisable, best_selling, is_new, weight_grams, hs_code, origin_country, published_at, archived_at, search_text FROM product WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- variant ---

    pub async fn insert_variant(&self, row: &model::Variant) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO variant (id, product_id, colour_slug, sku, sizes, archived_at) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(row.id)
        .bind(row.product_id)
        .bind(&row.colour_slug)
        .bind(&row.sku)
        .bind(&row.sizes)
        .bind(&row.archived_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_variant(&self, id: Uuid) -> Result<Option<model::Variant>, sqlx::Error> {
        sqlx::query_as::<_, model::Variant>("SELECT id, product_id, colour_slug, sku, sizes, archived_at FROM variant WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- price_tier ---

    pub async fn insert_price_tier(&self, row: &model::PriceTier) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO price_tier (id, product_id, currency, min_qty, unit_minor) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(row.product_id)
        .bind(&row.currency)
        .bind(row.min_qty)
        .bind(row.unit_minor)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_price_tier(&self, id: Uuid) -> Result<Option<model::PriceTier>, sqlx::Error> {
        sqlx::query_as::<_, model::PriceTier>("SELECT id, product_id, currency, min_qty, unit_minor FROM price_tier WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- decoration_option ---

    pub async fn insert_decoration_option(&self, row: &model::DecorationOption) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO decoration_option (id, product_id, print_method_slug, areas, currency, setup_minor, per_unit_minor) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.product_id)
        .bind(&row.print_method_slug)
        .bind(&row.areas)
        .bind(&row.currency)
        .bind(row.setup_minor)
        .bind(row.per_unit_minor)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_decoration_option(&self, id: Uuid) -> Result<Option<model::DecorationOption>, sqlx::Error> {
        sqlx::query_as::<_, model::DecorationOption>("SELECT id, product_id, print_method_slug, areas, currency, setup_minor, per_unit_minor FROM decoration_option WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- product_image ---

    pub async fn insert_product_image(&self, row: &model::ProductImage) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO product_image (id, product_id, variant_id, object_key, alt_text, position, view) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.product_id)
        .bind(row.variant_id)
        .bind(&row.object_key)
        .bind(&row.alt_text)
        .bind(row.position)
        .bind(&row.view)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_product_image(&self, id: Uuid) -> Result<Option<model::ProductImage>, sqlx::Error> {
        sqlx::query_as::<_, model::ProductImage>("SELECT id, product_id, variant_id, object_key, alt_text, position, view FROM product_image WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- supplier_offer ---

    pub async fn insert_supplier_offer(&self, row: &model::SupplierOffer) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO supplier_offer (id, product_id, supplier_slug, supplier_sku, currency, cost_minor, lead_days, preferred) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(row.id)
        .bind(row.product_id)
        .bind(&row.supplier_slug)
        .bind(&row.supplier_sku)
        .bind(&row.currency)
        .bind(row.cost_minor)
        .bind(row.lead_days)
        .bind(row.preferred)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_supplier_offer(&self, id: Uuid) -> Result<Option<model::SupplierOffer>, sqlx::Error> {
        sqlx::query_as::<_, model::SupplierOffer>("SELECT id, product_id, supplier_slug, supplier_sku, currency, cost_minor, lead_days, preferred FROM supplier_offer WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
