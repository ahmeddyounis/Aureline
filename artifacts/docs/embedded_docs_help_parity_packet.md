# Embedded docs / help stale-example, mirror/offline, and external-open parity packet

This packet is the cross-surface parity ledger that keeps embedded
docs / help surfaces truthful when the underlying content is cached,
mirrored, stale, locale-missing, not-installed, version-mismatched,
policy-narrowed, or offered only via an external open. It composes
the upstream contracts that already freeze each surface's individual
truth so this packet does not re-mint vocabulary; it only enumerates
which surfaces MUST agree on which axes for each named state, what
copy-rule bundle each state MUST render, what a parity failure looks
like, and where the audit row lands when a surface drifts.

The packet is intentionally narrow:

- It pins the cross-surface parity rules. The per-surface contracts
  (docs / help pane, About, onboarding, support export) own their
  own rules; this packet quotes them.
- It does not implement the docs browser, the search index, the
  pack publisher, or the support-export builder. Those land in
  later milestones.
- It supplies a reusable corpus for the docs-public-truth and
  embedded-boundary verification lanes.

## Sources of truth this packet quotes

Every parity row below cites at least one of:

- [`/docs/docs/docs_help_pane_contract.md`](../../docs/docs/docs_help_pane_contract.md)
  — embedded docs / help pane state-record contract.
- [`/schemas/docs/help_pane_state.schema.json`](../../schemas/docs/help_pane_state.schema.json)
  — boundary schema every embedded docs / help pane reads.
- [`/docs/docs/docs_pack_manifest_contract.md`](../../docs/docs/docs_pack_manifest_contract.md)
  — docs-pack source / version / signing / mirror lineage / locale
  / example-summary truth.
- [`/docs/docs/help_about_service_health_routes.md`](../../docs/docs/help_about_service_health_routes.md)
  — destination descriptor (trust, owner, boundary, route class,
  external-open policy, auth expectation, data-exit boundary).
- [`/docs/about/about_provenance_and_boundary_contract.md`](../../docs/about/about_provenance_and_boundary_contract.md)
  — About / reproducibility-packet provenance and boundary truth.
- [`/artifacts/docs/destination_descriptor_seed.yaml`](./destination_descriptor_seed.yaml)
  — destination-descriptor route / trust / boundary worked seeds.
- [`/artifacts/docs/help_badge_vocabulary.yaml`](./help_badge_vocabulary.yaml)
  — help / docs / About / service-health badge vocabulary.
- [`/artifacts/docs/stale_example_rules.yaml`](./stale_example_rules.yaml)
  — stale-example detection rubric and downgrade map.
- [`/artifacts/docs/stale_example_audit_rows.yaml`](./stale_example_audit_rows.yaml)
  — open / closed stale-example audit rows with remediation owner
  and release impact (companion to this packet).
- [`/fixtures/docs/help_pane_cases/`](../../fixtures/docs/help_pane_cases/)
  — single-surface help-pane state fixtures.
- [`/fixtures/docs/embedded_docs_help_cases/`](../../fixtures/docs/embedded_docs_help_cases/)
  — cross-surface parity fixtures keyed by this packet.

## Surfaces in scope

The parity packet pins one logical row per case across the four
surface lanes below. A surface that does not apply to a given case
sits at `not_applicable` rather than being silently dropped.

| Lane | Lane id | Read by |
|---|---|---|
| Embedded docs / help pane | `embedded_docs_help` | In-product docs pane, embedded docs-browser body, AI-explanation pane, release-notice pane. |
| Help / About pane | `help_about` | Help / About surfaces; the About card is the canonical owner for the About row's truth. |
| Onboarding guided surfaces | `onboarding_guided` | Onboarding help overlay, first-run tour, guided-step inline help. |
| Exported support / help links | `exported_support_help` | Support summary, support export packet, exported help shortcut catalog. |

## Parity axes that MUST match across lanes

For each case the four lane rows MUST agree on every axis below.
Disagreement on any axis is a parity failure and routes to a
`public_drift_item_record`.

- `source_class` (re-exported from
  `schemas/docs/help_status_badge.schema.json`).
- `version_match_state` (build-bound rows only).
- `freshness_class` (re-exported from
  `schemas/governance/capability_lifecycle.schema.json`).
- `cache_class` and `install_state` (re-exported from
  `schemas/docs/help_pane_state.schema.json` and
  `schemas/docs/onboarding_pack_state.schema.json`).
- `offline_posture` and `mirror_chain_status`.
- `locale_availability_state` and
  `locale_fallback_disclosure_class`.
