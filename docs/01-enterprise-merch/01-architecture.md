# 01: Architecture

One Rust binary with two replicas over one Postgres database, made of seven nanoservices. Stores and integrations will add two more if round 2 includes them. They are not in the manifest yet.

## The main path

1. A visitor browses server-rendered `/shop/*` pages from `catalog`. The basket stays in their browser.
2. `RequestQuote` stores the request in `order`, re-priced by `catalog`, and starts the 24-hour clock.
3. A consultant uploads proofs to `design` and publishes a quote version. The customer is emailed through `notifier`.
4. The customer signs in with an email code, reviews, asks for changes or signs. Signing creates the company in `account` if it is new, approves the proofs in `design`, and creates an `order`.
5. The `order` pass asks `billing` for an invoice. The `invoice` pass numbers it, renders the PDF, books it on the ledger and emails it. A customer pays by bank transfer (recorded by finance) or by card.
6. Once paid, or straight away for a company on terms, the `order` pass creates a `production_run` per supplier. Each run emails a purchase order, and staff record what the supplier reports.
7. Goods arrive at a warehouse in `fulfilment`, or the supplier ships directly. The `shipment` pass reserves, waits for picking, buys a label and follows tracking until delivery.
8. The order settles. Stock held for later can be shipped to many addresses at any time, each batch with its own invoice.

## Nanoservices

**`account`** owns who a customer is and what they may do: companies, departments, members, staff, addresses, recipients and the audit trail, all plain tables, because people change them and nothing waits. It answers `ResolveMemberRequest` for every other nanoservice, sends invitations through `notifier`, and has one schedule erasing personal data past its retention date. API: `AccountService`. No external systems.

**`catalog`** owns what can be bought: products, variants, price tiers, decoration options, images and supplier offers in plain tables, with categories, brands, colours, print methods, suppliers and pricing policy as config catalogs. No lifecycle, because staff edit and readers read. It is the only pricing logic (`PriceLinesRequest`) and renders the public `/shop` pages (`RenderShopPageRequest`). API: `CatalogService`. No external systems.

**`design`** owns brand assets and proofs in plain tables and uses the private bucket. A proof waits for a person, but the quote already tracks that wait, so it needs no lifecycle. It answers `order` for proofs and artwork. It has one idempotent external call (`delete_object`) and one cleanup schedule. API: `DesignService`.

**`order`** owns the path from request to delivery. Quotes are plain tables with a ticker, because their waits are timestamps. `order` is a processing-object type because it waits days on payment, production and delivery and must be re-driven until it matches. `production_run` is a second type because each purchase order waits on a supplier with no API, and staff input arrives unfenced through a side table. It talks to almost every other nanoservice, always carrying its generation on derived intent. No external systems of its own. API: `OrderService`.

**`fulfilment`** owns physical goods. Stock, kits, assembly orders, inbound goods and shipment batches are plain tables, because a stock count is a quantity under a row lock and not money. `shipment` is a processing-object type because it waits on stock and pickers, drives a carrier and then follows tracking. External: `carrier` (`create_label`, `void_label`) and a carrier webhook. Schedules: `check_low_stock`, `purge_shipment_addresses`. API: `FulfilmentService`.

**`billing`** owns everything owed, held or moved, and the project's one ledger. `invoice` is a type because issuing is several effects (number, PDF, book, email) followed by a long wait for payment with reminders. `payment` is a type because a card payment drives Stripe and waits for it. `vat_return` is a type because it waits on finance and then books. Invoice numbering is a counter table, and the webhook inbox is a plain table. External: `payments` (Stripe) and `object_storage` (`store_document`), plus a Stripe webhook. API: `BillingService`.

**`notifier`** owns one external call, `send_email`, and the template catalog. It has no lifecycle, because the caller's own pass decides whether an email went out. No API.

## Deployment shape

- One binary, `replicas: 2`, Connect over protobuf at `/api/*`, plain Rust messages inside.
- One CloudNativePG cluster, `instances: 1`, one application database with one schema and role per stateful nanoservice, and a separate `kratos` database.
- Frontend from the public bucket as plain HTML, CSS and JavaScript. `/shop/*` is rendered by the binary, a deviation from the standard shape explained in `02-catalog.md` and recorded in `05-deployment.md`.
- Ory Kratos at `/.ory/*`, passwordless email code and passkeys.
- Schedules are workers in each nanoservice's `loops()`, and webhooks are raw routes.

### Why a static frontend

The dashboard and staff console are forms, tables and status views, so plain pages over Connect's JSON are enough. React is justified only by an editor. The Design Studio was deferred in round 1, so when it is scheduled it arrives as its own React page, and this plan is updated then.

