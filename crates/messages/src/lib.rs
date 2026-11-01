//! The messages nanoservices exchange. Every `message:` named in
//! `routing.yaml` is a struct here — the messenger codegen references them by
//! name, so a missing one is a build error naming it.
//!
//! Rules: a message carries the SENDER's generation when it hands derived
//! state to another nanoservice (the Directive §7); it names things by the
//! caller's seam (a name, never an id the receiver mints); only derived
//! state crosses. Keep them plain data: no methods that reach a database.

#![allow(unused_imports)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// basable:messages-modules-begin — the ledger's snapshot types, when a nanoservice owns the ledger
pub mod ledger;
pub use ledger::*;
// basable:messages-modules-end

// basable:messages-begin — declare one struct per message in routing.yaml.

/// `GetCompanyRequest` — answered with `Company` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyRequest {
    // TODO: the fields.
}

/// `UpdateCompanyRequest` — answered with `Company` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCompanyRequest {
    // TODO: the fields.
}

/// `EnsureCompanyRequest` — answered with `Company` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureCompanyRequest {
    // TODO: the fields.
}

/// `ResolveMemberRequest` — answered with `Member` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveMemberRequest {
    // TODO: the fields.
}

/// `ListMembersRequest` — answered with `MemberList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMembersRequest {
    // TODO: the fields.
}

/// `InviteMemberRequest` — answered with `Member` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteMemberRequest {
    // TODO: the fields.
}

/// `UpdateMemberRequest` — answered with `Member` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMemberRequest {
    // TODO: the fields.
}

/// `RemoveMemberRequest` — answered with `Member` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveMemberRequest {
    // TODO: the fields.
}

/// `ListDepartmentsRequest` — answered with `DepartmentList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDepartmentsRequest {
    // TODO: the fields.
}

/// `UpsertDepartmentRequest` — answered with `Department` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertDepartmentRequest {
    // TODO: the fields.
}

/// `ListAddressesRequest` — answered with `AddressList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAddressesRequest {
    // TODO: the fields.
}

/// `UpsertAddressRequest` — answered with `Address` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertAddressRequest {
    // TODO: the fields.
}

/// `ListRecipientsRequest` — answered with `RecipientList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRecipientsRequest {
    // TODO: the fields.
}

/// `UpsertRecipientRequest` — answered with `Recipient` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertRecipientRequest {
    // TODO: the fields.
}

/// `ImportRecipientsRequest` — answered with `RecipientImportResult` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRecipientsRequest {
    // TODO: the fields.
}

/// `EraseRecipientRequest` — answered with `Recipient` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EraseRecipientRequest {
    // TODO: the fields.
}

/// `ResolveRecipientsRequest` — answered with `RecipientList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveRecipientsRequest {
    // TODO: the fields.
}

/// `ListAuditEventsRequest` — answered with `AuditEventList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAuditEventsRequest {
    // TODO: the fields.
}

/// `AuditEvent` — a void fan-out event (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEvent {
    // TODO: the fields.
}

/// `SendEmailRequest` — answered with `EmailReceipt` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    // TODO: the fields.
}

/// `ListProductsRequest` — answered with `ProductPage` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProductsRequest {
    // TODO: the fields.
}

/// `GetProductRequest` — answered with `Product` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetProductRequest {
    // TODO: the fields.
}

/// `ListCategoriesRequest` — answered with `CategoryList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCategoriesRequest {
    // TODO: the fields.
}

/// `PriceLinesRequest` — answered with `PricedLines` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLinesRequest {
    // TODO: the fields.
}

/// `UpsertProductRequest` — answered with `Product` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertProductRequest {
    // TODO: the fields.
}

/// `UpsertVariantRequest` — answered with `Variant` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertVariantRequest {
    // TODO: the fields.
}

/// `SetPriceTiersRequest` — answered with `Product` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetPriceTiersRequest {
    // TODO: the fields.
}

/// `SetDecorationOptionsRequest` — answered with `Product` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetDecorationOptionsRequest {
    // TODO: the fields.
}

/// `SetSupplierOffersRequest` — answered with `SupplierOffers` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSupplierOffersRequest {
    // TODO: the fields.
}

