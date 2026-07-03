# M5 advisory-component release proof

Generated from the seeded packet in
[`crate::m5_advisory_component_release_proof`](../../crates/aureline-shell/src/m5_advisory_component_release_proof/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_component_release_proof -- markdown > \
  artifacts/security/m5-advisory-component-release-proof.md
```

- Packet id: `m5-advisory-component-release-proof:stable:0001`
- Source schema ref: `schemas/security/m5-advisory-component-release-proof.schema.json`
- Certifies matrix packet: `m5-advisory-components:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 6
- Green: 3
- Yellow (auto-narrowed): 3
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-07-03T00:00:00Z`

## Certification dimensions

- `advisory_contract_truth`
- `cross_channel_parity`
- `support_export_proof`
- `proof_freshness`

## Truth pillars

- `affected_scope_exposure_and_continuity`
- `emergency_blast_radius_and_forced_disable`
- `disclosure_provenance_and_history`

## Certification rows

| Component family | Status | Qualification | Advisory truth | Cross-channel parity | Support-export | Proof freshness | No-hidden-advisory-truth | Waiver |
| ---------------- | ------ | ------------- | -------------- | -------------------- | -------------- | --------------- | ------------------------ | ------ |
| Security-advisory card | `green` | `stable` | `advisory_truth_certified_every_channel` | `parity_certified_across_channels` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |
| Emergency notice | `green` | `stable` | `advisory_truth_certified_every_channel` | `parity_certified_across_channels` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |
| Affected-install panel | `yellow` | `stable` | `advisory_truth_certified_every_channel` | `disclosed_reduced_channel_projection` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | `waiver:reduced-channel-projection:0001` |
| Disclosure / history block | `yellow` | `stable` | `advisory_truth_certified_every_channel` | `parity_certified_across_channels` | `disclosed_partial_capture` | `exported_proof_fresh_and_current` | `true` | — |
| Advisory activity row | `yellow` | `stable` | `advisory_truth_certified_every_channel` | `parity_certified_across_channels` | `reconstructable_in_export_and_screenshot` | `disclosed_partial_refresh` | `true` | — |
| Native-notification handoff | `green` | `stable` | `advisory_truth_certified_every_channel` | `parity_certified_across_channels` | `reconstructable_in_export_and_screenshot` | `exported_proof_fresh_and_current` | `true` | — |

## Auto-narrowed rows

- `affected_install_panel` (`yellow`) — The affected-install panel serves a disclosedly reduced channel projection on the most compact secondary channel (a collapsed exposure summary in place of the full per-lane install breakdown) while the shared row grammar, severity vocabulary, mirror-freshness state, and local-continuity claim stay identical to the primary channel; the reduction is disclosed behind a waiver, so the row is narrowed below green while it is in force.
- `disclosure_block` (`yellow`) — Under the seeded release the disclosure/history block reconstructs its copy-safe advisory/CVE/GHSA ids, disclosure path, and resolved-versus-active history from the support export and screenshot baselines but discloses a partial capture of some low-priority provenance annotation while the export queue is throttled; the partial capture is disclosed and the row is narrowed below green.
- `advisory_activity_row` (`yellow`) — Under the seeded release the advisory activity row's exported proof is refreshed for every mandatory export field but discloses a partial refresh of a low-priority disclosure-visibility annotation that awaits the next scheduled refresh while the current claim stays backed; the partial refresh is disclosed and the row is narrowed below green.

## Exact certification causes

- `affected_install_panel` — `affected_scope_hidden` (disclosed: `true`) — A channel projection is disclosedly reduced (a compact secondary channel shows a summarized projection) while the shared row grammar is preserved; the reduction is disclosed behind a waiver and the row is narrowed below green.
- `disclosure_block` — `proof_stale` (disclosed: `true`) — The support export reconstructs the advisory truth and discloses a partial capture (some low-priority advisory detail is trimmed) while the reduction is disclosed and the row is narrowed below green.
- `advisory_activity_row` — `proof_stale` (disclosed: `true`) — The exported proof is refreshed and discloses a partial refresh (a low-priority slice awaits the next refresh) while the current claim stays backed; the reduction is disclosed and the row is narrowed below green.

## Active waivers

- `waiver:reduced-channel-projection:0001` (`affected_install_panel`, owner: Install/update component owner, expires `2026-10-31T00:00:00Z`) — Under the seeded release the affected-install panel shows a disclosedly summarized projection on the most compact secondary channel (a collapsed exposure summary in place of the full per-lane install breakdown) while the shared row grammar, the severity vocabulary, the mirror-freshness state, and the local-continuity claim stay identical to the primary channel. The narrowing is disclosed, never hides an affected lane or the mirror-lag state, and keeps one row grammar.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_component_release_proof -- validate
cargo test -p aureline-shell --test m5_advisory_component_release_proof_fixtures
```
