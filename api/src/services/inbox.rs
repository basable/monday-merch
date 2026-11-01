//! `InboxService`: the Connect service of `inbox`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::inbox::v1::InboxService;
use proto::proto::inbox::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct InboxServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> InboxService for InboxServiceImpl<R> {
    async fn list_inbound_messages(&self, ctx: RequestContext, request: ServiceRequest<'_, ListInboundMessagesRequest>) -> ServiceResult<ListInboundMessagesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListInboundMessagesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListInboundMessagesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_inbound_messages")))
    }
    async fn list_review_queue(&self, ctx: RequestContext, request: ServiceRequest<'_, ListReviewQueueRequest>) -> ServiceResult<ListReviewQueueResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListReviewQueueRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListReviewQueueResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_review_queue")))
    }
    async fn get_inbound_document(&self, ctx: RequestContext, request: ServiceRequest<'_, GetInboundDocumentRequest>) -> ServiceResult<GetInboundDocumentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetInboundDocumentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetInboundDocumentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_inbound_document")))
    }
    async fn get_inbound_file_link(&self, ctx: RequestContext, request: ServiceRequest<'_, GetInboundFileLinkRequest>) -> ServiceResult<GetInboundFileLinkResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetInboundFileLinkRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetInboundFileLinkResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_inbound_file_link")))
    }
    async fn decide_inbound_document(&self, ctx: RequestContext, request: ServiceRequest<'_, DecideInboundDocumentRequest>) -> ServiceResult<DecideInboundDocumentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::DecideInboundDocumentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `DecideInboundDocumentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.decide_inbound_document")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(InboxServiceImpl { sender: ApiSender::new(router) }));
}
