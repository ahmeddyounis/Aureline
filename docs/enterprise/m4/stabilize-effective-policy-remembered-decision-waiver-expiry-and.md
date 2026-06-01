# Stabilize effective policy, remembered-decision, waiver-expiry, and exception-preview UX

This stable lane makes the effective-policy preview surface — including
exception and waiver expiry, remembered-decision drift, and exception-preview
UX links — visible and verifiable enough that product, security review, support
export, and release packets can all explain: what the effective policy is for a
pending change, which exceptions or waivers apply and when they expire, what
remembered decisions affect the result, whether any drift has been explained,
and where the exception-preview UX can route a reviewer. The runtime owner is
`aureline_policy::stabilize_effective_policy_remembered_decision_waiver_expiry_and`.

The packet does **not** re-derive policy bundle bodies, raw rule text, or raw
identity truth. The `aureline_policy::simulation` beta audit remains canonical
for its own slice. This packet re-exports those qualification tokens verbatim
and adds the stability invariants needed for a single evidence packet.

## Contract

For the stable claim to hold, **all six** of the following conditions must be
verified simultaneously:

1. **Upstream beta page clean** — `aureline_policy::simulation::audit_policy_simulation_beta_page` returns zero defects.
2. **Required change classes covered** — both `policy_bundle_change` and `settings_lock_change` have at least one simulation with complete affected-surface truth (personas, action families, degraded modes, protected-path changes).
3. **Exceptions bounded and attributable** — every exception or waiver has an explicit expiry horizon, a named renewal path, a revocation path, and an owner; dashboard buckets reflect current lifecycle status.
4. **Remembered-decision drift explained** — every remembered decision in a drifted, expired, or force-retired state names at least one typed `RememberedDecisionDriftReason` in `invalidation_reasons`.
5. **Exception-preview links present** — when exceptions exist in the page, at least one simulation links them via `exception_preview_refs` so the UX layer can populate previews from typed records rather than cloned status text.
6. **Action-time policy truth preserved** — all `action_time_policy_states` carry `preserves_historical_truth: true`, so support and admin exports never overwrite historical truth with current-only truth.

## Required behavior

`validate_effective_policy_stabilize_page` rejects a page when its `defects` list is non-empty.

`audit_effective_policy_stabilize_page` runs the combined check and returns a
typed `Vec<EffectivePolicyStabilizeDefect>`. Each defect carries a closed
`narrow_reason_token` and an export-safe `note`. The absence of defects is the
stable claim.

One condition forces `Withdrawn` immediately and cannot be overridden:

- A `RawPrivateMaterialExposed` defect in the upstream beta page (narrow reason:
  `raw_private_material_exposed`). The function returns immediately with this
  single defect and skips all other checks.

A missing required change class narrows to `Preview` rather than `Beta` because
the coverage gap prevents any verifiable claim for that class.

## Boundary

The following material stays outside this packet's support boundary:

- Raw policy bundle bodies or raw rule text.
- Raw identities, raw hostnames, raw file paths, raw extension ids.
- Raw credentials or secret material.
- Raw exception justification text (`raw_justification_excluded` is always `true` in exported records).

Every exported field carries either a closed-vocabulary token, a plain-language
label, an opaque ref, a count, or a schema-version integer.

## Truth source

| Slice | Canonical source |
|-------|-----------------|
| Policy simulation posture | `aureline_policy::simulation` |
| Exception and waiver state | `aureline_policy::simulation` (embedded) |
| Remembered-decision drift | `aureline_policy::simulation` (embedded) |
| Stable qualification | this module (derived from all of the above) |
| Artifact evidence | `artifacts/enterprise/m4/stabilize-effective-policy-remembered-decision-waiver-expiry-and.md` |

## Verify

```bash
# Build
cargo build -p aureline-policy

# Tests
cargo test -p aureline-policy -- stabilize_effective_policy
```

All tests under `stabilize_effective_policy_remembered_decision_waiver_expiry_and::tests` must pass.
`seeded_effective_policy_stabilize_page()` must produce zero defects and a `stable` overall
qualification token.
