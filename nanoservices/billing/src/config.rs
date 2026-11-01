//! The declarative config types this nanoservice owns: a person edits
//! `config/base/<name>.json`, the loader applies it at boot, and the
//! binders here map each message onto its subtype table in
//! `basable_config`. The messages are the proto types in
//! `proto/billing/v1/config.proto`; a reference field arrives as the
//! referenced object's id.

use basable_config::{BinderError, ConfigHeader, ConfigTypesBuilder, TypeInfo, TypedBinder};
use sqlx::PgConnection;
use uuid::Uuid;

pub use proto::proto::billing::v1::VatRuleConfiguration;

/// `VatRuleConfiguration` (`vatr_…`), id 112.
pub const VAT_RULE_TYPE: TypeInfo = TypeInfo {
    id: 112,
    name: "VatRuleConfiguration",
    prefix: "vatr",
};

/// The binder over `basable_config.vat_rule_configuration`.
pub struct VatRuleBinder;

impl TypedBinder for VatRuleBinder {
    type Msg = VatRuleConfiguration;

    fn type_info(&self) -> TypeInfo {
        VAT_RULE_TYPE
    }

    fn header(&self, msg: &VatRuleConfiguration) -> ConfigHeader {
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
        msg: &VatRuleConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.vat_rule_configuration (id, country, standard_rate_bps, valid_from)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (id) DO UPDATE SET
                 country = EXCLUDED.country,
                 standard_rate_bps = EXCLUDED.standard_rate_bps,
                 valid_from = EXCLUDED.valid_from",
        )
        .bind(id)
        .bind(&msg.country)
        .bind(msg.standard_rate_bps)
        .bind(basable_config::parse_timestamp(&msg.valid_from)?)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.vat_rule_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<VatRuleConfiguration>, BinderError> {
        let row: Option<(String, i32, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
            "SELECT country, standard_rate_bps, valid_from
             FROM basable_config.vat_rule_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(country, standard_rate_bps, valid_from)| VatRuleConfiguration {
            country,
            standard_rate_bps,
            valid_from: valid_from.map(|t| t.to_rfc3339()).unwrap_or_default(),
            ..Default::default()
        }))
    }
}

pub use proto::proto::billing::v1::BillingPolicyConfiguration;

/// `BillingPolicyConfiguration` (`bpol_…`), id 113.
pub const BILLING_POLICY_TYPE: TypeInfo = TypeInfo {
    id: 113,
    name: "BillingPolicyConfiguration",
    prefix: "bpol",
};

/// The binder over `basable_config.billing_policy_configuration`.
pub struct BillingPolicyBinder;

impl TypedBinder for BillingPolicyBinder {
    type Msg = BillingPolicyConfiguration;

    fn type_info(&self) -> TypeInfo {
        BILLING_POLICY_TYPE
    }

    fn header(&self, msg: &BillingPolicyConfiguration) -> ConfigHeader {
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
        msg: &BillingPolicyConfiguration,
    ) -> Result<(), BinderError> {
        sqlx::query(
            "INSERT INTO basable_config.billing_policy_configuration (id, default_terms_days, card_fee_bps, reminder_days, match_tolerance_bps)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (id) DO UPDATE SET
                 default_terms_days = EXCLUDED.default_terms_days,
                 card_fee_bps = EXCLUDED.card_fee_bps,
                 reminder_days = EXCLUDED.reminder_days,
                 match_tolerance_bps = EXCLUDED.match_tolerance_bps",
        )
        .bind(id)
        .bind(msg.default_terms_days)
        .bind(msg.card_fee_bps)
        .bind(basable_config::parse_json(&msg.reminder_days)?)
        .bind(msg.match_tolerance_bps)
        .execute(tx)
        .await?;
        Ok(())
    }

    async fn delete(&self, tx: &mut PgConnection, id: Uuid) -> Result<(), BinderError> {
        // Refuse with BinderError::StillReferenced here while another
        // object still names this one; a prune retries once it is gone.
        sqlx::query("DELETE FROM basable_config.billing_policy_configuration WHERE id = $1")
            .bind(id)
            .execute(tx)
            .await?;
        Ok(())
    }

    async fn read(
        &self,
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<BillingPolicyConfiguration>, BinderError> {
        let row: Option<(i32, i32, Option<serde_json::Value>, i32)> = sqlx::query_as(
            "SELECT default_terms_days, card_fee_bps, reminder_days, match_tolerance_bps
             FROM basable_config.billing_policy_configuration WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(conn)
        .await?;
        Ok(row.map(|(default_terms_days, card_fee_bps, reminder_days, match_tolerance_bps)| BillingPolicyConfiguration {
            default_terms_days,
            card_fee_bps,
            reminder_days: reminder_days.map(|v| v.to_string()).unwrap_or_default(),
            match_tolerance_bps,
            ..Default::default()
        }))
    }
}

/// Registers this nanoservice's config types with the app's builder.
pub fn register(b: &mut ConfigTypesBuilder) {
    b.register(VatRuleBinder);
    b.register(BillingPolicyBinder);
}
