# M5 Build-Farm, Cache-Trust, Clean-Room-Rebuild, and Exact-Build-Supportability Contract

Status: frozen (B144 opening matrix)

This contract freezes Aureline's concrete build-lane trust domains, remote-cache discipline, clean-room
rebuild proof, and exact-build supportability into one export-safe matrix. It is the canonical source of
release-train reproducibility truth for M5: later release-center, shiproom, diagnostics, admin, docs/help,
and support/export surfaces consume it directly rather than copying ad hoc CI prose by hand.

- Matrix schema: `schemas/release/m5-build-lane-trust-matrix.schema.json`
- Build-lane-descriptor domain schema (contributor / PR / protected-merge): `schemas/release/m5-build-lane-descriptor.schema.json`
- Reproducibility-proof domain schema (release / emergency-hotfix): `schemas/release/m5-reproducibility-proof.schema.json`
- Support export: `artifacts/release/m5-exact-build-supportability-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-exact-build-supportability-proof/matrix.csv`
- Design report: `artifacts/release/m5-build-lane-trust-matrix.md`
- Narrowed fixtures: `fixtures/release/m5-clean-room-rebuild/`
- Authoritative validator: `crates/aureline-ui` (`m5_build_lane_trust_matrix`)
- Emitter (single mint-from-truth path): `cargo run -p aureline-ui --example dump_m5_build_lane_trust_matrix`

## Governed build lanes

The matrix freezes **four** build lanes, each qualified independently and each pointing at one canonical
domain schema:

| Lane | Build-lane concern | Owner | Domain schema |
| --- | --- | --- | --- |
| `contributor_pr` | May read shared caches but never publishes release artifacts; PR-scoped credentials | Contributor-lane owner | build-lane-descriptor |
| `protected_merge` | Controlled credentials and verified caches only; a missing digest blocks promotion | Protected-merge owner | build-lane-descriptor |
| `release` | Verified or re-materialized inputs converging on one exact build identity | Release-engineering owner | reproducibility-proof |
| `emergency_hotfix` | Expedited yet still verified inputs and one exact build identity | Emergency-hotfix owner | reproducibility-proof |

## Shared build-lane-trust-role vocabulary

Every consumer binds to one controlled role vocabulary; no surface invents a parallel word:

`cache_posture`, `publication_authority`, `credential_boundary`, `hermetic_input`, `reproducibility_proof`,
`artifact_convergence`, `support_identity`.

The cache-posture / publication-authority / reproducibility-proof / artifact-convergence roles
(`cache_posture`, `publication_authority`, `reproducibility_proof`, `artifact_convergence`) must verify
inputs and prove replay before promotion — a PR cache may never publish, a remote-cache hit is never
reproducibility proof, a sidecar may never drift from the binary build identity, and clean-room parity is
never overclaimed. The descriptive structure roles (`credential_boundary`, `hermetic_input`,
`support_identity`) are inspectable descriptors.

## Hard invariants (release blockers)

Every row carries five hard-invariant booleans that must be `false`, and the governance-review block
asserts the corresponding fleet-level guarantees:

1. A PR cache never publishes release artifacts.
2. A remote-cache hit is never treated as reproducibility proof.
3. Docs / schema / SBOM / symbol sidecars never drift from the binary build identity.
4. Clean-room parity is never overclaimed when only partial artifact classes were rebuilt.
5. Non-hermetic inputs, cache poisoning, and unreplayable artifacts never hide behind green publication
   rows.

The frozen downgrade triggers also enumerate the remaining release blockers: an untrusted cache use, a
missing digest (a lane leaving its publication authority or build identity unstated), a stale clean-room
proof, and a missing registry reference.

## Automatic narrowing

Claim publication and support/export narrow release-lane claims automatically when the B144 registry is
missing, stale, or not yet qualified. Two narrowed fixtures demonstrate honest narrowing while keeping every
lane visible:

- `clean_room_release_beta_narrowed.json` — the release lane held at **Beta** pending clean-room parity
  across every artifact class.
- `clean_room_emergency_hotfix_preview_narrowed.json` — the emergency-hotfix lane narrowed to **Preview**
  pending complete exact-build supportability evidence.

## Bound source contracts

The matrix binds back to already-landed truth so build-lane truth is never split across scattered CI notes:
the artifact-publication row schema (`schemas/release/artifact_publication_row.schema.json`) and the
reproducible-RC packet schema (`schemas/release/reproducible_rc_packet.schema.json`).
