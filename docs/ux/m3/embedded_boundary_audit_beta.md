# Embedded boundary audit (beta)

This corpus turns the embedded-surface boundary toolkit into a regression-gated proof system. Every claimed embedded beta surface — docs/help panes, marketplace/account pages, provider-owned webviews, service dashboards, and auth-handoff sheets — has a worked drill that proves the host shell keeps the boundary honest, plus an adversarial drill that proves the gate rejects spoofing, flattening, approval bypass, and authority widening.

It is minted from `crates/aureline-shell/src/embedded_boundary_corpus/mod.rs` and replayed by `crates/aureline-shell/tests/embedded_boundary_corpus_fixtures.rs`. The boundary vocabulary schema of record is `schemas/ux/embedded_surface_boundary.schema.json`; the per-row audit validator is reused from the beta audit lane so a drill cannot drift from what ships.

## What every conformant surface must prove

1. **Owner / origin / publisher disclosure.** The host shell, not the embedded body, paints the owner, publisher/service, and origin — never hidden behind hover, scroll, or the embedded body.
2. **System-browser-first identity.** Auth and risky web surfaces prefer the system browser, offer device-code as the auditable fallback, and never collect a password inside the card.
3. **Honest failure naming.** Certificate failure, managed-policy deny, cross-origin limitation, stale snapshot, and offline snapshot are named in product, not rendered as a broken or blank page.
4. **Open-in-browser truth.** The fallback keeps its return target and reason, preserves object identity, and never widens in-product authority past the product-owned command.
5. **Host-owned approvals.** The six native-reserved surfaces stay host-owned and survive an app restart and a surface re-entry; the embedded body never inherits them.
6. **Support-export parity.** The support export reconstructs owner, boundary state, and approval origin from stable tokens with no raw payload.

## Drill coverage

| Drill | Cases |
| ----- | ----- |
| `owner_origin_verified` | 2 |
| `system_browser_first_auth` | 1 |
| `certificate_failure` | 1 |
| `managed_policy_deny` | 1 |
| `cross_origin_limitation` | 1 |
| `stale_snapshot` | 1 |
| `offline_snapshot` | 1 |
| `open_in_browser_fallback` | 4 |
| `device_code_fallback` | 1 |
| `native_approval_fence_persists_restart` | 1 |
| `native_approval_fence_persists_reentry` | 1 |

## Denial coverage

Each denial drill ships an adversarial row that the gate must reject. The case records the exact reason token the gate produces, so a regression that lets the behavior through fails the fixture replay.

| Denial | Cases | Proven by |
| ------ | ----- | --------- |
| `owner_origin_spoof` | 1 | `missing_owner_label` |
| `stale_masked_as_live` | 1 | `boundary_state_inconsistent_with_origin_verification` |
| `native_trust_chrome_spoof` | 1 | `embedded_minted_native_reserved_surface` |
| `approval_bypass` | 1 | `embedded_minted_native_reserved_surface` |
| `authority_widening` | 1 | `fallback_widens_authority_beyond_native` |
| `embedded_password_collection` | 1 | `embedded_auth_exception_missing_exception_ref` |
| `support_export_flattening` | 1 | `support_row_vocabulary_drift` |
| `browser_fallback_drops_target_or_reason` | 1 | `open_in_browser_drops_target_or_reason` |

## How to read a case

Each case in `fixtures/ux/m3/embedded_boundary_corpus/corpus_cases.json` carries the audited row, the support row, the authority pair (native vs fallback), the open-in-browser fallback truth, the lifecycle persistence snapshots (for approval-fence drills), the expected verdict, and the denial-reason tokens the gate actually produced. A conformant case must produce zero denial reasons; a denial case must produce at least the reasons it names.

## Regenerate and verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- packet         > fixtures/ux/m3/embedded_boundary_corpus/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- cases          > fixtures/ux/m3/embedded_boundary_corpus/corpus_cases.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- matrix-json     > fixtures/ux/m3/embedded_boundary_corpus/matrix.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- support-export  > fixtures/ux/m3/embedded_boundary_corpus/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- report-md       > artifacts/ux/m3/embedded_boundary_audit_report.md
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- doc-md          > docs/ux/m3/embedded_boundary_audit_beta.md
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_corpus -- validate
cargo test -p aureline-shell --test embedded_boundary_corpus_fixtures
```
