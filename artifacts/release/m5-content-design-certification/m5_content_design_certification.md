# M5 content-design certification

Generated from the seeded packet in
[`crate::content_design_certification`](../../../crates/aureline-shell/src/content_design_certification/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- markdown > \
  artifacts/release/m5-content-design-certification/m5_content_design_certification.md
```

- Packet id: `m5-content-design-certification:stable:0001`
- Source schema ref: `schemas/release/m5-content-design-certification.schema.json`
- Certifies matrix packet: `m5-content-wording-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Rows certified: 8
- Green (fully proven): 6
- Yellow (auto-narrowed): 2
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-26T00:00:00Z`

## Content-truth rows

| Protected concept | Status | Qualification | Proof freshness | Copy parity | Metadata | Waiver |
| ----------------- | ------ | ------------- | --------------- | ----------- | -------- | ------ |
| Safety-critical UI strings | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| Controlled glossary terms | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| Verb-first action labels | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| Error / recovery copy | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| AI copy guardrails | `yellow` | `beta` | `proven_current` | `in_parity` | `complete` | — |
| Count / scope language | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| Content-ops metadata | `green` | `stable` | `proven_current` | `in_parity` | `complete` | — |
| Commercial-boundary wording | `yellow` | `beta` | `proven_current` | `disclosed_drift` | `complete` | `waiver:content-boundary-copy-sync:0001` |

## Auto-narrowed rows

- `ai_copy_guardrail` (`yellow`) — AI copy guardrail is qualified at Beta in the frozen content matrix; AI wording ships with a disclosed Low confidence / Review required posture and is narrowed below a Stable wording claim.
- `commercial_boundary_wording` (`yellow`) — Commercial-boundary wording is qualified at Beta; the marketplace upgrade prompt and Help/About disclose a known hosted/managed boundary phrasing difference that is waivered pending the next cross-surface copy sync.

## Exact stale-proof causes

- `ai_copy_guardrail` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen content-wording matrix qualifies this object at `beta`, below a Stable wording claim.
- `commercial_boundary_wording` — `upstream_dependency_narrowed` (disclosed: `true`) — Frozen content-wording matrix qualifies this object at `beta`, below a Stable wording claim.
- `commercial_boundary_wording` — `commercial_boundary_drift` (disclosed: `true`) — Disclosed wording drift across consumer surfaces, held under a waiver.

## Active waivers

- `waiver:content-boundary-copy-sync:0001` (`commercial_boundary_wording`, owner: Commercial boundary owner, expires `2026-09-30T00:00:00Z`) — The marketplace upgrade prompt and Help/About disclose the same hosted/managed boundary, but the exact phrasing is being unified in the next cross-surface copy sync. The difference is disclosed, never hidden.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- validate
cargo test -p aureline-shell --test m5_content_design_certification_fixtures
python3 tools/ci/m5/content_design_certification_check.py --repo-root .
```
