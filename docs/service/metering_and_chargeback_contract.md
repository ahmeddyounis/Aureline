# Commercial control-plane metering, spend-forecast, and chargeback-honesty contract

This document freezes the visible **service-economics** vocabulary
Aureline surfaces use so optional managed features stay inspectable
and reversible — even when a hosted billing or metering surface is
stale, unavailable, mid-rotation, or under policy suppression.

The goal is that any user or admin who reaches an entitlement summary,
a usage-and-forecast view, a chargeback-scope switcher, or a
grace-period or offboarding card can answer five questions from the
surface itself:

- **What is being measured, and in which unit?** Closed meter family,
  closed unit, closed aggregation window — never an unlabelled number.
- **As of when, by whose authority?** RFC 3339 UTC `as_of_time`, plus
  the closed measurement-authority class re-exported from the
  metering and usage-export contract (authoritative, cached,
  estimated, unavailable, policy-suppressed).
- **Who owns the quota and the chargeback?** Closed `quota_owner`
  re-exported from the operating-mode and capacity contract, plus the
  closed `chargeback_scope` (personal, workspace, organization,
  tenant, BYOK external, mixed-requires-split, not-applicable) — and
  no surface MAY collapse personal, workspace, and org usage into a
  single misleading total.
- **What does forecast confidence look like?** Closed
  `forecast_confidence_class` so a "projected to exhaust in N days"
  copy never appears under estimated, unavailable, or
  policy-suppressed authority.
- **What still works locally if metering itself fails?** The retained
  local-safe action list, separated from the managed-only paths that
  pause; plus the closed prompt-ordering rules that preserve export,
  support, delete, and local-continuation actions ahead of upgrade or
  billing pressure prompts.

This contract is **vocabulary-only**: it does not implement billing
backends, invoicing systems, rating engines, taxation, payment
processors, or commercial operations.

## Companion artifacts

- [`/schemas/service/meter_row.schema.json`](../../schemas/service/meter_row.schema.json)
  — boundary schema for the `meter_row_record`. Carries one meter
  family, its closed unit and aggregation window, the owner-scope
  binding, the entitlement summary, the usage and forecast view, the
  chargeback scope switcher, the grace-period or offboarding card,
  the export-parity posture, the fail-open / fail-closed posture,
  the closed prompt-ordering rules, and the display-copy invariants
  that forbid surprise-billing language.
- [`/fixtures/service/metering_cases/`](../../fixtures/service/metering_cases/)
  — worked fixtures exercising the contract.

## Inherited contracts

This contract stands on top of earlier seeds and contracts and MUST
NOT recast any of them:

- [`/docs/service/operating_mode_and_capacity_contract.md`](./operating_mode_and_capacity_contract.md)
  and [`/schemas/service/operating_mode.schema.json`](../../schemas/service/operating_mode.schema.json)
  — re-exports the closed `operating_mode_class`, `quota_owner_class`,
  `service_family_class`, `affected_action_family_class`, and
  `fail_posture_class` vocabularies. A `meter_row_record` MUST resolve
  through a `service_family_class` admitted by the linked
  operating-mode card.
- [`/docs/service/managed_service_seed.md`](./managed_service_seed.md)
  and [`/artifacts/service/slo_rows.yaml`](../../artifacts/service/slo_rows.yaml)
  — re-exports the closed `service_id` vocabulary and the
  `degradation_mode_class`. A `meter_row_record` MUST cite at least
  one `service_id` whose row exists in the SLO row file. The
  `managed_entitlement_usage` and `managed_offboarding_export` SLO
  rows are the primary anchors for entitlement and grace surfaces.
- [`/docs/service/api_inventory_seed.md`](./api_inventory_seed.md)
  and [`/schemas/service/api_capability_row.schema.json`](../../schemas/service/api_capability_row.schema.json)
  — the row's `fail_posture` MUST be admissible under the linked API
  surface row's `local_fallback_posture`.
- [`/docs/managed/metering_and_usage_export_contract.md`](../managed/metering_and_usage_export_contract.md)
  and the companion `quota_state.schema.json` and
  `usage_export_row.schema.json` — re-exports
  `authority_class`, `meter_reading_class`, `quota_status_class`,
  `quota_family_class`, `quota_unit_class`, `precision_class`,
  `reset_window_class`, `time_basis_class`, `stale_label_class`,
  and the spend-attribution dimension vocabulary. A
  `meter_row_record` is **not** a replacement for a
  `quota_state_record`; it is the family-level row a surface reads
  before drilling into one or more linked `quota_state_record` and
  `usage_export_row_record` entries.
