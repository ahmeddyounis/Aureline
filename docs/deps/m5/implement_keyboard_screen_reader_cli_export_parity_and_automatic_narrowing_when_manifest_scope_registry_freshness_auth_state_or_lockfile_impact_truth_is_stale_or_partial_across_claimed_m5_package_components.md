# Package-management component accessibility, headless, and export parity

Status: Implemented (M05-978, batch B115)

This contract is the accessibility / headless / export capstone over the reusable
M5 package-management components frozen in
[`m5-package-management-component-matrix`](freeze_the_m5_package_management_component_matrix.md)
(M05-972) and implemented by the package-explorer-row (M05-973), manifest-scope /
registry-or-mirror (M05-974), install-review / lockfile-impact (M05-975), and
script-risk / grouped-update / rollback-checkpoint (M05-976) lanes, and adopted by
the shared consumers in
[`add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers`](add_shared_package_explorer_search_detail_help_support_diagnostics_and_export_consumers.md)
(M05-977). Where the consumer lane proves scope / auth / lockfile parity across
desktop surfaces, this lane proves the harder claim — by fixtures, not screenshots —
that manifest scope, registry source / auth posture, script / native-build
side-effect class, lockfile churn, and rollback / checkpoint truth are exposed just
as honestly in assistive, headless, and exported forms, and that a claim-bearing
component automatically narrows the moment its underlying truth stops being
trustworthy.

- Boundary schema: [`schemas/ui/m5-package-management-component-accessibility-parity.schema.json`](../../../schemas/ui/m5-package-management-component-accessibility-parity.schema.json)
- Producer: `aureline_deps::…::current_package_component_accessibility_export`
- Release proof: [`artifacts/release/m5-package-management-accessibility-proof/`](../../../artifacts/release/m5-package-management-accessibility-proof/)
- Protected fixtures: [`fixtures/ui/m5-package-management-component-accessibility-parity/`](../../../fixtures/ui/m5-package-management-component-accessibility-parity/)

## Parity across forms (AC1)

Every claimed component carries five accessibility fields — a keyboard label, a
screen-reader label, a CLI enum token, an export enum token, and a human-readable
explanation — and must render on all three rendering surfaces (`desktop_full`,
`cli_headless`, `support_export`). Three guardrail row-invariants are pinned to
`false`; any true value is a violation:

- `is_pointer_only` — the component is reachable only by pointer.
- `is_export_opaque` — the component omits itself from the export.
- `desktop_stronger_than_cli` — the component claims more on the desktop than in
  CLI or support output.

## Automatic narrowing (AC2)

Each component carries a claim about how much reviewable package-management
capability it asserts, drawn from `PackageComponentClaimTier`. The claim condition
pins that claim to a ceiling via `resolve_package_component_claim_narrowing`:

| Condition | Permitted ceiling | Trigger | Required notes |
| --- | --- | --- | --- |
| `package_truth_trusted` | `full_reviewable_management` | — (no narrowing) | none |
| `manifest_scope_partial` | `manifest_range_scoped` | `manifest_scope_partial` | scope disclosure |
| `registry_freshness_stale` | `mirror_or_offline_sourced` | `registry_freshness_stale` | scope disclosure + continuity |
| `auth_state_unsatisfied` | `auth_required_read_only` | `auth_state_unsatisfied` | scope disclosure + auth |
| `lockfile_impact_unavailable` | `lockfile_impact_unknown` | `lockfile_impact_unavailable` | scope disclosure |
| `rollback_checkpoint_unavailable` | `rollback_unavailable_manual_recovery` | `rollback_checkpoint_unavailable` | scope disclosure + rollback |

A row whose `effective_claim` outranks its ceiling trips `claim_ceiling_exceeded` —
the core AC2 device: a component may never keep asserting full reviewable
management while a weakening condition holds. Each weakening condition additionally
requires an explicit narrow disclosure (`narrowing`) naming its trigger, the tier it
narrows to, the preserved-truth note, and a next action.

The scope-disclosure note keeps the target manifest scope and any script /
native-build side-effect class explicit whenever a claim narrows, so no generic
manage-package or one-click language can hide it. Stale registry freshness keeps the
mirror / offline continuity explicit; an unsatisfied auth state keeps the
registry-auth posture explicit; an unavailable rollback / checkpoint truth stays
explicit before any write.

## Canonical source of truth

Each row must point at the frozen component matrix **and** the canonical
implement-lane schema for its component (`points_at_canonical_contracts`), grouped
by `component_canonical_schema_ref`:

- `package_explorer_row` → `m5-package-explorer-row`
- `manifest_scope_switcher`, `registry_or_mirror_row` → `m5-manifest-scope-registry-controls`
- `install_review_sheet`, `lockfile_impact_card` → `m5-install-review-lockfile-controls`
- `script_risk_notice`, `grouped_update_planner`, `rollback_checkpoint_strip` → `m5-script-risk-grouped-update-rollback-controls`

## Coverage

`validate()` proves every one of the eight components, every one of the six claim
conditions, and every one of the six claim tiers appears among the rows. Raw
manifests, raw lockfile bodies, registry credentials, private registry URLs, and
live registry responses stay outside the support boundary.

## Regenerate

```sh
GEN_PACKAGE_COMPONENT_ACCESSIBILITY_ARTIFACTS=1 cargo test -p aureline-deps --lib \
  regenerate_package_component_accessibility_artifacts
```
