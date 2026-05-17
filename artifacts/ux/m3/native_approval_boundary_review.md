# Native Approval Boundary Review

This review records the current beta evidence that embedded surfaces do
not own high-risk approval UI. The source of truth is the toolkit fixture:

[`fixtures/ux/m3/embedded_boundary/page.json`](../../../fixtures/ux/m3/embedded_boundary/page.json)

## Evidence

| Check | Evidence |
| --- | --- |
| Every embedded surface has a native approval owner | `native_approval_owner_token = host_product_native` on every toolkit row |
| High-risk approval surfaces stay host-owned | `native_approval_surface_tokens` includes all six reserved surfaces on every row |
| Event log reconstructs the owner split | `native_approval_fence_confirmed` events record embedded owner, origin, native owner, and support row |
| Support export preserves the same tokens | `support_export.rows[*].native_approval_surface_tokens` matches live toolkit rows |
| Seed has no defects | `fixtures/ux/m3/embedded_boundary/defects.json` is `[]` |

## Reserved Native Surfaces

- `product_security_messaging`
- `update_verification`
- `workspace_trust_elevation`
- `rollback_or_restore_confirmation`
- `ai_apply_review`
- `high_risk_approval_sheet`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- validate
cargo test -p aureline-shell --test embedded_boundary_toolkit_fixtures
```

The failure drills in the Rust tests drop browser handoff events, disable
the auth default, or drift support export fields. Each mutation must produce
a typed toolkit defect before this review can be considered current.