- [`/docs/auth/managed_auth_and_session_continuity_contract.md`](../auth/managed_auth_and_session_continuity_contract.md)
  — managed identity remains additive capability; entitlement
  summaries compose against managed-session state and never block
  local-core work waiting for a fresh session.
- [`/docs/policy/admin_policy_and_bundle_cache_contract.md`](../policy/admin_policy_and_bundle_cache_contract.md)
  — admin policy MAY suppress amounts, scope dimensions, or chargeback
  detail. Suppression rides the policy-suppressed authority class and
  the policy_ref on the meter row; this contract does not mint a
  parallel suppression vocabulary.
- [`/docs/release/assurance_claim_matrix.md`](../release/assurance_claim_matrix.md)
  — chargeback-honesty claims are assurance claims; the contract
  re-uses the assurance vocabulary rather than minting parallel
  "honesty" terms.

Normative sources:

- `.t2/docs/Aureline_PRD.md` §5.23, §5.24, §5.33, §5.34, §13.15, and
  Appendix AN.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  §18.10, and §21.9.
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.4.2 and §11.4.3.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §18.31, §18.42, and
  Appendix BL.

If this document disagrees with those sources, those sources win and
this document plus its companion schema update in the same change.
If this document and the companion schema disagree, this document
wins and the schema updates in the same change.

## Why this exists

Without a frozen service-economics vocabulary, an optional managed
feature reaches for one of five failure modes when its commercial
posture narrows:

1. **Stale-quota surprise.** A user opens an entitlement summary,
   sees a number, and acts on it — without learning that the number
   is cached beyond its freshness floor or that the upstream meter
   has been unavailable for hours.
2. **Ambiguous chargeback ownership.** A "monthly usage" total
   collapses personal, workspace, and organization usage into a
   single number, so neither the user nor the admin can tell which
   cost centre will see the bill.
3. **Support-hostile lockout.** A "quota exhausted" or "entitlement
   expired" copy displays an upgrade button before the export
   action, the support-export action, the delete-data action, or
   the local-continuation cue, leaving customers under-equipped to
   recover their own data.
4. **Local-core blocking on hosted-meter outage.** A managed billing
   ingest or entitlement service is unreachable and the desktop core
   refuses to open files, save, search, or run local Git "until the
   meter is current".
5. **Forecast theatre.** A "projected to exhaust in N days" copy
   renders under estimated, unavailable, or policy-suppressed
   authority — the surface confidently extrapolates from data the
   service explicitly declined to vouch for.

This contract closes those gaps by freezing one record family —
`meter_row_record` — that any service-economics surface MUST consult
before it claims a number, a forecast, an entitlement, a chargeback
owner, a grace deadline, or an offboarding hold.

## Scope

Frozen at this revision:

- one `meter_row_record` carrying record id, the closed
  `meter_family_class` (`profile_or_settings_sync_meter_family`,
  `collaboration_relay_meter_family`,
  `remote_workspace_control_plane_meter_family`,
  `ai_gateway_meter_family`,
  `registry_or_mirror_meter_family`,
  `support_ingest_meter_family`), the linked service family class,
  the linked service ids, the closed `meter_unit_class` and
  `aggregation_window_class` (re-exported from the metering
  contract), the `as_of_time`, the closed
  `measurement_authority_class`, the closed `meter_freshness_class`,
  the entitlement summary block, the usage and forecast view, the
  chargeback scope switcher, the optional grace-period or
  offboarding card, the export-parity posture, the fail-posture
  block, the prompt-ordering block, the retained-local-safe and
  blocked-managed-only capability lists, and the display-copy
  invariants;
- the closed `chargeback_scope_class` vocabulary
  (`personal_scope`, `workspace_scope`, `organization_scope`,
  `tenant_scope`, `byok_external_scope`, `mixed_requires_split`,
  `not_applicable`);
- the closed `forecast_confidence_class` vocabulary
  (`forecast_authoritative`, `forecast_best_effort_local`,
  `forecast_unavailable`, `forecast_policy_suppressed`,
  `forecast_not_applicable`);
