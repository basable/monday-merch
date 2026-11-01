# Architecture

One binary, one Postgres, 8 nanoservices. This file is the
map; `routing.yaml` is the topology the build enforces; the plan under
`docs/01-enterprise-merch/` is the design record it was rendered from.

## Nanoservices and what they own

### `account`

Owns companies, members, departments, addresses, recipients and the audit trail.

- Plain tables: `company`, `department`, `membership`, `staff_member`, `address`, `recipient`, `audit_event`.
- Schedules (ticker workers): `purge_recipient_pii` every 24h.
- Handles: `GetCompanyRequest` → `Company`, `UpdateCompanyRequest` → `Company`, `EnsureCompanyRequest` → `Company`, `ResolveMemberRequest` → `Member`, `ListMembersRequest` → `MemberList`, `InviteMemberRequest` → `Member`, `UpdateMemberRequest` → `Member`, `RemoveMemberRequest` → `Member`, `ListDepartmentsRequest` → `DepartmentList`, `UpsertDepartmentRequest` → `Department`, `ListAddressesRequest` → `AddressList`, `UpsertAddressRequest` → `Address`, `ListRecipientsRequest` → `RecipientList`, `UpsertRecipientRequest` → `Recipient`, `ImportRecipientsRequest` → `RecipientImportResult`, `EraseRecipientRequest` → `Recipient`, `ResolveRecipientsRequest` → `RecipientList`, `ListAuditEventsRequest` → `AuditEventList`, `AuditEvent`.
- Sends: `SendEmailRequest`.
- API: `AccountService` (GetCompany, UpdateCompany, ListMembers, InviteMember, UpdateMember, RemoveMember, ListDepartments, UpsertDepartment, ListAddresses, UpsertAddress, ListRecipients, UpsertRecipient, ImportRecipients, EraseRecipient, ListAuditEvents).

### `catalog`

Owns products, variants, list prices and decoration options, and renders the public shop pages.

- Plain tables: `product`, `variant`, `price_tier`, `decoration_option`, `product_image`, `supplier_offer`.
- Config catalogs: `category`, `collection`, `brand`, `colour`, `print_method`, `supplier`, `pricing_policy`.
- Handles: `ListProductsRequest` → `ProductPage`, `GetProductRequest` → `Product`, `ListCategoriesRequest` → `CategoryList`, `PriceLinesRequest` → `PricedLines`, `UpsertProductRequest` → `Product`, `UpsertVariantRequest` → `Variant`, `SetPriceTiersRequest` → `Product`, `SetDecorationOptionsRequest` → `Product`, `SetSupplierOffersRequest` → `SupplierOffers`, `CreateImageUploadRequest` → `ImageUpload`, `ArchiveProductRequest` → `Product`, `GetSupplierOffersRequest` → `SupplierOffers`, `GetCustomsDataRequest` → `CustomsData`, `MatchSupplierRequest` → `SupplierMatch`, `RenderShopPageRequest` → `ShopPage`.
- Sends: `ResolveMemberRequest`, `AuditEvent`.
- API: `CatalogService` (ListProducts, GetProduct, ListCategories, PriceLines, UpsertProduct, UpsertVariant, SetPriceTiers, SetDecorationOptions, SetSupplierOffers, CreateImageUpload, ArchiveProduct).

### `design`

Owns each company's brand assets and the proofs consultants prepare for quotes.

