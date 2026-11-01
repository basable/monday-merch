//! `AccountService`: the Connect service of `account`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::account::v1::AccountService;
use proto::proto::account::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct AccountServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> AccountService for AccountServiceImpl<R> {
    async fn get_company(&self, ctx: RequestContext, request: ServiceRequest<'_, GetCompanyRequest>) -> ServiceResult<GetCompanyResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetCompanyRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetCompanyResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_company")))
    }
    async fn update_company(&self, ctx: RequestContext, request: ServiceRequest<'_, UpdateCompanyRequest>) -> ServiceResult<UpdateCompanyResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpdateCompanyRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpdateCompanyResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.update_company")))
    }
    async fn list_members(&self, ctx: RequestContext, request: ServiceRequest<'_, ListMembersRequest>) -> ServiceResult<ListMembersResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListMembersRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListMembersResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_members")))
    }
    async fn invite_member(&self, ctx: RequestContext, request: ServiceRequest<'_, InviteMemberRequest>) -> ServiceResult<InviteMemberResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::InviteMemberRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `InviteMemberResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.invite_member")))
    }
    async fn update_member(&self, ctx: RequestContext, request: ServiceRequest<'_, UpdateMemberRequest>) -> ServiceResult<UpdateMemberResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpdateMemberRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpdateMemberResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.update_member")))
    }
    async fn remove_member(&self, ctx: RequestContext, request: ServiceRequest<'_, RemoveMemberRequest>) -> ServiceResult<RemoveMemberResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RemoveMemberRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RemoveMemberResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.remove_member")))
    }
    async fn list_departments(&self, ctx: RequestContext, request: ServiceRequest<'_, ListDepartmentsRequest>) -> ServiceResult<ListDepartmentsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListDepartmentsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListDepartmentsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_departments")))
    }
    async fn upsert_department(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertDepartmentRequest>) -> ServiceResult<UpsertDepartmentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertDepartmentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertDepartmentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_department")))
    }
    async fn list_addresses(&self, ctx: RequestContext, request: ServiceRequest<'_, ListAddressesRequest>) -> ServiceResult<ListAddressesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListAddressesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListAddressesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_addresses")))
    }
    async fn upsert_address(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertAddressRequest>) -> ServiceResult<UpsertAddressResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertAddressRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertAddressResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_address")))
    }
    async fn list_recipients(&self, ctx: RequestContext, request: ServiceRequest<'_, ListRecipientsRequest>) -> ServiceResult<ListRecipientsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListRecipientsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListRecipientsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_recipients")))
    }
    async fn upsert_recipient(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertRecipientRequest>) -> ServiceResult<UpsertRecipientResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertRecipientRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertRecipientResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_recipient")))
    }
    async fn import_recipients(&self, ctx: RequestContext, request: ServiceRequest<'_, ImportRecipientsRequest>) -> ServiceResult<ImportRecipientsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ImportRecipientsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ImportRecipientsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.import_recipients")))
    }
    async fn erase_recipient(&self, ctx: RequestContext, request: ServiceRequest<'_, EraseRecipientRequest>) -> ServiceResult<EraseRecipientResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::EraseRecipientRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `EraseRecipientResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.erase_recipient")))
    }
    async fn list_audit_events(&self, ctx: RequestContext, request: ServiceRequest<'_, ListAuditEventsRequest>) -> ServiceResult<ListAuditEventsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListAuditEventsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListAuditEventsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_audit_events")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(AccountServiceImpl { sender: ApiSender::new(router) }));
}
