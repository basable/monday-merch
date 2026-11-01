# 04: Config catalogs

Declarative catalogs a person edits as JSON under `config/base/`, loaded in one transaction at boot by `basable-config`. A change is a commit and a deploy. The system never writes them. Other nanoservices read them through messages from the owner.

| Catalog | Owner | Fields | Read by | Seed |
| --- | --- | --- | --- | --- |
| `category` | `catalog` | slug, parent slug (empty for a root), title, sort order | `catalog` | The competitor's own category tree |
| `collection` | `catalog` | slug, title, active | `catalog` | Seasonal and pack collections |
| `brand` | `catalog` | slug, title | `catalog` | Brands actually stocked |
| `colour` | `catalog` | slug, title, hex, family (for the colour filter) | `catalog` | Standard colour list |
| `print_method` | `catalog` | slug, title, constraints (such as maximum colours, for the Studio to warn on) | `catalog` | Screen print, embroidery, digital transfer |
| `supplier` | `catalog` | slug, title, purchase order email, country, sender domains, bank accounts | `catalog`, `order` through `GetSupplierOffersRequest`, `inbox` through `MatchSupplierRequest` | Signed suppliers. A bank account change here is a reviewed commit, which is the protection against invoice fraud. |
| `pricing_policy` | `catalog` | express fee in basis points, express days faster | `catalog` | 1000 and 2, their observed +10% and 2 days, as a starting point |
| `quote_policy` | `order` | SLA hours, alert hours, validity days | `order` | 24, 20, 30. Validity is an assumption. |
| `warehouse` | `fulfilment` | slug, title, country, operator (`own` or a provider) | `fulfilment` | One EU warehouse |
| `shipping_zone` | `fulfilment` | slug, countries, standard and express transit days, currency, per-pack amounts, incoterm | `fulfilment` | From real carrier contracts |
| `fulfilment_fee` | `fulfilment` | currency, pack handling, warehouse inbound per unit | `fulfilment` | From real warehouse costs |
| `vat_rule` | `billing` | country, standard rate in basis points, valid from | `billing` | Every EU member state, checked by a tax advisor |
| `billing_policy` | `billing` | default terms days, card fee basis points, reminder days, supplier bill match tolerance | `billing` | 7, 0, [3, 10, 20], 100 (1%) as an assumption |
| `studio_font` | `design` | family, files, licence | `design` | Fonts licensed for print and for this use |
| `inbox_policy` | `inbox` | maximum attachment bytes, allowed types, auto-apply kinds, retention days, model, effort | `inbox` | 25 MiB, PDF and common images, none, 365, `claude-opus-5-5`, `low`. Retention needs legal review. |
| `email_template` | `notifier` | slug, locale, subject, HTML and text body | `notifier` | One per email the system sends |

Notes:

- **Amounts in seeds must come from the business.** Their public "from" prices (€2 handling, €5.99 shipping, €0.25 warehousing) are a benchmark only, not costs.
- **Cross-references use slugs**, so seeds read cleanly and no `#{...}` references are needed yet.
- **Supplier and category changes require a deploy.** That is fine at launch. If suppliers change weekly or staff need to edit them without an engineer, the catalog becomes a plain table with an admin API (Directive §1: something both people and the system change is two things).
- **A VAT rate change is a new `vat_rule` row** with a later `valid_from`, so a past invoice's rate stays explainable. Issued invoices carry their VAT in the spec regardless.
- Type ids are allocated by position, so new catalogs are appended at the end of the owner's list once the scaffold has rendered.