| Path | Audience | Served by |
| --- | --- | --- |
| `/`, `/services/`, `/use-cases/`, `/articles/` and so on | Public | Bucket |
| `/shop/*` | Public | Binary, server-rendered |
| `/app/` | Customers | Bucket, data over `/api` |
| `/ops/` | Staff | Bucket, data over `/api`, authorised by `staff_member` |

The platform answers an unknown path with the root `index.html`, so dashboard pages take ids in the query string (`/app/orders/?id=ord_...`).

### Auth

`project.basableLogin` is `false`. The default offers "Sign in with Basable", but customers here are employees of other companies who have no reason to hold a Basable account, and an enterprise sign-in page should not show a hosting provider's button. The cost is that email-code sign-in depends entirely on Kratos's SMTP, so nobody can sign in until that secret is set (`05-deployment.md`).

Authorisation lives in each nanoservice. The `api` crate validates the Kratos session, and each handler resolves the actor with `ResolveMemberRequest` and filters by that actor's company. The company id is never taken from a request body.

**Risk for round 2: SSO and SAML.** Standard Kratos providers are set in deployment configuration, not per customer. Ory documents per-organisation SAML as an Ory Network enterprise feature. For self-hosting, Ory Polis looks like the component to evaluate, but Ory's own sources disagree on whether it needs a paid licence. Confirm with Ory before committing to it. The alternatives are a separate SAML-to-OIDC bridge, or moving identity to Ory Network.

## Topology

Every message, generated from `manifest.yaml`. "`api` crate" means the message is also a Connect method's request.

