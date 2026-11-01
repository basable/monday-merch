//! Typed sqlx queries over this nanoservice's own pool (schema
//! `nano_account`). Only this crate queries these tables; another
//! nanoservice that needs the data sends a message (the Directive §2).
//! Errors are `sqlx::Error` as they come; the handler decides what a
//! failure means to its caller (`AppError::wrap`).

use uuid::Uuid;

// Rows are `model::<Table>`, qualified: the component struct below and a
// message type in `messages` may carry the same name as a table.
use crate::model;
use crate::Account;

impl Account {
    // --- company ---

    pub async fn insert_company(&self, row: &model::Company) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO company (id, source_name, display_name, legal_name, vat_number, billing_address, country, currency, payment_gate, terms_days, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(row.id)
        .bind(&row.source_name)
        .bind(&row.display_name)
        .bind(&row.legal_name)
        .bind(&row.vat_number)
        .bind(&row.billing_address)
        .bind(&row.country)
        .bind(&row.currency)
        .bind(&row.payment_gate)
        .bind(row.terms_days)
        .bind(&row.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_company(&self, id: Uuid) -> Result<Option<model::Company>, sqlx::Error> {
        sqlx::query_as::<_, model::Company>("SELECT id, source_name, display_name, legal_name, vat_number, billing_address, country, currency, payment_gate, terms_days, created_at FROM company WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- department ---

    pub async fn insert_department(&self, row: &model::Department) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO department (id, company_id, name, cost_center) VALUES ($1, $2, $3, $4)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.name)
        .bind(&row.cost_center)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_department(&self, id: Uuid) -> Result<Option<model::Department>, sqlx::Error> {
        sqlx::query_as::<_, model::Department>("SELECT id, company_id, name, cost_center FROM department WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- membership ---

    pub async fn insert_membership(&self, row: &model::Membership) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO membership (id, company_id, email, identity_id, role, department_id, invited_at, accepted_at, removed_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.email)
        .bind(row.identity_id)
        .bind(&row.role)
        .bind(row.department_id)
        .bind(&row.invited_at)
        .bind(&row.accepted_at)
        .bind(&row.removed_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_membership(&self, id: Uuid) -> Result<Option<model::Membership>, sqlx::Error> {
        sqlx::query_as::<_, model::Membership>("SELECT id, company_id, email, identity_id, role, department_id, invited_at, accepted_at, removed_at FROM membership WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- staff_member ---

    pub async fn insert_staff_member(&self, row: &model::StaffMember) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO staff_member (id, email, identity_id, role, active) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(row.id)
        .bind(&row.email)
        .bind(row.identity_id)
        .bind(&row.role)
        .bind(row.active)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_staff_member(&self, id: Uuid) -> Result<Option<model::StaffMember>, sqlx::Error> {
        sqlx::query_as::<_, model::StaffMember>("SELECT id, email, identity_id, role, active FROM staff_member WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- address ---

    pub async fn insert_address(&self, row: &model::Address) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO address (id, company_id, label, contact_name, street, house_number, postcode, city, region, country, phone, archived_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.label)
        .bind(&row.contact_name)
        .bind(&row.street)
        .bind(&row.house_number)
        .bind(&row.postcode)
        .bind(&row.city)
        .bind(&row.region)
        .bind(&row.country)
        .bind(&row.phone)
        .bind(&row.archived_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_address(&self, id: Uuid) -> Result<Option<model::Address>, sqlx::Error> {
        sqlx::query_as::<_, model::Address>("SELECT id, company_id, label, contact_name, street, house_number, postcode, city, region, country, phone, archived_at FROM address WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- recipient ---

    pub async fn insert_recipient(&self, row: &model::Recipient) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO recipient (id, company_id, email, first_name, last_name, phone, home_address, labels, source, erase_after, erased_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.email)
        .bind(&row.first_name)
        .bind(&row.last_name)
        .bind(&row.phone)
        .bind(&row.home_address)
        .bind(&row.labels)
        .bind(&row.source)
        .bind(&row.erase_after)
        .bind(&row.erased_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_recipient(&self, id: Uuid) -> Result<Option<model::Recipient>, sqlx::Error> {
        sqlx::query_as::<_, model::Recipient>("SELECT id, company_id, email, first_name, last_name, phone, home_address, labels, source, erase_after, erased_at FROM recipient WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
    // --- audit_event ---

    pub async fn insert_audit_event(&self, row: &model::AuditEvent) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO audit_event (id, company_id, actor, action, subject, detail, occurred_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(row.id)
        .bind(row.company_id)
        .bind(&row.actor)
        .bind(&row.action)
        .bind(&row.subject)
        .bind(&row.detail)
        .bind(&row.occurred_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_audit_event(&self, id: Uuid) -> Result<Option<model::AuditEvent>, sqlx::Error> {
        sqlx::query_as::<_, model::AuditEvent>("SELECT id, company_id, actor, action, subject, detail, occurred_at FROM audit_event WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }
}
