# M5 Governance-Dashboard Component Consumer Contract

This contract is the adoption lane over the frozen M5 governance-dashboard
component matrix. It proves the nine governed component families — the **fitness
dashboard tile**, **governance report row**, **waiver-expiry queue item**,
**release-gate banner**, **mitigation note card**, **service-ownership card**,
**on-call strip**, **decision-right card**, and **milestone dashboard row** — are
reusable components, not one governance pipeline plus a few admin-only dashboards,
by binding every claimed M5 governance-dashboard consumer to the same canonical
component schemas and the same governance vocabulary.

- Rust module:
  `crates/aureline-release/src/add_shared_assurance_center_release_center_operator_dashboard_support_export_shiproom_about_help_consumers_so_governance_dashboard_components_keep_fitness_ownership_waiver_and_decision_language_aligned/`
- Boundary schema:
  `schemas/ui/m5-governance-dashboard-component-consumer.schema.json`
- Component matrix schema:
  `schemas/ui/m5-governance-dashboard-component-matrix.schema.json`
- Support-export proof:
  `artifacts/release/m5-governance-dashboard-component-consumer-proof/`
- Narrowed fixtures:
  `fixtures/ui/m5-governance-dashboard-component-consumers/`

## Consumers

Every claimed governance-dashboard consumer adopts the shared components:

| Consumer | Token | Docs/help |
| --- | --- | --- |
| Assurance Center | `assurance_center` | no |
| Release Center | `release_center` | no |
| Operator Dashboard | `operator_dashboard` | no |
| Shiproom Summary | `shiproom_summary` | no |
| Support Export | `support_export` | no |
| About / Help | `about_help` | yes |
| Docs Portal | `docs_portal` | yes |
| CLI Inspect | `cli_inspect` | no |

## Nine families → four canonical controls packets

The nine matrix families narrow into four canonical `implement_*` controls packets.
Every consumer that adopts a family points at that packet's canonical schema and
support-export artifact rather than re-wording the facts in local prose:

| Component family | Canonical controls schema |
| --- | --- |
| `fitness_dashboard_tile`, `governance_report_row` | `m5-fitness-governance-report-controls.schema.json` |
| `waiver_expiry_queue_item`, `release_gate_banner`, `mitigation_note_card` | `m5-waiver-gate-mitigation-controls.schema.json` |
| `service_ownership_card`, `on_call_strip` | `m5-service-ownership-on-call-controls.schema.json` |
| `decision_right_card`, `milestone_dashboard_row` | `m5-decision-right-milestone-controls.schema.json` |

Each of the nine families is adopted by at least two distinct consumers — the
acceptance-criterion proof that the families are reusable components.

## Shared governance vocabulary

Every binding keeps the same five descriptors explicit, so no consumer invents a
new badge or stale wording:

- `readiness` — the frozen readiness state (passing / warning / blocked / waived /
  expired_waiver / evidence_stale / owner_unresolved / forum_unresolved /
  not_evaluated).
- `evidence_freshness` — the evidence / proof freshness behind the reading.
- `waiver_state` — active / expiring / expired waiver truth.
- `owner_coverage` — owner, backup, and escalation route.
- `decision_forum` — which forum can approve the next move.

## Degrading identically when evidence or ownership state is stale

The resolver `resolve_governance_consumer_binding` derives a projection mode from
the governance evidence state a consumer renders under. Any state below full, fresh
truth stays at descriptor parity but discloses a **self-contained narrow banner**
that names the exact reason, the readiness floor the narrowing must never read past
as a clean pass, the descriptors that stay preserved, and the next action:

| Evidence state | Projection mode | Narrow reason | Readiness floor | Next action |
| --- | --- | --- | --- | --- |
| `full_truth_fresh` | `full_parity` | — | — | — |
| `evidence_stale` | `stale_narrowed` | `evidence_stale` | `evidence_stale` | `refresh_evidence` |
| `waiver_expiring_or_expired` | `waiver_narrowed` | `waiver_expiring` | `expired_waiver` | `renew_or_escalate_waiver` |
| `owner_coverage_missing` | `ownership_narrowed` | `owner_coverage_missing` | `owner_unresolved` | `assign_owner_and_backup` |
| `forum_unresolved` | `forum_narrowed` | `forum_unresolved` | `forum_unresolved` | `route_to_authorized_forum` |
| `not_evaluated_here` | `not_evaluated_narrowed` | `not_evaluated_here` | `not_evaluated` | `request_evaluation` |

Because the readiness floor is always an explicit non-passing state, a waived or
stale reading can never render as a clean pass, and an ownerless or forumless
blocker can never read as resolved.

## Guardrails

Each consumer row asserts five hard invariants (all `false`):

- `renders_waived_or_stale_as_clean_pass`
- `lets_ownerless_or_forumless_blocker_read_resolved`
- `hides_mitigation_behind_internal_jargon`
- `rewords_governance_vocabulary_per_surface`
- `invents_new_dashboard_local_status`

## Acceptance criteria mapping

- **User/admin/support/release consumers reuse one governance vocabulary and
  degrade identically when evidence or ownership state is stale** — the canonical
  vocabulary set is frozen once and every consumer reads it; the resolver produces
  one projection mode + narrow banner per evidence state regardless of consumer.
- **The same blocker, waiver, owner, and forum truth appears consistently across
  GUI, CLI, export, and help surfaces** — the GUI (assurance/release/operator/
  shiproom), the CLI (`cli_inspect`), the export (`support_export`), and the help
  surfaces (`about_help`, `docs_portal`) all appear as rows pointing at the same
  canonical controls schemas, and the support export reconstructs consumer parity
  from the shared model.
