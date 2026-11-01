//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{FulfilmentHandler, FulfilmentRoutes, FulfilmentSender};

use crate::Fulfilment;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: FulfilmentRoutes> FulfilmentHandler<R> for Fulfilment {
    async fn handle_estimate_delivery_request(&self, _ctx: &Ctx, _msg: messages::EstimateDeliveryRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::DeliveryEstimate, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_estimate_delivery_request"))
    }
    async fn handle_list_stock_request(&self, _ctx: &Ctx, _msg: messages::ListStockRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::StockList, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_list_stock_request"))
    }
    async fn handle_set_low_stock_rule_request(&self, _ctx: &Ctx, _msg: messages::SetLowStockRuleRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::LowStockRule, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_set_low_stock_rule_request"))
    }
    async fn handle_adjust_stock_request(&self, _ctx: &Ctx, _msg: messages::AdjustStockRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::StockLevel, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_adjust_stock_request"))
    }
    async fn handle_list_kits_request(&self, _ctx: &Ctx, _msg: messages::ListKitsRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::KitList, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_list_kits_request"))
    }
    async fn handle_upsert_kit_request(&self, _ctx: &Ctx, _msg: messages::UpsertKitRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Kit, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_upsert_kit_request"))
    }
    async fn handle_create_assembly_order_request(&self, _ctx: &Ctx, _msg: messages::CreateAssemblyOrderRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::AssemblyOrder, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_create_assembly_order_request"))
    }
    async fn handle_record_assembly_request(&self, _ctx: &Ctx, _msg: messages::RecordAssemblyRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::AssemblyOrder, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_record_assembly_request"))
    }
    async fn handle_list_inbound_request(&self, _ctx: &Ctx, _msg: messages::ListInboundRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::InboundList, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_list_inbound_request"))
    }
    async fn handle_record_inbound_receipt_request(&self, _ctx: &Ctx, _msg: messages::RecordInboundReceiptRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Inbound, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_record_inbound_receipt_request"))
    }
    async fn handle_create_shipments_request(&self, _ctx: &Ctx, _msg: messages::CreateShipmentsRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::ShipmentBatch, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_create_shipments_request"))
    }
    async fn handle_list_shipments_request(&self, _ctx: &Ctx, _msg: messages::ListShipmentsRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::ShipmentList, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_list_shipments_request"))
    }
    async fn handle_get_shipment_request(&self, _ctx: &Ctx, _msg: messages::GetShipmentRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Shipment, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_get_shipment_request"))
    }
    async fn handle_record_pick_request(&self, _ctx: &Ctx, _msg: messages::RecordPickRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Shipment, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_record_pick_request"))
    }
    async fn handle_cancel_shipment_request(&self, _ctx: &Ctx, _msg: messages::CancelShipmentRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Shipment, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_cancel_shipment_request"))
    }
    async fn handle_ensure_inbound_request(&self, _ctx: &Ctx, _msg: messages::EnsureInboundRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Inbound, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_ensure_inbound_request"))
    }
    async fn handle_get_inbound_request(&self, _ctx: &Ctx, _msg: messages::GetInboundRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Inbound, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_get_inbound_request"))
    }
    async fn handle_ensure_shipment_request(&self, _ctx: &Ctx, _msg: messages::EnsureShipmentRequest, _s: FulfilmentSender<'_, R>) -> Result<messages::Shipment, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_ensure_shipment_request"))
    }
    async fn handle_carrier_webhook_event(&self, _ctx: &Ctx, _msg: messages::CarrierWebhookEvent, _s: FulfilmentSender<'_, R>) -> Result<messages::WebhookAck, AppError> {
        Err(crate::unimplemented_step("fulfilment.handle_carrier_webhook_event"))
    }
}
