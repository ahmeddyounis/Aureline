# M5 advisory-component release-proof contract

Task: **M05-771** — Certify advisory, emergency-notice, affected-install, and disclosure-link truth on every claimed M5 channel, install topology, and mirror/offline profile.

This lane is the **release-evidence certification capstone** for the frozen M5 advisory-component model. The matrix freezes the governed user-facing security surfaces; this proof certifies that those surfaces keep the same advisory truth across release, help/about, support, notification, activity-center, marketplace/update, and mirror/offline evidence paths.

The packet is intentionally narrow. It does not redesign incident management or the release pipeline. It decides whether the first claimed advisory consumers can keep their M5 claim, and it gives release/help/support one bundle to point at when advisory, emergency, affected-install, disclosure, and revocation truth is reviewed.

## Governed component families

The proof covers exactly the six advisory component families frozen by the M5 advisory-component matrix:

- `advisory_card` — Security-advisory card
- `emergency_notice` — Emergency notice
- `affected_install_panel` — Affected-install panel
- `disclosure_block` — Disclosure / history block
- `advisory_activity_row` — Advisory activity row
- `native_notification_handoff` — Native-notification handoff

Each row pulls its severity classes, advisory anatomy, action and dismissal states, continuity claims, delivery and freshness states, disclosure and export fields, notification behaviors, projection surfaces, accessibility routes, required labels, shell zone, responsive classes, window classes, surface families, consumer surfaces, downgrade triggers, owner role, scope summary, and qualification from the frozen matrix row for that family.

## Truth pillars

The bundle refuses to ship unless the union of certified families covers the whole advisory track invariant:

- `affected_scope_exposure_and_continuity` — affected object, severity, current exposure, fixed version or mitigation, signer/source state, primary actions, and what still works locally.
- `emergency_blast_radius_and_forced_disable` — blast radius, acknowledge/snooze/dismiss rules, and forced-disable scope.
- `disclosure_provenance_and_history` — copy-safe advisory/CVE/GHSA ids, disclosure path, provenance, and resolved-versus-active history.

## Derived status and auto-narrowing

Each family is certified across four posture axes: `advisory_contract_truth`, `cross_channel_parity`, `support_export_proof`, and `proof_freshness`.

The green/yellow/red status is **derived, never asserted**:

- `red` when advisory truth collapses or drifts, channel grammar diverges, support capture omits the truth, exported proof is stale or divergent, advisory truth is hidden off a claimed channel, a claimed M5 surface family is uncertified, or the row declares no truth pillar.
- `yellow` when the row honestly discloses a reduced advisory detail, a waiver-backed reduced channel projection, a partial support-export capture, or a partial proof refresh.
- `green` when all four axes are fully certified and the row has no narrowing.

The affected-install panel currently carries a disclosed reduced-channel-projection waiver, the disclosure block carries a disclosed partial support capture, and the advisory activity row carries a disclosed partial proof refresh. Those rows are publishable but narrowed below green. Any stale proof, unsigned distribution, mirror lag, or hidden continuity state that is not disclosed becomes a blocker instead of staying green.

## Boundary and artifacts

The boundary schema is `schemas/security/m5-advisory-component-release-proof.schema.json`. The records carry stable ids, closed vocabulary, counts, refs, and short labels only; raw URLs, raw local paths, hostnames, usernames, tokens, and credentials are rejected by the Rust validator.

The headless emitter `aureline_shell_m5_advisory_component_release_proof` is the only mint-from-truth path:

```sh
BIN=aureline_shell_m5_advisory_component_release_proof

cargo run -q -p aureline-shell --bin $BIN -- packet > artifacts/release/m5-advisory-component-release-proof/packet.json
cargo run -q -p aureline-shell --bin $BIN -- dashboard > artifacts/release/m5-advisory-component-release-proof/dashboard.json
cargo run -q -p aureline-shell --bin $BIN -- support-export > artifacts/release/m5-advisory-component-release-proof/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- csv > artifacts/release/m5-advisory-component-release-proof/matrix.csv
cargo run -q -p aureline-shell --bin $BIN -- markdown > artifacts/security/m5-advisory-component-release-proof.md

cargo run -q -p aureline-shell --bin $BIN -- packet > fixtures/security/m5-advisory-component-release-proof/packet.json
cargo run -q -p aureline-shell --bin $BIN -- dashboard > fixtures/security/m5-advisory-component-release-proof/dashboard.json
cargo run -q -p aureline-shell --bin $BIN -- support-export > fixtures/security/m5-advisory-component-release-proof/support_export.json
cargo run -q -p aureline-shell --bin $BIN -- compact > fixtures/security/m5-advisory-component-release-proof/compact.txt
```

## Companion artifacts

- Markdown report: `artifacts/security/m5-advisory-component-release-proof.md`
- Published packet: `artifacts/release/m5-advisory-component-release-proof/packet.json`
- Published dashboard: `artifacts/release/m5-advisory-component-release-proof/dashboard.json`
- Published support export: `artifacts/release/m5-advisory-component-release-proof/support_export.json`
- Published CSV: `artifacts/release/m5-advisory-component-release-proof/matrix.csv`
- Protected fixture packet: `fixtures/security/m5-advisory-component-release-proof/packet.json`
- Boundary schema: `schemas/security/m5-advisory-component-release-proof.schema.json`
- Frozen advisory matrix: `schemas/security/m5-advisory-component-matrix.schema.json`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_component_release_proof -- validate
cargo test -p aureline-shell --lib m5_advisory_component_release_proof
cargo test -p aureline-shell --test m5_advisory_component_release_proof_fixtures
```
