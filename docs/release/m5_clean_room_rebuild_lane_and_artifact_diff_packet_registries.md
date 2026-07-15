# M5 clean-room-rebuild-lane and artifact-diff-packet registries

This lane is the clean-room-rebuild and deterministic-artifact-diff implement lane over the frozen
[M5 build-lane-trust matrix](./m5_build_lane_trust_contract.md). It turns the *clean-room-rebuild-lane* grammar
(how a protected lane replays its inputs without relying on shared remote-cache state as authority — the rebuild
source it classifies, the rebuild-config digest, the replay receipt, the protected-input ledger, the rebuild
authority it is bounded to, the artifact families it expects, the hermetic-rebuild posture, and the shared-cache
isolation rule) and the *artifact-diff-packet* grammar (how a release or emergency-hotfix lane emits a
deterministic diff comparing rebuild outputs across every claimed artifact family — binaries, packages, docs
packs, schemas, SBOMs, symbols, source maps, and rollback metadata — bound to one exact build identity) into
registry resolvers that produce export-safe, honest projections, so the build-farm, cache-service, release-center,
shiproom, provenance, diagnostics, docs, CLI, and support surfaces resolve one canonical exact-build truth instead
of a per-lane, hand-copied reconstruction. The clean-room rebuild lane and the artifact-diff packet are separated
in runtime and serialized state: the rebuild source, rebuild-config digest, replay receipt, protected-input
ledger, rebuild authority, expected artifact families, hermetic-rebuild posture, and shared-cache isolation rule
live on the clean-room-rebuild lane, while the resolved exact build identity, artifact families compared,
deterministic-diff ledger, candidate-vs-rebuild check, divergence-or-missing reference, attestation state, and
last diff revision live on the artifact-diff packet, and a lane's rebuild authority stays bounded so a clean-room
rebuild never relies on a shared remote cache as authority.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries` (the authoritative
  validator).
- **Combined schema:**
  `schemas/release/m5-clean-room-rebuild-lane-and-artifact-diff-packet-registries.schema.json`.
- **Domain schemas:** every row points at
  [`schemas/release/m5-clean-room-rebuild-lane.schema.json`](../../schemas/release/m5-clean-room-rebuild-lane.schema.json)
  and
  [`schemas/release/m5-artifact-diff-packet.schema.json`](../../schemas/release/m5-artifact-diff-packet.schema.json)
  as its canonical domain contracts.
- **Checked proof:**
  `artifacts/release/m5-clean-room-rebuild-lane-and-artifact-diff-packet-registries-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:** `fixtures/release/m5-clean-room-rebuild-lane-and-artifact-diff-packet-registries/`
  (`hermetic_rebuild_beta_narrowed.json`, `artifact_diff_preview_narrowed.json`).

## Two registries

1. **Clean-room-rebuild lane** (`resolve_clean_room_rebuild_lane_entry`) — publishes one typed
   clean-room-rebuild-lane object per lane: the rebuild source and canonical source mode, the rebuild-config
   digest, the replay receipt, the protected-input ledger, the rebuild authority, the expected artifact families,
   the hermetic-rebuild posture, and the shared-cache isolation rule. A clean entry names a canonical registry
   token, a classified rebuild source, and a build-lane-trust role, covers the canonical / accessible / audit
   resolution forms, publishes a complete object, bounds its rebuild authority so the lane never relies on a
   shared remote cache as authority, and discloses the cache-trust marker before a shared-cache-shortcut or
   unreplayable-reference source is admitted. Otherwise it degrades honestly — a lane whose rebuild authority is
   unbounded (a shared-cache shortcut or unreplayable reference claiming protected-lane authority) or a
   trust-risk source that hides its cache-trust marker degrades to
   `lane_relies_on_shared_cache_or_hides_replay_receipt`, the structured blocker reason a
   relies-on-shared-cache attempt must surface.
2. **Artifact-diff packet** (`resolve_artifact_diff_packet_entry`) — keeps the deterministic artifact-diff packet
   honest. A clean entry names a classified diff scope (byte-identical / normalized-equivalent / hermetic-rebuild)
   and provides the complete build-identity / compared-families / deterministic-diff-ledger / candidate-vs-rebuild
   / divergence-or-missing / attestation / last-diff-revision object; a packet that would let a green build omit a
   claimed artifact family from the diff, diff an artifact family against a different build identity, or treat a
   material divergence or an omitted family as warning-only degrades to
   `artifact_diff_diverges_or_omits_family_or_drifts_build_identity`.

## Per-entry rebuild reference

The rebuild source carries its canonical mode, and the resolver publishes the full clean-room-rebuild-lane object,
so the registry — never a hand-copied per-lane assumption — is the single source of truth.
`clean_room_rebuild_lane_object_is_complete` rejects an object missing any field,
`shared_cache_cannot_authorize_rebuild` rejects an unbounded rebuild authority or a hidden cache-trust marker, and
`artifact_diff_stays_deterministic` rejects a packet that has let a material divergence or an omitted family read
as a clean pass.

| rebuild source | source mode | rebuild-config digest | rebuild authority | expected artifact families |
| --- | --- | --- | --- | --- |
| hermetic clean-room rebuild | hermetic_clean_room_rebuild_mode | `build-config.sha256.release-0007` | `verification.release-signing-scoped` | `artifacts.binaries-packages-sboms-symbols-docs` |
| rematerialized input replay | rematerialized_input_replay_mode | `build-config.sha256.protected-merge-0007` | `verification.controlled-scoped-to-lane` | `artifacts.binaries-packages-sboms` |
| shared-cache shortcut | shared_cache_shortcut_mode | `build-config.sha256.contributor-pr-0007` | `verification.pr-scoped-only` | `artifacts.none-release-bearing` |

A relies-on-shared-cache attempt degrades to `lane_relies_on_shared_cache_or_hides_replay_receipt`, an incomplete
object degrades to `clean_room_rebuild_lane_object_incomplete`, and a divergent or omitted-family diff degrades to
`artifact_diff_diverges_or_omits_family_or_drifts_build_identity`, so a shared-cache shortcut, an incomplete
rebuild object, or a divergent diff can never turn release evidence green.

## Acceptance criteria (proven by resolved examples)

- **At least one protected lane rebuilds every claimed M5 artifact family into a comparable exact-build packet.**
  Clean clean-room-rebuild-lane entries cover the canonical hermetic-clean-room-rebuild / rematerialized-input-replay
  / pinned-digest-replay / shared-cache-shortcut / unreplayable-reference rebuild sources and the first
  release-center / shiproom / diagnostics / provenance / support surfaces, an object-incomplete example degrades,
  and no clean entry published an incomplete object.
- **A clean-room rebuild that would rely on a shared remote cache as authority fails with a structured blocker
  reason.** A relies-on-shared-cache example and an unbound example degrade, a clean bounded entry is present, and
  no clean entry is unbounded or unbound.
- **Deterministic diffs become structured release evidence and stable/LTS promotion is blocked when parity fails
  or ages out.** Clean artifact-diff entries cover the byte-identical / normalized-equivalent / hermetic-rebuild
  diff scopes with full resolution-form coverage while providing the complete object — the resolved exact build
  identity and the deterministic-diff ledger — and a packet that would let a green build omit a claimed artifact
  family or drift the build identity degrades.

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- support-export
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- csv
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- report
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- clean-room-rebuild-lane-table
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- fixture-hermetic-rebuild-beta-narrowed
cargo run -p aureline-ui --example dump_m5_clean_room_rebuild_lane_and_artifact_diff_packet_registries -- fixture-artifact-diff-preview-narrowed
```
