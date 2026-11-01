//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/fulfilment/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::fulfilment::v1::WarehouseConfiguration;

/// `WarehouseConfiguration` (`wh_…`), id 109.
pub const WAREHOUSE_TYPE: TypeInfo = TypeInfo {
    id: 109,
    name: "WarehouseConfiguration",
    prefix: "wh",
};

/// The binder over `basable_config.warehouse_configuration`.
pub struct WarehouseBinder;

impl TypedBinder for WarehouseBinder {
    type Msg = WarehouseConfiguration;

    fn type_info(&self) -> TypeInfo {
        WAREHOUSE_TYPE
    }

    fn header(&self, msg: &WarehouseConfiguration) -> ConfigHeader {
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
        msg: &WarehouseConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.warehouse_configuration (id, slug, title, country, operator)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 title = EXCLUDED.title,
                 country = EXCLUDED.country,
                 operator = EXCLUDED.operator",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.title)
        .bind(&msg.country)
        .bind(&msg.operator)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.warehouse_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<WarehouseConfiguration>, BinderError> {
        let row: Option<(String, String, String, String)> = sqlx::query_as(
            "SELECT slug, title, country, operator
             FROM basable_config.warehouse_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, title, country, operator)| WarehouseConfiguration {
            slug,
            title,
            country,
            operator,
            ..Default::default()
        }))
    }
}

pub use proto::proto::fulfilment::v1::ShippingZoneConfiguration;

/// `ShippingZoneConfiguration` (`szn_…`), id 110.
pub const SHIPPING_ZONE_TYPE: TypeInfo = TypeInfo {
    id: 110,
    name: "ShippingZoneConfiguration",
    prefix: "szn",
};

/// The binder over `basable_config.shipping_zone_configuration`.
pub struct ShippingZoneBinder;

impl TypedBinder for ShippingZoneBinder {
    type Msg = ShippingZoneConfiguration;

    fn type_info(&self) -> TypeInfo {
        SHIPPING_ZONE_TYPE
    }

    fn header(&self, msg: &ShippingZoneConfiguration) -> ConfigHeader {
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
        msg: &ShippingZoneConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.shipping_zone_configuration (id, slug, countries, standard_transit_days, express_transit_days, currency, standard_pack_minor, express_pack_minor, incoterm)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 countries = EXCLUDED.countries,
                 standard_transit_days = EXCLUDED.standard_transit_days,
                 express_transit_days = EXCLUDED.express_transit_days,
                 currency = EXCLUDED.currency,
                 standard_pack_minor = EXCLUDED.standard_pack_minor,
                 express_pack_minor = EXCLUDED.express_pack_minor,
                 incoterm = EXCLUDED.incoterm",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(basable_config::parse_json(&msg.countries)?)
        .bind(msg.standard_transit_days)
        .bind(msg.express_transit_days)
        .bind(&msg.currency)
        .bind(msg.standard_pack_minor)
        .bind(msg.express_pack_minor)
        .bind(&msg.incoterm)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.shipping_zone_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<ShippingZoneConfiguration>, BinderError> {
        let row: Option<(String, Option<serde_json::Value>, i32, i32, String, i64, i64, String)> = sqlx::query_as(
            "SELECT slug, countries, standard_transit_days, express_transit_days, currency, standard_pack_minor, express_pack_minor, incoterm
             FROM basable_config.shipping_zone_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, countries, standard_transit_days, express_transit_days, currency, standard_pack_minor, express_pack_minor, incoterm)| ShippingZoneConfiguration {
            slug,
            countries: countries.map(|v| v.to_string()).unwrap_or_default(),
            standard_transit_days,
            express_transit_days,
            currency,
            standard_pack_minor,
            express_pack_minor,
            incoterm,
            ..Default::default()
        }))
    }
}

pub use proto::proto::fulfilment::v1::FulfilmentFeeConfiguration;

/// `FulfilmentFeeConfiguration` (`ffee_…`), id 111.
pub const FULFILMENT_FEE_TYPE: TypeInfo = TypeInfo {
    id: 111,
    name: "FulfilmentFeeConfiguration",
    prefix: "ffee",
};

/// The binder over `basable_config.fulfilment_fee_configuration`.
pub struct FulfilmentFeeBinder;

impl TypedBinder for FulfilmentFeeBinder {
    type Msg = FulfilmentFeeConfiguration;

    fn type_info(&self) -> TypeInfo {
        FULFILMENT_FEE_TYPE
    }

    fn header(&self, msg: &FulfilmentFeeConfiguration) -> ConfigHeader {
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
        msg: &FulfilmentFeeConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.fulfilment_fee_configuration (id, currency, pack_handling_minor, warehouse_inbound_minor)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 currency = EXCLUDED.currency,
                 pack_handling_minor = EXCLUDED.pack_handling_minor,
                 warehouse_inbound_minor = EXCLUDED.warehouse_inbound_minor",
        )
        .bind(id)
        .bind(&msg.currency)
        .bind(msg.pack_handling_minor)
        .bind(msg.warehouse_inbound_minor)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.fulfilment_fee_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<FulfilmentFeeConfiguration>, BinderError> {
        let row: Option<(String, i64, i64)> = sqlx::query_as(
            "SELECT currency, pack_handling_minor, warehouse_inbound_minor
             FROM basable_config.fulfilment_fee_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(currency, pack_handling_minor, warehouse_inbound_minor)| FulfilmentFeeConfiguration {
            currency,
            pack_handling_minor,
            warehouse_inbound_minor,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(WarehouseBinder);
    b.register(ShippingZoneBinder);
    b.register(FulfilmentFeeBinder);
}
