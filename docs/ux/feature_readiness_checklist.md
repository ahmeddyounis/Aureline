# Launch-Critical Feature Readiness Checklist

Checklist id: `checklist.feature_readiness.launch_critical`

This checklist is the design-complete gate for launch-critical UX
features and public-surface state changes. It turns release bars into
reviewable answers that can be joined to QE, compatibility, migration,
public-proof, component, and release evidence without relying on
private design-tool state.

Companion artifacts:

- [`docs/ux/design_release_evidence_pack_template.md`](./design_release_evidence_pack_template.md)
  - reusable evidence-pack template for feature design signoff.
- [`artifacts/ux/review_gate_manifest.yaml`](../../artifacts/ux/review_gate_manifest.yaml)
  - machine-readable gate policy for launch-critical surfaces and
  reusable component packets.
- [`artifacts/ux/surface_traceability_matrix.yaml`](../../artifacts/ux/surface_traceability_matrix.yaml)
  - launch-critical surface rows that this checklist gates.
- [`artifacts/design/component_review_checklist.md`](../../artifacts/design/component_review_checklist.md)
  - reusable component checklist inherited by feature surfaces that
  introduce or consume component packets.
- [`docs/design/component_state_taxonomy.md`](../design/component_state_taxonomy.md)
  - shared state taxonomy for default, locked, degraded, pending,
  loading, selected, current, and related states.
- [`docs/qe/qe_strategy_seed.md`](../qe/qe_strategy_seed.md),
  [`artifacts/qe/test_lane_registry.yaml`](../../artifacts/qe/test_lane_registry.yaml),
  and
  [`artifacts/qe/release_blocking_rules.yaml`](../../artifacts/qe/release_blocking_rules.yaml)
  - QE lane, scenario, and release-blocking policy refs.