- Plain tables: `brand_profile`, `brand_asset`, `proof`, `design_document`, `design_version`, `design_template`.
- Config catalogs: `studio_font`.
- External systems: `object_storage` (delete_object: idempotent).
- Schedules (ticker workers): `purge_orphan_uploads` every 24h.
- Handles: `GetBrandProfileRequest` → `BrandProfile`, `UpdateBrandProfileRequest` → `BrandProfile`, `ListBrandAssetsRequest` → `BrandAssetList`, `CreateAssetUploadRequest` → `AssetUpload`, `ConfirmAssetUploadRequest` → `BrandAsset`, `DeleteBrandAssetRequest` → `BrandAsset`, `CreateProofRequest` → `Proof`, `ListProofsRequest` → `ProofList`, `ApproveProofsRequest` → `ProofApproval`, `GetApprovedArtworkRequest` → `ArtworkBundle`, `CreateDesignRequest` → `DesignDocument`, `GetDesignRequest` → `DesignDocument`, `ListDesignsRequest` → `DesignList`, `SaveDesignDraftRequest` → `DesignDocument`, `SaveDesignVersionRequest` → `DesignVersion`, `ArchiveDesignRequest` → `DesignDocument`, `CreatePreviewUploadRequest` → `PreviewUpload`, `ListDesignTemplatesRequest` → `DesignTemplateList`, `GetDesignTemplateRequest` → `DesignTemplate`, `UpsertDesignTemplateRequest` → `DesignTemplate`, `ResolveDesignVersionRequest` → `DesignVersionRef`, `AdoptDesignsRequest` → `DesignAdoption`.
- Sends: `ResolveMemberRequest`, `AuditEvent`.
- API: `DesignService` (GetBrandProfile, UpdateBrandProfile, ListBrandAssets, CreateAssetUpload, ConfirmAssetUpload, DeleteBrandAsset, CreateProof, ListProofs, CreateDesign, GetDesign, ListDesigns, SaveDesignDraft, SaveDesignVersion, ArchiveDesign, CreatePreviewUpload, ListDesignTemplates, GetDesignTemplate, UpsertDesignTemplate).

### `order`

Takes a request from quote to signature, then drives each order through invoicing, production and delivery.

- Processing-object types (a lifecycle each, with spec + status tables, an
  adapter, a reconciler and a worker): `quote` (`quo_…`), `order` (`ord_…`), `production_run` (`prun_…`).
- Plain tables: `quote_version`, `quote_signature`, `quote_comment`, `production_event`, `lead`.
- Config catalogs: `quote_policy`.
- Schedules (ticker workers): `purge_leads` every 24h.
- Handles: `RequestQuoteRequest` → `Quote`, `ListQuotesRequest` → `QuoteList`, `GetQuoteRequest` → `Quote`, `AssignQuoteRequest` → `Quote`, `PublishQuoteVersionRequest` → `Quote`, `RequestQuoteChangesRequest` → `Quote`, `AcceptQuoteRequest` → `Order`, `ReorderRequest` → `Quote`, `ListOrdersRequest` → `OrderList`, `GetOrderRequest` → `Order`, `CancelOrderRequest` → `Order`, `ListProductionRunsRequest` → `ProductionRunList`, `RecordProductionEventRequest` → `ProductionRun`, `SubmitLeadRequest` → `LeadAck`, `RecordSupplierUpdateRequest` → `ProductionRun`, `IngestCustomerEmailRequest` → `Quote`, `GetPurchaseMatchRequest` → `PurchaseMatch`, `InvoiceChanged`, `InboundChanged`, `ShipmentChanged`.
- Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `EnsureCompanyRequest`, `PriceLinesRequest`, `GetSupplierOffersRequest`, `ListProofsRequest`, `ApproveProofsRequest`, `GetApprovedArtworkRequest`, `ResolveDesignVersionRequest`, `AdoptDesignsRequest`, `EnsureInvoiceRequest`, `GetInvoiceRequest`, `EnsureInboundRequest`, `GetInboundRequest`, `EnsureShipmentRequest`, `GetShipmentRequest`, `SendEmailRequest`, `AuditEvent`.
- API: `OrderService` (RequestQuote, ListQuotes, GetQuote, AssignQuote, PublishQuoteVersion, RequestQuoteChanges, AcceptQuote, Reorder, ListOrders, GetOrder, CancelOrder, ListProductionRuns, RecordProductionEvent, SubmitLead).

### `fulfilment`

Owns held stock, kits and inbound goods, and drives each shipment from reservation to delivery.

- Processing-object types (a lifecycle each, with spec + status tables, an
  adapter, a reconciler and a worker): `shipment` (`shp_…`).
