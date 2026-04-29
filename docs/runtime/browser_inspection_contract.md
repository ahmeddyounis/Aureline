# Browser Inspection Evidence Contract

This document freezes the packet family Aureline uses when browser
inspection surfaces show console events, network evidence, and client
storage state. It extends the browser-runtime contract rather than
replacing it: every inspection packet cites one
`browser_runtime_session_record` and, when available, the same preview
route, preview snapshot, task channel, support bundle, and exact-build
identity refs used by the surrounding preview or evidence reader.

The goal is to prevent browser inspection panes from becoming a hidden
source of truth. Runtime evidence can be live, imported, replayed,
cached, or blocked. Source maps can be exact, approximate, stale, or
unavailable. Storage and request payloads can contain secrets. Each of
those facts MUST be visible at the point where a user opens source,
copies evidence, replays a request, mutates storage, or adds evidence
to a support bundle.

If this document disagrees with the PRD, Technical Architecture
Document, Technical Design Document, UI / UX Spec, Design System Style
Guide, the preview-runtime contract, or the browser-runtime contract,
those source documents win and this document plus its companion schemas
MUST be updated in the same change.

## Companion Artifacts

- [`/schemas/runtime/console_event.schema.json`](../../schemas/runtime/console_event.schema.json)
  - boundary schema for `console_event_record`.
- [`/schemas/runtime/network_event_ref.schema.json`](../../schemas/runtime/network_event_ref.schema.json)
  - boundary schema for `network_event_ref_record`.
- [`/schemas/runtime/storage_object_state.schema.json`](../../schemas/runtime/storage_object_state.schema.json)
  - boundary schema for `storage_object_state_record`.
- [`/fixtures/runtime/browser_inspection_cases/`](../../fixtures/runtime/browser_inspection_cases/)
  - worked cases covering live console evidence, stale source maps,
    replayed network traffic, service-worker cache disclosure,
    sensitive storage redaction, and inspect-only fallback.
- [`/docs/runtime/browser_runtime_contract.md`](./browser_runtime_contract.md)
  - owning contract for browser-runtime session identity, source-map
    freshness, mutation-admissibility, inspection lifecycle, and
    preview-route linkage.