- `external_open_path`, `browser_handoff_reason`,
  `data_exit_boundary`, and `disclosure_safety_class` (re-exported
  from `schemas/docs/destination_descriptor.schema.json`; the
  handoff reason MUST come from the screenshot-safe / export-safe
  subset frozen in ADR 0013).
- `policy_limited_behavior`.
- `stale_example_disclosure_class` and `stale_example_refs`.
- `copy_rule_class` (the disclosure copy bundle each surface MUST
  render).
- `renderable` and `denial_reasons` when at least one lane is
  suppressed.

## State classes covered

Each case row below resolves to exactly one named state. Adding a
state is additive-minor and requires a paired help-pane-state
schema bump.

| Case id | State | Primary cache / install / locale axis | Copy rule the lanes MUST render | Renderable across lanes |
|---|---|---|---|---|
| `docs.embedded_help.parity.cached_only_owner_unreachable` | Cached-only because the canonical owner is unreachable. | `cache_class = cached_snapshot_only_owner_unreachable`, `install_state = remote_unavailable`. | `cached_snapshot_only_owner_unreachable_copy`. | All four lanes renderable; in-product copy MUST disclose cached-only and offer the optional same-object browser fallback. |
| `docs.embedded_help.parity.mirror_only_offline` | Air-gapped signed-mirror copy with no live owner contact. | `cache_class = mirror_only_verified_offline`, `offline_posture = air_gapped_signed_bundle`. | `mirror_only_verified_offline_copy`. | All lanes renderable; external-open is forbidden by deployment policy. |
| `docs.embedded_help.parity.stale_example_disclosed_inline` | Partially-stale curated pack rendering a stale example with the typed disclosure. | `cache_class = warm_cached_within_window`, `stale_example_disclosure_class = stale_examples_disclosed_inline`. | `stale_example_disclosed_copy`. | Pane and onboarding renderable with inline disclosure; About lane reflects the curated source class; support export carries the stale-example caveat. |
| `docs.embedded_help.parity.locale_unavailable_not_installed` | Requested locale missing and not installed; pane suppressed. | `locale_availability_state = requested_locale_not_installed`. | `unavailable_locale_copy`. | Pane and onboarding suppressed with `locale_missing_not_installed`; About and exported support lanes degrade to primary locale with the typed disclosure. |
| `docs.embedded_help.parity.not_installed_pack` | Referenced pack with no copy resident. | `cache_class = not_cached_not_installed`, `install_state = not_installed`. | `not_installed_pack_copy`. | Pane / onboarding suppressed with `pack_not_installed`; About surface lists the pack as not-installed; support-export omits the pack body. |
| `docs.embedded_help.parity.external_open_allowed_optional_same_object` | In-product render of the same logical object plus optional system-browser route to the canonical owner. | `external_open_path = optional_same_object`, `browser_handoff_reason = external_docs_or_runbook`. | `support_export_known_limits_copy` for About; pane retains its pre-existing copy class. | All lanes renderable; lanes MUST quote the same destination descriptor and screenshot-safe / export-safe handoff reason. |
| `docs.embedded_help.parity.external_open_blocked_by_policy` | Admin policy forbids browser handoff. | `policy_limited_behavior = external_open_blocked_by_policy`, `external_open_path = not_permitted`. | `policy_limited_copy`. | All lanes renderable; the policy narrowing MUST appear on the primary surface of each lane (tooltip-only narrowings are non-conforming). |
| `docs.embedded_help.parity.policy_limited_pane_suppressed` | Pane suppressed entirely by admin policy. | `policy_limited_behavior = pane_suppressed_by_policy`, `denial_reasons = [policy_blocked]`. | `policy_limited_copy`. | Pane suppressed; onboarding overlay reflects suppression; About and support-export lanes carry the policy narrowing in the known-limits row. |

## Parity rules

The packet's normative parity rules:

1. **No surface may imply live, current, or authoritative truth
   when content is cached, mirrored, stale, missing, locale-blocked,
   or version-mismatched.** A lane that selects
   `copy_rule_class = live_authoritative_truth_copy` while another
   lane in the same case selects any other class fails parity and
   routes to `public_drift_item_record` with category
   `projection_broader_than_owner`.
2. **The `external_open_path` chosen by any lane MUST be the
   most-restrictive value the case allows.** A lane that promotes
   from `optional_same_object` to `required_primary_route` without
   the canonical owner declaring browser-primary is a parity
   failure.
3. **The `browser_handoff_reason` MUST come from the screenshot-safe
   / export-safe subset re-exported from
   `schemas/docs/destination_descriptor.schema.json`**. A lane that
   mints a parallel reason vocabulary fails parity and routes to
   `denial_reasons.browser_handoff_reason_outside_subset`.
