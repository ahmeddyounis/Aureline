# Doctor probe-pack family catalog beta

The doctor probe-pack family catalog completes the beta Project Doctor lane
by covering the seven most common beta failure families with concrete
read-only probe packs that map directly to recovery-ladder actions. The
[Project Doctor beta probe-pack catalog](project_doctor_beta.md) names the
runtime packs and their finding contract; this row names the per-family
probe-pack catalog whose every row pins **prerequisites**, **outputs that
route a stable `doctor.finding.*` code to a recovery-ladder action**, and
**unsupported-state handling** so blocked users diagnose, repair, or hand
off through governed paths instead of guesswork.

The implementation lives in
[`crates/aureline-doctor/src/probe_packs/`](../../../crates/aureline-doctor/src/probe_packs/)
with the support/export consumer at
[`crates/aureline-support/src/project_doctor/probe_pack_coverage.rs`](../../../crates/aureline-support/src/project_doctor/probe_pack_coverage.rs).
The boundary schema lives at
[`/schemas/support/doctor_probe_pack.schema.json`](../../../schemas/support/doctor_probe_pack.schema.json)
and the protected fixture corpus lives at
[`/fixtures/support/m3/doctor_probe_packs/`](../../../fixtures/support/m3/doctor_probe_packs/).

## What this beta row owns

- A typed `DoctorProbePackCatalog` that names exactly one
  `DoctorProbePackRecord` per closed `failure_family_class`
  (`entry`, `toolchain`, `search_index`, `trust_policy`, `git`,
  `provider`, `restore`). Each record carries a stable `pack_id`,
  `pack_version`, `pack_class` mirrored from the
  [Project Doctor beta catalog](project_doctor_beta.md), and a
  `doctor_pack_ref` back to the owning beta probe pack.
- A closed `prerequisite_class` vocabulary (`admission_manifest`,
  `entry_intent_record`, `execution_context_manifest`,
  `search_index_status_record`, `policy_decision_record`,
  `trust_state_record`, `git_workspace_state`,
  `credential_state_record`, `session_restore_manifest`) so every pack
  declares the typed evidence rows it consumes instead of inferring
  state from raw filesystem or network probes.
- A closed `recovery_ladder_action_class` vocabulary
  (`enter_safe_mode`, `start_extension_bisect`, `open_repair_preview`,
  `locate_missing_target`, `reresolve_toolchain`, `open_index_status`,
  `open_policy_details`, `open_git_baseline_details`,
  `reauthenticate_provider`, `open_without_restore`,
  `handoff_to_support`) so every output finding code routes to a
  governed recovery-ladder step instead of free-form guidance.
- A closed `unsupported_state_class` vocabulary
  (`evidence_unavailable`, `scope_out_of_bounds`, `dependency_missing`,
  `context_not_admitted`, `unsupported_platform`) plus an
  `unsupported_finding_code` field so packs declare what they refuse to
  diagnose without inventing a recovery path.
- A `DoctorProbePackEvaluator` that validates each pack, refuses
  duplicate families, missing families, mis-prefixed finding codes, or
  an `unsupported_finding_code` that collides with an output code; and
  that folds the catalog into a metadata-safe
  `DoctorProbePackCoverageScorecard` projecting one row per family for
  supportability scorecards.

## Acceptance and how this row meets it

- **The supported beta failure families have named probe packs with
  clear prerequisites, outputs, and unsupported-state handling.** Every
  `doctor_probe_pack_record` declares at least one prerequisite from
  the closed `prerequisite_class` vocabulary, at least one output, and
  a typed `unsupported_state_handling` block. The validator refuses any
  pack that omits prerequisites, outputs, or unsupported-state handling
  and any pack whose `pack_class` does not match its
  `failure_family_class`.
- **Probe results route to safe mode, bisect, repair preview, or other
  recovery steps through stable finding codes.** Every output names a
  stable `doctor.finding.*` finding code, a closed
  `recovery_action_class`, and a non-empty `recovery_step_ref` that
  pins the safe-mode profile, extension-bisect session, repair-preview
  transaction, or other governed handoff target. The catalog exercises
  every recovery-action class in the closed vocabulary at least once.
- **Probe-pack coverage is visible in supportability scorecards
  instead of assumed.** The `DoctorProbePackCoverageScorecard`
  projection — owned in `aureline-support` via
  [`probe_pack_coverage::doctor_probe_pack_coverage`](../../../crates/aureline-support/src/project_doctor/probe_pack_coverage.rs)
  — names one row per family, counts the supported finding codes and
  recovery actions, and pins `families_covered` and
  `families_uncovered` so a supportability scorecard cannot silently
  show partial coverage.

## Failure-drill posture

The evaluator fails closed before widening diagnosis:

- A catalog missing a failure family, with duplicate `pack_id`s, with
  duplicate families, or with a mismatched `doc_ref` / `schema_ref` is
  refused.
- A pack with no prerequisites, no outputs, an empty
  `recovery_step_ref`, a finding code that does not start with
  `doctor.finding.`, or an `unsupported_finding_code` that collides
  with one of its output codes is refused.
- A pack whose `pack_class` does not belong to its
  `failure_family_class` is refused.
- The coverage scorecard refuses to project when the catalog fails
  validation, so a supportability scorecard cannot show "all green"
  while the underlying catalog is incomplete.

## First consumers

- The `aureline-support` consumer is the canonical projection for
  supportability scorecards. The
  [`doctor_probe_pack_coverage`](../../../crates/aureline-support/src/project_doctor/probe_pack_coverage.rs)
  function folds the catalog into a typed coverage scorecard that
  supportability surfaces serialize verbatim.
- The boundary schema is the contract the headless export writer, the
  CLI finding renderer, and the support-export chrome share — every
  surface reconstructs the same packet shape from the on-disk record
  verbatim, never re-derives it from a side channel.

## Related contracts

- [Project Doctor beta probe-pack catalog](project_doctor_beta.md) —
  the parent beta lane that names runtime packs, finding records, and
  the support packet projection.
- [Safe-mode beta](safe_mode_beta.md) — the `enter_safe_mode` action
  in this catalog points at a typed safe-mode profile from that lane.
- [Extension-bisect beta](extension_bisect_beta.md) — the
  `start_extension_bisect` action points at a typed extension-bisect
  session id from that lane.
- [Recovery ladder alpha](../recovery_ladder_alpha.md) — the recovery
  ladder this catalog routes findings into.

## Out of scope for this beta row

- Live runtime probe orchestration (scheduling, retries, throttling) —
  this row owns the named, versioned probe-pack family catalog and
  does not host the runtime.
- Cross-tenant managed catalog distribution; the catalog ships as a
  protected fixture under
  `fixtures/support/m3/doctor_probe_packs/`.
- Repair application; outputs that point at
  `open_repair_preview` open a typed repair-preview transaction
  without applying any mutation.
