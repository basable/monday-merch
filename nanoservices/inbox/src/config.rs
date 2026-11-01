//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/inbox/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::inbox::v1::InboxPolicyConfiguration;

/// `InboxPolicyConfiguration` (`ipol_…`), id 115.
pub const INBOX_POLICY_TYPE: TypeInfo = TypeInfo {
    id: 115,
    name: "InboxPolicyConfiguration",
    prefix: "ipol",
};

/// The binder over `basable_config.inbox_policy_configuration`.
pub struct InboxPolicyBinder;

impl TypedBinder for InboxPolicyBinder {
    type Msg = InboxPolicyConfiguration;

    fn type_info(&self) -> TypeInfo {
        INBOX_POLICY_TYPE
    }

    fn header(&self, msg: &InboxPolicyConfiguration) -> ConfigHeader {
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
        msg: &InboxPolicyConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.inbox_policy_configuration (id, max_attachment_bytes, allowed_mime_types, auto_apply_kinds, retention_days, model, effort)
             VALUES ($1, $2, $3, $4, $5, $6, $7)
             ON CONFLICT (id) DO UPDATE SET
                 max_attachment_bytes = EXCLUDED.max_attachment_bytes,
                 allowed_mime_types = EXCLUDED.allowed_mime_types,
                 auto_apply_kinds = EXCLUDED.auto_apply_kinds,
                 retention_days = EXCLUDED.retention_days,
                 model = EXCLUDED.model,
                 effort = EXCLUDED.effort",
        )
        .bind(id)
        .bind(msg.max_attachment_bytes)
        .bind(basable_config::parse_json(&msg.allowed_mime_types)?)
        .bind(basable_config::parse_json(&msg.auto_apply_kinds)?)
        .bind(msg.retention_days)
        .bind(&msg.model)
        .bind(&msg.effort)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.inbox_policy_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<InboxPolicyConfiguration>, BinderError> {
        let row: Option<(i64, Option<serde_json::Value>, Option<serde_json::Value>, i32, String, String)> = sqlx::query_as(
            "SELECT max_attachment_bytes, allowed_mime_types, auto_apply_kinds, retention_days, model, effort
             FROM basable_config.inbox_policy_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(max_attachment_bytes, allowed_mime_types, auto_apply_kinds, retention_days, model, effort)| InboxPolicyConfiguration {
            max_attachment_bytes,
            allowed_mime_types: allowed_mime_types.map(|v| v.to_string()).unwrap_or_default(),
            auto_apply_kinds: auto_apply_kinds.map(|v| v.to_string()).unwrap_or_default(),
            retention_days,
            model,
            effort,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(InboxPolicyBinder);
}