- [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  - owning contract for preview snapshots, mapping confidence, source
    sync, hot reload, and stale-editability.
- [`/docs/execution/task_event_and_evidence_contract.md`](../execution/task_event_and_evidence_contract.md)
  - task-channel and evidence linkage.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  - support-bundle manifest, redaction, and inclusion review rules.
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  - exact-build identity propagation.

## Scope

Frozen at this revision:

- one console-event evidence shape for browser logs, errors,
  exceptions, unhandled rejections, framework runtime events, and
  protocol events;
- one network-event reference shape for request rows, copied-request
  reviews, HAR-like evidence packets, replay reviews, and service-
  worker cache disclosures;
- one storage-object state shape for cookies, local storage, session
  storage, IndexedDB, Cache Storage, service-worker registrations, and
  partitioned storage metadata;
- shared capture-mode, liveness, mapping-fidelity, source-map state,
  redaction, policy-restriction, drift-disclosure, export, and viewer-
  linkage vocabularies;
- point-of-action disclosure rules for live DOM / CSS edits, stale
  source maps, replayed network traffic, cached service-worker state,
  storage mutations, and weak runtime identity;
- export and privacy rules for copied requests, HAR-like evidence,
  storage keys, cookies, blocked sensitive values, and partner /
  provider data.

Out of scope:

- building a browser debugging tool, network inspector, storage
  inspector, request replayer, service-worker manager, or live style
  editor;
- browser-engine protocol details such as CDP, WebDriver BiDi, or WDP;
- raw console text, raw request or response bodies, raw header values,
  raw cookie values, raw storage values, raw URLs, raw hostnames, raw
  ports, raw query strings, raw stack frames, raw source files, raw
  screenshots, or raw provider payloads.

## Shared Inspection Spine

Every console, network, and storage record MUST carry:

- `browser_runtime_session_record_ref` - the runtime session that owns
  runtime identity, source-map freshness, inspection lifecycle, and
  mutation-admissibility.
- `viewer_linkage` - refs back to the preview route, preview snapshot,
  task channel, support bundle, exact-build identity, and evidence
  packet when those objects exist. A missing ref is expressed as null,
  not as a surface-specific alias.
- `capture_context` - capture mode, liveness, evidence origin, and
  inspected-at timestamp.
- `source_map_state` - source-map freshness and mapping fidelity
  projected from the browser-runtime and preview-runtime contracts.
- `redaction` - the declared redaction class and whether raw payloads,
  keys, values, cookies, headers, or provider data are permitted to
  escape.
- `policy_restrictions[]` - typed restrictions active at capture,
  action, copy, export, or mutation time.
- `drift_disclosures[]` - typed disclosures surfaces MUST render at
  the point where the user acts.

These fields are intentionally duplicated by reference on each packet.
A console row copied into a support bundle, a network row opened from a
task channel, and a storage row shown in a hosted review reader must
all preserve the same runtime and build lineage without scraping labels
from a UI pane.

## Console Event Records

`console_event_record` represents one browser-side console or runtime
event. It records severity, event class, occurrence counts, source
anchor posture, stack-trace posture, message redaction, policy
restrictions, export posture, and viewer linkage.

Required disclosures:

- The row MUST show whether the event is live, imported, replayed,
  cached, blocked, or unavailable.
- The row MUST show mapping fidelity separately from the source label.
  `app code` or a source ref is not enough; the user must see whether
  mapping is exact, approximate, runtime-only, unavailable, or under
  review.
- A stale, redacted, missing, or unknown source map MUST downgrade
  source-jump actions and mark export evidence as approximate or
  metadata-only.
- Console message bodies are redacted by default. A raw message body
  may only be represented by an opaque local-only ref after a reviewed
  local reveal step.

## Network Event References

`network_event_ref_record` is a reference row, not a raw HAR entry. It
records method class, request target class, initiator, result posture,
timing posture, cache posture, payload/header/cookie presence classes,
copy/export policy, replay admissibility, and viewer linkage.

Required disclosures:

- Raw URLs, headers, cookies, query strings, request bodies, and
  response bodies MUST NOT appear in the record. Opaque target refs,
  digest refs, count metadata, and typed classes do.
- A copied request MUST declare whether it is a redacted summary,
  header/key-only summary, local-only reveal candidate, or forbidden.
- HAR-like evidence MUST declare whether it is metadata-only, redacted,
  local-only, or forbidden before export starts.
- Replayed traffic MUST be labeled as replayed at the row and action
  sheet. The record MUST preserve the capture target, current runtime
  target, and replay admissibility so a captured request cannot be
  silently run against a different boundary.
- Service-worker cache state MUST be explicit. Stale or unverifiable
  service-worker state blocks replay and must be surfaced before copy
  or export if it affects the evidence.
- Partner or provider data MUST remain by reference or metadata-only
  unless policy explicitly allows broader export.

## Storage Object State Records

`storage_object_state_record` represents one storage object or storage
scope row. It covers cookies, local storage, session storage,
IndexedDB, Cache Storage, service-worker registrations, service-worker
cache entries, and partitioned storage metadata.

Required disclosures:

- Storage keys are redacted or hashed by default unless the key class
  is explicitly review-safe.
- Values are never exported by default. The row records value presence,
  digest refs, size class, and reveal / export posture instead.
- Cookie rows MUST distinguish cookie-name exposure from cookie-value
  exposure; cookie values are secret-bearing by default.
- Storage mutation actions MUST show target origin/scope, session
  impact, side-effect risk, policy restriction, and support-bundle
  inclusion posture before execution.
- Imported, replayed, cached, or weak-identity storage rows are
  inspect-only. They may explain state, but they do not grant live
  mutation authority.

## Drift and Disclosure Rules

Browser inspection surfaces MUST label drift where users act, not in a
secondary diagnostic panel.

### Live DOM / CSS Edits

If a live DOM or CSS edit exists in the inspected runtime and has not
been reconciled to canonical source:

- console, network, and storage records associated with that runtime
  MUST carry `live_dom_css_edit_uncommitted`;
- source mapping MUST NOT be shown as better than the mapping class
  reported by the paired browser-runtime session and preview snapshot;
- exports MAY include an overlay or transform ref, but MUST NOT imply
  that the canonical source already contains the runtime edit;
- a mapped write or replay action that depends on edited runtime state
  MUST require review or downgrade to inspect-only.

### Stale Source Maps

When `source_map_state.source_map_state_class` is stale, missing,
redacted, or unknown:

- source jump, copied evidence, replay review, and storage mutation
  surfaces MUST show a stale-mapping disclosure at the action site;
- console source anchors cannot claim exact source mapping;
- modified request replay is blocked or requires a renewed capture
  with exact mapping;
- support exports carry the stale mapping class and source-map handle
  ref instead of raw source-map bytes.

### Replayed Network Traffic

When a request row comes from replay capture or imported evidence:

- liveness MUST be `replayed_read_only` or `imported_read_only`;
- replay and copy actions MUST state the original target boundary and
  current target boundary by opaque refs;
- auth/session state is disclosed as a class, never by copying cookies
  or bearer tokens;
- replay against a live runtime requires a new review packet and cannot
  reuse a stale approval from capture time.

### Cached Service-Worker State

When network or storage evidence depends on a service worker:

- `cache_state_class` or `storage_area_class` MUST name service-worker
  live, stale, imported, or unverifiable state;
- stale or unverifiable service-worker state blocks request replay and
  service-worker mutations;
- exported evidence includes the service-worker registration ref,
  scope class, and cache posture, not script bodies or cached payloads.

### Storage Mutations

Before clearing or deleting client storage:

- the action sheet MUST show storage area, origin/scope class, side-
  effect risk, liveness, redaction posture, and support-bundle
  inclusion posture;
- sensitive values remain hidden even when the action is permitted;
- imported, replayed, cached, or weak-identity rows must stay
  inspect-only;
- support bundles exclude raw values by default and record any local-
  only reveal as a reviewed local step.

### Weak Runtime Identity

When runtime identity, target identity, protocol scope, or source-map
identity is weak:

- the associated packet MUST carry
  `inspect_only_runtime_identity_weak` or a more specific policy
  restriction;
- all mutation and replay actions resolve to blocked or inspect-only;
- export defaults to metadata-only or forbidden for sensitive rows;
- viewer linkage still preserves the best available runtime, route,
  task, support, and exact-build refs so support can reconstruct why
  the surface degraded.

## Export and Privacy Rules

Exports cannot silently widen beyond the declared redaction and policy
rules.

- **Copied requests:** default to `copy_redacted_summary`. Raw request
  copy requires a local-only reviewed reveal and cannot include cookies,
  bearer tokens, client certificates, or secret-bearing headers.
- **HAR-like evidence:** default to metadata-only or redacted. Request
  and response bodies are omitted unless a policy-approved local export
  explicitly includes them. Provider or partner data stays by reference
  unless the provider contract permits export.
- **Storage keys:** default to hash or opaque ref. Full keys are
  allowed only for review-safe classes and never for cookies or
  secret-bearing stores by default.
- **Cookies:** cookie values are always sensitive. Cookie names may be
  redacted, hashed, or shown as review-safe labels depending on policy.
- **Blocked sensitive values:** if a classifier or policy marks a
  value blocked, the record uses `blocked_sensitive_value` and export
  posture `export_forbidden` or `excluded_always`.
- **Support bundles:** inclusion posture travels in each record. A
  bundle manifest must list included, redacted, by-reference,
  local-only, and excluded browser inspection evidence separately.

## Viewer Linkage Rules

Browser inspection evidence MUST preserve joinability without alias
drift:

- Preview surfaces join through `preview_route_record_ref` and
  `preview_snapshot_record_ref`.
- Execution surfaces join through `task_channel_ref`.
- Support/export surfaces join through `support_bundle_ref` and
  evidence packet refs.
- Build and mapping surfaces join through `exact_build_identity_ref`
  and source-map handle refs.
- Hosted review and companion handoff readers preserve the original
  refs and add reader-specific refs instead of rewriting identity.

If a ref is unavailable, the field is null and a policy restriction or
drift disclosure explains why. A surface MUST NOT replace the missing
ref with a display label, provider URL, hostname, or route string.

## Change Discipline

Adding a new capture mode, liveness class, mapping-fidelity class,
source-map state, redaction class, policy restriction, drift
disclosure, export class, request target class, cache state, storage
area class, mutation action, or mutation admissibility value is
additive-minor and bumps the relevant schema-version const.

Repurposing an existing value, adding raw payload fields to these
schemas, or weakening default redaction is breaking and requires a
new governance decision plus fixture updates in the same change.