- the closed `entitlement_state_class` vocabulary
  (`entitlement_active`, `entitlement_in_grace`,
  `entitlement_expired`, `entitlement_suspended_admin`,
  `entitlement_pending_recheck`, `entitlement_not_applicable`);
- the closed `offboarding_phase_class` vocabulary
  (`not_offboarding`, `notice_period`, `grace_window`,
  `access_end_window`, `post_access_export_window`,
  `offboarding_complete`);
- the closed `export_parity_class` vocabulary
  (`parity_with_csv_and_json`, `parity_with_json_only`,
  `parity_with_csv_only`, `manifest_only_no_row_export`,
  `no_export_documented_local_only`,
  `parity_pending_recheck`,
  `parity_policy_suppressed`);
- the closed `meter_freshness_class` vocabulary
  (`fresh`, `cached_within_freshness_floor`,
  `cached_beyond_freshness_floor`, `estimate_pending_reconciliation`,
  `unavailable_meter`, `policy_suppressed_meter`);
- the closed `commercial_prompt_class` vocabulary, ordered by
  user-preserving priority (`export_usage_summary`,
  `export_support_bundle`, `export_offboarding_packet`,
  `delete_data`, `continue_local_work`, `manage_entitlement`,
  `request_admin_review`, `increase_budget`, `upgrade_plan`,
  `add_payment_method`);
- the rule that prompts MUST render in the priority order above —
  a surface MAY omit prompts that do not apply, but MUST NOT show
  `upgrade_plan`, `increase_budget`, or `add_payment_method` ahead
  of any of `export_usage_summary`, `export_support_bundle`,
  `export_offboarding_packet`, `delete_data`, or
  `continue_local_work` when the latter applies;
- fail-open / fail-closed rules for stale or unreachable metering
  services, expressed against the inherited `fail_posture_class`
  vocabulary;
- the display-copy invariants that forbid surprise-billing language,
  collapsed-scope totals, forecast-under-unauthoritative-state, and
  upgrade-before-export prompt ordering.

Out of scope at this revision (named explicitly so reviewers know
what is *not* being decided here):

- billing backends, pricing models, invoicing systems, rating
  ledgers, payment processors, taxation, or commercial operations;
- raw vendor invoices, raw payment processor records, raw billing
  account ids, raw invoice numbers, raw provider URLs, raw user
  emails, raw tenant names, or raw price-list values;
- final user-facing copy, status-strip integration, notification
  routing, animation, iconography, or upsell modal layouts;
- replacement of the operating-mode card in
  `docs/service/operating_mode_and_capacity_contract.md`, the
  per-quota explainability object in
  `schemas/managed/quota_state.schema.json`, or the export row in
  `schemas/managed/usage_export_row.schema.json`. Surfaces compose
  against those records; this contract freezes the family-level
  service-economics row that points into them.

## The five questions every meter row answers

Any Aureline surface that renders an entitlement summary, a usage
and forecast view, a chargeback scope switcher, a grace-period card,
or an offboarding card MUST answer these questions mechanically
against the records here:

1. **Which meter family, in which unit, over which window, by whose
   measurement authority?** `meter_family_class`,
   `linked_service_family_class`, `linked_service_ids`,
   `meter_unit_class`, `aggregation_window_class`,
   `measurement_authority_class`, `meter_freshness_class`, and
   `as_of_time` answer this. Surfaces that cannot resolve the
   measurement authority MUST render
   `unavailable_meter` or `policy_suppressed_meter` rather than
   guess.
2. **Who owns the quota? Who owns the chargeback?**
   `quota_owner_class` and `chargeback_scope_class` answer this. A
   surface MAY render multiple chargeback scopes side-by-side via
   the `chargeback_scope_switcher` block, but MUST NOT collapse
   personal, workspace, and organization scopes into one total.
3. **What is the entitlement state?** `entitlement_summary` answers
   this. A non-`entitlement_active` row MUST surface
   `recovery_cue` and the linked grace-or-offboarding card before
   any upgrade prompt.
4. **What is the forecast and at what confidence?**
   `usage_and_forecast` answers this. A `forecast_value` MAY only
   render under `forecast_authoritative` or
   `forecast_best_effort_local` confidence; under any other
   confidence class the forecast block MUST render the typed
   confidence class without a numeric projection.
5. **What still works if the meter or its service is stale or
   unreachable?** `local_continuation` and `fail_posture` answer
   this. Local edit, save, undo, search, Git, BYOK AI, local
   tasks, local export, and any documented local-fallback action
   MUST remain available unless the API surface row's
   `local_fallback_posture` explicitly forbids it.

