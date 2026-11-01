//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/catalog/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::catalog::v1::CategoryConfiguration;

/// `CategoryConfiguration` (`cat_…`), id 100.
pub const CATEGORY_TYPE: TypeInfo = TypeInfo {
    id: 100,
    name: "CategoryConfiguration",
    prefix: "cat",
};

/// The binder over `basable_config.category_configuration`.
pub struct CategoryBinder;

impl TypedBinder for CategoryBinder {
    type Msg = CategoryConfiguration;

    fn type_info(&self) -> TypeInfo {
        CATEGORY_TYPE
    }

    fn header(&self, msg: &CategoryConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &CategoryConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.category_configuration (id, slug, parent_slug, title, sort_order)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 parent_slug = EXCLUDED.parent_slug,
                 title = EXCLUDED.title,
                 sort_order = EXCLUDED.sort_order",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.parent_slug)
        .bind(&msg.title)
        .bind(msg.sort_order)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.category_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<CategoryConfiguration>, BinderError> {
        let row: Option<(String, String, String, i32)> = sqlx::query_as(
            "SELECT slug, parent_slug, title, sort_order
             FROM basable_config.category_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, parent_slug, title, sort_order)| CategoryConfiguration {
            slug,
            parent_slug,
            title,
            sort_order,
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::CollectionConfiguration;

/// `CollectionConfiguration` (`coll_…`), id 101.
pub const COLLECTION_TYPE: TypeInfo = TypeInfo {
    id: 101,
    name: "CollectionConfiguration",
    prefix: "coll",
};

/// The binder over `basable_config.collection_configuration`.
pub struct CollectionBinder;

impl TypedBinder for CollectionBinder {
    type Msg = CollectionConfiguration;

    fn type_info(&self) -> TypeInfo {
        COLLECTION_TYPE
    }

    fn header(&self, msg: &CollectionConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &CollectionConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.collection_configuration (id, slug, title, active)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title,
                 active = EXCLUDED.active",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .bind(msg.active)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.collection_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<CollectionConfiguration>, BinderError> {
        let row: Option<(String, String, bool)> = sqlx::query_as(
            "SELECT slug, title, active
             FROM basable_config.collection_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title, active)| CollectionConfiguration {
            slug,
            title,
            active,
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::BrandConfiguration;

/// `BrandConfiguration` (`brnd_…`), id 102.
pub const BRAND_TYPE: TypeInfo = TypeInfo {
    id: 102,
    name: "BrandConfiguration",
    prefix: "brnd",
};

/// The binder over `basable_config.brand_configuration`.
pub struct BrandBinder;

impl TypedBinder for BrandBinder {
    type Msg = BrandConfiguration;

    fn type_info(&self) -> TypeInfo {
        BRAND_TYPE
    }

    fn header(&self, msg: &BrandConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &BrandConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.brand_configuration (id, slug, title)
             VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.brand_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<BrandConfiguration>, BinderError> {
        let row: Option<(String, String)> = sqlx::query_as(
            "SELECT slug, title
             FROM basable_config.brand_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title)| BrandConfiguration {
            slug,
            title,
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::ColourConfiguration;

/// `ColourConfiguration` (`clr_…`), id 103.
pub const COLOUR_TYPE: TypeInfo = TypeInfo {
    id: 103,
    name: "ColourConfiguration",
    prefix: "clr",
};

/// The binder over `basable_config.colour_configuration`.
pub struct ColourBinder;

impl TypedBinder for ColourBinder {
    type Msg = ColourConfiguration;

    fn type_info(&self) -> TypeInfo {
        COLOUR_TYPE
    }

    fn header(&self, msg: &ColourConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &ColourConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.colour_configuration (id, slug, title, hex, family)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title,
                 hex = EXCLUDED.hex,
                 family = EXCLUDED.family",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .bind(&msg.hex)
        .bind(&msg.family)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.colour_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<ColourConfiguration>, BinderError> {
        let row: Option<(String, String, String, String)> = sqlx::query_as(
            "SELECT slug, title, hex, family
             FROM basable_config.colour_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title, hex, family)| ColourConfiguration {
            slug,
            title,
            hex,
            family,
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::PrintMethodConfiguration;

/// `PrintMethodConfiguration` (`pm_…`), id 104.
pub const PRINT_METHOD_TYPE: TypeInfo = TypeInfo {
    id: 104,
    name: "PrintMethodConfiguration",
    prefix: "pm",
};

/// The binder over `basable_config.print_method_configuration`.
pub struct PrintMethodBinder;

impl TypedBinder for PrintMethodBinder {
    type Msg = PrintMethodConfiguration;

    fn type_info(&self) -> TypeInfo {
        PRINT_METHOD_TYPE
    }

    fn header(&self, msg: &PrintMethodConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &PrintMethodConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.print_method_configuration (id, slug, title, constraints)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title,
                 constraints = EXCLUDED.constraints",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .bind(basable_config::parse_json(&msg.constraints)?)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.print_method_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<PrintMethodConfiguration>, BinderError> {
        let row: Option<(String, String, Option<serde_json::Value>)> = sqlx::query_as(
            "SELECT slug, title, constraints
             FROM basable_config.print_method_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title, constraints)| PrintMethodConfiguration {
            slug,
            title,
            constraints: constraints.map(|v| v.to_string()).unwrap_or_default(),
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::SupplierConfiguration;

/// `SupplierConfiguration` (`sup_…`), id 105.
pub const SUPPLIER_TYPE: TypeInfo = TypeInfo {
    id: 105,
    name: "SupplierConfiguration",
    prefix: "sup",
};

/// The binder over `basable_config.supplier_configuration`.
pub struct SupplierBinder;

impl TypedBinder for SupplierBinder {
    type Msg = SupplierConfiguration;

    fn type_info(&self) -> TypeInfo {
        SUPPLIER_TYPE
    }

    fn header(&self, msg: &SupplierConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &SupplierConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.supplier_configuration (id, slug, title, po_email, country, sender_domains, bank_accounts)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title,
                 po_email = EXCLUDED.po_email,
                 country = EXCLUDED.country,
                 sender_domains = EXCLUDED.sender_domains,
                 bank_accounts = EXCLUDED.bank_accounts",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .bind(&msg.po_email)
        .bind(&msg.country)
        .bind(basable_config::parse_json(&msg.sender_domains)?)
        .bind(basable_config::parse_json(&msg.bank_accounts)?)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.supplier_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<SupplierConfiguration>, BinderError> {
        let row: Option<(String, String, String, String, Option<serde_json::Value>, Option<serde_json::Value>)> = sqlx::query_as(
            "SELECT slug, title, po_email, country, sender_domains, bank_accounts
             FROM basable_config.supplier_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title, po_email, country, sender_domains, bank_accounts)| SupplierConfiguration {
            slug,
            title,
            po_email,
            country,
            sender_domains: sender_domains.map(|v| v.to_string()).unwrap_or_default(),
            bank_accounts: bank_accounts.map(|v| v.to_string()).unwrap_or_default(),
            ..Default::default()
        }))
    }
}

pub use proto::proto::catalog::v1::PricingPolicyConfiguration;

/// `PricingPolicyConfiguration` (`ppol_…`), id 106.
pub const PRICING_POLICY_TYPE: TypeInfo = TypeInfo {
    id: 106,
    name: "PricingPolicyConfiguration",
    prefix: "ppol",
};

/// The binder over `basable_config.pricing_policy_configuration`.
pub struct PricingPolicyBinder;

impl TypedBinder for PricingPolicyBinder {
    type Msg = PricingPolicyConfiguration;

    fn type_info(&self) -> TypeInfo {
        PRICING_POLICY_TYPE
    }

    fn header(&self, msg: &PricingPolicyConfiguration) -> ConfigHeader {
        // The proto's header is a message field, absent when unset.
        let h = msg.header.as_option();
        ConfigHeader {
            namespace: h.map(|h| h.namespace.clone()).unwrap_or_default(),
            name: h.map(|h| h.name.clone()).unwrap_or_default(),
            external_id: h.map(|h| h.external_id.clone()).unwrap_or_default(),
            labels: h
                .map(|h| h.labels.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default(),
        }
    }

    async fn upsert(
        &self,
        tx: &mut PgConnection,
        id: Uuid,
        msg: &PricingPolicyConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.pricing_policy_configuration (id, express_fee_bps, express_days_faster)
             VALUES ($1, $2, $3)
             ON CONFLICT (id) DO UPDATE SET
                 express_fee_bps = EXCLUDED.express_fee_bps,
                 express_days_faster = EXCLUDED.express_days_faster",
        )
        .bind(id)
        .bind(msg.express_fee_bps)
        .bind(msg.express_days_faster)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.pricing_policy_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<PricingPolicyConfiguration>, BinderError> {
        let row: Option<(i32, i32)> = sqlx::query_as(
            "SELECT express_fee_bps, express_days_faster
             FROM basable_config.pricing_policy_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(express_fee_bps, express_days_faster)| PricingPolicyConfiguration {
            express_fee_bps,
            express_days_faster,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(CategoryBinder);
    b.register(CollectionBinder);
    b.register(BrandBinder);
    b.register(ColourBinder);
    b.register(PrintMethodBinder);
    b.register(SupplierBinder);
    b.register(PricingPolicyBinder);
}
