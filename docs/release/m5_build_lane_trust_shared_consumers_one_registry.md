# M5 Build-Lane-Trust Shared Consumers: One Registry Across Surfaces

**Status:** Stable · B144 consumer-adoption capstone
**Module:** `aureline_ui::m5_build_lane_trust_shared_consumers_one_registry_across_surfaces`
**Schema:** [`schemas/release/m5-build-lane-trust-shared-consumers.schema.json`](../../schemas/release/m5-build-lane-trust-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-build-lane-trust-shared-consumers-proof/`](../../artifacts/release/m5-build-lane-trust-shared-consumers-proof/)
**Fixtures:** [`fixtures/release/m5-build-lane-trust-shared-consumers/`](../../fixtures/release/m5-build-lane-trust-shared-consumers/)

This lane is the consumer-adoption capstone for the four governed build lanes frozen in the
[build-lane-trust matrix](m5_build_lane_trust_contract.md) and implemented by the build-lane-descriptor /
reproducibility-proof, verified-input / sidecar-completeness, clean-room-rebuild / artifact-diff,
remote-cache-integrity / cache-bypass-drill, and exact-build-symbolication / mirror-offline-parity lanes. It
binds each shared build-lane-trust family to the concrete About / provenance, Help, service-health,
release-center, and support-export consumers — projected through the build-farm, cache-service,
release-center, shiproom, provenance-service, diagnostics, docs / help, CLI / export, and support-export
surfaces that render it — and proves, by fixtures rather than screenshots, that the same build profile
presents the **same registry** everywhere it appears.

## Why this exists

The sheet already hardens artifact-graph publication, provenance / advisory UI, crash / symbolication packets,
and contract-CI linkage, but it left Aureline's actual build-lane trust domains and remote-cache discipline
too implicit for each claimed release-bearing surface. This lane wires those rules into the daily-driver
provenance surfaces so build lane, cache posture, clean-room parity, stale-proof state, and mirror / offline
build identity cannot drift between the About / provenance card, the Help pages, service health, the release
center, and support exports: every surface consumes the shared registry rather than private wording or
hand-copied CI prose. When two consumers describe the same build state differently, the regression suite
fails. Users and support staff can inspect the build lane and reproducibility state without reading CI logs.

## The three honesty axes

1. **Reuse.** Each of the four build-lane-trust families is adopted by **at least two distinct consumers**, so
   a lane is proven shared build-lane infrastructure rather than a one-surface fork of build-lane-descriptor or
   reproducibility-proof copy.
2. **One registry / no drift.** For a given build profile every consumer surface presents the identical
   six-word grammar — `build_lane_trust_role_word`, `family_word`, `registry_reference_word`,
   `build_context_word`, `surface_context_word`, and `replay_continuity_word`. The role word must be a token
   from the frozen `M5BuildLaneTrustRole` vocabulary (`cache_posture`, `publication_authority`,
   `credential_boundary`, `hermetic_input`, `reproducibility_proof`, `artifact_convergence`,
   `support_identity`), so no surface rewrites a role in its own words. A surface may narrow *how much* it shows
   across desktop, compact, remote, and exported representations, but never reword the grammar per surface — and
   a role that carries cache-posture, publication-authority, reproducibility-proof, or artifact-convergence
   meaning may never let a PR cache publish release artifacts, treat a remote-cache hit as reproducibility
   proof, drift a docs / schema / SBOM / symbol sidecar from the binary build identity, or overclaim clean-room
   parity on a partial rebuild.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain schema and the
   frozen matrix by id, so an exported packet always maps a release-center / provenance / diagnostics / support
   surface back to one shared contract family.

## Guardrails (each MUST be false on every binding)

- `pr_caches_publish_release_artifacts`
- `treats_remote_cache_hits_as_reproducibility_proof`
- `lets_docs_schema_sbom_or_symbol_sidecars_drift_from_binary_build_identity`
- `overclaims_clean_room_parity_when_only_partial_artifact_classes_were_rebuilt`
- `hides_non_hermetic_inputs_cache_poisoning_or_unreplayable_artifacts_behind_green_publication_rows`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an exported
representation names its export-safe detail boundary rather than collapsing the profile out of view. When a
route only supports a compact projection, remote-backed inspect, or an export-safe redaction rather than a full
desktop disclosure, the narrowing is surfaced consistently. Stale proof, a missing canonical reference, or
contradictory B144 evidence **narrows** the claim via a `BuildLaneTrustSharedConsumersDowngradeTrigger` rather
than hiding the family.

## Seeded coverage

Four build profiles — one per family — fan out to twelve consumer bindings covering all nine consumers and all
four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `contributor_pr` | `cache_posture` | build farm, cache service, CLI export |
| `protected_merge` | `publication_authority` | release center, shiproom, diagnostics |
| `release` | `reproducibility_proof` | provenance service, diagnostics, support export |
| `emergency_hotfix` | `support_identity` | docs/help, release center, support export |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted forms without
rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_build_lane_trust_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown summary, and
narrowed fixtures; the module tests fail if any drifts from the seed builder.