- Plain tables: `stock_item`, `stock_level`, `stock_movement`, `kit_component`, `assembly_order`, `inbound`, `low_stock_rule`, `shipment_batch`, `shipment_address`, `shipment_event`.
- Config catalogs: `warehouse`, `shipping_zone`, `fulfilment_fee`.
- External systems: `carrier` (create_label: declared, irreversible, void_label: idempotent).
- Schedules (ticker workers): `check_low_stock` every 1h, `purge_shipment_addresses` every 24h.
- Inbound webhooks: `carrier`.
- Handles: `EstimateDeliveryRequest` → `DeliveryEstimate`, `ListStockRequest` → `StockList`, `SetLowStockRuleRequest` → `LowStockRule`, `AdjustStockRequest` → `StockLevel`, `ListKitsRequest` → `KitList`, `UpsertKitRequest` → `Kit`, `CreateAssemblyOrderRequest` → `AssemblyOrder`, `RecordAssemblyRequest` → `AssemblyOrder`, `ListInboundRequest` → `InboundList`, `RecordInboundReceiptRequest` → `Inbound`, `CreateShipmentsRequest` → `ShipmentBatch`, `ListShipmentsRequest` → `ShipmentList`, `GetShipmentRequest` → `Shipment`, `RecordPickRequest` → `Shipment`, `CancelShipmentRequest` → `Shipment`, `EnsureInboundRequest` → `Inbound`, `GetInboundRequest` → `Inbound`, `EnsureShipmentRequest` → `Shipment`, `CarrierWebhookEvent` → `WebhookAck`.
- Sends: `ResolveMemberRequest`, `ResolveRecipientsRequest`, `GetCustomsDataRequest`, `EnsureInvoiceRequest`, `SendEmailRequest`, `InboundChanged`, `ShipmentChanged`, `AuditEvent`.
- API: `FulfilmentService` (EstimateDelivery, ListStock, SetLowStockRule, AdjustStock, ListKits, UpsertKit, CreateAssemblyOrder, RecordAssembly, ListInbound, RecordInboundReceipt, CreateShipments, ListShipments, GetShipment, RecordPick, CancelShipment).

### `billing`

Owns the ledger, invoices and credit notes, payments and VAT returns.

- Processing-object types (a lifecycle each, with spec + status tables, an
  adapter, a reconciler and a worker): `invoice` (`inv_…`), `payment` (`pay_…`), `vat_return` (`vret_…`), `supplier_bill` (`sbill_…`).
- Plain tables: `invoice_counter`, `invoice_number`, `payment_webhook_event`, `supplier_bill_key`.
- Config catalogs: `vat_rule`, `billing_policy`.
- The project's ledger (`docs/frameworks/ledger.md`), booking `invoice` from `billing`, `credit_note` from `billing`, `payment` from `billing`, `vat_return` from `billing`, `supplier_bill` from `billing`; every producer sends a `Book`, the owner answers `Booked`.
- External systems: `payments` (create_checkout_session: keyed_replay, irreversible, expire_checkout_session: idempotent, create_refund: keyed_replay, irreversible), `object_storage` (store_document: idempotent).
- Inbound webhooks: `payments`.
- Handles: `EnsureInvoiceRequest` → `Invoice`, `ListInvoicesRequest` → `InvoiceList`, `GetInvoiceRequest` → `Invoice`, `GetInvoiceDownloadRequest` → `DownloadLink`, `PayInvoiceByCardRequest` → `CheckoutLink`, `RecordBankPaymentRequest` → `Payment`, `IssueCreditNoteRequest` → `Invoice`, `RecordRefundRequest` → `Payment`, `GetSpendReportRequest` → `SpendReport`, `DraftVatReturnRequest` → `VatReturn`, `MarkVatReturnFiledRequest` → `VatReturn`, `GetTrialBalanceRequest` → `TrialBalance`, `EnsureSupplierBillRequest` → `SupplierBill`, `ListSupplierBillsRequest` → `SupplierBillList`, `GetSupplierBillRequest` → `SupplierBill`, `ApproveSupplierBillRequest` → `SupplierBill`, `VoidSupplierBillRequest` → `SupplierBill`, `RecordSupplierPaymentRequest` → `Payment`, `PaymentWebhookEvent` → `WebhookAck`, `Book` → `Booked`.
- Sends: `ResolveMemberRequest`, `GetCompanyRequest`, `GetPurchaseMatchRequest`, `SendEmailRequest`, `InvoiceChanged`, `AuditEvent`.
- API: `BillingService` (ListInvoices, GetInvoice, GetInvoiceDownload, PayInvoiceByCard, RecordBankPayment, IssueCreditNote, RecordRefund, GetSpendReport, DraftVatReturn, MarkVatReturnFiled, GetTrialBalance, ListSupplierBills, GetSupplierBill, ApproveSupplierBill, VoidSupplierBill, RecordSupplierPayment).

### `notifier`

Sends one templated email per request, keyed by the caller.

