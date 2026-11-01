//! The handler trait implementation: one method per message this
//! nanoservice handles (routing.yaml). Handlers are the INTENT boundary:
//! they validate, write desired state through the typed store (`create`,
//! `update_spec`, `mark_deleted`, `nudge`) or the repository, dispatch
//! request-path adapters `Unfenced`, and send through `s` (the sender that
//! exposes exactly the declared sends). They never write status.

use basable_core::{AppError, Ctx};
use interfaces::{BillingHandler, BillingRoutes, BillingSender};

use crate::Billing;

// Messages stay qualified: a message may share its name with this
// component (an `order` nanoservice handling an `Order` response).
impl<R: BillingRoutes> BillingHandler<R> for Billing {
    async fn handle_ensure_invoice_request(&self, _ctx: &Ctx, _msg: messages::EnsureInvoiceRequest, _s: BillingSender<'_, R>) -> Result<messages::Invoice, AppError> {
        Err(crate::unimplemented_step("billing.handle_ensure_invoice_request"))
    }
    async fn handle_list_invoices_request(&self, _ctx: &Ctx, _msg: messages::ListInvoicesRequest, _s: BillingSender<'_, R>) -> Result<messages::InvoiceList, AppError> {
        Err(crate::unimplemented_step("billing.handle_list_invoices_request"))
    }
    async fn handle_get_invoice_request(&self, _ctx: &Ctx, _msg: messages::GetInvoiceRequest, _s: BillingSender<'_, R>) -> Result<messages::Invoice, AppError> {
        Err(crate::unimplemented_step("billing.handle_get_invoice_request"))
    }
    async fn handle_get_invoice_download_request(&self, _ctx: &Ctx, _msg: messages::GetInvoiceDownloadRequest, _s: BillingSender<'_, R>) -> Result<messages::DownloadLink, AppError> {
        Err(crate::unimplemented_step("billing.handle_get_invoice_download_request"))
    }
    async fn handle_pay_invoice_by_card_request(&self, _ctx: &Ctx, _msg: messages::PayInvoiceByCardRequest, _s: BillingSender<'_, R>) -> Result<messages::CheckoutLink, AppError> {
        Err(crate::unimplemented_step("billing.handle_pay_invoice_by_card_request"))
    }
    async fn handle_record_bank_payment_request(&self, _ctx: &Ctx, _msg: messages::RecordBankPaymentRequest, _s: BillingSender<'_, R>) -> Result<messages::Payment, AppError> {
        Err(crate::unimplemented_step("billing.handle_record_bank_payment_request"))
    }
    async fn handle_issue_credit_note_request(&self, _ctx: &Ctx, _msg: messages::IssueCreditNoteRequest, _s: BillingSender<'_, R>) -> Result<messages::Invoice, AppError> {
        Err(crate::unimplemented_step("billing.handle_issue_credit_note_request"))
    }
    async fn handle_record_refund_request(&self, _ctx: &Ctx, _msg: messages::RecordRefundRequest, _s: BillingSender<'_, R>) -> Result<messages::Payment, AppError> {
        Err(crate::unimplemented_step("billing.handle_record_refund_request"))
    }
    async fn handle_get_spend_report_request(&self, _ctx: &Ctx, _msg: messages::GetSpendReportRequest, _s: BillingSender<'_, R>) -> Result<messages::SpendReport, AppError> {
        Err(crate::unimplemented_step("billing.handle_get_spend_report_request"))
    }
    async fn handle_draft_vat_return_request(&self, _ctx: &Ctx, _msg: messages::DraftVatReturnRequest, _s: BillingSender<'_, R>) -> Result<messages::VatReturn, AppError> {
        Err(crate::unimplemented_step("billing.handle_draft_vat_return_request"))
    }
    async fn handle_mark_vat_return_filed_request(&self, _ctx: &Ctx, _msg: messages::MarkVatReturnFiledRequest, _s: BillingSender<'_, R>) -> Result<messages::VatReturn, AppError> {
        Err(crate::unimplemented_step("billing.handle_mark_vat_return_filed_request"))
    }
    async fn handle_get_trial_balance_request(&self, _ctx: &Ctx, _msg: messages::GetTrialBalanceRequest, _s: BillingSender<'_, R>) -> Result<messages::TrialBalance, AppError> {
        Err(crate::unimplemented_step("billing.handle_get_trial_balance_request"))
    }
    async fn handle_ensure_supplier_bill_request(&self, _ctx: &Ctx, _msg: messages::EnsureSupplierBillRequest, _s: BillingSender<'_, R>) -> Result<messages::SupplierBill, AppError> {
        Err(crate::unimplemented_step("billing.handle_ensure_supplier_bill_request"))
    }
    async fn handle_list_supplier_bills_request(&self, _ctx: &Ctx, _msg: messages::ListSupplierBillsRequest, _s: BillingSender<'_, R>) -> Result<messages::SupplierBillList, AppError> {
        Err(crate::unimplemented_step("billing.handle_list_supplier_bills_request"))
    }
    async fn handle_get_supplier_bill_request(&self, _ctx: &Ctx, _msg: messages::GetSupplierBillRequest, _s: BillingSender<'_, R>) -> Result<messages::SupplierBill, AppError> {
        Err(crate::unimplemented_step("billing.handle_get_supplier_bill_request"))
    }
    async fn handle_approve_supplier_bill_request(&self, _ctx: &Ctx, _msg: messages::ApproveSupplierBillRequest, _s: BillingSender<'_, R>) -> Result<messages::SupplierBill, AppError> {
        Err(crate::unimplemented_step("billing.handle_approve_supplier_bill_request"))
    }
    async fn handle_void_supplier_bill_request(&self, _ctx: &Ctx, _msg: messages::VoidSupplierBillRequest, _s: BillingSender<'_, R>) -> Result<messages::SupplierBill, AppError> {
        Err(crate::unimplemented_step("billing.handle_void_supplier_bill_request"))
    }
    async fn handle_record_supplier_payment_request(&self, _ctx: &Ctx, _msg: messages::RecordSupplierPaymentRequest, _s: BillingSender<'_, R>) -> Result<messages::Payment, AppError> {
        Err(crate::unimplemented_step("billing.handle_record_supplier_payment_request"))
    }
    async fn handle_payment_webhook_event(&self, _ctx: &Ctx, _msg: messages::PaymentWebhookEvent, _s: BillingSender<'_, R>) -> Result<messages::WebhookAck, AppError> {
        Err(crate::unimplemented_step("billing.handle_payment_webhook_event"))
    }
    /// Books the snapshots sent: any mix of object types across processes,
    /// the objects of one process atomically or not at all, one outcome per
    /// snapshot in the order sent. Rendered whole; the rules it books by are
    /// in `src/ledger/`.
    async fn handle_book(&self, _ctx: &Ctx, msg: messages::Book, _s: BillingSender<'_, R>) -> Result<messages::Booked, AppError> {
        let mut request = basable_ledger::Request::new();
        for snapshot in msg.snapshots {
            match snapshot {
                messages::LedgerSnapshot::Invoice(o) => {
                    request.add(o);
                }
                messages::LedgerSnapshot::CreditNote(o) => {
                    request.add(o);
                }
                messages::LedgerSnapshot::Payment(o) => {
                    request.add(o);
                }
                messages::LedgerSnapshot::VatReturn(o) => {
                    request.add(o);
                }
                messages::LedgerSnapshot::SupplierBill(o) => {
                    request.add(o);
                }
            }
        }
        let outcomes = self
            .ledger
            .book(request)
            .await
            .map_err(|e| AppError::internal(format!("ledger: {e}")))?;
        Ok(messages::Booked {
            outcomes: outcomes.into_iter().map(Into::into).collect(),
        })
    }
}
