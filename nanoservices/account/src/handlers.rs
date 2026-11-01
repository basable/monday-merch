//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{AccountHandler, AccountRoutes, AccountSender};

use crate::Account;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: AccountRoutes> AccountHandler<R> for Account {
    async fn handle_get_company_request(&self, _ctx: &Ctx, _msg: messages::GetCompanyRequest, _s: AccountSender<'_, R>) -> Result<messages::Company, AppError> {
        Err(crate::unimplemented_step("account.handle_get_company_request"))
    }
    async fn handle_update_company_request(&self, _ctx: &Ctx, _msg: messages::UpdateCompanyRequest, _s: AccountSender<'_, R>) -> Result<messages::Company, AppError> {
        Err(crate::unimplemented_step("account.handle_update_company_request"))
    }
    async fn handle_ensure_company_request(&self, _ctx: &Ctx, _msg: messages::EnsureCompanyRequest, _s: AccountSender<'_, R>) -> Result<messages::Company, AppError> {
        Err(crate::unimplemented_step("account.handle_ensure_company_request"))
    }
    async fn handle_resolve_member_request(&self, _ctx: &Ctx, _msg: messages::ResolveMemberRequest, _s: AccountSender<'_, R>) -> Result<messages::Member, AppError> {
        Err(crate::unimplemented_step("account.handle_resolve_member_request"))
    }
    async fn handle_list_members_request(&self, _ctx: &Ctx, _msg: messages::ListMembersRequest, _s: AccountSender<'_, R>) -> Result<messages::MemberList, AppError> {
        Err(crate::unimplemented_step("account.handle_list_members_request"))
    }
    async fn handle_invite_member_request(&self, _ctx: &Ctx, _msg: messages::InviteMemberRequest, _s: AccountSender<'_, R>) -> Result<messages::Member, AppError> {
        Err(crate::unimplemented_step("account.handle_invite_member_request"))
    }
    async fn handle_update_member_request(&self, _ctx: &Ctx, _msg: messages::UpdateMemberRequest, _s: AccountSender<'_, R>) -> Result<messages::Member, AppError> {
        Err(crate::unimplemented_step("account.handle_update_member_request"))
    }
    async fn handle_remove_member_request(&self, _ctx: &Ctx, _msg: messages::RemoveMemberRequest, _s: AccountSender<'_, R>) -> Result<messages::Member, AppError> {
        Err(crate::unimplemented_step("account.handle_remove_member_request"))
    }
    async fn handle_list_departments_request(&self, _ctx: &Ctx, _msg: messages::ListDepartmentsRequest, _s: AccountSender<'_, R>) -> Result<messages::DepartmentList, AppError> {
        Err(crate::unimplemented_step("account.handle_list_departments_request"))
    }
    async fn handle_upsert_department_request(&self, _ctx: &Ctx, _msg: messages::UpsertDepartmentRequest, _s: AccountSender<'_, R>) -> Result<messages::Department, AppError> {
        Err(crate::unimplemented_step("account.handle_upsert_department_request"))
    }
    async fn handle_list_addresses_request(&self, _ctx: &Ctx, _msg: messages::ListAddressesRequest, _s: AccountSender<'_, R>) -> Result<messages::AddressList, AppError> {
        Err(crate::unimplemented_step("account.handle_list_addresses_request"))
    }
    async fn handle_upsert_address_request(&self, _ctx: &Ctx, _msg: messages::UpsertAddressRequest, _s: AccountSender<'_, R>) -> Result<messages::Address, AppError> {
        Err(crate::unimplemented_step("account.handle_upsert_address_request"))
    }
    async fn handle_list_recipients_request(&self, _ctx: &Ctx, _msg: messages::ListRecipientsRequest, _s: AccountSender<'_, R>) -> Result<messages::RecipientList, AppError> {
        Err(crate::unimplemented_step("account.handle_list_recipients_request"))
    }
    async fn handle_upsert_recipient_request(&self, _ctx: &Ctx, _msg: messages::UpsertRecipientRequest, _s: AccountSender<'_, R>) -> Result<messages::Recipient, AppError> {
        Err(crate::unimplemented_step("account.handle_upsert_recipient_request"))
    }
    async fn handle_import_recipients_request(&self, _ctx: &Ctx, _msg: messages::ImportRecipientsRequest, _s: AccountSender<'_, R>) -> Result<messages::RecipientImportResult, AppError> {
        Err(crate::unimplemented_step("account.handle_import_recipients_request"))
    }
    async fn handle_erase_recipient_request(&self, _ctx: &Ctx, _msg: messages::EraseRecipientRequest, _s: AccountSender<'_, R>) -> Result<messages::Recipient, AppError> {
        Err(crate::unimplemented_step("account.handle_erase_recipient_request"))
    }
    async fn handle_resolve_recipients_request(&self, _ctx: &Ctx, _msg: messages::ResolveRecipientsRequest, _s: AccountSender<'_, R>) -> Result<messages::RecipientList, AppError> {
        Err(crate::unimplemented_step("account.handle_resolve_recipients_request"))
    }
    async fn handle_list_audit_events_request(&self, _ctx: &Ctx, _msg: messages::ListAuditEventsRequest, _s: AccountSender<'_, R>) -> Result<messages::AuditEventList, AppError> {
        Err(crate::unimplemented_step("account.handle_list_audit_events_request"))
    }
    async fn handle_audit_event(&self, _ctx: &Ctx, _msg: messages::AuditEvent, _s: AccountSender<'_, R>) {
        // A void fan-out event: observe it, nudge an object or write a side
        // table owned by this nanoservice; never a status write.
        tracing::warn!(step = "account.handle_audit_event", "unimplemented step reached");
    }
}