/// `CreateImageUploadRequest` — answered with `ImageUpload` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImageUploadRequest {
    // TODO: the fields.
}

/// `ArchiveProductRequest` — answered with `Product` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveProductRequest {
    // TODO: the fields.
}

/// `GetSupplierOffersRequest` — answered with `SupplierOffers` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupplierOffersRequest {
    // TODO: the fields.
}

/// `GetCustomsDataRequest` — answered with `CustomsData` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomsDataRequest {
    // TODO: the fields.
}

/// `MatchSupplierRequest` — answered with `SupplierMatch` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSupplierRequest {
    // TODO: the fields.
}

/// `RenderShopPageRequest` — answered with `ShopPage` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderShopPageRequest {
    // TODO: the fields.
}

/// `GetBrandProfileRequest` — answered with `BrandProfile` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBrandProfileRequest {
    // TODO: the fields.
}

/// `UpdateBrandProfileRequest` — answered with `BrandProfile` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBrandProfileRequest {
    // TODO: the fields.
}

/// `ListBrandAssetsRequest` — answered with `BrandAssetList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListBrandAssetsRequest {
    // TODO: the fields.
}

/// `CreateAssetUploadRequest` — answered with `AssetUpload` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssetUploadRequest {
    // TODO: the fields.
}

/// `ConfirmAssetUploadRequest` — answered with `BrandAsset` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfirmAssetUploadRequest {
    // TODO: the fields.
}

/// `DeleteBrandAssetRequest` — answered with `BrandAsset` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteBrandAssetRequest {
    // TODO: the fields.
}

/// `CreateProofRequest` — answered with `Proof` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateProofRequest {
    // TODO: the fields.
}

/// `ListProofsRequest` — answered with `ProofList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProofsRequest {
    // TODO: the fields.
}

/// `ApproveProofsRequest` — answered with `ProofApproval` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveProofsRequest {
    // TODO: the fields.
}

/// `GetApprovedArtworkRequest` — answered with `ArtworkBundle` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApprovedArtworkRequest {
    // TODO: the fields.
}

/// `CreateDesignRequest` — answered with `DesignDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDesignRequest {
    // TODO: the fields.
}

/// `GetDesignRequest` — answered with `DesignDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDesignRequest {
    // TODO: the fields.
}

/// `ListDesignsRequest` — answered with `DesignList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDesignsRequest {
    // TODO: the fields.
}

/// `SaveDesignDraftRequest` — answered with `DesignDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDesignDraftRequest {
    // TODO: the fields.
}

/// `SaveDesignVersionRequest` — answered with `DesignVersion` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveDesignVersionRequest {
    // TODO: the fields.
}

/// `ArchiveDesignRequest` — answered with `DesignDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveDesignRequest {
    // TODO: the fields.
}

/// `CreatePreviewUploadRequest` — answered with `PreviewUpload` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePreviewUploadRequest {
    // TODO: the fields.
}

/// `ListDesignTemplatesRequest` — answered with `DesignTemplateList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDesignTemplatesRequest {
    // TODO: the fields.
}

/// `GetDesignTemplateRequest` — answered with `DesignTemplate` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDesignTemplateRequest {
    // TODO: the fields.
}

/// `UpsertDesignTemplateRequest` — answered with `DesignTemplate` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertDesignTemplateRequest {
    // TODO: the fields.
}

/// `ResolveDesignVersionRequest` — answered with `DesignVersionRef` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolveDesignVersionRequest {
    // TODO: the fields.
}

/// `AdoptDesignsRequest` — answered with `DesignAdoption` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdoptDesignsRequest {
    // TODO: the fields.
}

/// `RequestQuoteRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestQuoteRequest {
    // TODO: the fields.
}

/// `ListQuotesRequest` — answered with `QuoteList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListQuotesRequest {
    // TODO: the fields.
}

/// `GetQuoteRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetQuoteRequest {
    // TODO: the fields.
}

/// `AssignQuoteRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssignQuoteRequest {
    // TODO: the fields.
}

/// `PublishQuoteVersionRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishQuoteVersionRequest {
    // TODO: the fields.
}

