# Managed API/contract mismatch, supported-range disclosure, and local-safe fallback contract

This document freezes the shared runtime contract Aureline uses to make
managed API and contract skew **honest**, **visible**, and **locally
survivable**. When a hosted, managed, or privileged path is narrowed by
contract skew, schema drift, stale contract metadata, or policy gating,
surfaces must say so explicitly and must preserve a local-safe baseline
instead of turning local work into unexplained failure.

The contract exists to satisfy the UI/UX requirement that **Managed API
or contract mismatch** always discloses:
expected vs actual version, supported range, fallback behavior, and a
support/export action — without making online schema discovery a
prerequisite for local use.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this document
plus the companion schema and artifacts must update in the same change.

## Companion artifacts

- [`/schemas/runtime/contract_mismatch_state.schema.json`](../../schemas/runtime/contract_mismatch_state.schema.json)
  — boundary schema for `contract_mismatch_state_record`, the record
  emitted by mismatch evaluators and consumed by UI, CLI, diagnostics,
  service health, and support exports.
- [`/artifacts/runtime/supported_range_rows.yaml`](../../artifacts/runtime/supported_range_rows.yaml)
  — machine-readable registry of supported contract windows and required
  fallback posture. Surfaces use it to show the supported range
  explicitly and to keep export parity.
- [`/fixtures/runtime/contract_mismatch_cases/`](../../fixtures/runtime/contract_mismatch_cases/)
  — worked cases for same-build match, current-plus-previous-minor
  support, unsupported old service, future service, offline cache, and
  direct-service route denied by policy.

## Upstream contracts this contract rides on

This contract reuses upstream vocabulary; it does not mint parallel
names for already-frozen terms.

- [`/docs/adr/0013-docs-help-service-health-truth.md`](../adr/0013-docs-help-service-health-truth.md)
  and [`/schemas/docs/help_status_badge.schema.json`](../../schemas/docs/help_status_badge.schema.json)
  — shared `service_contract_state` vocabulary and parity rules across
  Help/About, service health, docs panes, diagnostics, and support
  exports.
- [`/docs/compat/boundary_matrix.md`](../compat/boundary_matrix.md),
  [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml),
  and [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json)
  — governed skew windows (including
  `current_plus_previous_minor_or_lts`) and the cross-boundary
  negotiation envelope these mismatch states may join to.
- [`/docs/runtime/connectivity_and_reconciliation_contract.md`](./connectivity_and_reconciliation_contract.md)
  — local-safe continuity posture and outbox-style refusal rules used
  when contract mismatch narrows write authority.

## Who reads this contract

- **Service health and Help/About** surfaces — to disclose supported
  range, fallback posture, and repair/export actions without inventing
  per-surface copy.
- **Managed-service and provider clients** — to decide whether a path is
  full, narrowed to read-only/cached safe operations, or blocked; and to
  produce an exportable record instead of prose-only logs.
- **Contract metadata cache** publishers — to mark stale/unknown state
  explicitly instead of silently falling back.
- **Doctor/diagnostics and support export** — to include the mismatch
  record and supported-range snapshot so “what happened” can be answered
  offline.
- **Command/action projection layers** — to narrow or block mutating
  actions with typed reasons when mismatch posture requires it.

## Out of scope

This contract does not define:

- protocol negotiation mechanisms, adapters, or transport stacks;
- provider-specific request/response bodies or raw payload schemas;
- the full UI microcopy deck for every mismatch banner and action sheet;
- shipping remote features; it freezes the disclosure and fallback
  contract those features must obey.

## Invariants (frozen)

Every surface that emits or consumes this contract follows these rules:

1. **Skew is explicit.** If a path is narrowed or blocked due to contract
   mismatch, schema drift, stale cache, mirror delay, or policy gating,
   surfaces MUST show the mismatch-domain cue and MUST NOT imply parity.
2. **Supported range is visible and exportable.** Supported range and
   fallback posture MUST be visible in-product and MUST be serializable
   into support bundles and diagnostics without scraping logs.
3. **Local-safe work survives.** When mismatch blocks only hosted,
   managed, or privileged paths, local-safe work remains available and
   is labeled as such (local-only or cached read-only) instead of
   degenerating into generic failure.
4. **Write authority never widens under uncertainty.** Unknown or stale
   contract metadata can narrow writes (read-only, idempotent-only) but
   must not widen write authority.
5. **No silent auto-retry into undefined behavior.** When the mismatch
   posture is outside the supported window, privileged writes are refused
   rather than partially executed.

## 1. Mismatch-domain taxonomy and required user cues

Every `contract_mismatch_state_record` carries one
`mismatch_domain_class` value. Each value has a required user-facing cue
family.

### 1.1 `managed_api_contract_mismatch`

Required cues:

- expected vs observed contract version;
- supported range label (from the supported-range row);
- fallback posture (full / cached read-only / local-safe / blocked);
- at least one repair/export action.

### 1.2 `provider_schema_drift`

Required cues:

