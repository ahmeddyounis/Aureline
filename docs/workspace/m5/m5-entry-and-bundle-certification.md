# M5 entry-and-bundle certification report

The **entry-and-bundle certification report** is the single qualification packet that graduates
the M5 switching and project-entry depth lanes — workflow-bundle install, source acquisition,
project open, project import, session resume, recent-work, and workspace admission — **only
where their evidence is current and provable**, and automatically narrows the rest to a smaller
label before publication.

- Typed model: `aureline-workspace` crate, module `m5_entry_and_bundle_certification`.
- Packet: [`artifacts/workspace/m5/m5-entry-and-bundle-certification.json`](../../../artifacts/workspace/m5/m5-entry-and-bundle-certification.json).
- Reviewer artifact: [`artifacts/workspace/m5/m5-entry-and-bundle-certification.md`](../../../artifacts/workspace/m5/m5-entry-and-bundle-certification.md).
- Schema: [`schemas/workspace/m5-entry-and-bundle-certification.schema.json`](../../../schemas/workspace/m5-entry-and-bundle-certification.schema.json).
- Fixtures: [`fixtures/workspace/m5/m5-entry-and-bundle-certification/`](../../../fixtures/workspace/m5/m5-entry-and-bundle-certification/).

## Where it sits

This packet is the **certification layer above** the
[entry-and-bundle governance matrix](m5-entry-and-bundle-governance.md). It does not re-derive
each lane's truth. For every claimed lane it:

1. ingests the governance matrix's published assurance label as the row's `governance_claim`;
2. runs the per-lane qualification drills the certification suite owns;
3. scores how fresh the certification evidence is; and
4. publishes the certification label no input can exceed.

Because the certification claim is bounded by the governance claim it ingests, the two surfaces
can never disagree: a lane the governance matrix narrowed cannot be re-broadened here.

## The non-inheriting, fail-closed gate

The published label of each row is the **weakest ceiling** implied by three inputs:

| Input | Ceiling it imposes |
| --- | --- |
| `governance_claim` (verified / bounded / retest_pending / withheld) | the upstream label, never re-broadened |
| `evidence_freshness` (current → aging → expired → missing) | verified → bounded → retest_pending → withheld |
| `drill_ceiling` (weakest drill outcome) | passed → verified, narrowed → bounded, failed/not-run → withheld |

`published_label = min(declared_label, governance_claim, evidence_ceiling, drill_ceiling)`.

So a governance-narrowed lane, stale or missing certification evidence, or an unproven,
narrowed, or failed drill **all narrow or withhold the certified label automatically** rather
than leaving a lane green by inertia. The recorded `published_label`, `certification_decision`,
`downgrade_reasons`, and `downgrade_path` must each equal the gate's recomputed value, so a
downgrade can never be asserted or hidden by hand. A missing required drill caps the row at
`withheld`, so an incompletely drilled lane is never certified by omission.

## Drills

Every claimed lane runs all seven drills exactly once, and a drill that ran must carry an
evidence ref:

- `project_entry` — first-useful-work routing keeps setup-later, open-minimal, and local-safe paths.
- `recent_work` — recent-work registry resolves and restores entries honestly.
- `source_acquisition` — clone, open, import, and resume stay distinct verbs.
- `bundle_lifecycle` — bundle install / update / remove shows a diff and a rollback checkpoint.
- `admission` — workspace-admission detection never silently widens trust.
- `accessibility` — keyboard, list/table, and screen-reader behavior holds.
- `downgrade` — automatic claim narrowing and recovery routing fire as specified.

## Recovery paths

A narrowed or withheld row always names an exact recovery path, lists a caveat, and names what
is stale or missing:

| Path | When |
| --- | --- |
| `rerun_drills` | a drill narrowed, failed, or did not run |
| `refresh_evidence` | the certification evidence is aging, expired, or missing |
| `adopt_governance_narrowing` | the governance claim alone narrows the lane |
| `withhold_row` | the lane is withheld from publication |
| `none` | the lane is certified verified and whole |

## Consumer surfaces

Six surfaces bind to this one packet and must ingest it, preserve its labels and recovery paths,
and narrow with it, so a lane narrowed here cannot stay green on a tile, banner, or badge by
inertia: `start_center`, `migration_center`, `help_about`, `release_center`, `docs_help`, and
`support_export`. Each binding is stamped with the active scope snapshot so support and evidence
packets can reconstruct the scope the certification answered.

## Guardrail

No blanket "best-in-class onboarding" or "one-click project entry" label survives without
row-level qualification, freshness, and a downgrade path. A row only reads as certified when its
governance claim is verified, its evidence is current, every drill passed, its declared label is
verified, and nothing narrows it.

## Export safety

The packet is metadata-only: every field is a typed state, a count, or an opaque ref. It carries
no credential bodies, raw provider payloads, or workspace contents, and the support-export
wrapper preserves the report verbatim with `raw_private_material_excluded = true`.
