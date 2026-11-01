//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_design`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Design;

impl Design {
    // --- brand_profile ---

    pub async fn insert_brand_profile(&self, row: &model::BrandProfile) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO brand_profile (company_id, colours, fonts, guidelines, updated_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.company_id)
        .bind(&row.colours)
        .bind(&row.fonts)
        .bind(&row.guidelines)
        .bind(&row.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_brand_profile(&self, id: Uuid) -> Result<Option<model::BrandProfile>, sqlx::Error> {
        sqlx::query_as::<_, model::BrandProfile>("SELECT company_id, colours, fonts, guidelines, updated_at FROM brand_profile WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- brand_asset ---

    pub async fn insert_brand_asset(&self, row: &model::BrandAsset) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO brand_asset (id, company_id, owner_identity_id, kind, filename, object_key, mime_type, size_bytes, created_at, confirmed_at, deleted_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(row.owner_identity_id)
        .bind(&row.kind)
        .bind(&row.filename)
        .bind(&row.object_key)
        .bind(&row.mime_type)
        .bind(row.size_bytes)
        .bind(&row.created_at)
        .bind(&row.confirmed_at)
        .bind(&row.deleted_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_brand_asset(&self, id: Uuid) -> Result<Option<model::BrandAsset>, sqlx::Error> {
        sqlx::query_as::<_, model::BrandAsset>("SELECT id, company_id, owner_identity_id, kind, filename, object_key, mime_type, size_bytes, created_at, confirmed_at, deleted_at FROM brand_asset WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- proof ---

    pub async fn insert_proof(&self, row: &model::Proof) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO proof (id, subject_name, line_ref, version, preview_key, artwork_key, print_spec, created_by, created_at, approved_at, approved_by, source_design_version_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(row.id)
        .bind(&row.subject_name)
        .bind(&row.line_ref)
        .bind(row.version)
        .bind(&row.preview_key)
        .bind(&row.artwork_key)
        .bind(&row.print_spec)
        .bind(&row.created_by)
        .bind(&row.created_at)
        .bind(&row.approved_at)
        .bind(&row.approved_by)
        .bind(row.source_design_version_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_proof(&self, id: Uuid) -> Result<Option<model::Proof>, sqlx::Error> {
        sqlx::query_as::<_, model::Proof>("SELECT id, subject_name, line_ref, version, preview_key, artwork_key, print_spec, created_by, created_at, approved_at, approved_by, source_design_version_id FROM proof WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- design_document ---

    pub async fn insert_design_document(&self, row: &model::DesignDocument) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO design_document (id, company_id, owner_identity_id, product_id, variant_id, name, draft, schema_version, revision, archived_at, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(row.owner_identity_id)
        .bind(row.product_id)
        .bind(row.variant_id)
        .bind(&row.name)
        .bind(&row.draft)
        .bind(row.schema_version)
        .bind(row.revision)
        .bind(&row.archived_at)
        .bind(&row.created_at)
        .bind(&row.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_design_document(&self, id: Uuid) -> Result<Option<model::DesignDocument>, sqlx::Error> {
        sqlx::query_as::<_, model::DesignDocument>("SELECT id, company_id, owner_identity_id, product_id, variant_id, name, draft, schema_version, revision, archived_at, created_at, updated_at FROM design_document WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- design_version ---

    pub async fn insert_design_version(&self, row: &model::DesignVersion) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO design_version (id, design_id, version, document, schema_version, preview_keys, created_by, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
        )
        .bind(row.id)
        .bind(row.design_id)
        .bind(row.version)
        .bind(&row.document)
        .bind(row.schema_version)
        .bind(&row.preview_keys)
        .bind(&row.created_by)
        .bind(&row.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_design_version(&self, id: Uuid) -> Result<Option<model::DesignVersion>, sqlx::Error> {
        sqlx::query_as::<_, model::DesignVersion>("SELECT id, design_id, version, document, schema_version, preview_keys, created_by, created_at FROM design_version WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- design_template ---

    pub async fn insert_design_template(&self, row: &model::DesignTemplate) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO design_template (id, slug, title, kind, product_id, document, schema_version, preview_key, published_at, sort_order) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)",
        )
        .bind(row.id)
        .bind(&row.slug)
        .bind(&row.title)
        .bind(&row.kind)
        .bind(row.product_id)
        .bind(&row.document)
        .bind(row.schema_version)
        .bind(&row.preview_key)
        .bind(&row.published_at)
        .bind(row.sort_order)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_design_template(&self, id: Uuid) -> Result<Option<model::DesignTemplate>, sqlx::Error> {
        sqlx::query_as::<_, model::DesignTemplate>("SELECT id, slug, title, kind, product_id, document, schema_version, preview_key, published_at, sort_order FROM design_template WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
