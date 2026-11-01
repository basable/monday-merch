# 04: Config catalogs

Declarative catalogs a person edits as JSON under `config/base/`, loaded in one transaction at boot by `basable-config`. A change is a commit and a deploy. The system never writes them. Other nanoservices read them through messages from the owner.

| Catalog | Owner | Fields | Read by | Seed |
| --- | --- | --- | --- | --- |
| `category` | `catalog` | slug, parent slug (empty for a root), title, sort order | `catalog` | The competitor's own category tree |
| `collection` | `catalog` | slug, title, active | `catalog` | Seasonal and pack collections |
| `brand` | `catalog` | slug, title | `catalog` | Brands actually stocked |
| `colour` | `catalog` | slug, title, hex, family (for the colour filter) | `catalog` | Standard colour list |
| `print_method` | `catalog` | slug, title | `catalog` | Screen print, embroidery, digital transfer |
| `supplier` | `catalog` | slug, title, purchase order email, country | `catalog`, and `order` through `GetSupplierOffersRequest` | Signed suppliers |
| `pricing_policy` | `catalog` | express fee in basis points, express days faster | `catalog` | 1000 and 2, their observed +10% and 2 days, as a starting point |
| `quote_policy` | `order` | SLA hours, alert hours, validity days | `order` | 24, 20, 30. Validity is an assumption. |
| `warehouse` | `fulfilment` | slug, title, country, operator (`own` or a provider) | `fulfilment` | One EU warehouse |
| `shipping_zone` | `fulfilment` | slug, countries, standard and express transit days, currency, per-pack amounts, incoterm | `fulfilment` | From real carrier contracts |
| `fulfilment_fee` | `fulfilment` | currency, pack handling, warehouse inbound per unit | `fulfilment` | From real warehouse costs |
| `vat_rule` | `billing` | country, standard rate in basis points, valid from | `billing` | Every EU member state, checked by a tax advisor |
| `billing_policy` | `billing` | default terms days, card fee basis points, reminder days | `billing` | 7, 0, [3, 10, 20] |
| `email_template` | `notifier` | slug, locale, subject, HTML and text body | `notifier` | One per email the system sends |

Notes:

- **Amounts in seeds must come from the business.** Their public "from" prices (€2 handling, €5.99 shipping, €0.25 warehousing) are a benchmark only, not costs.
- **Cross-references use slugs**, so seeds read cleanly and no `#{...}` references are needed yet.
- **Supplier and category changes require a deploy.** That is fine at launch. If suppliers change weekly or staff need to edit them without an engineer, the catalog becomes a plain table with an admin API (Directive §1: something both people and the system change is two things).
- **A VAT rate change is a new `vat_rule` row** with a later `valid_from`, so a past invoice's rate stays explainable. Issued invoices carry their VAT in the spec regardless.
- Type ids are allocated by position, so new catalogs are appended at the end of the owner's list once the scaffold has rendered.
