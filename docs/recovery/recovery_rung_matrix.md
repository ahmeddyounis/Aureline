# Recovery-rung matrix: safe mode, extension bisect, quarantine, cache reset, restricted reopen, and rollback candidate

This document freezes the **named recovery rungs** Aureline uses when a startup crash loop, a suspect extension, disposable-state corruption, uncertain trust posture, or a bad update/remote-helper skew makes a normal reopen unsafe.

Each rung is a versioned recovery profile with **bounded blast radius**: it narrows specific capability surfaces while preserving user work, evidence, and build identity so recovery does not devolve into improvised “turn things off until it works” behavior.

This contract defines, for each rung:

- typical entry triggers (re-exported from the frozen `recovery_entry_reason_class` vocabulary);
- disabled / narrowed capabilities by default;
- preserved state / capabilities;
- required user or admin review gates;
- the exit artifact(s) the rung MUST emit; and
- the escalation and exit paths that keep recovery attributable and auditable.

Out of scope: implementing a supervisor, UI, or automation that chooses rungs. This document publishes semantics only.

## Companion artifacts

- [`/artifacts/recovery/recovery_rungs.yaml`](../../artifacts/recovery/recovery_rungs.yaml)
  — machine-readable rung matrix and transition rules for tooling, docs integrity checks, and reviewer reference.
- [`/fixtures/recovery/recovery_rung_examples/`](../../fixtures/recovery/recovery_rung_examples/)
  — worked examples for crash-loop safe mode, extension regression bisect/quarantine, cache reset candidate, restricted reopen, and rollback candidate.

## Normative sources projected here