- Config catalogs: `email_template`.
- External systems: `email` (send_email: keyed_replay, irreversible).
- Handles: `SendEmailRequest` → `EmailReceipt`.

### `inbox`

Receives email, stores it, extracts structured data from each document, and routes the result to its owner or a review queue.

- Processing-object types (a lifecycle each, with spec + status tables, an
  adapter, a reconciler and a worker): `inbound_message` (`imsg_…`), `inbound_document` (`idoc_…`).
- Plain tables: `inbound_seen`, `review_decision`.
- Config catalogs: `inbox_policy`.
- External systems: `object_storage` (store_inbound_object: idempotent, delete_inbound_object: idempotent), `llm` (extract_document: idempotent).
- Schedules (ticker workers): `purge_inbound` every 24h.
- Inbound webhooks: `inbound_mail`.
- Handles: `ListInboundMessagesRequest` → `InboundMessageList`, `ListReviewQueueRequest` → `InboundDocumentList`, `GetInboundDocumentRequest` → `InboundDocument`, `GetInboundFileLinkRequest` → `DownloadLink`, `DecideInboundDocumentRequest` → `InboundDocument`, `InboundEmailEvent` → `WebhookAck`.
- Sends: `ResolveMemberRequest`, `MatchSupplierRequest`, `EnsureSupplierBillRequest`, `RecordSupplierUpdateRequest`, `IngestCustomerEmailRequest`, `SendEmailRequest`, `AuditEvent`.
- API: `InboxService` (ListInboundMessages, ListReviewQueue, GetInboundDocument, GetInboundFileLink, DecideInboundDocument).

## Topology