- [`docs/compat/compatibility_row_seed.md`](../compat/compatibility_row_seed.md)
  and [`docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  - compatibility and migration linkage expected when a feature changes
  public or migration-sensitive behavior.
- [`docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  and [`fixtures/qe/public_proof_packets/`](../../fixtures/qe/public_proof_packets/)
  - public-proof packet seeds for claim-bearing surfaces.

## Gate Rule

A launch-critical feature or public-surface state change is not
design-complete until:

- every required checklist item below is answered `pass`,
  `not_applicable`, or `waived`;
- each `not_applicable` answer names the reason and reviewer;
- each `waived` answer names an active waiver with owner, scope,
  mitigation, expiry, user-visible impact, and exit signal;
- a reviewable design release evidence pack exists and cites the same
  feature, surface, requirement, evidence, compatibility, migration, and
  waiver refs; and
- the review gate manifest row for the surface points at the checklist
  id, evidence-pack id, and waiver fields.

`fail`, `unanswered`, `stale`, or `waiver_expired` blocks
design-complete. A waiver may narrow the claim or defer a proof item; it
does not convert the feature to `verified`.

Answer vocabulary:

| Answer | Meaning |
| --- | --- |
| `pass` | The evidence pack proves the item for the declared scope. |
| `not_applicable` | The item truly does not apply, and the packet states why. |
| `waived` | The gap is explicitly time-boxed and linked to an active waiver. |
| `fail` | The current design or evidence does not satisfy the item. |
| `unanswered` | No reviewable answer exists yet. |

## Checklist

| Item id | Required answer |
| --- | --- |
| `readiness.state.default` | Default or ready state is designed with primary action, available scope, source truth, and recovery entry where relevant. |
| `readiness.state.empty` | Empty state names why there is no content, what scope was checked, and the next safe action without implying readiness that does not exist. |
| `readiness.state.loading` | Loading or warming state preserves stable layout, distinguishes initial preparation from user-submitted pending work, and avoids progress spam. |
| `readiness.state.success` | Success state names what completed, what changed, and where evidence, undo, history, or follow-up lives when the action has durable effect. |
| `readiness.state.warning` | Warning state uses semantic status plus non-color cues and states the risk, affected scope, and safe next action. |
| `readiness.state.degraded` | Degraded state names preserved capability, reduced capability, freshness or certainty impact, and recovery or inspect route. |
| `readiness.state.error` | Error state includes plain-language cause, retry/repair/continue path, support or evidence route when needed, and no destructive default. |
| `readiness.lifecycle.transitions` | State transitions are listed from entry through success, failure, cancellation, recovery, and re-entry, including announcements and focus return. |
| `readiness.keyboard.path` | Complete keyboard path reaches entry, navigation, primary and alternate actions, details, recovery, dismissal, and focus return. |
| `readiness.screen_reader.behavior` | Role, name, description, state announcements, live-region behavior, and disabled/locked/read-only semantics are specified. |
| `readiness.theme.dark_light_high_contrast` | Dark, light, high-contrast, forced-colors, zoom, and density behavior preserve state meaning without hue-only reliance. |
| `readiness.motion.reduced` | Reduced-motion, low-motion, power-saver, and critical-hot-path postures keep feedback understandable without required animation. |
| `readiness.restricted_mode` | Restricted workspace or trust-narrowed behavior is explicit and separate from disabled, policy-blocked, and read-only behavior. |
| `readiness.remote_disconnected_mode` | Remote, provider, collaboration, or companion disconnection preserves local continuity or safe read-only fallback where the product allows it. |
| `readiness.ai_disabled_mode` | AI-disabled, provider-unavailable, budget-limited, or policy-disabled behavior has a truthful non-AI path or states that the feature is unavailable. |
| `readiness.evidence.stale_partial` | Stale, partial, imported, cached, heuristic, or outside-scope evidence is labeled and blocks unsafe widening or mutation where needed. |
| `readiness.preview_apply_revert` | Preview, apply, cancel, undo, revert, checkpoint, or rollback story is explicit for any action that mutates durable state. |
| `readiness.command_palette.integration` | Command-palette, menu, keybinding, context-menu, CLI, or automation entry points cite stable command ids or documented non-command routes. |
| `readiness.tokens.components` | Semantic tokens, component contracts, state taxonomy refs, theme rows, density rows, motion refs, and icon/asset refs are named by stable refs. |
| `readiness.extension.parity` | Extension, embedded, companion, browser, or handoff surfaces either match the host behavior or disclose a narrower scope and hand off. |
| `readiness.public_schema.stability` | Public schemas, exported packets, CLI/headless output, automation rows, telemetry event names, and docs-visible IDs carry stability labels and compatibility refs where relevant. |
| `readiness.copy.localization` | Copy is reviewed for source/lock explanation, high-risk wording, exact dates when needed, raw identifier handling, pseudoloc expansion, and bidi/IME safety. |
| `readiness.performance.hot_path` | Feature design names hot-path budget impact, hidden/background work, protected metrics, and any performance evidence or budget waiver. |
| `readiness.benchmark_efficiency` | User-visible benchmark, latency, power, thermal, or efficiency claims cite benchmark evidence or state that no user-visible claim is made. |
| `readiness.policy_trust` | Policy, permission, credential, trust, tenant, entitlement, and source-authority states are visible, inspectable, and export-safe. |
| `readiness.telemetry.validation` | Telemetry, usability validation, supportability, and evidence-refresh plan name metrics, events, data boundaries, and no-raw-secret posture. |
| `readiness.rollout.lifecycle` | Lifecycle label, rollout cohort, kill switch, downgrade path, support note, and claim-narrowing plan are declared. |
| `readiness.compat_migration_public_proof` | Compatibility rows, migration or importer impact, public-proof packet refs, known limits, and docs/release impacts are linked when the feature changes a public claim or import/export path. |

## Reviewer Summary

Every completed checklist stores the following summary in its evidence
pack and in the review gate manifest row:

| Field | Required value |
| --- | --- |
| `checklist_id` | `checklist.feature_readiness.launch_critical` |
| `feature_or_surface_id` | Stable feature, route, surface, component, or packet id. |
| `answer_counts` | Counts for `pass`, `not_applicable`, `waived`, `fail`, and `unanswered`. |
| `blocking_item_ids` | Checklist item ids still blocking design-complete. Empty only when the feature is eligible. |
| `evidence_pack_id` | Stable `evidence.*` id for the design release evidence pack. |
| `waiver_refs` | Active waiver ids or `[]`; expired waiver refs remain blockers. |
| `qe_lane_refs` | QE lane or scenario refs that will validate the feature. |
| `compatibility_row_refs` | Compatibility rows affected by the feature, or `[]`. |
| `migration_packet_refs` | Migration/import/export packet refs affected by the feature, or `[]`. |
| `public_proof_refs` | Public-proof packet or scoreboard refs when the feature supports a public claim, or `[]`. |
| `review_decision` | `design_complete`, `blocked`, `waived_narrow`, or `needs_review`. |
