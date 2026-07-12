# M5 Install / Update / Disable / Rollback Review-Sheet Controls

- Packet: `m5-install-update-disable-rollback-review-sheet-controls:stable:0001`
- Label: `M5 install / update / disable / rollback review-sheet controls with one reviewed transaction grammar, permission deltas, publisher-continuity warnings, runtime-interruption preview, disable-scope clarity, rollback-compatibility truth, and public / mirror / enterprise source-class continuity across marketplace, install, help, and export`
- Consumer surfaces: 5
- Mutation flows: install, update, disable, rollback
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Consumer surfaces

- **marketplace_ui**: `stable`
  - Owner: Marketplace catalog owner
  - Scope: The marketplace listing opens the same reviewed install transaction as every other surface, naming the public / mirrored / enterprise source class, the permission delta, the runtime-interruption preview, and the publisher continuity before install, and degrades honestly when an incompatible artifact reads as ready
  - Review-sheet examples: 2
- **extensions_ui**: `stable`
  - Owner: Extensions manager owner
  - Scope: The extensions manager reuses the same reviewed transaction grammar for updates, names the transitive permission widening and the exact rollback path before commit, and degrades honestly when the review / confirm / cancel grammar is incomplete
  - Review-sheet examples: 2
- **install_review_ui**: `stable`
  - Owner: Install-review owner
  - Scope: The install-review sheet is the canonical mutation surface: it names the disable scope on a disable and the rollback compatibility on a rollback before commit, keeps a data-loss rollback disclosed, and degrades honestly when a disable leaves its scope unstated or a rollback leaves its compatibility unresolved
  - Review-sheet examples: 4
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved review-sheet truth, so a collapsed source class, a hidden publisher transfer, a hidden data-loss rollback, an unverified permission delta, or a stale Certified overclaim is visible in evidence rather than hidden behind compact chrome from review through help / support / export handoff
  - Review-sheet examples: 5
- **product_ui**: `stable`
  - Owner: In-product lifecycle owner
  - Scope: In-product install / update / disable / rollback surfaces reuse the same reviewed transaction grammar and keep the registry source class explicit, and degrade honestly when the source class is unresolved, the runtime-interruption preview is unavailable, or the artifact identity is unstated so no opaque mutation is quietly carried forward
  - Review-sheet examples: 4