Generic copy such as "billing service is unreachable, please try
again later", "quota exceeded — upgrade now", "we couldn't load
your usage", or "contact support" is **not admissible** on these
paths when a typed state is available from the records.

## Required surfaces

Every meter row MUST be reachable from the four service-economics
surface families. A surface that wants to render any of these MUST
read a `meter_row_record` first; the record is the cross-tool
boundary every status-strip, palette, support packet, admin
console, and CLI/headless reader consumes.

### Entitlement summary

The entitlement summary surfaces the closed
`entitlement_state_class` plus its scope, the as-of time, the
linked managed-session state, and the next-step recovery cue. It
MUST cite both:

- the linked `quota_state_record` ref(s), so the surface can drill
  into the per-quota explainability object frozen in the metering
  and usage-export contract; and
- the linked `service_id` (typically `managed_entitlement_usage`),
  so the surface can read the SLO row's degradation mode and
  freshness floor.

A `entitlement_in_grace` row MUST cite the grace-or-offboarding
card on the same record. An `entitlement_expired` or
`entitlement_suspended_admin` row MUST set the `fail_posture` to
`fail_closed_managed_only` for the affected action families and
MUST NOT silently widen export, telemetry, or AI-evidence scope.

### Usage and forecast view

The usage and forecast view carries:

- `usage_value` — nullable amount, with the precision class
  re-exported from the metering contract. Values are nullable
  because unavailable, policy-suppressed, and pending-recheck are
  explicit states, not numbers.
- `usage_unit_class` — re-export of the metering-contract
  `quota_unit_class`. The unit is explicit even when the amount
  is suppressed.
- `aggregation_window_class` — re-export of the metering-contract
  `reset_window_class`. The window is explicit even when start
  and end timestamps are unknown.
- `as_of_time` — RFC 3339 UTC. Required.
- `forecast` — typed block carrying `forecast_confidence_class`,
  optional `forecast_value`, optional `forecast_horizon_summary`,
  and a `forecast_caveats` array. A `forecast_value` MAY only
  render under `forecast_authoritative` or
  `forecast_best_effort_local`. Any other confidence class MUST
  set `forecast_value` to `null`.

A view that cannot resolve `as_of_time`, `usage_unit_class`,
`aggregation_window_class`, or the chargeback-scope owner MUST
render `unavailable_meter` and a non-null `recovery_cue` rather
than rendering with missing context.

### Chargeback scope switcher

The chargeback scope switcher exposes the closed
`chargeback_scope_class` vocabulary plus the linked
`spend_attribution_dimensions` array re-exported from the
metering contract. It MUST:

- list every chargeback scope that the user has visibility into,
  per admin policy and entitlement;
- pin a non-null `current_scope`;
- forbid collapsing personal, workspace, and organization usage
  into a single total — `personal_workspace_org_collapsed`
  remains `false`;
- mark `mixed_requires_split` when more than one scope contributes
  to the displayed total and a per-scope breakdown is not yet
  resolvable; a `mixed_requires_split` row MUST cite at least one
  `recovery_cue` naming how the user obtains the per-scope split.

A scope absent from the switcher is not implicitly zero. Absence
is a typed state — the row MUST either include the scope under
`policy_suppressed_meter` authority with the suppression reason,
or omit it only when it does not apply (and `not_applicable`
appears as the chargeback class).

### Grace-period or offboarding card

The grace-period or offboarding card surfaces the closed
`offboarding_phase_class` plus the entitlement-grace-until
timestamp (when applicable). It MUST:

- list the action families that pause and the action families
  that remain available — symmetry with the operating-mode card's
  service-continuity note;
- preserve the export-usage-summary, export-support-bundle,
  export-offboarding-packet, delete-data, and
  continue-local-work prompts ahead of any
  manage-entitlement, increase-budget, upgrade-plan, or
  add-payment-method prompt — see *Prompt ordering rules* below;
- cite the linked `managed_offboarding_export` SLO row when the
  phase is `access_end_window` or `post_access_export_window`,
  so the customer-visible offboarding export remains scriptable
  and partiality-aware.

`offboarding_complete` does **not** invalidate retained local
files, BYOK AI, local Git history, or already-exported usage
summaries. The card MUST display the local-continuation cue even
under `offboarding_complete`.