/// `RequestQuoteChangesRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestQuoteChangesRequest {
    // TODO: the fields.
}

/// `AcceptQuoteRequest` — answered with `Order` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AcceptQuoteRequest {
    // TODO: the fields.
}

/// `ReorderRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReorderRequest {
    // TODO: the fields.
}

/// `ListOrdersRequest` — answered with `OrderList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListOrdersRequest {
    // TODO: the fields.
}

/// `GetOrderRequest` — answered with `Order` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetOrderRequest {
    // TODO: the fields.
}

/// `CancelOrderRequest` — answered with `Order` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelOrderRequest {
    // TODO: the fields.
}

/// `ListProductionRunsRequest` — answered with `ProductionRunList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListProductionRunsRequest {
    // TODO: the fields.
}

/// `RecordProductionEventRequest` — answered with `ProductionRun` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordProductionEventRequest {
    // TODO: the fields.
}

/// `SubmitLeadRequest` — answered with `LeadAck` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitLeadRequest {
    // TODO: the fields.
}

/// `RecordSupplierUpdateRequest` — answered with `ProductionRun` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSupplierUpdateRequest {
    // TODO: the fields.
}

/// `IngestCustomerEmailRequest` — answered with `Quote` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestCustomerEmailRequest {
    // TODO: the fields.
}

/// `GetPurchaseMatchRequest` — answered with `PurchaseMatch` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPurchaseMatchRequest {
    // TODO: the fields.
}

/// `InvoiceChanged` — a void fan-out event (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceChanged {
    // TODO: the fields.
}

/// `InboundChanged` — a void fan-out event (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundChanged {
    // TODO: the fields.
}

/// `ShipmentChanged` — a void fan-out event (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentChanged {
    // TODO: the fields.
}

/// `EnsureInvoiceRequest` — answered with `Invoice` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureInvoiceRequest {
    // TODO: the fields.
}

/// `GetInvoiceRequest` — answered with `Invoice` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceRequest {
    // TODO: the fields.
}

/// `EnsureInboundRequest` — answered with `Inbound` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureInboundRequest {
    // TODO: the fields.
}

/// `GetInboundRequest` — answered with `Inbound` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInboundRequest {
    // TODO: the fields.
}

/// `EnsureShipmentRequest` — answered with `Shipment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureShipmentRequest {
    // TODO: the fields.
}

/// `GetShipmentRequest` — answered with `Shipment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetShipmentRequest {
    // TODO: the fields.
}

/// `EstimateDeliveryRequest` — answered with `DeliveryEstimate` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EstimateDeliveryRequest {
    // TODO: the fields.
}

/// `ListStockRequest` — answered with `StockList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListStockRequest {
    // TODO: the fields.
}

/// `SetLowStockRuleRequest` — answered with `LowStockRule` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetLowStockRuleRequest {
    // TODO: the fields.
}

/// `AdjustStockRequest` — answered with `StockLevel` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustStockRequest {
    // TODO: the fields.
}

/// `ListKitsRequest` — answered with `KitList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListKitsRequest {
    // TODO: the fields.
}

/// `UpsertKitRequest` — answered with `Kit` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpsertKitRequest {
    // TODO: the fields.
}

/// `CreateAssemblyOrderRequest` — answered with `AssemblyOrder` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAssemblyOrderRequest {
    // TODO: the fields.
}

/// `RecordAssemblyRequest` — answered with `AssemblyOrder` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordAssemblyRequest {
    // TODO: the fields.
}

/// `ListInboundRequest` — answered with `InboundList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInboundRequest {
    // TODO: the fields.
}

/// `RecordInboundReceiptRequest` — answered with `Inbound` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordInboundReceiptRequest {
    // TODO: the fields.
}

/// `CreateShipmentsRequest` — answered with `ShipmentBatch` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShipmentsRequest {
    // TODO: the fields.
}

/// `ListShipmentsRequest` — answered with `ShipmentList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListShipmentsRequest {
    // TODO: the fields.
}

/// `RecordPickRequest` — answered with `Shipment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordPickRequest {
    // TODO: the fields.
}

