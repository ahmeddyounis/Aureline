# Extension-bisect orchestration with attributable findings and bounded blast radius

The extension-bisect rung is the recovery posture a blocked user enters
when an extension regression is suspected — typically after a startup
crash loop, an extension restart-budget exhaustion, an explicit user
request, or a managed-policy force — and the user wants to *attribute*
the offender by progressively activating cohorts rather than disabling
every extension by hand. Every cohort activation result, suspected
extension set, and user-visible finding is recorded as a typed row; the
bisect leaves the prior extension state restorable so the user (or
managed policy) can always return to the state they were in before the
bisect began.

The implementation lives in
[`crates/aureline-support/src/extension_bisect/mod.rs`](../../../crates/aureline-support/src/extension_bisect/mod.rs)
and the boundary schema lives at
[`/schemas/support/extension_bisect.schema.json`](../../../schemas/support/extension_bisect.schema.json).
The protected fixture corpus lives under
[`fixtures/recovery/m3/extension_bisect/`](../../../fixtures/recovery/m3/extension_bisect/).

## What this beta row owns

- A typed `ExtensionBisectSession` record that declares the active bisect
  posture as a typed list of `CandidateExtension`, `TestedStateRow`,
  `SuspectedExtensionSet`, and `UserVisibleFinding` rows, plus a
  `RestorePlan` that names the prior-state snapshot, restore action,
  and restore conditions.
- A typed `ExtensionBisectStep` record for every cohort activation,
  cohort verification, and baseline check. The record observes the
  cohort id, member refs, activation result class, and verdict class and
  pins both `user_owned_state_deleted` and `durable_state_deleted` to
  `false`.
- A typed `ExtensionBisectFinding` record that names the user-visible
  attribution. The evaluator refuses a finding whose class requires
  suspect refs (`single_extension_suspected`,
  `multi_extension_suspected`) without naming them.
- A typed `ExtensionBisectRestore` record that records the return of
  the prior extension state. The evaluator refuses a restore record
  that deletes user-owned or durable state, drops the
  `user_authored_files` preservation observation, declares a
  restore disposition pending review, or omits restored extensions.
- An `ExtensionBisectEvaluator` that validates each record class,
  cross-checks that every step / finding / restore `session_ref`
  matches the bound session, and projects one
  `ExtensionBisectSupportPacket` per session lifecycle. The packet
  excludes raw private material and ambient authority, quotes the doc
  and schema refs verbatim, and carries the declared bisect rows plus
  the matching steps, findings, and restore row.
- The boundary JSON schema and three canonical fixtures covering the
  `post_crash_loop_session`, `regression_suspected_session`, and
  `policy_forced_session` classes.

## Acceptance and how this row meets it

- **Bisect sessions preserve a log of tested states, suspected
  extension sets, and user-visible findings.** Every
  `tested_states`, `suspected_extension_sets`, and `findings` row
  carries an opaque id, a closed class (activation result, suspect
  confidence, finding class), and a reviewer-safe summary. The
  evaluator refuses a session with empty candidates, missing evidence,
  duplicate ids, or a suspect set whose confidence requires members
  without naming them.
- **The process disables only the minimal needed scope and can
  restore the prior extension state afterward.** Every session
  declares an explicit `blast_radius_class` and a
  `disabled_capability_classes` list that the evaluator requires to
  include `extension_auto_activation`. Every session preserves
  `local_editing`, `extension_bisect_exit_action`,
  `user_authored_files`, and `extension_prior_state_snapshot` so the
  user can keep working and explicitly exit the bisect. The bound
  `ExtensionBisectRestore` record names a prior-state snapshot ref,
  enumerates every restored extension's prior and restored state
  class, and pins `user_owned_state_deleted = false` and
  `durable_state_deleted = false`.
- **Support exports can include the bisect packet without requiring
  manual notes.** `ExtensionBisectEvaluator::support_packet` folds
  one session, its steps, its finding, and its restore into a
  metadata-safe `ExtensionBisectSupportPacket`. The packet quotes the
  doc and schema refs verbatim, projects every candidate, tested
  state, suspect set, user-visible finding, step, and restore row,
  and pins `raw_private_material_excluded = true`,
  `ambient_authority_excluded = true`, and
  `destructive_resets_present = false`.

## Failure-drill posture

The evaluator fails closed before widening the bisect or hiding evidence:

- A session that declares a destructive reset, deletes user-owned
  state, or deletes durable non-disposable state is refused.
- A session that drops `local_editing`,
  `extension_bisect_exit_action`, `user_authored_files`, or
  `extension_prior_state_snapshot` preservation is refused.
- A session that fails to disable `extension_auto_activation` is
  refused; the bisect cannot silently allow the host to re-enable
  extensions.
- A session without a `doctor.finding.*` ref or without any candidate
  extension is refused.
- Duplicate `extension_id`, `tested_state_id`, or `suspect_set_id`
  values are refused.
- A step that deletes user-owned or durable state is refused.
- A cohort-activation step with no cohort members, a baseline step
  with cohort members, or an aborted activation whose verdict is not
  `cohort_run_aborted` is refused.
- A finding whose class requires suspects (single/multi-extension
  suspected) but does not name any suspect ref is refused.
- A restore that deletes user-owned or durable state, drops the
  `user_authored_files` observation, declares a deferred-pending-
  review disposition, or omits restored extensions is refused.
- A finding, step, or restore whose `session_ref` does not match the
  bound session is refused at `support_packet` time.

## First consumers

- The `aureline-support` extension-bisect module is the canonical
  projection for support-export and recovery-ladder review.
  `ExtensionBisectEvaluator::support_packet` folds one session and
  its bound records into a metadata-safe
  `ExtensionBisectSupportPacket` that the support-export pipeline can
  serialize verbatim.
- The boundary schema is the contract the headless export writer and
  the support-export chrome share — both reconstruct the same packet
  shape from the on-disk record verbatim, never re-derive it from a
  side channel.

## Related contracts

- [Recovery-ladder alpha](../recovery_ladder_alpha.md) — the parent
  rung contract for recovery rungs that include extension bisect.
- [Recovery-rung matrix](../../recovery/recovery_rung_matrix.md) —
  declares the closed `extension_bisect` rung, its disabled and
  preserved capability classes, and its exit artifacts.
- [Safe-mode beta](safe_mode_beta.md) — the parent safe-mode profile
  the bisect frequently coexists with. Safe-mode entry typically
  precedes a bisect session for crash-loop attributions; safe-mode
  exit follows once the bisect produces an attributable finding.
- [Support-bundle contract](../support_bundle_contract.md) — the
  parent contract for every metadata-safe support projection.

## Out of scope for this beta row

- Live runtime enforcement (extension supervisor, cohort selector,
  host launcher gating) — those bindings land with the runtime-host
  and chrome consumers that quote this session shape.
- Automated cohort-selection heuristics; the beta surfaces explicit
  user-confirmed cohorts only.
- Cross-tenant managed-bisect orchestration; the
  `policy_forced_session` fixture covers single-tenant policy abort
  only.