## Meter family vocabulary

A `meter_row_record` carries exactly one `meter_family_class`. The
families align with the inherited `service_family_class` vocabulary
but may be narrower so a single service family can split into
multiple meter rows when its unit, window, owner scope, or export
parity differ.

| Meter family | Meaning | Linked service-family class | Typical service ids | Typical unit | Typical aggregation window |
| --- | --- | --- | --- | --- | --- |
| `profile_or_settings_sync_meter_family` | Profile, settings, project, and workspace state synchronisation usage. | `sync_family` | `managed_settings_sync` | `bytes_transferred` | `rolling_30d` |
| `collaboration_relay_meter_family` | Hosted review session, collaboration archive, and presence relay usage. | `collaboration_relay_family` | `managed_collaboration_review`, `managed_relay` | `participant_minutes` | `calendar_month_utc` |
| `remote_workspace_control_plane_meter_family` | Remote attach, remote execute, and remote workspace control-plane session usage. | `remote_workspace_control_plane_family` | `managed_relay` | `workspace_hours` | `calendar_month_utc` |
| `ai_gateway_meter_family` | Managed AI broker / inference gateway usage. | `ai_gateway_family` | `managed_ai_broker` | `tokens` | `rolling_24h` |
| `registry_or_mirror_meter_family` | Marketplace, catalog, and docs-pack registry / mirror metadata usage. | `registry_or_mirror_metadata_family` | `managed_marketplace`, `managed_catalog`, `managed_docs_pack` | `download_count` | `rolling_30d` |
| `support_ingest_meter_family` | Telemetry sink, support export, entitlement-usage, and offboarding-export ingest usage. | `telemetry_or_support_ingest_family` | `managed_telemetry_sink`, `managed_support_export`, `managed_entitlement_usage`, `managed_offboarding_export` | `support_bundle_count` or `bytes_transferred` | `calendar_month_utc` |

Each meter row carries:

- `meter_family_class` — one of the above.
- `linked_service_family_class` — re-export from the operating-mode
  contract.
- `linked_service_ids` — non-empty subset of the SLO row vocabulary.
- `meter_unit_class` — re-export of the metering-contract
  `quota_unit_class`.
- `aggregation_window_class` — re-export of the metering-contract
  `reset_window_class`.
- `owner_scope` — closed `scope_class` re-export plus an opaque
  `scope_ref`. The owner scope is the "who is metered" scope; it
  MAY differ from the chargeback scope.
- `chargeback_scope` — block carrying `chargeback_scope_class`,
  the offered scopes for the switcher, and the
  `spend_attribution_dimensions` the row exposes. Personal,
  workspace, and organization scopes do not collapse.
- `as_of_time` — RFC 3339 UTC. Required (non-null).
- `measurement_authority_class` — re-export of the metering-contract
  `authority_class`.
- `meter_freshness_class` — re-export of the metering-contract
  `stale_label_class` minus the `no_stale_label` value (which maps
  here to `fresh`).
- `quota_owner_class` — re-export of the operating-mode-contract
  `quota_owner_class`.
- `linked_quota_state_refs` — opaque refs to the per-quota
  explainability objects (`quota_state_record`).
- `linked_usage_export_row_refs` — opaque refs to bounded-export
  rows (`usage_export_row_record`).
- `entitlement_summary` — typed block (see *Required surfaces*).
- `usage_and_forecast` — typed block.
- `chargeback_scope_switcher` — typed block.
- `grace_or_offboarding_card` — optional typed block.
- `export_parity` — block carrying `export_parity_class`, the
  documented CSV/JSON manifest ref, and the partiality posture.
- `fail_posture` — re-export of the operating-mode-contract
  `fail_posture_class`. Capacity or meter-state failures fail
  closed only for the managed action family that cannot be bounded
  safely; the row fails open for documented local-safe workflows
  where the linked API surface row admits it.
- `prompt_ordering` — typed block carrying the ordered prompt list
  and the invariants that block billing-pressure prompts from
  preceding user-preserving prompts.
- `local_continuation` — non-empty list of short reviewable
  sentences naming what continues to work without the managed
  meter; mirrors the operating-mode card's
  `retained_local_safe_capabilities`.
- `blocked_managed_only_actions` — list of short reviewable
  sentences naming the managed-only paths paused by the active
  meter posture.