| Message | Response | Sent by | Handled by |
|---|---|---|---|
| `GetCompanyRequest` | `Company` | `order` `billing`  | `account`  |
| `UpdateCompanyRequest` | `Company` |  | `account`  |
| `EnsureCompanyRequest` | `Company` | `order`  | `account`  |
| `ResolveMemberRequest` | `Member` | `catalog` `design` `order` `fulfilment` `billing` `inbox`  | `account`  |
| `ListMembersRequest` | `MemberList` |  | `account`  |
| `InviteMemberRequest` | `Member` |  | `account`  |
| `UpdateMemberRequest` | `Member` |  | `account`  |
| `RemoveMemberRequest` | `Member` |  | `account`  |
| `ListDepartmentsRequest` | `DepartmentList` |  | `account`  |
| `UpsertDepartmentRequest` | `Department` |  | `account`  |
| `ListAddressesRequest` | `AddressList` |  | `account`  |
| `UpsertAddressRequest` | `Address` |  | `account`  |
| `ListRecipientsRequest` | `RecipientList` |  | `account`  |
| `UpsertRecipientRequest` | `Recipient` |  | `account`  |
| `ImportRecipientsRequest` | `RecipientImportResult` |  | `account`  |
| `EraseRecipientRequest` | `Recipient` |  | `account`  |
| `ResolveRecipientsRequest` | `RecipientList` | `fulfilment`  | `account`  |
| `ListAuditEventsRequest` | `AuditEventList` |  | `account`  |
| `AuditEvent` | (event) | `catalog` `design` `order` `fulfilment` `billing` `inbox`  | `account`  |
| `SendEmailRequest` | `EmailReceipt` | `account` `order` `fulfilment` `billing` `inbox`  | `notifier`  |
| `ListProductsRequest` | `ProductPage` |  | `catalog`  |
| `GetProductRequest` | `Product` |  | `catalog`  |
| `ListCategoriesRequest` | `CategoryList` |  | `catalog`  |
| `PriceLinesRequest` | `PricedLines` | `order`  | `catalog`  |
| `UpsertProductRequest` | `Product` |  | `catalog`  |
| `UpsertVariantRequest` | `Variant` |  | `catalog`  |
| `SetPriceTiersRequest` | `Product` |  | `catalog`  |
| `SetDecorationOptionsRequest` | `Product` |  | `catalog`  |
| `SetSupplierOffersRequest` | `SupplierOffers` |  | `catalog`  |
| `CreateImageUploadRequest` | `ImageUpload` |  | `catalog`  |
| `ArchiveProductRequest` | `Product` |  | `catalog`  |
| `GetSupplierOffersRequest` | `SupplierOffers` | `order`  | `catalog`  |
| `GetCustomsDataRequest` | `CustomsData` | `fulfilment`  | `catalog`  |
| `MatchSupplierRequest` | `SupplierMatch` | `inbox`  | `catalog`  |
| `RenderShopPageRequest` | `ShopPage` |  | `catalog`  |
| `GetBrandProfileRequest` | `BrandProfile` |  | `design`  |
| `UpdateBrandProfileRequest` | `BrandProfile` |  | `design`  |
| `ListBrandAssetsRequest` | `BrandAssetList` |  | `design`  |
| `CreateAssetUploadRequest` | `AssetUpload` |  | `design`  |
| `ConfirmAssetUploadRequest` | `BrandAsset` |  | `design`  |
| `DeleteBrandAssetRequest` | `BrandAsset` |  | `design`  |
| `CreateProofRequest` | `Proof` |  | `design`  |
| `ListProofsRequest` | `ProofList` | `order`  | `design`  |
| `ApproveProofsRequest` | `ProofApproval` | `order`  | `design`  |
| `GetApprovedArtworkRequest` | `ArtworkBundle` | `order`  | `design`  |
| `CreateDesignRequest` | `DesignDocument` |  | `design`  |
| `GetDesignRequest` | `DesignDocument` |  | `design`  |
| `ListDesignsRequest` | `DesignList` |  | `design`  |
| `SaveDesignDraftRequest` | `DesignDocument` |  | `design`  |
| `SaveDesignVersionRequest` | `DesignVersion` |  | `design`  |
| `ArchiveDesignRequest` | `DesignDocument` |  | `design`  |
| `CreatePreviewUploadRequest` | `PreviewUpload` |  | `design`  |
| `ListDesignTemplatesRequest` | `DesignTemplateList` |  | `design`  |
| `GetDesignTemplateRequest` | `DesignTemplate` |  | `design`  |
| `UpsertDesignTemplateRequest` | `DesignTemplate` |  | `design`  |
| `ResolveDesignVersionRequest` | `DesignVersionRef` | `order`  | `design`  |
| `AdoptDesignsRequest` | `DesignAdoption` | `order`  | `design`  |
| `RequestQuoteRequest` | `Quote` |  | `order`  |
| `ListQuotesRequest` | `QuoteList` |  | `order`  |
| `GetQuoteRequest` | `Quote` |  | `order`  |
| `AssignQuoteRequest` | `Quote` |  | `order`  |
| `PublishQuoteVersionRequest` | `Quote` |  | `order`  |
| `RequestQuoteChangesRequest` | `Quote` |  | `order`  |
| `AcceptQuoteRequest` | `Order` |  | `order`  |
| `ReorderRequest` | `Quote` |  | `order`  |
| `ListOrdersRequest` | `OrderList` |  | `order`  |
| `GetOrderRequest` | `Order` |  | `order`  |
| `CancelOrderRequest` | `Order` |  | `order`  |
| `ListProductionRunsRequest` | `ProductionRunList` |  | `order`  |
| `RecordProductionEventRequest` | `ProductionRun` |  | `order`  |
| `SubmitLeadRequest` | `LeadAck` |  | `order`  |
| `RecordSupplierUpdateRequest` | `ProductionRun` | `inbox`  | `order`  |
| `IngestCustomerEmailRequest` | `Quote` | `inbox`  | `order`  |
| `GetPurchaseMatchRequest` | `PurchaseMatch` | `billing`  | `order`  |
| `InvoiceChanged` | (event) | `billing`  | `order`  |
| `InboundChanged` | (event) | `fulfilment`  | `order`  |
| `ShipmentChanged` | (event) | `fulfilment`  | `order`  |
| `EnsureInvoiceRequest` | `Invoice` | `order` `fulfilment`  | `billing`  |
| `GetInvoiceRequest` | `Invoice` | `order`  | `billing`  |
| `EnsureInboundRequest` | `Inbound` | `order`  | `fulfilment`  |
| `GetInboundRequest` | `Inbound` | `order`  | `fulfilment`  |
| `EnsureShipmentRequest` | `Shipment` | `order`  | `fulfilment`  |
| `GetShipmentRequest` | `Shipment` | `order`  | `fulfilment`  |
| `EstimateDeliveryRequest` | `DeliveryEstimate` |  | `fulfilment`  |
| `ListStockRequest` | `StockList` |  | `fulfilment`  |
| `SetLowStockRuleRequest` | `LowStockRule` |  | `fulfilment`  |
| `AdjustStockRequest` | `StockLevel` |  | `fulfilment`  |
| `ListKitsRequest` | `KitList` |  | `fulfilment`  |
| `UpsertKitRequest` | `Kit` |  | `fulfilment`  |
| `CreateAssemblyOrderRequest` | `AssemblyOrder` |  | `fulfilment`  |
| `RecordAssemblyRequest` | `AssemblyOrder` |  | `fulfilment`  |
| `ListInboundRequest` | `InboundList` |  | `fulfilment`  |
| `RecordInboundReceiptRequest` | `Inbound` |  | `fulfilment`  |
| `CreateShipmentsRequest` | `ShipmentBatch` |  | `fulfilment`  |
| `ListShipmentsRequest` | `ShipmentList` |  | `fulfilment`  |
| `RecordPickRequest` | `Shipment` |  | `fulfilment`  |
| `CancelShipmentRequest` | `Shipment` |  | `fulfilment`  |
| `CarrierWebhookEvent` | `WebhookAck` |  | `fulfilment`  |
| `ListInvoicesRequest` | `InvoiceList` |  | `billing`  |
| `GetInvoiceDownloadRequest` | `DownloadLink` |  | `billing`  |
| `PayInvoiceByCardRequest` | `CheckoutLink` |  | `billing`  |
| `RecordBankPaymentRequest` | `Payment` |  | `billing`  |
| `IssueCreditNoteRequest` | `Invoice` |  | `billing`  |
| `RecordRefundRequest` | `Payment` |  | `billing`  |
| `GetSpendReportRequest` | `SpendReport` |  | `billing`  |
| `DraftVatReturnRequest` | `VatReturn` |  | `billing`  |
| `MarkVatReturnFiledRequest` | `VatReturn` |  | `billing`  |
| `GetTrialBalanceRequest` | `TrialBalance` |  | `billing`  |
| `EnsureSupplierBillRequest` | `SupplierBill` | `inbox`  | `billing`  |
| `ListSupplierBillsRequest` | `SupplierBillList` |  | `billing`  |
| `GetSupplierBillRequest` | `SupplierBill` |  | `billing`  |
| `ApproveSupplierBillRequest` | `SupplierBill` |  | `billing`  |
| `VoidSupplierBillRequest` | `SupplierBill` |  | `billing`  |
| `RecordSupplierPaymentRequest` | `Payment` |  | `billing`  |
| `PaymentWebhookEvent` | `WebhookAck` |  | `billing`  |
| `ListInboundMessagesRequest` | `InboundMessageList` |  | `inbox`  |
| `ListReviewQueueRequest` | `InboundDocumentList` |  | `inbox`  |
| `GetInboundDocumentRequest` | `InboundDocument` |  | `inbox`  |
| `GetInboundFileLinkRequest` | `DownloadLink` |  | `inbox`  |
| `DecideInboundDocumentRequest` | `InboundDocument` |  | `inbox`  |
| `InboundEmailEvent` | `WebhookAck` |  | `inbox`  |

