# Auth sheets, secrets, sign-in, and portable collections

Aureline keeps **authentication** and **collection portability** explicit. An
auth sheet tells you how a request authenticates and where its secret comes from
— never by pasting the secret into a file — and exporting or importing a
collection keeps its contract, retention, and redaction state intact.

## What an auth sheet shows

For each request you always see:

- the **auth scheme** — none, Basic, Bearer, API key, OAuth (authorization code,
  client credentials, or device code), a browser session, or mTLS,
- the **secret source** — where the credential resolves from, shown as a cue, not
  a value,
- the **token lifetime / expiry** — no expiry, short-lived, refreshable,
  expired, session-bound, or unknown,
- the **browser or device-code state** when the scheme uses one, and
- any **policy notes** that constrain the request.

Secrets are never stored in your versioned request files. A **secret-source cue**
tells you where the value lives — the secret broker, your local encrypted store,
a managed rotation, or a policy lock — and where the reference came from, without
ever showing the value.

## Signing in with a browser or device code

Some schemes (OAuth authorization code, OAuth device code, and browser sessions)
need you to finish signing in. Aureline keeps that flow **resumable**: if it is
interrupted, you can pick it back up from a verification handle rather than
starting over. While the flow waits on you, it shows exactly what to do — open
the sign-in page or enter the device code. If a grant **expires** or you **deny**
it, that is shown plainly and no token is issued. Raw tokens are never shown or
stored.

## Exporting and importing collections

When you export a collection — or reopen one someone shared with you — Aureline
keeps:

- the **contract source** (live, cached, an imported snapshot, plugin-provided,
  or unavailable),
- the **retention mode** (text-first, metadata-only, redacted, or opt-in full
  capture), and
- the **redaction posture** of the export.

Request definitions stay **text-first** and versionable, and no secret is ever
written into the export.

## Offline and mirror-safe collections

A collection you reopen offline, from a mirror, or with networking disabled is
**honest about its contract**. If the contract is cached, stale, an imported
snapshot, or unavailable, it is labeled that way — it never pretends to be a live
contract just because it opened. Contract source and freshness always agree.

## Trust and safety

Managed-workspace and browser-companion requests keep their own origin and never
inherit desktop-local trust or naming assumptions, so a localhost name and a
managed or companion target are never confused in your auth sheets or your
collections.