- `display_copy` — invariants block.

## Fail-open / fail-closed rules for stale or unreachable metering

The fail posture for any action under a stale or unreachable meter
resolves through the inherited `fail_posture_class` vocabulary and
MUST be consistent with the linked API surface row's
`local_fallback_posture`:

| Meter freshness | Local-core action | Managed action requiring bounded spend | Managed action with documented local fallback |
| --- | --- | --- | --- |
| `fresh` | `fail_open_local_safe` (no narrowing) | `not_applicable` | `not_applicable` |
| `cached_within_freshness_floor` | `fail_open_local_safe` | `fail_open_local_safe_with_label` (cached chip) | `fail_open_local_safe_with_label` |
| `cached_beyond_freshness_floor` | `fail_open_local_safe` | `fail_closed_managed_only` | `fail_open_local_safe_with_label` (route to the local fallback; never silent) |
| `estimate_pending_reconciliation` | `fail_open_local_safe` | `fail_closed_managed_only` | `fail_open_local_safe_with_label` |
| `unavailable_meter` | `fail_open_local_safe` | `fail_closed_managed_only` | `fail_open_local_safe_with_label` |
| `policy_suppressed_meter` | `fail_open_local_safe` | `fail_closed_managed_only` (with policy ref) | `fail_open_local_safe_with_label` (with policy ref) |

`unknown_pending_recheck` (re-exported from the operating-mode
contract) routes through `boundary_recheck_required` for managed
writes and managed routes, never through silent fail-open.

A meter row MUST NOT block any local-core workflow solely because
a hosted billing or metering surface is unavailable. A row that
sets `local_core_blocking` to `true` is invalid by construction;
the schema enforces `local_core_blocking: false` as a `const`.

## Prompt ordering rules

Every meter row carries a typed `prompt_ordering` block listing
the prompts the surface MAY render. The block enforces this
priority, top-to-bottom:

1. `continue_local_work` — the surface MUST always offer the
   local-continuation cue when at least one local-safe action
   remains available.
2. `export_usage_summary` — the user MUST be able to export the
   currently displayed usage and forecast in CSV and JSON
   wherever `export_parity_class` admits the parity. The export
   action MUST NOT require an upgrade or a payment method.
3. `export_support_bundle` — when the linked
   `managed_support_export` SLO row admits it, the
   support-export action MUST be reachable from the meter row's
   prompt list.
4. `export_offboarding_packet` — required on every row whose
   `offboarding_phase_class` is in
   {`grace_window`, `access_end_window`,
   `post_access_export_window`}.
5. `delete_data` — required on every row whose
   `entitlement_state_class` is in
   {`entitlement_expired`, `entitlement_suspended_admin`,
   `entitlement_in_grace`}; the surface MUST link to the
   deletion-job and offboarding contract for the receipt.
6. `manage_entitlement` — admin-facing. MAY render after items
   1–5.
7. `request_admin_review` — user-facing escalation. MAY render
   after items 1–5.
8. `increase_budget` — MUST NOT render before any of items 1–5.
9. `upgrade_plan` — MUST NOT render before any of items 1–5.
10. `add_payment_method` — MUST NOT render before any of
    items 1–5.

A row that flips `upgrade_prompted_before_export_or_support` or
`local_continuation_path_obscured` is invalid by construction.

The prompt-ordering block does **not** mandate every prompt
renders; it mandates *order* and *user-preserving precedence*.
A surface MAY render a subset, but the relative order between the
rendered subset MUST honour items 1–10.

## Export-parity posture

`export_parity_class` pins the row's promise about the bounded
export of the displayed usage summary:

- `parity_with_csv_and_json` — both the CSV and JSON exports
  preserve the row's fields, including caveats and source class.
  This is the default for the `support_ingest_meter_family` and
  the `ai_gateway_meter_family` when entitlement is healthy.
- `parity_with_json_only` — only the canonical JSON export
  preserves the row. CSV is deferred until parity is added.
- `parity_with_csv_only` — only the CSV export is admitted (for
  example when policy suppresses the JSON manifest).
- `manifest_only_no_row_export` — only an export-job manifest is
  available; row-level export is held under partiality.
- `no_export_documented_local_only` — the row is local-only and
  no managed export is admitted.
- `parity_pending_recheck` — parity is held until a
  boundary-recheck packet completes.
- `parity_policy_suppressed` — admin policy suppresses one or
  more export forms; the row carries the policy ref.

