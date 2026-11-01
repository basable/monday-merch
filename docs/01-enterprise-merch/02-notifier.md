# 02: notifier

Sends one templated email per request. It owns one external call and one catalog, and no lifecycle, because the caller's own pass owns the "did it go out" question. Directive §6 binds the call, §7 the message.

## External call

**`send_email`** on `email`, strategy `keyed_replay`, `irreversible` because an email cannot be unsent, `replayWindow: 24h`.

The working provider is **Resend**, because its documentation states the window: "Idempotency keys are kept in the system for 24 hours." On replay inside the window it returns the first result. A different payload under one key returns `409 invalid_idempotent_request`, and a replay during the original returns `409 concurrent_idempotent_requests`, which is safe to retry. A different provider would need a re-read of this section. Before sign-up, confirm the sending region suits GDPR.

**Key and intent:** the caller supplies a deterministic key from its own identity (such as `invoice:<id>`) and the time its intent was written to its own row. `dispatch` refuses an intent older than the window, so past 24 hours the caller holds for a person and never risks a second email. This is why callers use `claim.write_status` before the send. This nanoservice dispatches `Unfenced`, because it holds no claim of its own.

Classification is `classify_fail_closed`: only a provider-confirmed rejection counts as definitive. The simulator models replay, ack loss, a concurrent replay and a payload mismatch.

## Config types

`email_template` per `slug` and `locale`. See `04-config.md`. Rendering is a pure function of the template and the caller's values, so the same request produces the same payload, which keeps a replay valid.

## Messages

`SendEmailRequest` → `EmailReceipt`, a typed request so the sender learns the outcome. Senders: `account`, `order`, `fulfilment`, `billing`.

Emails link to the dashboard or a short-lived download, rather than attaching documents. Purchase orders link to artwork through a presigned URL.

**Kratos's own emails** (sign-in codes) do not pass through here. Kratos sends them itself over SMTP. See `05-deployment.md`.

## Binding Directive sections

§6 (`send_email`), §7 (message), §10 (docs).

## Tests

- `effecttest` audit for `send_email` with ack loss.
- A replay with the same key returns the first receipt and sends once.
- An intent older than the window is refused, with no send.
- Every seeded template renders with its documented variables.