/// `CancelShipmentRequest` — answered with `Shipment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelShipmentRequest {
    // TODO: the fields.
}

/// `CarrierWebhookEvent` — answered with `WebhookAck` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierWebhookEvent {
    // TODO: the fields.
}

/// `ListInvoicesRequest` — answered with `InvoiceList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInvoicesRequest {
    // TODO: the fields.
}

/// `GetInvoiceDownloadRequest` — answered with `DownloadLink` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceDownloadRequest {
    // TODO: the fields.
}

/// `PayInvoiceByCardRequest` — answered with `CheckoutLink` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayInvoiceByCardRequest {
    // TODO: the fields.
}

/// `RecordBankPaymentRequest` — answered with `Payment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordBankPaymentRequest {
    // TODO: the fields.
}

/// `IssueCreditNoteRequest` — answered with `Invoice` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueCreditNoteRequest {
    // TODO: the fields.
}

/// `RecordRefundRequest` — answered with `Payment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordRefundRequest {
    // TODO: the fields.
}

/// `GetSpendReportRequest` — answered with `SpendReport` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpendReportRequest {
    // TODO: the fields.
}

/// `DraftVatReturnRequest` — answered with `VatReturn` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DraftVatReturnRequest {
    // TODO: the fields.
}

/// `MarkVatReturnFiledRequest` — answered with `VatReturn` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkVatReturnFiledRequest {
    // TODO: the fields.
}

/// `GetTrialBalanceRequest` — answered with `TrialBalance` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrialBalanceRequest {
    // TODO: the fields.
}

/// `EnsureSupplierBillRequest` — answered with `SupplierBill` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsureSupplierBillRequest {
    // TODO: the fields.
}

/// `ListSupplierBillsRequest` — answered with `SupplierBillList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSupplierBillsRequest {
    // TODO: the fields.
}

/// `GetSupplierBillRequest` — answered with `SupplierBill` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupplierBillRequest {
    // TODO: the fields.
}

/// `ApproveSupplierBillRequest` — answered with `SupplierBill` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveSupplierBillRequest {
    // TODO: the fields.
}

/// `VoidSupplierBillRequest` — answered with `SupplierBill` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidSupplierBillRequest {
    // TODO: the fields.
}

/// `RecordSupplierPaymentRequest` — answered with `Payment` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSupplierPaymentRequest {
    // TODO: the fields.
}

/// `PaymentWebhookEvent` — answered with `WebhookAck` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentWebhookEvent {
    // TODO: the fields.
}

/// `ListInboundMessagesRequest` — answered with `InboundMessageList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListInboundMessagesRequest {
    // TODO: the fields.
}

/// `ListReviewQueueRequest` — answered with `InboundDocumentList` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListReviewQueueRequest {
    // TODO: the fields.
}

/// `GetInboundDocumentRequest` — answered with `InboundDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInboundDocumentRequest {
    // TODO: the fields.
}

/// `GetInboundFileLinkRequest` — answered with `DownloadLink` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInboundFileLinkRequest {
    // TODO: the fields.
}

/// `DecideInboundDocumentRequest` — answered with `InboundDocument` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecideInboundDocumentRequest {
    // TODO: the fields.
}

/// `InboundEmailEvent` — answered with `WebhookAck` (see routing.yaml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundEmailEvent {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipient {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipientImportResult {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEventList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailReceipt {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPage {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricedLines {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variant {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierOffers {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageUpload {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomsData {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierMatch {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopPage {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandProfile {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandAssetList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUpload {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandAsset {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofApproval {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtworkBundle {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDocument {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignVersion {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewUpload {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTemplateList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTemplate {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignVersionRef {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignAdoption {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuoteList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRunList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionRun {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeadAck {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseMatch {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbound {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryEstimate {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LowStockRule {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLevel {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KitList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kit {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssemblyOrder {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentBatch {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAck {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadLink {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutLink {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendReport {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VatReturn {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalance {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierBill {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierBillList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundMessageList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundDocumentList {
    // TODO: the fields.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundDocument {
    // TODO: the fields.
}
// basable:messages-end