Every CSV export MUST ship with a manifest mapping CSV columns to
the row fields, as already required by the metering and
usage-export contract. This contract narrows that posture for
the family-level row.

## Display-copy invariants

Every row's `display_copy` block MUST keep eight invariants
false:

- `whole_product_failure_implied` — a stale or unreachable meter
  cannot imply that the whole product is broken.
- `as_of_time_omitted` — usage, forecast, and entitlement
  surfaces cannot render without an as-of timestamp.
- `measurement_unit_omitted` — the unit is explicit even when
  the amount is suppressed or unavailable.
- `owner_scope_omitted` — the owner scope is explicit on every
  row.
- `personal_workspace_org_collapsed` — personal, workspace, and
  organization usage do not collapse into one number.
- `forecast_under_unauthoritative_state` — a numeric forecast
  cannot render under estimated, unavailable, or
  policy-suppressed authority.
- `upgrade_prompted_before_export_or_support` — upgrade, budget,
  and payment prompts do not precede export, support, delete,
  or local-continuation prompts when the latter apply.
- `local_continuation_path_obscured` — a stale or unreachable
  meter cannot hide the local-continuation cue.

The schema enforces these invariants as `const: false` so a
fixture or record that flips them is invalid by construction.

## Local-core non-dependence

Every meter row preserves the local-core non-dependence clause
from the managed-service seed, the locality seed, the metering
and usage-export contract, and the operating-mode and capacity
contract. A surface that renders a meter row MUST NOT block the
desktop core's first-run, startup, edit, save, search, local
Git, BYOK AI, or local export paths waiting for a meter to
materialise. Surfaces that lack a meter row resolve to the
`local_only` operating-mode posture by default until a managed
control plane is reached.

## Worked fixtures

See [`/fixtures/service/metering_cases/`](../../fixtures/service/metering_cases/)
for fixtures exercising the contract:

- `personal_vs_org_chargeback_view.yaml` — an `enterprise_saas`
  row whose chargeback scope switcher exposes both
  `personal_scope` and `organization_scope` side-by-side without
  collapsing.
- `stale_quota_data.yaml` — an `enterprise_saas` row whose AI
  gateway meter is `cached_beyond_freshness_floor`; managed
  inference fails closed for the bounded-spend action while
  local edit, BYOK AI, and local export remain available.
- `grace_period_offboarding.yaml` — an `enterprise_saas` row in
  `grace_window` offboarding, exposing
  `export_offboarding_packet`, `delete_data`, and
  `continue_local_work` prompts ahead of any
  `manage_entitlement` or upgrade prompt.
- `ai_gateway_quota_exhaustion.yaml` — an `enterprise_saas` row
  whose AI gateway meter is `unavailable_meter` after the daily
  inference budget reached `quota_exhausted`. Prompt ordering
  preserves export, support, delete, and local-continuation
  ahead of the `increase_budget` and `upgrade_plan` prompts.
- `usage_summary_csv_json_export.yaml` — a `support_ingest`
  meter row whose `export_parity_class` is
  `parity_with_csv_and_json` and whose linked usage-export rows
  carry the canonical fields.

## Evolution rules

- Adding a new `meter_family_class`, `chargeback_scope_class`,
  `forecast_confidence_class`, `entitlement_state_class`,
  `offboarding_phase_class`, `export_parity_class`,
  `meter_freshness_class`, or `commercial_prompt_class` value is
  additive-minor and bumps the `schema_version` on the affected
  schema. Repurposing an existing value is breaking and requires
  a new decision row in
  `artifacts/governance/decision_index.yaml`.
- Promoting a service family from "no metered quota" to a
  metered meter row requires a paired update to the linked SLO
  row's `degradation_modes`, the linked API surface row's
  `local_fallback_posture`, and at least one fixture under
  `fixtures/service/metering_cases/` so the fail posture remains
  consistent across the four surfaces.
- New chargeback or prompt vocabulary lands a paired decision row
  before the schema and fixture changes ship; vocabulary that is
  admissible only under a specific operating mode MUST be
  enforced by a schema-level `if/then` gate so the value cannot
  escape the mode it was scoped for.
- This document, the JSON Schema, and the worked fixtures stay
  in sync by review. Tooling MAY reject PRs that introduce a
  vocabulary value, family, or prompt class in only one of the
  surfaces.