`api` sends every API method's request; it handles nothing.

## Deployment shape

- `app/`: the binary, `replicas: 2`, every nanoservice in every replica.
- `frontend/react/`: the React app (Vite, built by rules_js into `dist/`), uploaded by the workflow as the `static` artifact and served at `/` from the environment's public bucket (no image, no pod), calling the API through Connect clients generated from `proto/` (`frontend/react/README.md`).
- `k8s/postgres/`: one CNPG cluster, 1 instance(s), a schema and a role per stateful nanoservice (`nano_account`, `nano_catalog`, `nano_design`, `nano_order`, `nano_fulfilment`, `nano_billing`, `nano_inbox`).
- `k8s/kratos/`: Ory Kratos identity (passwordless email code + passkeys + Sign in with Basable, from the platform's `basable-login` Secret), its own database. The sign-in pages Kratos sends the browser to (`/auth/login`, `/auth/register`, `/auth/settings`, `/auth/error`) are drawn by the frontend from the flow's own nodes (`frontend/react/src/Auth.tsx`).
- `k8s/httproute.yaml`: `/api/` → app, `/.ory/` → Kratos; `/` is the platform's own route to the bucket, never claimed here.

See `docs/platform-contract.md` for the rules the platform enforces on this
tree and `docs/ci.md` for the workflow.
