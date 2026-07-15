# M5 build-lane-trust surface certification (M05-1211)

This contract is the **closing B144 surface-certification capstone** over the frozen M5 build-lane-trust
matrix (`m5_build_lane_trust_matrix`). Where the freeze matrix defines the four governed build lanes —
**contributor-PR, protected-merge, release, and emergency-hotfix** — the 1205–1209 implementation lanes resolve
their per-surface build-lane-descriptor, reproducibility-proof, verified-input, sidecar-completeness,
clean-room-rebuild, artifact-diff, remote-cache-integrity, cache-bypass, exact-build-symbolication, and
mirror/offline-parity truth, and the 1210 shared-consumer lane aligns their grammar across the build-farm,
cache-service, release-center, shiproom, provenance-service, diagnostics, docs / help, CLI / export, and
support-export consumers and proves keyboard / screen-reader / high-zoom / high-contrast / localization /
CLI-export parity, this capstone **certifies** that the shared build-lane-trust truth holds on every claimed
M5 **RC / stable / LTS / mirror-offline publication-bearing profile** and auto-narrows any profile that cannot
sustain it.

- **Module:** `crates/aureline-ui/src/m5_build_lane_trust_surface_certification/`
- **Schema:** `schemas/release/m5-build-lane-trust-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-build-lane-trust-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-build-lane-trust-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a release engineer, reviewer, operator, or support engineer
reads a build-lane, cache-posture, publication-authority, clean-room-rebuild, reproducibility-proof, or
exact-build-supportability surface through, not on the reusable build lane it renders:

1. **Live exact-build supportable lane** — a live, first-party, fully reproducible release lane whose binaries,
   packages, SBOMs, symbols, docs packs, schemas, source maps, rollback metadata, and support packets converge
   on one exact build identity. The **only** profile that may certify a `trusted_exact_build_supportable_lane`
   claim.
2. **Reviewable reproducibility structure** — a self-sufficient, inspectable build-lane descriptor /
   reproducibility proof / clean-room rebuild diff; certifies at most `reviewable_reproducibility_surface`.
3. **Disclosed cache-discipline profile** — a contributor / PR lane whose shared-remote-cache origin trust can
   only be partially disclosed; auto-narrows to `cache_discipline_disclosed_projection`.
4. **Unverified clean-room-parity profile** — a release lane whose clean-room rebuild covered only a partial
   set of artifact classes; auto-narrows to `clean_room_parity_unverified_projection`.
5. **Unverified exact-build-supportability profile** — an emergency-hotfix lane whose docs / schema / SBOM /
   symbol sidecar has drifted from the binary build identity or aged out; auto-narrows to
   `exact_build_supportability_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and build-lane-trust-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed publication tier delivered;
- **yellow** — a truth axis is not current, so the publication claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh trusted claim, a hard invariant breaks,
  CLI/export parity drops, a non-live profile claims a trusted exact-build supportable lane, or the narrowing is
  inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `trusted_exact_build_supportable_lane` / `reviewable_reproducibility_surface` claim while one of its truth
   axes is not current over-claims and blocks.
2. **Only a live first-party fully reproducible release lane may certify a trusted exact-build supportable
   lane.** Every other profile is at most a reviewable reproducibility structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the canonical
   build lane, cache posture, publication authority, exact build identity, clean-room rebuild diff,
   reproducibility proof, sidecar convergence, support packet, and registry reference as text / JSON / Markdown.
4. **Every B144 hard invariant holds per row.** No profile may let a PR cache publish release artifacts, treat
   a remote-cache hit as reproducibility proof, let docs / schema / SBOM / symbol sidecars drift from the binary
   build identity, overclaim clean-room parity when only partial artifact classes were rebuilt, or hide
   non-hermetic inputs, cache poisoning, or unreplayable artifacts behind green publication rows.
5. **One canonical proof bundle.** Every row cites exactly one canonical build-lane-trust proof bundle
   (`artifacts/release/m5-exact-build-supportability-proof/support_export.json`) — the frozen build-lane-trust
   matrix proof — so release, docs, and support consume a single build-lane-trust certification source rather
   than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_BUILD_LANE_TRUST_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_build_lane_trust_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
