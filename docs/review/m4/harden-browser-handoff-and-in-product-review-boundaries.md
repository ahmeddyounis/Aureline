# Hardened browser handoff and in-product review boundaries with provider/source identity and return paths

**Scope:** Harden browser handoff and in-product review boundaries with provider/source identity and return paths for daily-driver review lanes.

**Status:** Stable review lane — implemented in `crates/aureline-review`.

## Goal

Bind review-workspace browser handoff, provider overlay, stabilization truth, and ownership signals into a single coherent boundary-hardening packet. Every boundary crossing remains explicit about source, freshness, actor, target, and return path so the stable line never hides hosted authority behind local chrome.

## Design principles

1. **Explicit provider/source identity** — Every boundary record carries the exact `provider_class`, `provider_object_identity_ref`, `source_class`, and `actor_ref`. Generic "open in browser" affordances are forbidden.
2. **Typed reversible return paths** — Browser handoff records carry `return_anchor_kind`, `return_anchor_ref`, and `replay_posture`. Missing or expired return paths degrade the boundary explicitly.
3. **No hidden authority** — The `hidden_authority_detected` flag on the in-product review boundary prevents provider mutations from masquerading as local chrome. When detected, the boundary is degraded and mutation authority is blocked.
4. **Separable inspectable truths** — Boundary state, handoff class, authority class, return path class, freshness class, and ownership class are all independent fields.
5. **Boundary ownership split** — Advisory and enforceable ownership signals are modeled as separate records at the boundary. Conflicts degrade the boundary explicitly.
6. **Redaction-safe support export** — Raw URLs and raw provider payloads are explicitly forbidden from crossing the support boundary.

## Record kinds

| Record kind | Purpose |
|---|---|
| `review_boundary_hardening_packet` | Top-level packet consumed by review surfaces and support exports. |
| `review_boundary_hardening_record` | Core boundary hardening binding workspace, stabilization, and boundary state. |
| `review_browser_handoff_boundary_record` | Explicit browser handoff with typed origin, destination, object identity, and reversibility. |
| `review_in_product_review_boundary_record` | In-product review boundary with authority truth and hidden-authority detection. |
| `review_provider_source_identity_record` | Provider and source identity disclosure. |
| `review_return_path_record` | Typed reversible return path with anchor kind, anchor ref, and replay posture. |
| `review_boundary_freshness_observation_record` | Freshness observation at the boundary. |
| `review_boundary_ownership_signal_record` | Ownership signal at the boundary, keeping advisory and enforceable split. |
| `review_boundary_hardening_command_record` | Command-graph operations (preview, approve, refresh, invalidate, handoff, return). |
| `review_boundary_hardening_support_export_packet` | Redaction-safe export with reopen context. |
| `review_boundary_hardening_inspection_record` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Boundary hardening states
- `boundary_hardened`, `boundary_degraded_provider_overlay_stale`, `boundary_degraded_missing_return_path`, `boundary_degraded_hidden_authority`, `boundary_degraded_freshness_unknown`, `boundary_degraded_ownership_ambiguous`

### Handoff boundary classes
- `handoff_reversible_typed`, `handoff_typed_no_return_path`, `handoff_untyped`, `handoff_not_required`, `handoff_authority_revoked`

### Boundary authority classes
- `local_authoritative`, `provider_authoritative`, `local_and_provider_agree`, `local_and_provider_disagree`, `provider_overlay_missing`

### Return path classes
- `return_to_review_anchor`, `return_to_workspace`, `return_to_change_lineage`, `return_to_landing_strip`, `return_path_missing`, `return_path_expired`

### Boundary freshness classes
- `boundary_fresh`, `boundary_stale_within_grace`, `boundary_stale_blocks_mutation`, `boundary_freshness_unknown`

### Boundary ownership classes
- `ownership_advisory_at_boundary`, `ownership_enforceable_at_boundary`, `ownership_conflict_at_boundary`, `ownership_not_disclosed`

## Key invariants

- `boundary_hardened` state is incompatible with `hidden_authority_detected`.
- `reversible` handoff requires `handoff_boundary_class` to be `handoff_reversible_typed`.
- `expired` return path requires `return_path_class` to be `return_path_expired`.
- When both `local_authoritative` and `provider_authoritative` are true, `local_provider_agree` must be true.
- `local_provider_agree` requires at least one authority side to be authoritative.
- Workspace and stabilization packets must share the same `review_workspace_id`.
- All `raw_*_export_allowed` flags in support export must be `false`.
- Boundary ownership signals must be at least one of advisory or enforceable.

## File locations

| Artifact | Path |
|---|---|
| Implementation | `crates/aureline-review/src/harden_browser_handoff_and_in_product_review_boundaries/mod.rs` |
| Schema | `schemas/review/harden_browser_handoff_and_in_product_review_boundaries.schema.json` |
| Fixtures | `fixtures/review/m4/harden-browser-handoff-and-in-product-review-boundaries/` |
| Tests | `crates/aureline-review/tests/harden_browser_handoff_and_in_product_review_boundaries_alpha.rs` |

## Integration with existing lanes

- Consumes [`ReviewWorkspaceBetaPacket`] from the `workspace` module.
- Consumes [`ReviewStabilizationPacket`] from the `stabilize_review_workspace_anchors_stale_base_labels_approval` module.
- Projects into the same inspector/CLI/support-export surfaces as the `landing` and `stabilize_review_workspace_anchors_stale_base_labels_approval` modules.

## Verification

```bash
cargo test -p aureline-review --test harden_browser_handoff_and_in_product_review_boundaries_alpha
```
