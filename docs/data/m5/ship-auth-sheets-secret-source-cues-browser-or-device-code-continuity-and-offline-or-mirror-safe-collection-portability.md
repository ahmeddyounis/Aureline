# Auth sheets, secret-source cues, browser/device-code continuity, and offline or mirror-safe collection portability

## Scope

This document describes the records that make API auth configuration and
collection portability explicit and honest across the request workspace. Each
**auth sheet** states the auth scheme, the secret source mode, the token
lifetime, the expiry label, the browser/device-code continuity state, and the
policy note. Each **secret-source cue** names where a credential resolves from —
secret broker, local encrypted store, managed rotation, or policy lock — and its
provenance. Each **browser/device-code continuity** row keeps an interrupted
browser or device-code sign-in resumable behind a non-secret verification
handle. Each **collection-portability** row keeps export and import
contract-source, retention-mode, and redaction-posture state intact and labels
contract freshness honestly when a collection reopens offline or from a mirror.

No record carries a raw secret, raw token, raw credential body, raw cookie, or
raw certificate key. Secrets are never written into versioned request files;
exported or imported collections never lose contract/source or redaction state;
offline and mirror-safe collections never masquerade stale or imported truth as
live; and browser-companion and managed origins never inherit desktop-local
trust.

The records reuse the canonical frozen vocabulary (`contract_source_class`,
`contract_freshness_state`, `retention_mode`, `offline_mirror_behavior`,
`request_origin_kind`) from the API-collection matrix, the auth-source vocabulary
(`auth_source_mode`, `auth_source_provenance`) from the request-workspace lane,
the secret-safe storage and mirror/offline vocabulary
(`secret_safe_auth_storage_mode`, `mirror_or_offline_state_class`) from the
query-history lane, and the export redaction vocabulary
(`export_redaction_class`) from the composer redaction-safe export lane, rather
than minting a local synonym set.

## Truth sources

- Implementation: `crates/aureline-api/src/ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability/mod.rs`
- Schema: `schemas/data/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.schema.json`
- Checked-in packet: `artifacts/data/m5/ship-auth-sheets-secret-source-cues-browser-or-device-code-continuity-and-offline-or-mirror-safe-collection-portability.json`
- Fixtures: `fixtures/data/m5/ship_auth_sheets_secret_source_cues_browser_or_device_code_continuity_and_offline_or_mirror_safe_collection_portability/`
- Upstream matrix: `artifacts/data/m5/freeze-the-api-collection-contract-source-request-origin-and-persisted-operation-matrix.json`
- Upstream request workspace: `artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json`
- Upstream query history: `artifacts/data/m5/ship-query-history-connection-profile-portability-secret-safe-auth-storage-and-mirror-or-offline-truth.json`

## Locked vocabulary

| Term | Family | Meaning |
|---|---|---|
| `no_auth`, `basic`, `bearer`, `api_key`, `o_auth2_authorization_code`, `o_auth2_client_credentials`, `o_auth2_device_code`, `browser_session`, `mtls` | auth scheme | The authentication scheme an auth sheet configures. |
| `no_expiry`, `short_lived`, `refreshable`, `expired`, `session_bound`, `unknown` | token lifetime | The token lifetime / expiry posture shown on a sheet. |
| `not_applicable`, `pending`, `awaiting_user_authorization`, `authorized`, `expired`, `denied` | browser/device-code state | The continuity state of a browser-redirect or device-code flow. |
| `local_encrypted`, `secret_broker_only`, `managed_rotation`, `policy_locked` | storage mode | Where a credential value lives, reused from the query-history lane. |
| `request_file`, `workspace_default`, `policy_injection`, `ad_hoc_override`, `secret_broker` | provenance | The provenance of a credential reference, reused from the request-workspace lane. |
| `export`, `import` | portability direction | Whether a collection is exported or reopened/imported. |
| `live_contract`, `cached_schema`, `imported_snapshot`, `plugin_provided`, `contract_unavailable` | contract source | The contract source preserved across export/import, reused from the matrix. |
| `text_first_versioned`, `metadata_only`, `redacted_replayable`, `opt_in_full_capture` | retention mode | The retention mode preserved across export/import, reused from the matrix. |
| `full_redaction`, `metadata_only`, `safe_preview`, `unredacted_local_only` | export redaction | The redaction posture preserved across export/import, reused from the composer lane. |

## Consumer surfaces

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Auth sheet panel | stable | stable | Shows scheme, secret source, token lifetime, browser/device-code state, and policy notes without persisting a raw secret. |
| Secret-source cue | stable | stable | Names where a credential resolves from and its provenance without exposing the value. |
| Browser and device-code continuity flow | stable | stable | Keeps an interrupted browser or device-code sign-in resumable behind a non-secret verification handle. |
| Collection export and import portability | stable | stable | Preserves contract source, retention mode, and redaction posture and labels contract freshness honestly when reopened offline. |
| CLI and headless auth/portability output | stable | stable | Prints the auth scheme, secret source, expiry, continuity state, and portability posture without raw secrets. |
| Support export auth/portability truth | stable | stable | Carries the auth/portability posture with redaction-safe content, never raw secrets, tokens, or credential bodies. |
| Help and About auth/portability contract | stable | stable | Describes the auth scheme, secret source, expiry, continuity, and offline-safe portability contract. |

## Auth, continuity, and portability rules

- Every auth sheet keeps the scheme, secret source, token lifetime, expiry,
  browser/device-code state, and policy note visible; no sheet carries or
  persists a raw secret, and secrets are never written into versioned request
  files.
- A browser-redirect or device-code scheme always carries a real continuity
  state; every other scheme carries the not-applicable sentinel. A live-secret
  source mode is always backed by a secret-source cue; a no-live-secret mode
  carries none.
- Secret-source cues are visible without exposing the value and never persist a
  secret into the repo; each cue names its storage mode and provenance.
- A browser or device-code continuity row tracks only browser-redirect or
  device-code flows, never carries a raw token, surfaces a user-action prompt
  while it waits on the user, and stays resumable where the flow allows.
- Collection export and import never drop contract source, retention mode, or
  redaction posture, never carry or persist a raw secret, and keep request
  definitions text-first and versionable.
- An offline or imported collection labels its freshness honestly and never
  claims a live contract; contract source and freshness state always agree.
- Managed-workspace and browser-companion origins isolate desktop-local trust
  and never inherit local naming assumptions.
- The lane references the frozen API-collection matrix, the request-workspace
  auth-source lane, and the query-history secret-safe storage lane as verified
  upstream packets so its contract, origin, auth-source, and storage vocabularies
  stay aligned.