- `.t2/docs/Aureline_Technical_Architecture_Document.md` §24.2.2 and Appendix DL (recovery rungs and ladder rules).
- `.t2/docs/Aureline_PRD.md` recovery ladder, diagnostics, and support export sections.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md) and
  [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
  (closed vocabularies and rung decision-object fields used by support/export surfaces).
- [`/docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md) (exact-build identity continuity rules).
- [`/docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md) (rollback / downgrade / helper negotiation expectations).

If this document disagrees with a normative source above, the normative source wins and this document plus its companion artifacts MUST be updated in the same change.

## Closed vocabularies (re-exported, not redefined)

This document does not mint new `recovery_rung_class`, `recovery_entry_reason_class`, `recovery_exit_reason_class`, `authority_class`, `preserved_state_class`, `lost_capability_class`, or `escalation_trigger_class` values.

All closed tokens referenced here are re-exported from:

- [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json) (`recovery_rung_class`, entry/exit reasons),
- [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json) (authority/preserved/lost/escalation closed sets).

## Recovery-rung matrix

The matrix below uses the stable `recovery_rung_class` tokens to avoid copy drift across surfaces.

| Rung (`recovery_rung_class`) | Typical entry triggers | Disabled or narrowed by default | Preserved by rule | Required review gate | Exit artifact(s) |
|---|---|---|---|---|---|
| `safe_mode` | `repeated_startup_failure`, `crash_loop_detected`, `explicit_user_choice` | third-party extension host launch and auto-activation; repo-owned activators; automatic remote/collab reattach; optional background rebuild | user-authored files, workspace trust store (no silent widening), support/export capability, basic navigation/editing | MAY be suggested automatically; entering MUST be user-visible and logged | safe-mode session manifest + rung transition record |
| `extension_bisect` | `extension_regression_suspected`, `explicit_user_choice` | all extensions disabled except the active cohort; registry/network lookups minimized; permissions not widened | user-authored files, evidence store, durable settings/profiles, support/export | cohort selection and each “next cohort” step requires user confirmation (or admin policy) | bisect step record(s) + rung transition record |
| `extension_quarantine` | `extension_regression_suspected`, `manual_review_required`, `policy_blocked` | identified extension/host remains disabled; auto-reenable on restart/update forbidden | workspace open, diagnostics, targeted re-enable preview, support/export | user or admin must review before re-enable; managed policy MAY force quarantine | quarantine record + rung transition record |
| `cache_reset_candidate` | `cache_integrity_failure`, `manual_review_required` | only declared disposable storage classes eligible; durable profiles/history never reset as collateral | user-authored files, durable settings/profiles/history, support/export, state inventory | reset preview required; any mutation requires explicit user confirmation | reset transaction manifest + rung transition record |
| `restricted_reopen` | `policy_blocked`, `manual_review_required` | execution surfaces narrowed; repo-owned hooks and privileged launches suppressed; trust cannot be widened | safe browsing/editing/diagnostics; support/export; trust review | entry may be policy-forced; exiting restricted posture requires explicit user/admin trust review | restricted reopen manifest + rung transition record |
| `rollback_reinstall_candidate` | `manual_review_required`, `policy_blocked` (bad update / incompatible helper) | current build/helper/extension candidate not replaced until preview + eligibility checks pass | evidence review, support/export, compatibility explanation, explicit retry path | rollback/downgrade preview required; mutation requires explicit user confirmation or admin action | rollback manifest + rung transition record |

## Rung transition rules

### Selection rule: narrowest effective blast radius

When multiple rungs could plausibly address the same symptom, selection MUST prefer the narrowest rung whose disabled surfaces plausibly contain the fault domain:

1. isolate one extension cohort before quarantining many;
2. reset one disposable storage class before proposing broader reset;
3. narrow trust / execution surfaces (restricted reopen) instead of auto-running privileged recovery actions; and
4. prefer rollback only when build identity / helper negotiation / update evidence indicates the running build is the likely offender.

### Upward vs downward movement

This contract treats “upward” movement as moving to a rung that is more restrictive, more isolating, or more disruptive. “Downward” movement restores capabilities (typically returning toward full mode).

Rules:

- **Downward moves require evidence.** A rung may only be exited to a less-restrictive rung (or to full mode) after recording an exit reason and a minimal verification note in the rung exit artifact.
- **No silent trust widening.** Moving downward may not widen workspace trust or re-enable repo-owned activators automatically; any trust posture change requires an explicit review gate.
- **Policy can force narrowness.** Managed policy may force entry to, or persistence in, `restricted_reopen` or `extension_quarantine`. In those cases, “downward” moves are blocked until policy permits; exit artifacts MUST record the denial reason.
- **Rollback is a build-identity boundary.** Entering or exiting `rollback_reinstall_candidate` MUST preserve both the prior and target exact-build identity refs so support can reconstruct “what bytes were running” for both before/after states.

### Evidence, trust state, and exact-build identity continuity

Every rung entry and exit MUST preserve three truths:

- **Evidence continuity:** rung transitions are logged (see `support_bundle_record.recovery_context.rung_history`) and exit artifacts are stored in an exportable location. Rungs do not “solve” problems by deleting evidence.
- **Trust continuity:** the active trust state and policy epoch (see `support_bundle_record.policy_context.trust_state` / `policy_epoch`) carry across rungs unchanged unless the user/admin explicitly changes trust via a review gate; recovery is not a backdoor to elevate trust.
- **Exact-build continuity:** the primary exact-build identity ref is always recorded (see `support_bundle_record.build_and_install_context.primary_exact_build_identity_ref`). If rollback changes the running build, the exit artifact MUST link both the previous and new identity refs and record any helper/agent negotiation results that motivated the rollback.

## Worked examples

See [`/fixtures/recovery/recovery_rung_examples/`](../../fixtures/recovery/recovery_rung_examples/) for concrete rung selections and transitions covering:

- startup crash loop → `safe_mode`;
- extension regression suspicion → `extension_bisect` → `extension_quarantine`;
- suspected cache corruption → `cache_reset_candidate`;
- uncertain trust posture or policy block → `restricted_reopen`;
- bad update or remote-helper skew → `rollback_reinstall_candidate`.

