//! `BillingService`: the Connect service of `billing`. Each
//! method validates the request, converts the buffa message into the plain
//! `messages` struct, sends it through the api sender, and converts the
//! answer back. Errors map to Connect codes through basable_connect.

use std::sync::Arc;

use basable_connect::{ConnectRouter, into_connect_error, request_ctx};
use connectrpc::{RequestContext, ServiceRequest, ServiceResult};
use interfaces::{ApiRoutes, ApiSender};
use proto::connect::billing::v1::BillingService;
use proto::proto::billing::v1::*;

use crate::Api;

/// The service holds the api sender (exactly the sends `api` declares),
/// built once from the router at mount.
pub struct BillingServiceImpl<R: ApiRoutes + Send + Sync + 'static> {
    sender: ApiSender<'static, R>,
}

// The generated trait returns `impl Encodable`; answering with the owned
// message is the refinement this allow names.
#[allow(refining_impl_trait)]
impl<R: ApiRoutes + Send + Sync + 'static> BillingService for BillingServiceImpl<R> {
    async fn list_invoices(&self, ctx: RequestContext, request: ServiceRequest<'_, ListInvoicesRequest>) -> ServiceResult<ListInvoicesResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListInvoicesRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListInvoicesResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_invoices")))
    }
    async fn get_invoice(&self, ctx: RequestContext, request: ServiceRequest<'_, GetInvoiceRequest>) -> ServiceResult<GetInvoiceResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetInvoiceRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetInvoiceResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_invoice")))
    }
    async fn get_invoice_download(&self, ctx: RequestContext, request: ServiceRequest<'_, GetInvoiceDownloadRequest>) -> ServiceResult<GetInvoiceDownloadResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetInvoiceDownloadRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetInvoiceDownloadResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_invoice_download")))
    }
    async fn pay_invoice_by_card(&self, ctx: RequestContext, request: ServiceRequest<'_, PayInvoiceByCardRequest>) -> ServiceResult<PayInvoiceByCardResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::PayInvoiceByCardRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `PayInvoiceByCardResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.pay_invoice_by_card")))
    }
    async fn record_bank_payment(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordBankPaymentRequest>) -> ServiceResult<RecordBankPaymentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordBankPaymentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordBankPaymentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_bank_payment")))
    }
    async fn issue_credit_note(&self, ctx: RequestContext, request: ServiceRequest<'_, IssueCreditNoteRequest>) -> ServiceResult<IssueCreditNoteResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::IssueCreditNoteRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `IssueCreditNoteResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.issue_credit_note")))
    }
    async fn record_refund(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordRefundRequest>) -> ServiceResult<RecordRefundResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordRefundRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordRefundResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_refund")))
    }
    async fn get_spend_report(&self, ctx: RequestContext, request: ServiceRequest<'_, GetSpendReportRequest>) -> ServiceResult<GetSpendReportResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetSpendReportRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetSpendReportResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_spend_report")))
    }
    async fn draft_vat_return(&self, ctx: RequestContext, request: ServiceRequest<'_, DraftVatReturnRequest>) -> ServiceResult<DraftVatReturnResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::DraftVatReturnRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `DraftVatReturnResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.draft_vat_return")))
    }
    async fn mark_vat_return_filed(&self, ctx: RequestContext, request: ServiceRequest<'_, MarkVatReturnFiledRequest>) -> ServiceResult<MarkVatReturnFiledResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::MarkVatReturnFiledRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `MarkVatReturnFiledResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.mark_vat_return_filed")))
    }
    async fn get_trial_balance(&self, ctx: RequestContext, request: ServiceRequest<'_, GetTrialBalanceRequest>) -> ServiceResult<GetTrialBalanceResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetTrialBalanceRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetTrialBalanceResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_trial_balance")))
    }
    async fn list_supplier_bills(&self, ctx: RequestContext, request: ServiceRequest<'_, ListSupplierBillsRequest>) -> ServiceResult<ListSupplierBillsResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ListSupplierBillsRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ListSupplierBillsResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.list_supplier_bills")))
    }
    async fn get_supplier_bill(&self, ctx: RequestContext, request: ServiceRequest<'_, GetSupplierBillRequest>) -> ServiceResult<GetSupplierBillResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::GetSupplierBillRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `GetSupplierBillResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.get_supplier_bill")))
    }
    async fn approve_supplier_bill(&self, ctx: RequestContext, request: ServiceRequest<'_, ApproveSupplierBillRequest>) -> ServiceResult<ApproveSupplierBillResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::ApproveSupplierBillRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `ApproveSupplierBillResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.approve_supplier_bill")))
    }
    async fn void_supplier_bill(&self, ctx: RequestContext, request: ServiceRequest<'_, VoidSupplierBillRequest>) -> ServiceResult<VoidSupplierBillResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::VoidSupplierBillRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `VoidSupplierBillResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.void_supplier_bill")))
    }
    async fn record_supplier_payment(&self, ctx: RequestContext, request: ServiceRequest<'_, RecordSupplierPaymentRequest>) -> ServiceResult<RecordSupplierPaymentResponse> {
        let ctx = request_ctx(&ctx);
        let _req = request.to_owned_message();
        // TODO: validate `_req`, convert it into `messages::RecordSupplierPaymentRequest`,
        // send it with `self.sender.send_…(&ctx, m).await`,
        // and convert the answer into `RecordSupplierPaymentResponse` (`Ok(connectrpc::Response::new(..))`).
        let _ = (&ctx, self.sender);
        Err(into_connect_error(crate::unimplemented_step("api.record_supplier_payment")))
    }
}

pub fn mount<R>(connect: &mut ConnectRouter, _api: &'static Api, router: &'static R)
where
    R: ApiRoutes + Send + Sync + 'static,
{
    connect.add_service(Arc::new(BillingServiceImpl { sender: ApiSender::new(router) }));
}