4. **Stale-example disclosures MUST render on the primary surface
   of every lane that surfaces the example.** Tooltip-only
   disclosure is forbidden. A lane that drops the typed disclosure
   while another lane keeps it fails parity and routes to
   `denial_reasons.stale_examples_exceed_threshold` only when the
   ratio gate also tripped; otherwise the row is filed against
   `stale_example_audit_rows.yaml` directly.
5. **Locale fallbacks MUST be typed.** A lane that silently falls
   back to the primary locale without populating
   `locale_fallback_disclosure_class` is a parity failure and routes
   to `denial_reasons.locale_missing_not_installed` or
   `locale_policy_blocked` depending on the cause.
6. **Policy narrowings MUST surface on the primary surface of every
   lane.** A lane whose `policy_limited_behavior` is anything other
   than `not_policy_limited` MUST render the narrowing inline; a
   lane that hides the narrowing in a tooltip fails parity.
7. **Renderable booleans MUST agree where the upstream cause
   demands suppression.** When the docs-pack manifest is
   non-publishable, the offline expiration has passed, the locale
   is missing-and-not-installed, or the policy blocks the pane,
   every lane that consumes the pack MUST also flip
   `renderable = false` with the typed denial reason.

## Parity-failure routing

When a parity check fails, the audit emits a
`public_drift_item_record` against the affected pack / route /
example. The packet's lane vocabulary
(`embedded_docs_help`, `help_about`, `onboarding_guided`,
`exported_support_help`) maps to the drift item's closed
`audited_surface_class` taxonomy as follows:

| Lane id | `audited_surface_class` |
|---|---|
| `embedded_docs_help` | `help_pane` |
| `help_about` | `about_pane` (or `known_limits_section` when the row is the known-limits projection) |
| `onboarding_guided` | `help_pane` (onboarding overlays consume `help_pane_state_record`) |
| `exported_support_help` | `support_export_card` (or `known_limits_section` when the row is the known-limits export) |

The drift item then carries:

- `mismatch_category_class` — taken from the closed seven-class
  vocabulary in
  [`/schemas/public_truth/public_drift_item.schema.json`](../../schemas/public_truth/public_drift_item.schema.json):
  `projection_broader_than_owner` for any lane that widens past
  the canonical owner (live-authoritative copy claimed under a
  cached / mirrored / stale state, external-open promoted without
  the owner declaring browser-primary, browser-handoff reason
  outside the screenshot-safe / export-safe subset, silent locale
  fallback); `known_limit_missing` for any lane that drops a
  required known-limit / stale-example / policy-narrowing
  disclosure from the primary surface; `policy_disabled_hidden`
  for any lane that hides a policy-disabled pane without the
  typed narrowing.
- `severity_class` — paired with the category per the schema's
  `if/then` pairings: `projection_broader_than_owner` and
  `policy_disabled_hidden` pair with `release_blocking_overclaim`;
  `known_limit_missing` pairs with `same_change_blocker`.
- `narrowing_path_class` — typically `public_copy_narrowing` or
  `claim_row_narrowing` per the late-copy / claim-narrowing
  taxonomy frozen in the drift-item schema.

When the parity failure stems from a stale example specifically,
the audit also writes a row into
[`/artifacts/docs/stale_example_audit_rows.yaml`](./stale_example_audit_rows.yaml)
with the remediation owner and release impact filled in.

## Reuse by verification lanes

This packet is reusable by the following verification lanes
without being re-cut per surface:

- **Docs public truth lane.** Compares emitted lane rows
  field-for-field against the case's parity baseline; any
  mismatch routes to a drift item.
- **Embedded boundary verification lane.** Reads the lane's
  `disclosure_mode`, `data_exit_boundary`, and
  `disclosure_safety_class` to confirm the embedded surface
  preserves host-owned chrome and screenshot-safe / export-safe
  handoff reasons.
- **Onboarding parity lane.** Reads the onboarding overlay row
  alongside the docs / help pane row to confirm cached / not-
  installed / locale-missing states are first-class in tour copy.
- **Support export preview lane.** Reads the exported support /
  help link row to confirm support-export wording matches the
  pane's typed disclosure rather than restating it.

## Out-of-scope

- Docs browser runtime (search, index, history). ADR 0013
  reserves the docs-browser surface; this packet pins the parity
  rows every browser body reads, not the runtime.
- Docs-pack publishing pipeline (build, sign, distribute, fetch,
  refresh). The packet reads what the pipeline emits.
- The browser-handoff packet body and the embedded surface
  boundary card body. The packet points at those records and
  quotes their typed reason / boundary classes rather than
  re-embedding their contents.

## Versioning

This packet is additive-minor by default. Adding a case row,
parity axis, or routing class is additive-minor; repurposing an
existing value is breaking and requires a new decision row.
