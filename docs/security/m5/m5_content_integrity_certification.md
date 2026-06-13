# M5 Content-Integrity, Safe-Preview, and Representation-Honesty Certification

This document is the contract for the frozen M5 certification that binds the
per-dimension content-safety proofs to every claimed M5 artifact and viewer
family and auto-narrows any family that lacks a current proof on a required
dimension. The certification is the canonical M5 control source for this lane:
shiproom, docs, and support surfaces ingest the checked-in packet rather than
publishing their own viewer trust-status text.

- Record kind: `certify_m5_content_integrity_safe_preview_and_representation_honesty`
- Schema: [`schemas/security/m5-content-integrity-certification.schema.json`](../../../schemas/security/m5-content-integrity-certification.schema.json)
- Canonical support export: [`artifacts/security/m5/m5_content_integrity_certification/support_export.json`](../../../artifacts/security/m5/m5_content_integrity_certification/support_export.json)
- Summary artifact: [`artifacts/security/m5/m5_content_integrity_certification.md`](../../../artifacts/security/m5/m5_content_integrity_certification.md)
- Fixtures: [`fixtures/security/m5/m5_content_integrity_certification/`](../../../fixtures/security/m5/m5_content_integrity_certification/)
- Producer: `aureline_content_safety::frozen_m5_content_integrity_certification_packet`
- Headless tool: `m5_content_integrity_certification` (`--markdown`, `--validate <packet.json>`)

## What this certifies

Each earlier M5 content-safety lane proves one dimension of the track invariant.
This certification ingests those packets by id and certifies, per family, that
every required dimension holds with a current proof:

| Dimension | Backing proof lane | Upstream packet |
| --- | --- | --- |
| `suspicious_content_cues` | `suspicious_text_parity` | `m5-suspicious-text-parity:stable:0001` |
| `safe_preview_trust_class` | `safe_preview_limited_mode` | `m5-safe-preview-limited-mode:stable:0001` |
| `strong_decision_display` | `trust_decision_identity` | `m5-trust-decision-identity:stable:0001` |
| `raw_rendered_copy_export` | `raw_rendered_handoff` | `m5-raw-rendered-handoff:stable:0001` |
| `active_content_containment` | `trust_class_ladder` | `m5-trust-class-ladder:stable:0001` |
| `silent_rewrite_guard` | `mutation_path_fix_flow` | `m5-mutation-path-fix-flow:stable:0001` |

The family, qualification-class, and consumer-surface vocabularies are reused
from the frozen M5 content-integrity matrix
(`m5-content-integrity-matrix:stable:0001`) so the certification is about the
same claimed families. Raw suspicious bytes, raw rendered trees, raw provider
payloads, credentials, and live preview-origin responses never cross the export
boundary.

## Required dimensions

Every claimed family must certify these four universal dimensions:
`suspicious_content_cues`, `safe_preview_trust_class`, `raw_rendered_copy_export`,
and `active_content_containment`. Strong-decision families (provider overlays,
marketplace install/update, and remote preview targets) must additionally certify
`strong_decision_display`. A dimension marked `not_applicable` carries the
`not_applicable` proof state and never narrows a claim.

## Auto-narrow rule

`project_m5_content_integrity_certification` is the deterministic auto-narrow
rule. A family keeps its claimed qualification only when every required dimension
is `current_pass`:

- A `stale_pass` proof narrows the claim one rung (stable → beta → preview →
  experimental).
- A `missing` proof narrows the claim to `experimental`.
- A `failing` proof holds the family (`held`).

When more than one dimension narrows, the weakest result wins. The certified
qualification, its `narrowing_reasons`, and the packet `summary` counts are
recomputed during validation, so a checked-in row cannot record a certified
qualification that disagrees with its own proofs — regression, stale evidence, or
missing threat-class coverage visibly narrows the affected claim instead of
silently keeping it.

## Guardrails

The certification never normalizes suspicious bytes away to earn a proof, never
lets rendered copy masquerade as raw bytes, and never auto-executes active rich
content to earn a proof. Strong-decision surfaces certify stricter identity
rendering than ordinary browsing panes. These guardrails are encoded as the
`review` block and are required to be `true` for the packet to validate.

## Consumer projection

The `consumer_projection` block records that shiproom, docs, support export, CLI
/ headless, and diagnostics all ingest this single certification result, and that
narrowed families are visibly labeled rather than silently dropped.

## Fixtures

The checked-in fixtures exercise the auto-narrow path while keeping every family
present and every invariant satisfied:

- `notebook_safe_preview_proof_missing.json` — a missing safe-preview proof
  narrows the notebook family to `experimental`.
- `marketplace_strict_identity_stale.json` — a stale strict-identity proof
  narrows the marketplace family one rung from `stable` to `beta`.
