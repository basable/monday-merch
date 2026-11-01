//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/order/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::order::v1::QuotePolicyConfiguration;

/// `QuotePolicyConfiguration` (`qpol_…`), id 108.
pub const QUOTE_POLICY_TYPE: TypeInfo = TypeInfo {
    id: 108,
    name: "QuotePolicyConfiguration",
    prefix: "qpol",
};

/// The binder over `basable_config.quote_policy_configuration`.
pub struct QuotePolicyBinder;

impl TypedBinder for QuotePolicyBinder {
    type Msg = QuotePolicyConfiguration;

    fn type_info(&self) -> TypeInfo {
        QUOTE_POLICY_TYPE
    }

    fn header(&self, msg: &QuotePolicyConfiguration) -> ConfigHeader {
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
        msg: &QuotePolicyConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.quote_policy_configuration (id, sla_hours, sla_alert_hours, validity_days)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 sla_hours = EXCLUDED.sla_hours,
                 sla_alert_hours = EXCLUDED.sla_alert_hours,
                 validity_days = EXCLUDED.validity_days",
        )
        .bind(id)
        .bind(msg.sla_hours)
        .bind(msg.sla_alert_hours)
        .bind(msg.validity_days)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.quote_policy_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<QuotePolicyConfiguration>, BinderError> {
        let row: Option<(i32, i32, i32)> = sqlx::query_as(
            "SELECT sla_hours, sla_alert_hours, validity_days
             FROM basable_config.quote_policy_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(sla_hours, sla_alert_hours, validity_days)| QuotePolicyConfiguration {
            sla_hours,
            sla_alert_hours,
            validity_days,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(QuotePolicyBinder);
}
