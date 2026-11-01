//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/notifier/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::notifier::v1::EmailTemplateConfiguration;

/// `EmailTemplateConfiguration` (`etpl_…`), id 114.
pub const EMAIL_TEMPLATE_TYPE: TypeInfo = TypeInfo {
    id: 114,
    name: "EmailTemplateConfiguration",
    prefix: "etpl",
};

/// The binder over `basable_config.email_template_configuration`.
pub struct EmailTemplateBinder;

impl TypedBinder for EmailTemplateBinder {
    type Msg = EmailTemplateConfiguration;

    fn type_info(&self) -> TypeInfo {
        EMAIL_TEMPLATE_TYPE
    }

    fn header(&self, msg: &EmailTemplateConfiguration) -> ConfigHeader {
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
        msg: &EmailTemplateConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.email_template_configuration (id, slug, locale, subject, body_html, body_text)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET
                 slug = EXCLUDED.slug,
                 locale = EXCLUDED.locale,
                 subject = EXCLUDED.subject,
                 body_html = EXCLUDED.body_html,
                 body_text = EXCLUDED.body_text",
        )
        .bind(id)
        .bind(&msg.slug)
        .bind(&msg.locale)
        .bind(&msg.subject)
        .bind(&msg.body_html)
        .bind(&msg.body_text)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.email_template_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<EmailTemplateConfiguration>, BinderError> {
        let row: Option<(String, String, String, String, String)> = sqlx::query_as(
            "SELECT slug, locale, subject, body_html, body_text
             FROM basable_config.email_template_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(slug, locale, subject, body_html, body_text)| EmailTemplateConfiguration {
            slug,
            locale,
            subject,
            body_html,
            body_text,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(EmailTemplateBinder);
}
