//! `FulfilmentService`: the Connect service of `fulfilment`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::fulfilment::v1::FulfilmentService;
use proto::proto::fulfilment::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct FulfilmentServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> FulfilmentService for FulfilmentServiceImpl<R> {
    async fn estimate_delivery(&self, ctx: RequestContext, request: ServiceRequest<'_, EstimateDeliveryRequest>) -> ServiceResult<EstimateDeliveryResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::EstimateDeliveryRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `EstimateDeliveryResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.estimate_delivery")))
    }
    async fn list_stock(&self, ctx: RequestContext, request: ServiceRequest<'_, ListStockRequest>) -> ServiceResult<ListStockResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListStockRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListStockResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_stock")))
    }
    async fn set_low_stock_rule(&self, ctx: RequestContext, request: ServiceRequest<'_, SetLowStockRuleRequest>) -> ServiceResult<SetLowStockRuleResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::SetLowStockRuleRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `SetLowStockRuleResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.set_low_stock_rule")))
    }
    async fn adjust_stock(&self, ctx: RequestContext, request: ServiceRequest<'_, AdjustStockRequest>) -> ServiceResult<AdjustStockResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::AdjustStockRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `AdjustStockResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.adjust_stock")))
    }
    async fn list_kits(&self, ctx: RequestContext, request: ServiceRequest<'_, ListKitsRequest>) -> ServiceResult<ListKitsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListKitsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListKitsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_kits")))
    }
    async fn upsert_kit(&self, ctx: RequestContext, request: ServiceRequest<'_, UpsertKitRequest>) -> ServiceResult<UpsertKitResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::UpsertKitRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `UpsertKitResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.upsert_kit")))
    }
    async fn create_assembly_order(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateAssemblyOrderRequest>) -> ServiceResult<CreateAssemblyOrderResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateAssemblyOrderRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateAssemblyOrderResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_assembly_order")))
    }
    async fn record_assembly(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordAssemblyRequest>) -> ServiceResult<RecordAssemblyResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordAssemblyRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordAssemblyResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_assembly")))
    }
    async fn list_inbound(&self, ctx: RequestContext, request: ServiceRequest<'_, ListInboundRequest>) -> ServiceResult<ListInboundResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListInboundRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListInboundResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_inbound")))
    }
    async fn record_inbound_receipt(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordInboundReceiptRequest>) -> ServiceResult<RecordInboundReceiptResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordInboundReceiptRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordInboundReceiptResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_inbound_receipt")))
    }
    async fn create_shipments(&self, ctx: RequestContext, request: ServiceRequest<'_, CreateShipmentsRequest>) -> ServiceResult<CreateShipmentsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CreateShipmentsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CreateShipmentsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.create_shipments")))
    }
    async fn list_shipments(&self, ctx: RequestContext, request: ServiceRequest<'_, ListShipmentsRequest>) -> ServiceResult<ListShipmentsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListShipmentsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListShipmentsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_shipments")))
    }
    async fn get_shipment(&self, ctx: RequestContext, request: ServiceRequest<'_, GetShipmentRequest>) -> ServiceResult<GetShipmentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetShipmentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetShipmentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_shipment")))
    }
    async fn record_pick(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordPickRequest>) -> ServiceResult<RecordPickResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordPickRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordPickResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_pick")))
    }
    async fn cancel_shipment(&self, ctx: RequestContext, request: ServiceRequest<'_, CancelShipmentRequest>) -> ServiceResult<CancelShipmentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::CancelShipmentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `CancelShipmentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.cancel_shipment")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(FulfilmentServiceImpl { sender: ApiSender::new(router) }));
}
