# 00: Scope

## The idea, in the user's words

> "can you make a plan for mondaymerch.com it should have the same functionality at the .com website, i want a competitor for custom merch for enterprises."

Read as: a competing product under its own brand, matching what mondaymerch.com offers, aimed at enterprise buyers of custom merchandise.

## Fit gate

**Fits.** This is a backend system with its state in Postgres: quote and order workflows, supplier and warehouse lifecycles, carrier and payment integrations, invoices and a ledger. Two parts stay outside the binary. Marketing pages are plain files, and any image processing (mockup rendering, vectorising) runs in the browser or at an external provider, because media processing does not fit this architecture.

## What mondaymerch.com does

Observed on the public site on 2026-11-01. This is the parity reference. Their signed-in platform was not visible, so its behaviour comes from their marketing pages and may be overstated.

| Area | Observed | Where |
| --- | --- | --- |
| Order flow | Pick products, colours and quantity, upload a logo, request a quote with no payment. A quote and free designs follow within 24 hours. | `/faq`, `/shop/quote` |
| Quote form | Work email, name, company, optional VAT number, phone, delivery address, "ship to multiple addresses", standard or express timeline, "have a deadline?" | `/shop/quote` |
| After signing | Invoice within 48 hours, payment terms 7 days, no changes once signed. Website prices are a reference, and the quote is binding. | `/faq` |
| Payment | Bank transfer, or card with a 3.5% fee | `/faq` |
| VAT and duties | EU quotes exclude VAT for VAT-registered customers, and VAT is added on the invoice. Non-EU shipments go DDP. "Delivered duty paid to most countries", 150+ countries claimed. | `/faq`, `/use-cases/enterprise` |
| Product page | Quantity tiers (example: 15 at €19.46, 250 at €17.03), 55 colours, print methods and areas, materials, impact note, per-country delivery estimate, express at +10% and 2 days faster | `/shop/products/...` |
| Catalogue | 14 top-level categories, about 70 subcategories, collections, brands. Filters: price, timeline, brand, colour, sustainability. Cards show MOQ (most start at 15) and production days. | `/shop/...` |
| Design Studio | Brand import from a name or website, Brand Hub, templates, front/back/sleeve/label views, cut guides, 3D preview, Shuffle, Vectorize with AI, Figma and Canva import, print-method cost comparison | `/design-studio` |
| Platform | One dashboard for quotes, designs, orders, stock and shipments. Reorders, event planner, spend per order, team or event, invoices. Free. | `/services/merch-platform` |
| Warehousing | Live stock, low-stock alerts, restock requests, ship to offices, employees or events. From €0.25 per product, one-time. | `/services/warehousing`, `/pricing` |
| Shipping | Regular and express, CSV or HR/CRM recipients with labels, dispatch date, carrier and tracking per shipment. DHL, FedEx, PostNL. From €5.99 per pack, plus fulfilment from €2 per pack. | `/services/shipping`, `/pricing`, `/faq` |
| Distribution | Redeem Links (free, voucher budget, collects size and address). Team Store (€299/month, €10k/year minimum). Brand Store (€499/month, €20k/year minimum). Group Order (€599/order, €5k/order minimum). | `/team-store` |
| Enterprise | SSO and SAML, POs through Coupa, SAP Ariba or Oracle, punchout catalogue, HR/CRM connection, billing per department or cost centre, volume rebates, EU and US production and warehouses | `/use-cases/enterprise` |
| Integrations | Workday, Personio, Zapier, Salesforce. Triggers: onboarding, birthdays, anniversaries, offboarding, parental leave. REST API "possible" for custom stores. | `/uk/integrations`, `/faq` |
| Samples | Standard samples, 3 to 7 days | `/faq` |

Not found: their MOQ rules beyond product pages, warehouse locations, SLA terms, and how approvals work in the platform.

## What this plan does not copy

Functionality is the target. Everything below must be the competitor's own, because copying it invites trademark, copyright or misleading-advertising trouble:

- The name, domain, logo and visual identity. "Monday Merch" appears only in this research record. **The product still needs a name**, and the repository name `monday-merch` should not reach customers.
- Page copy, product photography, templates and design assets.
- Customer logos, stories and review scores.
- Certification claims. ISO 9001, 14001, 27001 and EcoVadis are earned by a company, so no page may claim them until they are held.

## What software cannot supply

These decide whether the business can run. The plan models them, but it does not create them:

- Production suppliers willing to work at low MOQs, with price lists and product data.
- A warehouse or third-party logistics provider, and carrier contracts.
- Consultants and designers to meet a 24-hour quote and design promise.
- Tax and legal advice on EU VAT, DDP import registrations, and invoice retention.
- The certifications enterprise procurement teams ask for. Budget time for ISO 27001 early.

## Scope ledger

Decisions: `include`, `exclude`, `later`, or `open` with the round it is planned for. Rows marked "round 1" are the user's answers. Any other `include` follows directly from "same functionality" or because the order flow cannot work without it, and has not been individually confirmed, so correct anything here that is wrong.

### Public site

| Feature | Decision | Rationale |
| --- | --- | --- |
| Marketing, service and use-case pages | `include` | Plain files in the bucket |
| Customer stories and reviews pages | `include` | Page templates only, filled with real customers once they exist |
| Articles | `include` | Plain files, important for search traffic |
| Contact and sales form | `include` | `SubmitLead`, routed to staff by email |
| Call scheduling | `later` | A link to a scheduling tool is enough |
| Live chat | `later` | Third-party widget, no backend |
| Trust centre | `later` | Needs real certifications to show |
| Languages and country sites | `open` round 3 | Affects templates and prices |
| Inspiration gallery | `open` round 4 | Content rather than system |

### Catalogue

| Feature | Decision | Rationale |
| --- | --- | --- |
| Categories, collections, brands | `include` | Core |
| Product pages with colours, materials, impact note, badges | `include` | Core |
| Quantity tier pricing and live total | `include` | Core |
| Filters on price, timeline, brand, colour, sustainability | `include` | Core |
| Search | `include` | Postgres full-text, no search engine |
| Delivery estimate per country, standard and express | `include` | Shown on the product page and quote form |
| Supplier catalogue feed import | `open` round 3 | Hand entry will not scale to thousands of products |
| Recommendations ("goes well with") | `later` | Conversion polish |

### Quote and order

| Feature | Decision | Rationale |
| --- | --- | --- |
| Quote request without an account | `include` | Their main entry point |
| Consultant workspace with 24-hour SLA queue | `include` | A quote-led flow cannot run without it |
| Human-made free design with the quote | `include` | Their core promise, and it works without the Studio |
| Online quote signature | `include` | Turns a quote into an order |
| Order tracking and reorder | `include` | Core platform |
| Deadline note on a quote | `include` | A text field a consultant reads |
| Self-serve checkout | `later` | Round 1. Every order starts as a signed quote at launch. |
| Samples | `open` round 3 | Could reuse the quote flow |
| Event planner | `open` round 3 | Works back from an event date |
| Pre-production sample approval for bespoke items | `open` round 3 | Adds a wait to production |

### Design

| Feature | Decision | Rationale |
| --- | --- | --- |
| Brand Hub with uploaded logos, colours, fonts | `include` | Consultants need the assets |
| Proofs and change requests on a quote | `include` | Signing a quote approves its designs |
| Design Studio editor | `later` | Round 1. Designers produce proofs at launch. |
| Brand import, templates, 3D preview, Shuffle, Vectorize, Figma and Canva import | `later` | Follow the Studio. Asked when it is scheduled. |

### Warehouse and shipping

| Feature | Decision | Rationale |
| --- | --- | --- |
| Ship an order to one address | `include` | Core |
| Carrier labels and tracking | `include` | Provider chosen in round 3 |
| Held stock, kits and shipping from stock to many addresses | `include` | Round 1. Needs a warehouse or logistics partner before launch. |
| Pack builder on the site | `open` round 3 | Kits are included, so this is now a real candidate |
| Third-party logistics integration | `open` round 3 | Depends on who runs the warehouse |
| Non-EU DDP | `open` round 3 | Needs tax registrations and a duty partner |
| US warehouse and production | `open` round 3 | Second region |
| Returns | `open` round 4 | Not described on their site |

