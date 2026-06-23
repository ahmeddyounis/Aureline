# Attention-qualification bundle — evidence companion

Human-readable companion to
[`/fixtures/activity/m5-attention-qualification/canonical_bundle.json`](../../fixtures/activity/m5-attention-qualification/canonical_bundle.json)
and its boundary schema
[`/schemas/activity/m5-attention-qualification.schema.json`](../../schemas/activity/m5-attention-qualification.schema.json).
It gives reviewers the frozen family, profile, dependency, and consumer tables without
reading the JSON. The contract narrative lives in
[`/docs/activity/m5-attention-qualification.md`](../../docs/activity/m5-attention-qualification.md).

- Bundle id: `m5-attention-qualification:bundle:0001`
- Record kind: `m5_attention_qualification_bundle`
- Binds back to: `m5-attention-routing:matrix:0001`
- Families: 7 · Profiles: 3 · Consumers: 6 · Release-evidence rows: 8 · Invariants: 11

## Certified attention families

Each claimed attention family is one frozen lane in `aureline-activity`, backed by a
checked-in fixture and a freeze gate. The canonical bundle freezes every family `fresh`;
release automation overrides the evidence state from the live freeze-gate result.

| Family | Proof packet (fixture) | Freeze gate |
| --- | --- | --- |
| `notification_envelope` | `m5-envelope-routing/canonical_bundle.json` | `tests/m5_envelope_routing.rs` |
| `activity_object` | `m5-activity-objects/canonical_bundle.json` | `tests/m5_activity_objects.rs` |
| `attention_action` | `m5-attention-actions/canonical_bundle.json` | `tests/m5_attention_actions.rs` |
| `quiet_hours_suppression` | `m5-quiet-hours-suppression/canonical_bundle.json` | `tests/m5_quiet_hours_suppression.rs` |
| `badge_aggregate` | `m5-badge-aggregates/canonical_bundle.json` | `tests/m5_badge_aggregates.rs` |
| `fanout_receipt` | `m5-fanout-receipts/canonical_bundle.json` | `tests/m5_fanout_receipts.rs` |
| `attention_routing_matrix` | `m5-attention-routing/canonical_matrix.json` | `tests/m5_attention_routing.rs` |

## Release-evidence rows

Every family declares which release-evidence rows its proof covers, so a release packet names
explicit attention evidence rather than a vague summary. Each row is covered by at least one
family.

| Release-evidence row | Covered by |
| --- | --- |
| `notification_envelopes` | `notification_envelope` |
| `durable_activity_objects` | `activity_object` |
| `action_semantics` | `attention_action` |
| `quiet_hours_suppression` | `quiet_hours_suppression` |
| `badge_dedupe_fidelity` | `badge_aggregate` |
| `fanout_privacy_reopen_parity` | `fanout_receipt` |
| `reopen_authoritative` | every family |
| `routing_object_model` | every family |

## Claimed profiles and dependencies

Each claimed shell, companion, and operator profile depends on a subset of families; its claim
state is **derived** from those families' evidence, never asserted. The routing matrix, the
quiet-hours/suppression policy, and the fanout receipt are shared spines every profile depends
on — so routing, privacy, or fanout staleness narrows every claim.

| Profile | Depends on | Standalone surfaces |
| --- | --- | --- |
| `shell_attention` | envelope, activity, action, quiet-hours, badge, fanout, matrix | activity center, OS notification, dock/taskbar badge |
| `companion_attention` | envelope, quiet-hours, badge, fanout, matrix | browser companion, mobile companion |
| `operator_attention` | activity, quiet-hours, badge, fanout, matrix | operator dashboard, chronology reuse |

## Automatic claim narrowing

The claim state is the worst of the dependencies' evidence severities. The table shows how a
single non-fresh family narrows the claims that depend on it; a profile that does not depend on
the family stays `full`.

| Family goes… | `stale` →  | `failing` / `missing` → |
| --- | --- | --- |
| `attention_action` | shell `narrowed` | shell `withdrawn` |
| `notification_envelope` | shell + companion `narrowed` | shell + companion `withdrawn` |
| `activity_object` | shell + operator `narrowed` | shell + operator `withdrawn` |
| `quiet_hours_suppression` | all three `narrowed` | all three `withdrawn` |
| `badge_aggregate` | all three `narrowed` | all three `withdrawn` |
| `fanout_receipt` | all three `narrowed` | all three `withdrawn` |
| `attention_routing_matrix` | all three `narrowed` | all three `withdrawn` |

`recompute_profiles(families, evidence)` is the release-automation entry point: feed it the live
freeze-gate results and it returns each profile's derived claim — `full`, `narrowed`, or
`withdrawn` — naming the family that caused the narrowing, without restating any claim by hand.

## Consumers reuse one projection

`projection()` is the one support-export-safe view all of these read instead of minting
per-surface attention quality vocabulary: `release_evidence`, `about_help`, `activity_center`,
`support_export`, `compatibility_report`, and `public_truth`.

## Computed invariants (all hold)

| Invariant |
| --- |
| `attention_qualification.every_family_proven` |
| `attention_qualification.release_rows_covered` |
| `attention_qualification.profile_claim_derived` |
| `attention_qualification.fresh_promotes_full` |
| `attention_qualification.stale_dependency_narrows` |
| `attention_qualification.failing_dependency_withdraws` |
| `attention_qualification.shared_spines_depended_everywhere` |
| `attention_qualification.no_standalone_green_surface` |
| `attention_qualification.security_never_silenced` |
| `attention_qualification.consumers_reuse_projection` |
| `attention_qualification.binds_routing_matrix` |

The freeze gate `crates/aureline-activity/tests/m5_attention_qualification.rs` rebuilds the
bundle in code and asserts it equals this fixture byte-for-byte; an inconsistent edit flips an
invariant or fails the round-trip.
