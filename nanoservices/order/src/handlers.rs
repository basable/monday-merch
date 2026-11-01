//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{OrderHandler, OrderRoutes, OrderSender};

use crate::Order;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: OrderRoutes> OrderHandler<R> for Order {
    async fn handle_request_quote_request(&self, _ctx: &Ctx, _msg: messages::RequestQuoteRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_request_quote_request"))
    }
    async fn handle_list_quotes_request(&self, _ctx: &Ctx, _msg: messages::ListQuotesRequest, _s: OrderSender<'_, R>) -> Result<messages::QuoteList, AppError> {
        Err(crate::unimplemented_step("order.handle_list_quotes_request"))
    }
    async fn handle_get_quote_request(&self, _ctx: &Ctx, _msg: messages::GetQuoteRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_get_quote_request"))
    }
    async fn handle_assign_quote_request(&self, _ctx: &Ctx, _msg: messages::AssignQuoteRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_assign_quote_request"))
    }
    async fn handle_publish_quote_version_request(&self, _ctx: &Ctx, _msg: messages::PublishQuoteVersionRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_publish_quote_version_request"))
    }
    async fn handle_request_quote_changes_request(&self, _ctx: &Ctx, _msg: messages::RequestQuoteChangesRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_request_quote_changes_request"))
    }
    async fn handle_accept_quote_request(&self, _ctx: &Ctx, _msg: messages::AcceptQuoteRequest, _s: OrderSender<'_, R>) -> Result<messages::Order, AppError> {
        Err(crate::unimplemented_step("order.handle_accept_quote_request"))
    }
    async fn handle_reorder_request(&self, _ctx: &Ctx, _msg: messages::ReorderRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_reorder_request"))
    }
    async fn handle_list_orders_request(&self, _ctx: &Ctx, _msg: messages::ListOrdersRequest, _s: OrderSender<'_, R>) -> Result<messages::OrderList, AppError> {
        Err(crate::unimplemented_step("order.handle_list_orders_request"))
    }
    async fn handle_get_order_request(&self, _ctx: &Ctx, _msg: messages::GetOrderRequest, _s: OrderSender<'_, R>) -> Result<messages::Order, AppError> {
        Err(crate::unimplemented_step("order.handle_get_order_request"))
    }
    async fn handle_cancel_order_request(&self, _ctx: &Ctx, _msg: messages::CancelOrderRequest, _s: OrderSender<'_, R>) -> Result<messages::Order, AppError> {
        Err(crate::unimplemented_step("order.handle_cancel_order_request"))
    }
    async fn handle_list_production_runs_request(&self, _ctx: &Ctx, _msg: messages::ListProductionRunsRequest, _s: OrderSender<'_, R>) -> Result<messages::ProductionRunList, AppError> {
        Err(crate::unimplemented_step("order.handle_list_production_runs_request"))
    }
    async fn handle_record_production_event_request(&self, _ctx: &Ctx, _msg: messages::RecordProductionEventRequest, _s: OrderSender<'_, R>) -> Result<messages::ProductionRun, AppError> {
        Err(crate::unimplemented_step("order.handle_record_production_event_request"))
    }
    async fn handle_submit_lead_request(&self, _ctx: &Ctx, _msg: messages::SubmitLeadRequest, _s: OrderSender<'_, R>) -> Result<messages::LeadAck, AppError> {
        Err(crate::unimplemented_step("order.handle_submit_lead_request"))
    }
    async fn handle_record_supplier_update_request(&self, _ctx: &Ctx, _msg: messages::RecordSupplierUpdateRequest, _s: OrderSender<'_, R>) -> Result<messages::ProductionRun, AppError> {
        Err(crate::unimplemented_step("order.handle_record_supplier_update_request"))
    }
    async fn handle_ingest_customer_email_request(&self, _ctx: &Ctx, _msg: messages::IngestCustomerEmailRequest, _s: OrderSender<'_, R>) -> Result<messages::Quote, AppError> {
        Err(crate::unimplemented_step("order.handle_ingest_customer_email_request"))
    }
    async fn handle_get_purchase_match_request(&self, _ctx: &Ctx, _msg: messages::GetPurchaseMatchRequest, _s: OrderSender<'_, R>) -> Result<messages::PurchaseMatch, AppError> {
        Err(crate::unimplemented_step("order.handle_get_purchase_match_request"))
    }
    async fn handle_invoice_changed(&self, _ctx: &Ctx, _msg: messages::InvoiceChanged, _s: OrderSender<'_, R>) {
        // A void fan-out event: observe it, nudge an object or write a side
        // table owned by this nanoservice; never a status write.
        tracing::warn!(step = "order.handle_invoice_changed", "unimplemented step reached");
    }
    async fn handle_inbound_changed(&self, _ctx: &Ctx, _msg: messages::InboundChanged, _s: OrderSender<'_, R>) {
        // A void fan-out event: observe it, nudge an object or write a side
        // table owned by this nanoservice; never a status write.
        tracing::warn!(step = "order.handle_inbound_changed", "unimplemented step reached");
    }
    async fn handle_shipment_changed(&self, _ctx: &Ctx, _msg: messages::ShipmentChanged, _s: OrderSender<'_, R>) {
        // A void fan-out event: observe it, nudge an object or write a side
        // table owned by this nanoservice; never a status write.
        tracing::warn!(step = "order.handle_shipment_changed", "unimplemented step reached");
    }
}