### Company stores and distribution

| Feature | Decision | Rationale |
| --- | --- | --- |
| Redeem Links | `open` round 2 | Adds a `stores` nanoservice and vouchers on the ledger |
| Team Store | `open` round 2 | Approvals, budgets, cost centres |
| Brand Store (public) | `open` round 3 | Consumer checkout changes VAT and returns rules |
| Group Orders | `open` round 3 | Could share Team Store machinery |

### Enterprise

| Feature | Decision | Rationale |
| --- | --- | --- |
| Companies, members and roles | `include` | "For enterprises" |
| Departments and cost centres | `include` | "For enterprises" |
| Spend reporting | `include` | Per order, department or event |
| Audit log | `include` | Procurement and security reviews ask for it |
| SSO and SAML | `open` round 2 | See the risk in `01-architecture.md` |
| Punchout and Coupa, Ariba, Oracle POs | `open` round 2 | Large integration, may be the enterprise door-opener |
| HR and CRM connectors with automations | `open` round 3 | Adds an `integrations` nanoservice |
| Purchase approvals in the platform | `open` round 3 | Their site only describes approvals in stores, so this may differentiate |
| Public REST API and webhooks | `open` round 3 | Their site barely mentions one, so this may differentiate |
| Volume rebates | `open` round 3 | Pricing rule plus a credit |

### Money

| Feature | Decision | Rationale |
| --- | --- | --- |
| Invoices on terms, default 7 days | `include` | Their flow |
| Bank transfer recorded by staff | `include` | Their flow |
| Credit notes, overdue reminders | `include` | Issued invoices are frozen, so corrections need them |
| EU VAT including reverse charge | `include` | Needed for the first EU invoice |
| Trial balance and VAT return | `include` | Directive §11 |
| Card payment of an invoice | `include` | Round 1. Stripe Checkout, without their 3.5% fee. |
| Multi-currency | `open` round 3 | Each currency is a ledger unit |
| Accounting software export | `open` round 3 | Finance will ask for it |
| E-invoicing (Peppol) | `open` round 3 | Enterprise buyers and EU rules are moving this way |
| Bank feed matching | `later` | Staff record payments at first |
| Supplier payables on the ledger | `later` | Cost stays a plain field, payables stay in accounting software |

### Compliance

| Feature | Decision | Rationale |
| --- | --- | --- |
| GDPR controls: retention, erasure, access logging | `include` | Recipients' home addresses are personal data |
| ISO and EcoVadis certification | `exclude` | Company certifications, not software. See "What software cannot supply". |

## Decided

**Round 1 (2026-11-01):** self-serve checkout `later`, card payment `include`, held stock and kitting `include`, Design Studio `later`. No notes were added. The answers match the assumptions the first commit was written for, so the manifest's nanoservices did not change. No warehouse or logistics partner was named, so that remains a business prerequisite.

## Round 2: asked now

Unlike round 1, these are not in the manifest yet, because they add a nanoservice or an identity decision. Answers add them.

| Candidate | Recommended | What the answer changes |
| --- | --- | --- |
| Redeem Links | `include` | Adds a `stores` nanoservice with a redemption lifecycle, and vouchers on the ledger |
| Team Store | `later` | `include` adds store catalogues, budgets, approvals and a monthly fee to `stores` |
| SSO and SAML | `later` | `include` requires settling the identity question in `01-architecture.md` first |
| Punchout and procurement systems | `later` | `include` adds cXML punchout and purchase order intake |

## Open candidates for later rounds

- **Round 3:** Brand Store, Group Orders, HR and CRM connectors, purchase approvals, pack builder, languages and regions, multi-currency, supplier catalogue feeds, third-party logistics and the carrier provider, DDP, US region, samples, event planner, bespoke approvals, public API, volume rebates, accounting export, e-invoicing.
- **Round 4:** inspiration gallery, returns, plus anything raised on the way.

## Status

Not ready. Round 2 is waiting for answers.