| Message | Sender(s) | Handler(s) | Response |
| --- | --- | --- | --- |
| `AcceptQuoteRequest` | `api` crate | order | `Order` |
| `AdjustStockRequest` | `api` crate | fulfilment | `StockLevel` |
| `ApproveProofsRequest` | order | design | `ProofApproval` |
| `ArchiveProductRequest` | `api` crate | catalog | `Product` |
| `AssignQuoteRequest` | `api` crate | order | `Quote` |
| `AuditEvent` | catalog, design, order, fulfilment, billing | account | void event |
| `CancelOrderRequest` | `api` crate | order | `Order` |
| `CancelShipmentRequest` | `api` crate | fulfilment | `Shipment` |
| `CarrierWebhookEvent` | `api` raw route | fulfilment | `WebhookAck` |
| `ConfirmAssetUploadRequest` | `api` crate | design | `BrandAsset` |
| `CreateAssemblyOrderRequest` | `api` crate | fulfilment | `AssemblyOrder` |
| `CreateAssetUploadRequest` | `api` crate | design | `AssetUpload` |
| `CreateImageUploadRequest` | `api` crate | catalog | `ImageUpload` |
| `CreateProofRequest` | `api` crate | design | `Proof` |
| `CreateShipmentsRequest` | `api` crate | fulfilment | `ShipmentBatch` |
| `DeleteBrandAssetRequest` | `api` crate | design | `BrandAsset` |
| `DraftVatReturnRequest` | `api` crate | billing | `VatReturn` |
| `EnsureCompanyRequest` | order | account | `Company` |
| `EnsureInboundRequest` | order | fulfilment | `Inbound` |
| `EnsureInvoiceRequest` | order, fulfilment | billing | `Invoice` |
| `EnsureShipmentRequest` | order | fulfilment | `Shipment` |
| `EraseRecipientRequest` | `api` crate | account | `Recipient` |
| `EstimateDeliveryRequest` | `api` crate | fulfilment | `DeliveryEstimate` |
| `GetApprovedArtworkRequest` | order | design | `ArtworkBundle` |
| `GetBrandProfileRequest` | `api` crate | design | `BrandProfile` |
| `GetCompanyRequest` | `api` crate, order, billing | account | `Company` |
| `GetCustomsDataRequest` | fulfilment | catalog | `CustomsData` |
| `GetInboundRequest` | order | fulfilment | `Inbound` |
| `GetInvoiceDownloadRequest` | `api` crate | billing | `DownloadLink` |
| `GetInvoiceRequest` | `api` crate, order | billing | `Invoice` |
| `GetOrderRequest` | `api` crate | order | `Order` |
| `GetProductRequest` | `api` crate | catalog | `Product` |
| `GetQuoteRequest` | `api` crate | order | `Quote` |
| `GetShipmentRequest` | `api` crate, order | fulfilment | `Shipment` |
| `GetSpendReportRequest` | `api` crate | billing | `SpendReport` |
| `GetSupplierOffersRequest` | order | catalog | `SupplierOffers` |
| `GetTrialBalanceRequest` | `api` crate | billing | `TrialBalance` |
| `ImportRecipientsRequest` | `api` crate | account | `RecipientImportResult` |
| `InboundChanged` | fulfilment | order | void event |
| `InviteMemberRequest` | `api` crate | account | `Member` |
| `InvoiceChanged` | billing | order | void event |
| `IssueCreditNoteRequest` | `api` crate | billing | `Invoice` |
| `ListAddressesRequest` | `api` crate | account | `AddressList` |
| `ListAuditEventsRequest` | `api` crate | account | `AuditEventList` |
| `ListBrandAssetsRequest` | `api` crate | design | `BrandAssetList` |
| `ListCategoriesRequest` | `api` crate | catalog | `CategoryList` |
| `ListDepartmentsRequest` | `api` crate | account | `DepartmentList` |
| `ListInboundRequest` | `api` crate | fulfilment | `InboundList` |
| `ListInvoicesRequest` | `api` crate | billing | `InvoiceList` |
| `ListKitsRequest` | `api` crate | fulfilment | `KitList` |
| `ListMembersRequest` | `api` crate | account | `MemberList` |
| `ListOrdersRequest` | `api` crate | order | `OrderList` |
| `ListProductionRunsRequest` | `api` crate | order | `ProductionRunList` |
| `ListProductsRequest` | `api` crate | catalog | `ProductPage` |
| `ListProofsRequest` | `api` crate, order | design | `ProofList` |
| `ListQuotesRequest` | `api` crate | order | `QuoteList` |
| `ListRecipientsRequest` | `api` crate | account | `RecipientList` |
| `ListShipmentsRequest` | `api` crate | fulfilment | `ShipmentList` |
| `ListStockRequest` | `api` crate | fulfilment | `StockList` |
| `MarkVatReturnFiledRequest` | `api` crate | billing | `VatReturn` |
| `PayInvoiceByCardRequest` | `api` crate | billing | `CheckoutLink` |
| `PaymentWebhookEvent` | `api` raw route | billing | `WebhookAck` |
| `PriceLinesRequest` | `api` crate, order | catalog | `PricedLines` |
| `PublishQuoteVersionRequest` | `api` crate | order | `Quote` |
| `RecordAssemblyRequest` | `api` crate | fulfilment | `AssemblyOrder` |
| `RecordBankPaymentRequest` | `api` crate | billing | `Payment` |
| `RecordInboundReceiptRequest` | `api` crate | fulfilment | `Inbound` |
| `RecordPickRequest` | `api` crate | fulfilment | `Shipment` |
| `RecordProductionEventRequest` | `api` crate | order | `ProductionRun` |
| `RecordRefundRequest` | `api` crate | billing | `Payment` |
| `RemoveMemberRequest` | `api` crate | account | `Member` |
| `RenderShopPageRequest` | `api` raw route | catalog | `ShopPage` |
| `ReorderRequest` | `api` crate | order | `Quote` |
| `RequestQuoteChangesRequest` | `api` crate | order | `Quote` |
| `RequestQuoteRequest` | `api` crate | order | `Quote` |
| `ResolveMemberRequest` | catalog, design, order, fulfilment, billing | account | `Member` |
| `ResolveRecipientsRequest` | fulfilment | account | `RecipientList` |
| `SendEmailRequest` | account, order, fulfilment, billing | notifier | `EmailReceipt` |
| `SetDecorationOptionsRequest` | `api` crate | catalog | `Product` |
| `SetLowStockRuleRequest` | `api` crate | fulfilment | `LowStockRule` |
| `SetPriceTiersRequest` | `api` crate | catalog | `Product` |
| `SetSupplierOffersRequest` | `api` crate | catalog | `SupplierOffers` |
| `ShipmentChanged` | fulfilment | order | void event |
| `SubmitLeadRequest` | `api` crate | order | `LeadAck` |
| `UpdateBrandProfileRequest` | `api` crate | design | `BrandProfile` |
| `UpdateCompanyRequest` | `api` crate | account | `Company` |
| `UpdateMemberRequest` | `api` crate | account | `Member` |
| `UpsertAddressRequest` | `api` crate | account | `Address` |
| `UpsertDepartmentRequest` | `api` crate | account | `Department` |
| `UpsertKitRequest` | `api` crate | fulfilment | `Kit` |
| `UpsertProductRequest` | `api` crate | catalog | `Product` |
| `UpsertRecipientRequest` | `api` crate | account | `Recipient` |
| `UpsertVariantRequest` | `api` crate | catalog | `Variant` |
