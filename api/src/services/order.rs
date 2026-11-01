//! `OrderService`: the Connect service of `order`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::order::v1::OrderService;
use proto::proto::order::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct OrderServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> OrderService for OrderServiceImpl<R> {
    async fn request_quote(&self, ctx: RequestContext, request: ServiceRequest<'_, RequestQuoteRequest>) -> ServiceResult<RequestQuoteResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RequestQuoteRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RequestQuoteResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.request_quote")))
    }
    async fn list_quotes(&self, ctx: RequestContext, request: ServiceRequest<'_, ListQuotesRequest>) -> ServiceResult<ListQuotesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListQuotesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListQuotesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_quotes")))
    }
    async fn get_quote(&self, ctx: RequestContext, request: ServiceRequest<'_, GetQuoteRequest>) -> ServiceResult<GetQuoteResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetQuoteRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetQuoteResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_quote")))
    }
    async fn assign_quote(&self, ctx: RequestContext, request: ServiceRequest<'_, AssignQuoteRequest>) -> ServiceResult<AssignQuoteResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::AssignQuoteRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `AssignQuoteResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.assign_quote")))
    }
    async fn publish_quote_version(&self, ctx: RequestContext, request: ServiceRequest<'_, PublishQuoteVersionRequest>) -> ServiceResult<PublishQuoteVersionResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::PublishQuoteVersionRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `PublishQuoteVersionResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.publish_quote_version")))
    }
    async fn request_quote_changes(&self, ctx: RequestContext, request: ServiceRequest<'_, RequestQuoteChangesRequest>) -> ServiceResult<RequestQuoteChangesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RequestQuoteChangesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RequestQuoteChangesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.request_quote_changes")))
    }
    async fn accept_quote(&self, ctx: RequestContext, request: ServiceRequest<'_, AcceptQuoteRequest>) -> ServiceResult<AcceptQuoteResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::AcceptQuoteRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `AcceptQuoteResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.accept_quote")))
    }
    async fn reorder(&self, ctx: RequestContext, request: ServiceRequest<'_, ReorderRequest>) -> ServiceResult<ReorderResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ReorderRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ReorderResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.reorder")))
    }
    async fn list_orders(&self, ctx: RequestContext, request: ServiceRequest<'_, ListOrdersRequest>) -> ServiceResult<ListOrdersResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListOrdersRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListOrdersResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_orders")))
    }
    async fn get_order(&self, ctx: RequestContext, request: ServiceRequest<'_, GetOrderRequest>) -> ServiceResult<GetOrderResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetOrderRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetOrderResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_order")))
    }
    async fn cancel_order(&self, ctx: RequestContext, request: ServiceRequest<'_, CancelOrderRequest>) -> ServiceResult<CancelOrderResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CancelOrderRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CancelOrderResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.cancel_order")))
    }
    async fn list_production_runs(&self, ctx: RequestContext, request: ServiceRequest<'_, ListProductionRunsRequest>) -> ServiceResult<ListProductionRunsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListProductionRunsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListProductionRunsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_production_runs")))
    }
    async fn record_production_event(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordProductionEventRequest>) -> ServiceResult<RecordProductionEventResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordProductionEventRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordProductionEventResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_production_event")))
    }
    async fn submit_lead(&self, ctx: RequestContext, request: ServiceRequest<'_, SubmitLeadRequest>) -> ServiceResult<SubmitLeadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SubmitLeadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SubmitLeadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.submit_lead")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(OrderServiceImpl { sender: ApiSender::new(router) }));
}
