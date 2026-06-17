# M5 Support Center matrix — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/support/m5/m5-support-center-matrix.json`. The full contract and gate semantics live in
`docs/help/support/m5-support-center-matrix.md`; the typed model lives in the `aureline-support`
crate (`m5_support_center_matrix`).

This matrix makes the Support Center an **explicit product surface** instead of a scatter of hidden
pages. It carries one row per Support Center module, binding each module to the one canonical
inspector vocabulary (environment-status, precedence-inspector, crash-intake, install/advisory-state,
credential-state, export-consent), the export-risk **data classes** (`metadata_only`,
`environment_adjacent`, `code_adjacent`, `high_risk`), a **redaction default**, and the
**local-save / team-share / formal-support** export modes it offers. A fail-closed readiness gate
auto-narrows any module whose evidence is stale, whose bound inspector is degraded or unavailable, or
whose export consent is ungranted or blocked.

## Module roll-up (as of 2026-06-16)

| Module | Evidence | Readiness | Claim | Recovery | Data classes | Export modes |
| --- | --- | --- | --- | --- | --- | --- |
| `doctor` | current | **operational** | published | none | metadata, env | local, team, formal |
| `safe_mode` | aging | **degraded** | narrowed | refresh_evidence | metadata, env | local |
| `bisect` | current | **degraded** | narrowed | restore_inspector | metadata, env | local, team |
| `performance` | current | **inspect_only** | published | none | metadata, env | local |
| `language` | current | **degraded** | narrowed | resolve_consent | metadata, env, code | local, formal |
| `index` | expired | **inspect_only** | narrowed | refresh_evidence | metadata, code | local |
| `ai_usage` | current | **inspect_only** | narrowed | resolve_consent | metadata, env, high | local, formal |
| `crash` | current | **operational** | published | none | metadata, env, code | local, formal |
| `network` | current | **unavailable** | withheld | withhold_module | metadata, env | local |
| `artifacts` | missing | **unavailable** | withheld | withhold_module | metadata, env | local |
| `issue_report_crash_intake` | current | **operational** | published | none | metadata, env, code | local, team, formal |
| `support_bundle_export_preview` | current | **operational** | published | none | metadata, env, code, high | local, team, formal |

Five modules publish at their declared readiness (four operational — Doctor, Crash, issue-report /
crash-intake, and the support-bundle export preview — plus the read-only performance inspector),
proving the gate is not a blanket downgrade; five are auto-narrowed and two are withheld. The
published readiness of every module equals the gate's recomputed ceiling.

## How each narrowed or withheld module narrows

- `safe_mode` — retained-capability evidence is **aging**, so the module is held at `degraded` until
  the capability probe is refreshed. Entry stays available.
- `bisect` — the install/advisory inspector is **degraded**, capping the module at `degraded` and
  pointing the owner at restoring the descriptor. Bisect and quarantine stay offered.
- `language` — formal-support export of code-adjacent captures awaits **consent**, so the module is
  held at `degraded`; local inspection and restart stay available.
- `index` — index-health evidence is **expired**, narrowing the module to `inspect_only` with a
  rebuild as the recovery path.
- `ai_usage` — formal-support export of transcript material is **blocked** by data policy; the module
  narrows to `inspect_only` and the high-risk class is excluded always.
- `network` — the environment-status descriptor is **unavailable**, so the module is **withheld**; it
  offers no actions until the descriptor is restored.
- `artifacts` — the artifact-graph evidence is **missing**, so the module is **withheld**; provenance
  review returns once the probe is captured.

## Invariants the gate enforces

- **One contract, no inheritance.** Every Support Center module carries exactly one row; a module is
  never green because a neighbouring module passed a similar check. The published readiness can never
  exceed the weakest of the declared readiness, the evidence freshness, the inspector availability,
  and the export consent.
- **Fail-closed.** Stale evidence, a degraded or unavailable inspector, or an ungranted/blocked
  consent narrows or withholds the module automatically. Every `published_readiness`,
  `module_publication`, `downgrade_reasons`, and `downgrade_path` equals the recomputed gate.
- **Data classes stay redaction-safe.** A module touching `high_risk` material must default to
  `excluded_always`; no consent can include secret-bearing material in a Support Center export.
- **Consent is bound where it matters.** Any module that shares off-machine (team-share or
  formal-support) must reuse the `export_consent` descriptor. Local-save is a first-class peer of the
  share/upload modes.
- **One source of truth.** Desktop shell, CLI/headless, Help/About, shiproom, and formal-support
  handoff each bind to this one packet and narrow with it, so a module narrowed here cannot stay
  authoritative downstream.
