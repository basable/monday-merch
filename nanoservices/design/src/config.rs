//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/design/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::design::v1::StudioFontConfiguration;

/// `StudioFontConfiguration` (`sfnt_…`), id 107.
pub const STUDIO_FONT_TYPE: TypeInfo = TypeInfo {
    id: 107,
    name: "StudioFontConfiguration",
    prefix: "sfnt",
};

/// The binder over `basable_config.studio_font_configuration`.
pub struct StudioFontBinder;

impl TypedBinder for StudioFontBinder {
    type Msg = StudioFontConfiguration;

    fn type_info(&self) -> TypeInfo {
        STUDIO_FONT_TYPE
    }

    fn header(&self, msg: &StudioFontConfiguration) -> ConfigHeader {
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
        msg: &StudioFontConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.studio_font_configuration (id, family, files, licence)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 family = EXCLUDED.family,
                 files = EXCLUDED.files,
                 licence = EXCLUDED.licence",
        )
        .bind(id)
        .bind(&msg.family)
        .bind(basable_config::parse_json(&msg.files)?)
        .bind(&msg.licence)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.studio_font_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<StudioFontConfiguration>, BinderError> {
        let row: Option<(String, Option<serde_json::Value>, String)> = sqlx::query_as(
            "SELECT family, files, licence
             FROM basable_config.studio_font_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(family, files, licence)| StudioFontConfiguration {
            family,
            files: files.map(|v| v.to_string()).unwrap_or_default(),
            licence,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(StudioFontBinder);
}
