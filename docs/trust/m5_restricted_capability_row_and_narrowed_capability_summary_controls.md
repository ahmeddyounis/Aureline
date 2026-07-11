# M5 restricted-capability-row and narrowed-capability-summary controls

This is the restricted-mode implement lane over the frozen
[M5 workspace-trust / guided-repair component matrix](./m5_workspace_trust_repair_components_contract.md).
It turns the matrix's `restricted_capability_row` component — and the narrowed-capability summary
that rolls it up — into two resolvers that produce export-safe, honest projections, so **restricted
mode reads as a stable operating posture** a user can act inside rather than a vague blocked state.

- Rust: `crates/aureline-shell/src/implement_the_m5_restricted_capability_row_and_narrowed_capability_summary_blocked_action_families_still_safe_actions_restriction_reason_and_command_backed_recovery_primitive/`
- Schema: `schemas/ui/m5-restricted-capability-row-narrowed-capability-summary-controls.schema.json`
- Component schema: `schemas/ui/m5-restricted-capability-row.schema.json`
- Proof packet: `artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/`
- Fixtures: `fixtures/ui/m5-restricted-capability-row-narrowed-capability-summary-controls/`

## Goal

Explain restricted mode as a stable operating posture rather than a vague blocked state. The
resolvers reuse the frozen matrix vocabulary directly — the single controlled trust / repair
disposition, trust-scope, grant-source, narrowed-capability, and per-root trust vocabularies — so
every claimed M5 restricted surface exposes the same restriction, still-safe, and recovery grammar
instead of forking its own "some features are unavailable" wording.

## Resolvers

### `resolve_restricted_capability_row`

Refuses to read as a clean, legible row unless it:

- names the restricted **object** and resolves the **restriction scope**;
- names the **restriction source** (which grant class imposed it) and the human-readable
  **restriction reason** (why it exists);
- names the **narrowed capability** it removes;
- enumerates at least one **blocked action family** (never a generic "unavailable");
- names at least one **still-safe action** so a user can tell what remains safe;
- keeps a **command-backed recovery set** reachable.

Otherwise it degrades to a typed `M5RestrictedCapabilityRowDegradeReason`. It never lets a restricted
surface collapse into generic unavailable copy (`collapsed_into_generic_unavailable`) and never reads
a mixed-root restriction as uniform across roots (`mixed_root_collapsed_into_uniform`).

### `resolve_narrowed_capability_summary`

Projects the same fields as a compact rollup — with blocked-family and still-safe counts — and the
same command-backed recovery grammar, so a narrowed-capability summary on any surface still names
what is blocked, what stays safe, and how to recover. It additionally degrades when distinct blocked
families collapse into a single generic count (`blocked_families_collapsed_into_generic_count`).

## Command-backed recovery paths

Every resolved row and summary carries a command-backed `M5RestrictedRecoveryAction` set:

- **inspect trust** — always offered; the command-backed entrypoint every recovery set is anchored on;
- **reopen restricted** — always offered;
- **continue limited** — offered when a capability is narrowed;
- **request approval** — offered only where the restriction allows it (`approval_allowed`).

Recovery choices stay consistent across consumers because every clean example is anchored on the same
inspect-trust entrypoint, never routed through docs or logs only.

## Hard invariants (MUST be false on every clean row)

- `collapses_restricted_into_generic_unavailable`
- `hides_blocked_families_or_still_safe_actions`
- `routes_recovery_through_docs_or_logs_only`
- `implies_blanket_restriction_across_roots_or_routes`

## Acceptance criteria, proven by examples

- **Restricted surfaces no longer collapse into generic unavailable copy.** Clean rows cover the
  restricted, policy-blocked, and mixed-root restriction scopes; at least one row and one summary
  degrade to `collapsed_into_generic_unavailable`; no clean example collapses; and every clean
  example enumerates at least one blocked action family.
- **Users can tell which actions remain safe with consistent command-backed recovery.** Every clean
  row/summary names a still-safe action and exposes a command-backed recovery path anchored on
  inspect-trust; missing-still-safe and missing-recovery examples degrade.

The Rust validator in `crates/aureline-shell` is the authoritative gate. Raw secret values and
private endpoints never cross this boundary.
