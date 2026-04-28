# Evidence Chart Contract

Evidence-bearing visuals are claim surfaces, not decoration. A chart,
sparkline, summary card, exported screenshot, PDF figure, or support
bundle image that represents benchmark, compatibility, migration, or
qualification evidence must preserve the same truth basis as the packet
it summarizes.

Machine-readable companions:

- [`/schemas/evidence/chart_card.schema.json`](../../schemas/evidence/chart_card.schema.json)
  defines one `evidence_chart_card_record`.
- [`/fixtures/evidence/chart_cases/`](../../fixtures/evidence/chart_cases/)
  contains worked cases for fresh pass, stale green downgrade,
  incomparable baseline, waived threshold, and changed-scope visuals.

This contract composes with:

- [`/docs/ux/overview_surface_contract.md`](./overview_surface_contract.md)
  for stale-green downgrade behavior on tiles and summary cards.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for freshness classes and captured-versus-live scope truth.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  for claim-bearing freshness windows and rerun triggers.
- [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  for public-proof packet identity.
- [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md)
  and [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  for benchmark comparability and public packet disclosure.
- [`/docs/release/compatibility_report_template.md`](../release/compatibility_report_template.md)
  and [`/docs/release/certified_archetype_report_template.md`](../release/certified_archetype_report_template.md)
  for compatibility and certified-archetype row semantics.
- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)
  for migration score and importer-outcome linkage.

## Core Rule

An evidence visual may simplify layout, but it may not simplify truth.
Every visual must answer:

- which packet, exact build, source row, and claim scope it summarizes;
- when the evidence was captured, when it goes stale, and which rerun
  triggers invalidate it;
- whether the threshold, baseline, and scope are comparable to the
  displayed result;
- whether a waiver, caveat, or known limit changes the claim; and
- what machine payload travels with screenshots, PDFs, copied images,
  public-proof exports, and support bundles.

If any of those answers is unavailable, the visual downgrades to a
non-green state or refuses claim-bearing export.

## Required Anatomy

Every evidence chart card has these parts:

| Part | Requirement |
| --- | --- |
| Chart identity | Stable chart id, chart family, generated time, and surface context. |
| Source bindings | Packet refs, exact-build identity refs, and the row ids that the visual projects from. |
| Scope | Captured scope, current scope, channel, deployment profile, and scope relationship. |
| Truth state | Headline state, claim posture, and whether green rendering is allowed. |
| Chart body | Chart kind, axes, units, series labels, data-source refs, and optional sparkline. |
| Threshold | Threshold value, source, state, visible threshold line, and threshold-change disclosure. |
| Baseline | Baseline source, comparability class, visible comparison label, and incomparable reason when relevant. |
| Freshness stamp | Freshness class, captured time, evaluated time, stale window, trigger revisions, and visible label. |
| Caveat block | Known limits, exclusions, scope caveats, and whether each affects the claim. |
| Waiver marker | Waiver state, owner, expiry, and visible marker whenever the result or threshold is waived. |
| Export receipt | Preservation flags proving packet ids, exact-build basis, freshness, thresholds, baseline, caveats, waivers, and source row ids travel with the export. |

## Source And Identity Binding

The visual record must carry `evidence_packet_refs` and
`exact_build_identity_refs`. It must also carry at least one
claim-bearing source row from this set:

- `benchmark_row_refs` for benchmark rows, protected fitness rows, run
  results, or dashboard rows;
- `certified_archetype_row_refs` for certified-archetype or reference
  workspace rows;
- `migration_score_refs` for migration parity scores, importer outcome
  packets, or migration report rows;
- `compatibility_claim_refs` for compatibility rows or claim-manifest
  claim ids; and
- `qualification_row_refs` for release qualification or cadence rows.

Screenshots and human-readable PDFs may render shortened labels, but
their machine payload must preserve the full ids. A chart that cannot
name its source rows is informational only and must render
`claim_posture = not_claim_bearing`.

## Chart And Sparkline Rules

Charts and sparklines use the same backing freshness and scope basis.
A sparkline may summarize trend direction, but it does not replace:

- the current value label;
- threshold state;
- baseline comparability;
- freshness stamp; or
- caveat and waiver disclosure.

If the sparkline has fewer points than the charted claim needs, the
visual renders `partial` or `approximate_derived`; it does not smooth the
gap into a green trend.

## Threshold Rules

Thresholds must be visible whenever the visual compares a value against
a budget, tolerance, support floor, migration score floor, or release
gate. The threshold block must show:

- threshold value or `not_applicable`;
- threshold source (`policy`, `benchmark_council`,
  `compatibility_report`, `migration_scorecard`, `release_gate`,
  `waiver`, or `unknown`);
- threshold state;
- whether the threshold changed since the prior basis; and
- the evidence ref for the threshold.

A changed, removed, or newly introduced threshold must carry a visible
change label. A waived threshold must carry a waiver marker; the chart
may render the measured value, but it must not look like an ordinary
pass.

## Baseline Rules

Baselines must be explicit. The comparison block must name the baseline
kind (`prior_run`, `ci_baseline`, `reference_hardware`, `competitor`,
`certified_archetype`, `migration_source`, `compatibility_row`, or
`not_applicable`) and comparability class.

When `comparability_class` is `incomparable`, `scope_changed`, or
`unknown`, the visual must:

- render a non-green headline state;
- show the baseline state visibly rather than hiding it in hover text;
- give an incomparable reason; and
- keep the prior baseline ref in the export payload.

## Freshness And Stale Green Downgrade

Green rendering is allowed only when all of these are true:

- headline state is `passing`;
- claim posture is `claim_bearing`;
- freshness class is `live_exact`, `fresh_enough`, or
  `snapshot_exact_fresh`;
- the evidence is inside the claim window;
- the threshold state is `passing`;
- the baseline is comparable or not applicable; and
- no stale-green downgrade applies.

If the last verified result was green but freshness expired, a rerun
trigger changed, or scope no longer matches, the current visual must:

1. render a non-green headline state;
2. cite the prior green state and last green evidence ref in
   `stale_green_downgrade`;
3. show a visible stale or scope reason;
4. preserve packet and exact-build refs in export; and
5. offer a refresh, requery, review, or open-evidence path when
   available.

The compact visual may include a last-known-good note, but the headline
state may not remain green.

## Caveats And Waivers

Caveats are first-class chart content. A caveat that affects the claim
must be visible in the chart card and present in the machine export.
Examples include excluded platforms, narrowed workflow bundles, changed
scope, corpus drift, docs/help mismatch, and known support-export
limits.

Waivers are never silent. Active or expired waivers require a visible
marker with owner and expiry. Expired waivers render as non-green and
block claim-bearing export until renewed or closed with fresh evidence.

## Human Versus Machine Payloads

Human-readable outputs may optimize for scanability, but they must not
discard the truth basis.

Screenshots, copied images, and PDF figures must visibly show:

- chart title and scope label;
- current state;
- freshness stamp;
- threshold state;
- baseline comparability;
- caveat or waiver marker when present; and
- a packet or evidence id short label.

Machine exports must carry the full
`evidence_chart_card_record`, or an embedded/sidecar record conforming
to the same schema. Support bundles, public-proof packets, and exported
PDF metadata must preserve:

- packet ids;
- exact-build identity refs;
- source row ids;
- captured and current scope refs;
- freshness metadata;
- threshold and baseline state;
- caveats and waivers;
- redaction class; and
- whether raw traces or logs are intentionally omitted.

If a human output cannot preserve the matching machine payload, the
export downgrades to a metadata summary and may not be used as
claim-bearing evidence.

## Accessibility And Localization

The accessible name must announce state, freshness, threshold state,
baseline comparability, and waiver or caveat presence. Color is never
the only carrier of pass, stale, waived, or incomparable state.
Localized labels may change wording, but they must preserve controlled
state classes and source ids in the machine payload.

## Fixture Coverage

The fixture set covers:

- fresh passing benchmark chart;
- stale green downgrade;
- incomparable baseline;
- waived threshold;
- changed scope.

Each fixture is intentionally synthetic and uses opaque refs only. Raw
trace bodies, raw logs, private repository names, absolute paths,
credentials, and unrestricted provider URLs are outside this boundary.
