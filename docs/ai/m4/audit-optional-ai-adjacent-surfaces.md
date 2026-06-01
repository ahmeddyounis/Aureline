# Audit optional AI-adjacent surfaces

This stable lane enforces that every optional AI-adjacent surface exposed in a
promoted build either carries its own current qualification proof, or is
visibly labeled below Stable in product, docs/help, and release packets. The
runtime owner is `aureline_ai::audit_optional_ai_adjacent_surfaces`.

Optional surfaces — notebook, voice, browser companion, preview/designer, and
background branch automation — may not inherit Stable simply because the core
composer and patch lanes are green. Each exposed lane requires its own
qualification matrix, downgrade rule, and boundary audit.

## Surface families

| Family | Default qualification | Key requirement |
|---|---|---|
| `notebook` | Limited | Explicit document/kernel/output trust labeling; kernel state consistency; debugger-bridge labels; output sandbox cues |
| `voice` | Experimental | Explicit consent; declared capture boundary; local-vs-retained transcript posture; disable path; accessibility-safe fallback |
| `browser_companion` | Limited | Scope limited to review/docs/light-handoff or source-first inspect; no native-depth authority; no silent write-back; no hidden runtime mutation |
| `preview_designer` | Limited | Scope honesty; no native-depth authority; no silent write-back; no hidden runtime mutation; write-capable sub-surfaces carry separate proof |
| `background_branch_automation` | Limited | Trust boundary explicit; cross-workspace job scope requires its own graduated proof |

## Qualification labels

The lane supports five qualification labels. Only `stable` requires no visible
below-stable label in product, docs/help, or release packets.

| Label | Meaning |
|---|---|
| `stable` | Lane has passed all stable-gate requirements under its own packet |
| `limited` | Production-ready with reduced feature scope |
| `preview` | Available to early adopters with explicit opt-in |
| `experimental` | Exploratory; not production-recommended |
| `unsupported` | Not supported in the shipped build |

## Downgrade automation

Downgrade automation fires automatically when:

- a qualification packet passes its freshness deadline (`packet_freshness_expired`);
- route truth regresses (`route_truth_regressed`); or
- support/export parity is missing for the lane (`support_export_parity_missing`).

The downgrade state propagates into product copy, docs/help, release packets,
and compatibility reports. No optional lane continues to read as Stable after
its packet expires or fails.

## Propagation contract

Help/About, docs, marketplace, CLI/headless inspect, and support-export surfaces
must all consume family-specific qualification state. No consumer may collapse
unqualified optional surfaces back into one optimistic "available in build"
stable label.

## Contract

The packet does **not** re-derive core AI graduation, route/spend receipts, or
mutation evidence. The core AI graduation packets under
`artifacts/ai/m4/publish_stable_ai_graduation_packets/` remain canonical for
the core composer, patch, and review surfaces. This audit adds the optional-lane
qualification matrix that the core graduation explicitly does not cover.

The packet carries refs, stable class tokens, booleans, and review labels only.
Raw prompts, provider payloads, endpoint URLs, credentials, and signing-key
material stay outside the boundary.

## Required behavior

`OptionalAiAdjacentSurfaceAuditPacket::validate` rejects a packet when:

- `record_kind` or `schema_version` do not match the expected constants;
- `surface_rows` is empty;
- any required surface family (`notebook`, `voice`, `browser_companion`,
  `preview_designer`, `background_branch_automation`) is absent;
- a surface with qualification other than `stable` is missing a visible
  below-stable label;
- a surface claims `stable` but has no own qualification packet;
- a surface claims stable inheritance from core AI graduation (freeloading);
- a notebook surface is missing or has unsatisfied `notebook_requirements`;
- a voice surface is missing or has unsatisfied `voice_requirements`;
- a browser companion surface is missing or has unsatisfied
  `browser_companion_requirements`;
- a preview/designer surface is missing or has unsatisfied
  `preview_designer_requirements`;
- downgrade automation does not fire on packet freshness expiry, route truth
  regression, or missing support/export parity;
- downgrade state does not propagate to at least one required consumer; or
- any consumer is not consuming family-specific qualification state, or an
  optimistic "available in build" stable collapse is allowed.

## Truth source

The checked artifact at
`artifacts/ai/m4/audit-optional-ai-adjacent-surfaces/support_export.json` is
canonical for this lane. Dashboards, docs, Help/About surfaces, and support
exports should ingest it instead of cloning status text. The boundary schema
is `schemas/ai/optional-ai-surface-qualification.schema.json`; the protected
fixture directory is `fixtures/ai/m4/audit-optional-ai-adjacent-surfaces/`.

Verify the checked packet with:

```sh
cargo test -p aureline-ai audit_optional_ai_adjacent_surfaces
```
