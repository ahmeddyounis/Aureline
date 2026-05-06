# Extension lifecycle, activation safety, crash-loop quarantine, and recovery sequence

This document freezes Aureline’s **extension lifecycle state vocabulary** plus a
shared **activation → crash-loop → quarantine → recovery** sequence so the shell,
install review, runtime status, crash-loop surfaces, and support exports can
explain extension isolation decisions consistently.

This is a sequencing and taxonomy contract, not an implementation of an
extension host, supervisor, quarantine engine, or marketplace backend.

## Companion artifacts

- [`/artifacts/extensions/extension_lifecycle_states.yaml`](../../artifacts/extensions/extension_lifecycle_states.yaml) —
  machine-readable lifecycle states, checkpoint catalog, and ordered variant
  paths.
- [`/fixtures/extensions/quarantine_sequence_cases/`](../../fixtures/extensions/quarantine_sequence_cases) —
  worked sequence cases covering first-party + third-party, online + mirror +
  offline bundle installs, crash-loop quarantine, safe reopen, bisect, recovery,
  and permanent removal.

Related contracts this sequence composes with (and does not replace):

- [`/docs/extensions/runtime_budget_packet.md`](./runtime_budget_packet.md) and
  [`/artifacts/extensions/quarantine_rules.yaml`](../../artifacts/extensions/quarantine_rules.yaml) —
  runtime-budget / crash-loop evidence and quarantine triggers.
- [`/docs/runtime/fault_domains_and_restart_policy.md`](../runtime/fault_domains_and_restart_policy.md) —
  restart budgets and the shared escalation-state vocabulary (`Degraded`,
  `Disabled`, `Quarantined`, `Warming`, `Ready`).
- [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md) —
  crash-loop screen action grammar (`Safe mode`, `Disable suspect extension`,
  `Open without restore`) and evidence preservation rules.
- [`/docs/recovery/recovery_rung_matrix.md`](../recovery/recovery_rung_matrix.md) and
  [`/artifacts/recovery/recovery_rungs.yaml`](../../artifacts/recovery/recovery_rungs.yaml) —
  named recovery rungs (`safe_mode`, `extension_bisect`, `extension_quarantine`,
  `restricted_reopen`, etc) used when reopening safely.
- [`/docs/extensions/registry_and_offline_bundle_seed.md`](./registry_and_offline_bundle_seed.md) and
  [`/docs/extensions/publisher_lifecycle_and_registry_parity_contract.md`](./publisher_lifecycle_and_registry_parity_contract.md) —
  online/mirror/offline/local-archive provenance and publisher-level quarantine /
  revocation vocabulary.
- [`/docs/verification/install_review_packet.md`](../verification/install_review_packet.md) —
  install-review quarantine posture (`quarantined_publisher`,
  `quarantined_by_policy_pack`, etc) and denial reasons (e.g.
  `publisher_quarantined`, `policy_pack_denies_extension`).

## Scope

Frozen here:

- A closed `extension_lifecycle_state` vocabulary covering `discovered`,
  `installed`, `disabled`, `pending_activation`, `active`, `degraded`,
  `quarantined`, `recovered`, `removed`, and `publisher_blocked`.
- A closed `checkpoint_id` catalog and ordered variant paths so every surface
  can cite “what happened” with stable identifiers.
- Mapping rules for **why** an extension was disabled/quarantined/blocked using
  existing upstream vocabularies (`trigger_rule_id`, `denial_reason_class`,
  `fault_domain_id`, `recovery_rung_class`) instead of inventing new ones.
- Integrity rules that preserve workspace state, evidence linkage, and
  support-friendly reasoning whenever an extension is isolated.

Out of scope:

- shipping the extension host, crash-loop detector, or quarantine engine;
- implementing UI screens; and
- defining new trust, revocation, or policy vocabularies beyond the existing
  contracts linked above.

## Lifecycle model (state is not the whole truth)

The lifecycle state is a **projection** used for user/support explanation. It is
intentionally narrower than the full system:

- **Registry / source truth** (online, mirror, offline bundle, local archive) is
  carried by the existing `registry_source_class` and mirror/offline continuity
  records — not by lifecycle state.
- **Trust and admission** are carried by install-review denial reasons and
  publisher lifecycle events — not by lifecycle state.
- **Recovery posture** is carried by `recovery_rung_class` (safe mode, bisect,
  quarantine, restricted reopen, rollback candidate) — not by lifecycle state.
- **Host health** (`Warming`, `Ready`, `Degraded`, `Disabled`, `Quarantined`) is
  governed by the fault-domain supervisor contract and may be surfaced alongside
  lifecycle state; the state names are intentionally shared.

## State vocabulary (closed)

The authoritative state catalog (definitions, admissible transitions, and the
required checkpoint ids per transition) lives in:

- [`/artifacts/extensions/extension_lifecycle_states.yaml`](../../artifacts/extensions/extension_lifecycle_states.yaml)

Surfaces MUST:

- render one of the frozen state ids (no surface-local enums);
- cite upstream evidence ids when a state is not `active` (rule ids, denial
  reasons, forensic packet refs, policy refs); and
- avoid collapsing distinct causes (publisher-level block vs runtime crash-loop
  quarantine vs user disable) into one generic “disabled” chip.

## Sequence: activation → crash-loop → quarantine → recovery

The sequence packet defines stable `checkpoint_id` values and ordered paths that
work across:

- first-party and third-party extensions;
- online installs, approved mirrors, and offline bundle restores; and
- crash-loop safe-mode reopen, extension bisect, quarantine, recovery, and
  permanent removal.

The machine-readable sequence lives in:

- [`/artifacts/extensions/extension_lifecycle_states.yaml`](../../artifacts/extensions/extension_lifecycle_states.yaml)

### Evidence linkage rules (binding)

When an extension is degraded, disabled, quarantined, or blocked:

1. **Workspace integrity first.** The recovery path MUST preserve user-authored
   files, durable settings/profiles, trust posture, and local evidence; recovery
   is not permitted to “fix” by deleting evidence or widening trust.
2. **Typed reason always.** The transition MUST carry a typed reason that
   resolves in an upstream contract:
   - runtime quarantine/disable MUST cite `trigger_rule_id` from
     `artifacts/extensions/quarantine_rules.yaml`;
   - publisher-level block MUST cite the install-review denial reason (for
     example `publisher_quarantined`) plus the publisher lifecycle event ref; and
   - crash-loop / restart-budget transitions MUST cite the fault-domain and a
     forensic packet ref.
3. **Recovery actions are rung-bound.** “Reopen without extension”, bisect, and
   safe mode MUST be recorded as recovery rungs (`recovery_rung_class`) rather
   than one-off toggles.
4. **Placeholders over deletion.** Missing/quarantined extension surfaces MUST
   render placeholders (per the crash-loop and restore-fidelity contract) rather
   than silently removing panes or rewriting layout state.

### Recovery decision points (what the user sees)

When the sequence reaches quarantine or repeated failure, the user-facing choice
set is constrained to the already-frozen action grammar:

- `Safe mode` (enter `recovery_rung_class = safe_mode`);
- `Disable suspect extension` / “Reopen without extension” (enter
  `recovery_rung_class = extension_quarantine` for the named subject);
- `Extension bisect` (enter `recovery_rung_class = extension_bisect`);
- `Open without restore` (skip applying the prior restore manifest for this
  launch; evidence remains); and
- `Remove extension` (permanent local removal; preservation rules still apply).

The sequence packet’s checkpoint ids exist so crash-loop surfaces, runtime status
surfaces, and support exports cite the **same** decision points without
inventing additional taxonomy.

