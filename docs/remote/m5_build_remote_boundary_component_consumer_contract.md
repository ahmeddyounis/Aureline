# M5 build/remote-boundary component consumer contract (M05-1082)

This is the consumer-adoption contract for the frozen M5 build/remote-boundary
component matrix (M05-1076) and its four B128 implement lanes (M05-1077 through
M05-1080). It proves that the eight reusable build / remote / managed-workspace
component families are adopted as **primitives** across the claimed M5 execution
and export surfaces, rather than being reinvented as per-feature build/remote
chrome.

- **Crate:** `aureline-remote`
- **Module:**
  `wire_run_test_debug_notebook_preview_ai_companion_and_support_consumers_so_build_and_remote_boundary_components_keep_one_vocabulary_across_claimed_m5_execution_and_export_surfaces`
- **Schema:** `schemas/ui/m5-build-remote-boundary-component-consumer.schema.json`
- **Release proof:**
  `artifacts/release/m5-build-remote-boundary-component-consumer-proof/`
- **Fixtures:** `fixtures/ui/m5-build-remote-boundary-component-consumers/`

## Component families and controls lanes

The eight frozen families group into the four B128 controls contracts. Every
consumer must point back to the one canonical family (its per-family matrix
schema) and the one canonical controls lane, never a feature-local clone.

| Controls lane | Component families |
| --- | --- |
| `adapter_discovery` | adapter-confidence chip, discovery-diff card |
| `host_origin` | host-boundary strip, execution-origin receipt row |
| `managed_lifecycle` | managed-workspace lifecycle card, suspend/resume/rebuild review sheet |
| `expiry_continuation` | workspace-expiry banner, local-safe continuation card |

## Consumer classes

Six claimed M5 execution / export consumer classes each adopt at least one
canonical family:

1. **run / test / debug** — the first claimed execution consumer (AC1 anchor).
2. **notebook**
3. **preview**
4. **AI tool routing**
5. **companion handoff**
6. **support / export + release packet** (incident / diagnostics + export; AC2).

Future execution surfaces must register a row against this shared boundary matrix
before claiming parity: a new surface is audited against the one shared component
registry rather than a bespoke translation table.

## Preserved truth pillars

Every consumer — even a read-only, inspect-only, export-only, or incident replay
— keeps the identical controlled labels and the identical frozen
boundary-disposition vocabulary:

- `adapter_confidence`
- `discovery_drift`
- `host_boundary`
- `execution_origin`
- `lifecycle_state`
- `persistence_class`
- `continuity`
- `expiry_timing`
- `local_safe_continuation`

A narrower consumer discloses the reduction with a reduced-capability banner (and,
when it punts to another surface, a desktop / companion / browser / support-packet
note) rather than renaming or dropping governed boundary truth.

## Guardrails (must all stay false per row)

1. `implies_exact_continuity_after_material_change` — a reused card must never
   imply exact continuity when the target identity, image, template, or
   persistence class changed materially.
2. `hides_local_safe_or_companion_handoff_in_overflow_only` — local-safe
   continuation and browser/companion handoff are never buried behind
   overflow-only affordances.
3. `lower_confidence_overwrites_resolved_target_without_review` — lower-confidence
   discovery never overwrites a higher-confidence resolved target without an
   explicit review state.

## Acceptance criteria

- **AC1** — the first claimed consumers all render the same host, confidence,
  lifecycle, and continuation language (one vocabulary, one component family).
- **AC2** — support / export and release artifacts no longer need feature-local
  translation tables for managed-workspace lifecycle and host-boundary state.
- New execution consumers can be audited against one shared component registry.

## Metadata-only boundary

The packet carries only typed class tokens, opaque boundary-state refs, booleans,
and redacted labels. Raw provider tokens, credential material, and bearer secrets
never cross this boundary.

## Regenerating the proof

```
GEN_BUILD_REMOTE_BOUNDARY_CONSUMER_ARTIFACTS=1 \
  cargo test -p aureline-remote generate_artifacts
```
