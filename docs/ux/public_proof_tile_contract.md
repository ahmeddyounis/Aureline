# Public-Proof Tile Contract

Public-proof and migration surfaces are claim surfaces. A benchmark
evidence card, compatibility row, importer diff row, migration handoff
tile, community issue handoff tile, or known-limits reference may be
compact, but it must preserve the same proof basis as the packet,
report, or migration outcome it summarizes.

Machine-readable companions:

- [`/schemas/evidence/benchmark_tile.schema.json`](../../schemas/evidence/benchmark_tile.schema.json)
  defines one `public_proof_tile_record`.
- [`/fixtures/evidence/public_proof_tile_cases/`](../../fixtures/evidence/public_proof_tile_cases/)
  contains worked cases for current benchmark proof, stale proof,
  partial migration, unsupported workflow gaps, and community-owned
  follow-up.

This contract composes with:

- [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  for scoreboard family and packet identity.
- [`/docs/ux/evidence_chart_contract.md`](./evidence_chart_contract.md)
  for chart, sparkline, stale-green, caveat, and export-preservation
  behavior.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  for importer outcome and migration handoff vocabulary.
- [`/docs/product/known_limits_contract.md`](../product/known_limits_contract.md)
  for caveat, exclusion, and downgrade-note refs.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  for freshness windows and rerun-trigger behavior.

## Core Rule

A public-proof tile may summarize evidence, but it may not summarize
away uncertainty. Every tile must answer:

- which workflow bundle, exact build, archetype row, claim row, packet,
  and source row it summarizes;
- what scope was captured, what scope is current, and whether those
  scopes still match;
- when the evidence was captured, when it expires, which rerun triggers
  invalidate it, and whether docs/help matches the exact build;
- which caveats, known-limit notes, unsupported mappings, or community
  ownership boundaries narrow the claim;
- which action opens the evidence packet; and
- which action opens a public issue, private issue, local export, or
  disabled issue route with an explicit reason.

If any required proof is missing, stale, narrower than the displayed
claim, or community-owned, the tile stays visible and downgrades
explicitly. It does not disappear from release packets, docs, Help/About,
or migration surfaces.

## Component Kinds

### Benchmark Evidence Card

Use a benchmark evidence card when a public claim cites a benchmark run,
fitness row, publication packet, or head-to-head comparison. Minimum
content:

| Field | Requirement |
| --- | --- |
| Identity | Tile id, packet refs, benchmark row refs, claim row refs. |
| Build basis | Exact-build identity ref, release channel, workspace version. |
| Scope | Workflow bundle, archetype row, deployment profiles, captured and current scope refs. |
| Result | Benchmark class, value label, threshold state, baseline comparability, support class. |
| Freshness | Captured time, stale window, expiry, rerun triggers, visible freshness class. |
| Caveats | Known-limit refs, exclusions, waiver or scope notes when present. |
| Actions | Open packet; open issue or disabled issue route with reason. |

Green rendering is allowed only when the result is passing, the evidence
is fresh enough, baseline and threshold state are comparable, docs/help
matches the build, and no active caveat narrows the claim.

### Compatibility Row

Use a compatibility row when the component summarizes a compatibility
report, certified-archetype row, support-class row, or skew row. Minimum
content:

| Field | Requirement |
| --- | --- |
| Row binding | Compatibility row refs and supporting evidence refs. |
| Scope | Archetype, deployment profile, release channel, and workflow bundle scope. |
| State | Support class, current state, qualification evidence status. |
| Downgrade | Known deviations, stale report state, unsupported or out-of-window posture. |
| Actions | Open packet/report; open mismatch or support issue route. |

A compatibility row with stale evidence renders as `evidence_stale` or
`retest_pending`, not as certified or supported.

### Importer Diff Row

Use an importer diff row when migration compares a source setting,
shortcut, extension, task, launch config, profile object, or workflow map
against Aureline's current state. Minimum content:

| Field | Requirement |
| --- | --- |
| Source and target | Source object ref, target object ref when one exists, domain, source value ref, target value ref. |
| Outcome | One of `imported`, `mapped`, `skipped`, `manual_review`, `bridge_required`, or `unsupported`. |
| Mapping basis | Exact, semantic, capability, bridge, heuristic, user override, or not applicable. |
| Caveat | Lossy mapping, unsupported feature, conflict, bridge requirement, or policy note. |
| Actions | Open outcome packet; open migration help or issue route. |

Partial migration remains partial across exports and support handoff. A
strong count of imported rows cannot hide `manual_review`,
`bridge_required`, or `unsupported` rows.

### Migration Handoff Tile

Use a migration handoff tile when a migration session recommends a
workflow bundle, bridge, restore path, docs guide, or follow-up issue.
Minimum content:

| Field | Requirement |
| --- | --- |
| Session binding | Migration session ref, outcome packet ref, restore/checkpoint ref when applicable. |
| Bundle binding | Recommended bundle id/revision and archetype row/revision. |
| Handoff state | Supported, partial, blocked, unsupported, or community follow-up. |
| Recovery | Rollback, restore, docs, or support refs that survive export. |
| Actions | Open packet; open issue or disabled route with reason. |

Unsupported workflows are honest end states. The tile may point to a
future issue or community route, but it must not imply the migration
reproduced the source tool when the outcome packet says otherwise.

### Community Issue Handoff Tile

Use a community issue handoff tile when ownership or support moves from a
first-party flow to a public issue tracker, RFC forum, community support
channel, or local export. Minimum content:

| Field | Requirement |
| --- | --- |
| Boundary | Ownership class, support boundary, destination visibility, and privacy class. |
| Packet preview | Included packet classes, redaction posture, and omitted raw data classes. |
| Evidence | Build, docs, packet, migration, known-limit, and support refs preserved by id. |
| Actions | Open packet; open issue, copy summary, local export, or disabled issue route. |

Community handoff cannot be rendered like guaranteed first-party support.
The tile must state the ownership boundary before navigation or export.

### Known-Limits Reference

Use a known-limits reference when a tile is mostly a caveat, exclusion,
or downgrade note. Minimum content:

| Field | Requirement |
| --- | --- |
| Note refs | Known-limit note refs and binding claim row refs. |
| Scope | Affected bundle, archetype, persona, deployment profile, and release channel. |
| Downgrade | Limitation class, severity, review rubric, and promotion-blocking state. |
| Publication | Destinations that must carry the note. |
| Actions | Open packet/note; open issue or disabled route with reason. |

Missing known-limit notes are themselves downgrade reasons. A tile with
an active known limit remains visible on every declared destination.

## Shared Anatomy

Every `public_proof_tile_record` has these required blocks:

| Block | Requirement |
| --- | --- |
| `tile_id` and `component_kind` | Stable identity and component discriminator. |
| `bundle_ref` | Workflow bundle id and revision. |
| `exact_build` | Exact-build identity ref, channel, and workspace version. |
| `archetype_row_ref` | Archetype row id and revision. |
| `scope` | Scope id, label, class, captured/current refs, relationship, deployment profiles. |
| `source_bindings` | Packet refs, row refs, migration refs, known-limit refs, claim refs, and support refs. |
| `evidence_state` | Headline state, support class, result posture, green-rendering flag, downgrade reasons. |
| `freshness` | Captured time, stale window, expiry, rerun triggers, freshness class. |
| `docs_version_match` | Docs/help state, pack revision, and repair hook ref. |
| `caveats` | Required object even when the caveat list is empty. |
| `surface_projection` | Export/release/docs/product projection rules and preservation flags. |
| `actions` | Required open-packet and open-issue action records. |

## Projection Rules

### Exported Packets

Support bundles, public-proof exports, release packets, and local review
exports must include either the full tile record or a sidecar record that
conforms to the same schema. The export may omit raw traces, logs,
screenshots, private repository names, local paths, account identifiers,
and credentials, but it must preserve packet ids, exact-build basis,
bundle/archetype ids, source row ids, freshness, caveats, docs-version
state, and action routing.

### Release Notes

Release notes may render a compact sentence, but the backing payload
must keep the tile id and all source refs. If evidence is stale,
incomplete, unsupported, or community-owned, release notes render the
downgraded state and the narrower supported path. They may not omit the
row simply because it is not green.

### Docs And Public Proof

Docs/public-proof pages may group tiles by workflow bundle, archetype, or
scoreboard family. The group header does not replace tile-level scope,
freshness, caveat, docs-version, and action state. Public pages must
prefer opaque refs and reviewed labels over raw internal links.

### In-Product Migration And Help

Migration Center, Help/About, compatibility detail, and guided-help
surfaces use the same tile payload. Importer rows and handoff tiles must
continue to show partial, bridge-required, unsupported, and community
states after apply, restore, export, or support handoff.

## Downgrade Rules

### Stale Evidence

When freshness expires, a rerun trigger changes, scope no longer matches,
or docs/help no longer matches the exact build:

1. set `green_rendering_allowed = false`;
2. render `headline_state = evidence_stale`, `retest_pending`, or a
   narrower non-green state;
3. include a typed downgrade reason;
4. keep the last packet and exact-build refs; and
5. offer an open-packet action plus a refresh, issue, or local-export
   route when available.

### Incomplete Import Mapping

When any importer row is `manual_review`, `bridge_required`, or
`unsupported`, the row and any aggregate migration handoff tile:

1. include the row in exported packets and support handoff;
2. carry `incomplete_import_mapping`, `unresolved_migration_gap`, or
   `unsupported_workflow_gap` as a downgrade reason;
3. expose the source and target refs that explain the gap; and
4. avoid complete, certified, or replacement-grade wording.

### Community-Owned Follow-Up

When follow-up is community-owned:

1. render `community_handoff.ownership_class = community_owned`;
2. state destination visibility and privacy class before navigation;
3. keep first-party support language at or below `community`;
4. preserve redaction and packet-preview state; and
5. keep the issue route visible even when the action is disabled offline.

### Unsupported Workflow Gaps

When a workflow, extension behavior, source-tool concept, platform, or
deployment profile is unsupported:

1. render `headline_state = unsupported` or `blocked`;
2. cite known-limit refs or explicit caveat refs;
3. set support class to `experimental`, `community`, or `not_supported`
   as appropriate;
4. keep fallback, restore, or export actions visible; and
5. block green rendering on every projection until fresh evidence
   supports a narrower or repaired claim.

## Fixture Coverage

The fixture set covers:

- green/current benchmark proof;
- stale benchmark proof that downgrades instead of disappearing;
- partial migration with importer-diff rows preserved;
- unsupported workflow handoff with known-limit refs; and
- community-owned issue handoff with public/private boundary cues.

Fixtures are synthetic and use opaque refs only. Raw traces, logs,
screenshots, local filesystem paths, private repository names, raw URLs,
account identifiers, and credentials stay outside this boundary.
