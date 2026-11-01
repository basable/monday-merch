import { BRAND } from "./brand";

const steps = [
  { title: "Choose", body: "Pick products, colours and quantities, with the price per piece in view." },
  { title: "Design", body: "Design it in the browser, or send your logo and let a designer prepare it." },
  { title: "Sign", body: "Review the proofs and the quote, then sign. Your invoice follows from there." },
  { title: "Receive", body: "Delivered to one office or many homes, or stored and shipped when you need it." },
];

const roadmap = [
  { name: "Email and invoice intake", note: "Mail is received, read and queued for review automatically", state: "First" },
  { name: "Ledger and supplier bills", note: "Every amount owed is booked, and finance approves each bill", state: "Next" },
  { name: "Shop and quotes", note: "The public catalogue and the quote and signature flow", state: "Planned" },
  { name: "Production, stock and shipping", note: "Purchase orders, held stock and carrier tracking", state: "Planned" },
  { name: "Design Studio", note: "A browser editor with print areas and live prices", state: "Planned" },
];

const checks = [
  { label: "Sender verified", ok: true },
  { label: "Totals add up", ok: true },
  { label: "Matches an open purchase order", ok: true },
  { label: "Bank account differs from the one on file", ok: false },
];

function ReviewCard() {
  return (
    <figure className="review" aria-label="Example of an invoice waiting for review">
      <figcaption className="review-head">
        <span className="review-title">Review queue</span>
        <span className="tag">Example</span>
      </figcaption>
      <dl className="review-fields">
        <div>
          <dt>Supplier</dt>
          <dd>Example Textiles B.V.</dd>
        </div>
        <div>
          <dt>Invoice</dt>
          <dd>INV-0142</dd>
        </div>
        <div>
          <dt>Total</dt>
          <dd>€4,235.00</dd>
        </div>
      </dl>
      <ul className="review-checks">
        {checks.map((c) => (
          <li key={c.label} className={c.ok ? "check-ok" : "check-flag"}>
            <span aria-hidden="true">{c.ok ? "✓" : "!"}</span>
            <span className="visually-hidden">{c.ok ? "Passed: " : "Flagged: "}</span>
            {c.label}
          </li>
        ))}
      </ul>
      <p className="review-foot">Held for finance. A person decides before anything is booked.</p>
    </figure>
  );
}

export function Landing({ apiReady, signedIn }: { apiReady: boolean | null; signedIn: boolean | null }) {
  const status = apiReady === null ? "Checking the API" : apiReady ? "API ready" : "API starting";
  return (
    <>
      <main>
        <section className="hero">
          <div className="hero-copy">
            <p className="eyebrow">Early preview</p>
            <h1>Custom merch for companies, without the email threads.</h1>
            <p className="lede">
              {BRAND} takes merch from first quote to final delivery in one place, and reads the supplier email and invoices
              that usually pile up around it.
            </p>
            <div className="actions">
              {signedIn === true ? (
                <a className="button button-primary" href="#plan">
                  See what is being built
                </a>
              ) : (
                <a className="button button-primary" href="/.ory/self-service/login/browser">
                  Sign in to the preview
                </a>
              )}
              <a className="button button-quiet" href="#how">
                How it will work
              </a>
            </div>
          </div>
          <ReviewCard />
        </section>

        <section id="how" className="section">
          <h2>How it will work</h2>
          <ol className="steps">
            {steps.map((s, i) => (
              <li key={s.title} className="step">
                <span className="step-number" aria-hidden="true">
                  {i + 1}
                </span>
                <h3>{s.title}</h3>
                <p>{s.body}</p>
              </li>
            ))}
          </ol>
        </section>

        <section id="plan" className="section">
          <h2>What is being built, in order</h2>
          <p className="section-lede">Nothing below is finished yet. This preview shows the running foundation.</p>
          <ul className="roadmap">
            {roadmap.map((r) => (
              <li key={r.name} className="roadmap-row">
                <div>
                  <h3>{r.name}</h3>
                  <p>{r.note}</p>
                </div>
                <span className="tag">{r.state}</span>
              </li>
            ))}
          </ul>
        </section>
      </main>
      <footer className="footer">
        <span>{BRAND} is a working name.</span>
        <span className="status" role="status">
          <span className={`status-dot ${apiReady ? "status-dot-ok" : ""}`} aria-hidden="true" />
          {status}
        </span>
      </footer>
    </>
  );
}