- drift is labeled as a contract boundary (not “provider is down”);
- reads may continue only as cached safe operations when declared;
- write paths are blocked until the drift is repaired or the user routes
  to a safe local-only alternative.

### 1.3 `stale_contract_cache`

Required cues:

- cache staleness/unknown is explicit (never silent);
- local-safe baseline remains available;
- write authority is narrowed until a fresh range can be established.

### 1.4 `companion_browser_runtime_mismatch`

Required cues:

- mismatch boundary is labeled (companion/browser/runtime);
- a handoff path is offered (desktop/local or review-only);
- cross-boundary writes are blocked under mismatch.

### 1.5 `mirror_delayed_metadata`

Required cues:

- mirror delay is a freshness downgrade, not “unknown error”;
- cached metadata may be used with a stale label where safe;
- mutation dependent on up-to-date metadata is blocked until refresh.

## 2. Supported-range rows (frozen export surface)

`artifacts/runtime/supported_range_rows.yaml` is the registry surfaces
use to disclose supported ranges and fallback posture. Each row is a
stable id (`supported_range_row:*`) and declares:

- which boundary family and mismatch domain it applies to;
- the supported window class (for example
  `current_plus_previous_minor_or_lts`);
- a short `range_label` suitable for UI chips and exports;
- required fallback posture, plus read/write posture expectations;
- the allowed suggested actions.

Every `contract_mismatch_state_record` embeds a `supported_range` snapshot
that cites one row id and repeats the label and window class so support
bundles remain self-contained.

## 3. Contract mismatch state record (frozen)

`contract_mismatch_state_record` is the single record emitted when a
surface needs to explain contract skew honestly.

Required spine:

- mismatch-domain classification, boundary family, and (when available)
  compat/skew refs;
- expected and observed contract versions;
- supported range snapshot (row ref, source class, label, window class);
- mismatch-state class and fallback posture;
- read/write posture (normal / narrowed / blocked);
- projection onto the service-health contract state vocabulary;
- at least one suggested action;
- a short summary sentence and timestamp.

## 4. Projection rules (service health, diagnostics, exports, actions)

### 4.1 Service health

- The service-health aggregator MUST project the mismatch into exactly
  one `service_contract_state` value (`ready`, `degraded`, `local_only`,
  `stale`, `contract_mismatch`, `policy_blocked`, `unavailable`) and MUST
  not mint per-feature service vocabularies.
- A `contract_mismatch` projection denies privileged writes and routes to
  repair. It MUST still disclose what remains safe locally (cached
  read-only or local-safe baseline).

### 4.2 Diagnostics / doctor

- Doctor probes MUST emit `contract_mismatch_state_record` for managed
  or provider boundaries they inspect.
- A probe that cannot resolve observed version or supported range MUST
  emit `supported_range_unknown` (or a narrowed posture) rather than
  collapsing to a generic “unavailable”.

### 4.3 Support bundles

Support exports MUST include:

- the `contract_mismatch_state_record` instances relevant to the case;
- the referenced supported-range row snapshot (embedded in the record);
- any referenced compat/skew refs (when present) so release and support
  tooling can join without scraping UI copy.

### 4.4 Action-level denial or downgrade

- When `write_path_posture = blocked`, mutating actions that cross the
  affected boundary MUST be disabled with a typed reason and MUST route
  to an export/help action rather than retrying.
- When `write_path_posture = narrowed_idempotent_only`, only explicitly
  idempotent managed writes may continue (typically via a queued/outbox
  posture); surfaces MUST make the narrowing visible.

## Worked cases

The fixture directory contains worked examples:

- same-build match:
  [`/fixtures/runtime/contract_mismatch_cases/same_build_supported_exact.yaml`](../../fixtures/runtime/contract_mismatch_cases/same_build_supported_exact.yaml)
- current-plus-previous-minor supported window:
  [`/fixtures/runtime/contract_mismatch_cases/current_plus_previous_minor_within_window.yaml`](../../fixtures/runtime/contract_mismatch_cases/current_plus_previous_minor_within_window.yaml)
- unsupported old service:
  [`/fixtures/runtime/contract_mismatch_cases/unsupported_old_service_too_old.yaml`](../../fixtures/runtime/contract_mismatch_cases/unsupported_old_service_too_old.yaml)
- future service:
  [`/fixtures/runtime/contract_mismatch_cases/future_service_too_new.yaml`](../../fixtures/runtime/contract_mismatch_cases/future_service_too_new.yaml)
- offline cache / unknown observed version:
  [`/fixtures/runtime/contract_mismatch_cases/offline_cache_supported_range_unknown.yaml`](../../fixtures/runtime/contract_mismatch_cases/offline_cache_supported_range_unknown.yaml)
- direct-service route denied by policy:
  [`/fixtures/runtime/contract_mismatch_cases/direct_service_route_policy_blocked.yaml`](../../fixtures/runtime/contract_mismatch_cases/direct_service_route_policy_blocked.yaml)
