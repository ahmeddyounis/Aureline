# M5 content-design certification

This page is the human entry point for the **content-design certification**: the
capstone lane that certifies, for every governed M5 wording object, that its
protected wording is currently proven — fresh proof, in-parity controlled terms,
and complete content-ops metadata — and that any object whose proof is stale,
whose wording drifted, or whose metadata is missing is **auto-narrowed** before it
can keep a Stable wording claim.

The product treats writing principles, controlled vocabulary, action-label and
error-copy patterns, AI copy guardrails, content-ops metadata, and
commercial-boundary honesty as governed contracts, not copy polish. Each of those
concepts already has its own frozen catalog and proof lane; the frozen
[content-wording matrix][matrix] enumerates them as eight governed object kinds.
This certification is the final row over all of them.

## What it certifies

One certification row per governed wording object, each carrying a derived
green/yellow/red content-truth status:

| Object kind | Protected concept |
| ----------- | ----------------- |
| `safety_critical_ui_string` | Safety-critical UI strings (stable message ids, controlled terms) |
| `glossary_term` | Controlled glossary / state-label terms |
| `action_label_pattern` | Verb-first, scope-honest action labels |
| `error_recovery_block` | What-failed / why / what-still-works / next-action copy |
| `ai_copy_guardrail` | AI copy guardrails (certainty, evidence, autonomy language) |
| `count_scope_phrase_set` | Count / scope / freshness language |
| `content_ops_artifact` | Content-ops metadata on docs/help/export/screenshot artifacts |
| `commercial_boundary_wording` | Hosted / managed / self-hosted / open boundary wording |

## How the status is derived (auto-narrowing)

The status is **derived, never asserted**. The builder recomputes it from each
object's proof freshness, copy-parity, and content-ops metadata posture:

- **green** — the protected wording is currently proven at full standing: Stable
  qualification, proven-current proof, in-parity wording, complete metadata.
- **yellow** — a *disclosed* narrowing: the object is qualified below Stable in the
  frozen matrix, runs on a disclosed cache / warming / waivered-stale proof,
  discloses a wording drift across surfaces, or carries partial content-ops
  metadata. A yellow row stays publishable but markets only the narrowed story.
- **red** — blocked: the object hides a wording drift, lost its content-ops
  metadata, claims Stable on stale / unverified / unbacked proof with no waiver.
  A red row may not keep a marketed wording claim until it is repaired.

This is the guarantee the milestone requires: a marketed wording row cannot keep a
green content-truth claim once its underlying proof goes stale, missing, or
drifted — feature behavior passing does not keep wording green.

## Waivers and exact stale-proof causes

A disclosed wording drift, or a Stable object running on stale proof, may stay
**yellow** (rather than red) only when an active, time-bounded
[`ContentCertificationWaiver`][waiver] discloses it — a waiver never lets a hidden
drift, missing metadata, or unbacked claim hide. Every narrowing records its exact
[`StaleProofCause`][cause] (the frozen downgrade trigger that fired and whether it
is disclosed), so the release packet names the precise reason each row is not
green.

## Wiring into release / public-truth automation

The lane exports two records the automation consumes:

- the release packet
  [`artifacts/release/m5-content-design-certification/m5_content_design_certification.md`](../../artifacts/release/m5-content-design-certification/m5_content_design_certification.md)
  (plus its `support_export.json`), which names the current green/yellow/red rows,
  the active waivers, and the exact stale-proof causes; and
- the content-truth dashboard
  [`artifacts/content/m5-content-truth-dashboard.json`](../../artifacts/content/m5-content-truth-dashboard.json),
  the light projection release / public-truth automation reads to auto-narrow
  marketed wording rows the moment evidence, copy-parity, or metadata freshness
  falls out of policy. The packet's `public_truth_refs` name the automation hooks.

## Canonical artifacts

| Artifact | Path |
| -------- | ---- |
| Typed source | `crates/aureline-shell/src/content_design_certification/mod.rs` |
| Headless emitter | `crates/aureline-shell/src/bin/aureline_shell_m5_content_design_certification.rs` |
| Boundary schema | `schemas/release/m5-content-design-certification.schema.json` |
| Packet fixture | `fixtures/release/m5-content-design-certification/packet.json` |
| Dashboard fixture | `fixtures/release/m5-content-design-certification/dashboard.json` |
| Support-export fixture | `fixtures/release/m5-content-design-certification/support_export.json` |
| Published report | `artifacts/release/m5-content-design-certification/m5_content_design_certification.md` |
| Published dashboard | `artifacts/content/m5-content-truth-dashboard.json` |
| CI gate | `tools/ci/m5/content_design_certification_check.py` |

The frozen [content-wording matrix][matrix] remains the canonical inventory of
governed wording objects; this certification certifies that matrix and mints no
parallel wording vocabulary.

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_content_design_certification -- validate
cargo test -p aureline-shell --test m5_content_design_certification_fixtures
python3 tools/ci/m5/content_design_certification_check.py --repo-root .
```

Regenerate the published artifacts and fixtures (the headless emitter is the only
mint-from-truth path):

```sh
BIN="aureline_shell_m5_content_design_certification"
cargo run -q -p aureline-shell --bin "$BIN" -- markdown        > artifacts/release/m5-content-design-certification/m5_content_design_certification.md
cargo run -q -p aureline-shell --bin "$BIN" -- support-export  > artifacts/release/m5-content-design-certification/support_export.json
cargo run -q -p aureline-shell --bin "$BIN" -- dashboard       > artifacts/content/m5-content-truth-dashboard.json
cargo run -q -p aureline-shell --bin "$BIN" -- packet          > fixtures/release/m5-content-design-certification/packet.json
cargo run -q -p aureline-shell --bin "$BIN" -- dashboard       > fixtures/release/m5-content-design-certification/dashboard.json
cargo run -q -p aureline-shell --bin "$BIN" -- support-export  > fixtures/release/m5-content-design-certification/support_export.json
cargo run -q -p aureline-shell --bin "$BIN" -- compact         > fixtures/release/m5-content-design-certification/compact.txt
```

[matrix]: ../content/m5/freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md
[waiver]: ../../crates/aureline-shell/src/content_design_certification/mod.rs
[cause]: ../../crates/aureline-shell/src/content_design_certification/mod.rs
