# Control-plane-versus-data-plane outage taxonomy

This contract describes optional-service impairment **by lane** so a person can
see exactly which managed service is affected — and, just as importantly, see
that local editing is not. It exists to keep a control-plane outage (identity,
catalog, relay, AI gateway, telemetry) from ever being reported as if the IDE
itself were down while autosave, search, and Git are still working on the device.

For each claimed optional-service family it produces one **degraded-state
descriptor** that answers the same questions everywhere:

1. Which optional-service lane is impaired — identity/policy,
   registry/updates/docs, collaboration, remote control plane, AI gateway, or
   telemetry/support — which plane does the impairment sit on (control plane or
   managed data plane), and how severe is it?
2. What is the typed degraded state, and what narrower fallback takes over?
3. What still works locally right now — editing, save, search, and version
   control — so local-first credibility is proven, not implied?

The packet is produced by
`aureline_continuity::m5_control_plane_vs_data_plane_outage`. It reuses the
control-plane/data-plane vocabulary (`PlaneImpairmentClass`) and the
qualification ladder (`ContinuityClaimQualificationClass`) from the frozen
continuity-claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`) so there is
exactly one outage-plane vocabulary across the product. The descriptor is then
projected identically onto the desktop activity center, CLI/headless explain,
service-health, support-center exports, shiproom, and docs/public-truth pages.

## What every surface answers the same way

- Which optional-service lane is impaired, and on which plane?
- How severe is the impairment, and what narrower fallback is active?
- Is local editing, save, search, and version control still available?
- Does the outage avoid setting a misleading global "IDE down" state?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every optional-service family has at least one outage packet (the taxonomy is
   complete).
2. The taxonomy classifies at least one control-plane outage and one data-plane
   outage, so the two planes are visibly distinct.
3. Every impaired lane names the narrower fallback it runs on; an operational
   lane claims none.
4. Every packet preserves the full local-core (editing, save, search, version
   control) and references current outage evidence.
5. Every packet is projected onto all six surfaces.
6. The outage and local-core vocabulary is identical across every surface
   projection.

## Fail-closed guardrail: no conflation with local editing failure

The load-bearing guardrail is that an optional-service outage may never conflate
itself with a local editing failure. A packet **fails closed** when it either:

- flips a global "IDE down" state (`sets_global_ide_down`), or
- marks any local-core capability — editing, save, search, or version control —
  unavailable while only a managed lane is impaired.

Such a packet's claim is **withdrawn** and its typed degraded state is recorded
as `local_core_conflated_misclaim`, surfaced honestly so reviewers can see the
misclaim instead of having it published as truth. The withdrawal is isolated to
the offending packet: every other lane's packet stays at its own qualification.

## Typed degraded states

| Degraded state | Meaning |
|---|---|
| `operational` | the lane is healthy |
| `control_plane_impaired_local_core_preserved` | control plane impaired; local-core work continues |
| `managed_data_plane_impaired_local_core_preserved` | a managed data plane impaired; local-core work continues |
| `both_managed_planes_impaired_local_core_preserved` | both managed planes impaired; local-core work continues |
| `local_core_conflated_misclaim` | the packet wrongly conflates the outage with local editing failure (withdrawn) |

## Narrowing reasons

| Reason | Effect |
|---|---|
| `local_core_conflated` | withdrawn (fail closed) |
| `fallback_undeclared` | beta |
| `outage_evidence_missing` | beta |
| `surface_reuse_incomplete` | beta |
| `plane_distinction_missing` | beta |
| `family_coverage_incomplete` | beta |
| `operational_state_inconsistent` | preview |
| `outage_evidence_stale` | preview |
| `outage_vocabulary_drift` | preview |

## Export safety

The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
plain-language labels, and opaque evidence refs. Raw provider payloads, raw
incident bodies, hostnames, and secret material never cross this boundary; the
summary and support-export records both assert `raw_payloads_excluded`.

## Inspect and validate

```sh
# Emit the canonical fixtures.
cargo run -q -p aureline-continuity --example dump_m5_control_plane_vs_data_plane_outage_fixtures -- page

# Re-audit a page and emit a redaction-safe support export.
cargo run -q -p aureline-continuity --bin aureline_outage_taxonomy_inspect -- \
  fixtures/continuity/outage_taxonomy/page.json

# Validate the fixtures against the schema.
python3 tools/validate_m5_control_plane_vs_data_plane_outage_fixtures.py
```

## Related contracts

- Schema: `schemas/continuity/control_vs_data_plane_packet.schema.json`
- Fixtures: `fixtures/continuity/outage_taxonomy/`
- Artifact: `artifacts/m5/continuity/control_plane_vs_data_plane_degradation.md`
- Truth source for the reused control-plane/data-plane vocabulary:
  `docs/m5/continuity/locality_tenant_keymode_and_drill_matrix.md`
