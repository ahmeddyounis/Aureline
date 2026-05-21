# Stable version-window freeze — proof packet

Reviewer-facing proof packet for the gated stable version-window freeze for the
release line's CLI, schema, API, and manifest surfaces, with deprecation packets.

Canonical machine source (do not clone status text from this packet — ingest the
JSON):

- Freeze: [`/artifacts/release/stable_version_windows.json`](../stable_version_windows.json)
- Schema: [`/schemas/release/stable_version_windows.schema.json`](../../../schemas/release/stable_version_windows.schema.json)
- Companion doc: [`/docs/release/stable_version_windows.md`](../../../docs/release/stable_version_windows.md)
- Validator: `ci/check_stable_version_windows.py`
- Validation capture:
  [`/artifacts/release/captures/stable_version_windows_validation_capture.json`](../captures/stable_version_windows_validation_capture.json)
- Typed consumer: `aureline_release::stable_version_windows`

## What this packet proves

1. **Each interface surface freezes a version window and a deprecation packet bound
   to a public claim.** Every row binds one surface (`surface_kind`, `surface_ref`)
   to the version window it pins (`version_window`), the deprecation packet that
   governs how older versions leave the window (`deprecation_packet`), the freeze
   packet that proves the freeze (`freeze_packet`), the waiver that holds it
   provisionally (`waiver`), and the public claim whose lifecycle label it backs
   (`claim_ref`, `claim_label`). The freeze reuses the stable claim level vocabulary
   rather than minting per-surface labels, so docs, Help/About, the release center,
   and support exports render one label per surface.

2. **The freeze ingests the stable claim manifest as a hard ceiling.** The CI gate
   reads the stable claim manifest named by `claim_manifest_ref` and fails when a
   row's `claim_label` is not the label that manifest publishes for the entry named by
   `claim_ref`, when a row names an entry the manifest does not carry, or when a freeze
   is backed wider than the public claim's canonical label. A surface's frozen label
   can never outrun the public claim it backs.

3. **The packet-freshness, waiver-expiry, and deprecation-removal automations narrow
   stale freezes before publication.** Each row's freeze packet carries a freshness
   SLO and a recorded `slo_state`; each deprecation notice carries a
   `removal_target_date` and `status`. The CI gate recomputes the freshness state and
   the removal-overdue state against the freeze `as_of` date and fails when a declared
   state overstates freshness, when a frozen row rides a `breached`/`missing` packet,
   when a surface holds on an expired waiver, or when a deprecation removal is overdue
   but undeclared. The state-snapshot schema (breached packet), the update-manifest
   format (overdue removal), and the rollback-orchestration API (expired waiver) are
   the worked examples: each narrows and holds publication even though its public claim
   is still published Stable.

4. **CLI/schema/API/manifest coverage stays complete.** The gate fails closed when a
   declared release-blocking surface has no covering row, when a release-blocking row
   is undeclared, when a surface ref repeats, or when any of the four surface kinds —
   CLI, schema, API, or manifest — has no row at all.

## Proof-index registration

Each row's freeze packet registers under one row of the canonical stable proof index
([`/artifacts/release/stable_proof_index.json`](../stable_proof_index.json)) via its
`freeze_packet.proof_index_ref`, so this lane's freeze is anchored to the stable proof
index rather than to ad hoc notes — the proof index's launch-blocking requirement rows
(provider routing/fallback, rollback drill and state integrity, export/offboarding,
localization, regulated sovereign evidence) are the requirements these interface
windows freeze the version surface for.

## Surface → window → claim matrix at this revision

Per surface, the version window and the lifecycle label the freeze backs (public claim
ceiling in parentheses):

| Surface (kind) | Window (floor → current → ceiling) | Public claim (ceiling) | Frozen label | Posture |
|---|---|---|---|---|
| Provider routing (api) | 1.0.0 → 1.4.0 → 1.9.0 | provider-aware language intelligence (stable) | stable | frozen |
| Repair/rollback commands (cli) | 1.0.0 → 1.3.0 → 1.6.0 | repair and rollback safety (stable) | stable | frozen on waiver |
| State snapshot (schema) | 1.0.0 → 1.2.0 → 1.4.0 | repair and rollback safety (stable) | **beta** | unfrozen — packet breached |
| Update manifest (manifest) | 1.0.0 → 1.1.0 → 1.3.0 | provider-aware language intelligence (stable) | **beta** | unfrozen — removal overdue |
| Export bundle (schema) | 0.9.0 → 1.0.0 → 1.2.0 | export and offboarding support (beta) | beta | inherits narrowed claim |
| Localization tooling (cli) | 0.5.0 → 0.7.0 → 0.9.0 | localization readiness (preview) | preview | unbacked under narrowed claim |
| Regulated deploy (manifest) | 0.4.0 → 0.6.0 → 0.8.0 | regulated-environment assurance (beta) | **withdrawn** | unfrozen — packet missing |
| Rollback orchestration (api) | 1.0.0 → 1.2.0 → 1.5.0 | repair and rollback safety (stable) | **beta** | unfrozen — waiver expired |

A non-release-blocking advisory surface (regulated control-plane api) is also frozen;
its deprecation packet is incomplete and unsigned under an already-narrowed claim, and
does not hold publication.

## Current posture

At this revision two of nine surfaces freeze a Stable window, seven are narrowed below
the cutline, and eight of the nine are release-blocking (two frozen, six unfrozen). All
four surface kinds are covered (cli ×2, schema ×2, api ×3, manifest ×2). Three
release-blocking surfaces whose public claims are still published Stable are themselves
unfrozen — the state-snapshot schema (packet breached), the update-manifest format
(removal overdue), and the rollback-orchestration API (waiver expired) — so three
blocking freeze rules fire and stable version-window publication **holds**. The
remaining narrowed rows inherit a public claim that is already below the cutline; they
record their gaps but do not hold publication beyond the upstream narrowing. That is the
honest posture: the repository is pre-implementation and the aggregate public claims are
optimistic, so the freeze narrows the interface-level truth beneath them.

## Accessibility of this lane

The freeze and its companion doc are text/JSON artifacts: the doc renders as headed
sections and plain Markdown tables (no color-only encoding), and the machine source
carries the same truth so Help/About, the release center, support exports, docs, and
shiproom dashboards ingest one record per surface rather than restating status text.

## How to refresh

1. Land the upstream stable claim manifest entry, the surface-level version inventory,
   refreshed freeze packets, completed deprecation packets, and waivers first; point
   each row's `claim_ref`, `claim_label`, `version_window`, `deprecation_packet`,
   `freeze_packet`, and `waiver` at the canonical records.
2. Set each row's `window_state`, `slo_state`, `active_gap_reasons`, and `frozen_label`
   to the honest posture.
3. Recompute the `publication` block and `summary`, then run
   `python3 ci/check_stable_version_windows.py --repo-root .` and commit the
   regenerated capture in the same change set.
4. If delivery proves a narrower window than planned, narrow the frozen label and
   update the freeze — do not paper over the gap with prose.
