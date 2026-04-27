# Design-partner and public-proof packet

This packet defines how design-partner inputs, fixture repositories,
reference workspaces, benchmark corpora, traces, and support packets
enter Aureline's external proof program without becoming ad hoc
trackers. It composes the existing benchmark, compatibility, public-
proof, exact-build, and known-limits contracts into one review path.

Companion artifacts:

- [`/artifacts/program/design_partner_intake_checklist.yaml`](../../artifacts/program/design_partner_intake_checklist.yaml)
  is the machine-readable intake checklist for design partners,
  fixture repositories, and reference workspaces.
- [`/artifacts/bench/publication_rehearsal_checklist.yaml`](../../artifacts/bench/publication_rehearsal_checklist.yaml)
  is the machine-readable rehearsal checklist for benchmark,
  compatibility, migration, and public-proof publication lanes.
- [`/fixtures/bench/privacy_clearance_cases/`](../../fixtures/bench/privacy_clearance_cases/)
  contains worked privacy-clearance fixture cases.
- [`/docs/qe/public_proof_scoreboards.md`](../qe/public_proof_scoreboards.md)
  and
  [`/artifacts/qe/workflow_bundle_ids.yaml`](../../artifacts/qe/workflow_bundle_ids.yaml)
  own the scoreboard-family and workflow-bundle identities.
- [`/docs/benchmarks/corpus_governance.md`](../benchmarks/corpus_governance.md),
  [`/docs/benchmarks/public_comparison_rules.md`](../benchmarks/public_comparison_rules.md),
  and
  [`/docs/benchmarks/benchmark_publication_pack_template.md`](../benchmarks/benchmark_publication_pack_template.md)
  govern benchmark corpus changes and public benchmark packets.
- [`/docs/compat/reference_workspace_program_seed.md`](../compat/reference_workspace_program_seed.md)
  and
  [`/artifacts/compat/reference_workspace_rows.yaml`](../../artifacts/compat/reference_workspace_rows.yaml)
  govern reference-workspace admission and support-class promotion.

If this packet and a machine-readable checklist disagree, the checklist
is authoritative for tooling and this packet is updated in the same
change.

## Goals

The external proof program exists so claims can be reviewed from stable
evidence rather than anecdotes. A prepared packet must let reviewers
answer:

1. Which workflow bundle, archetype row, corpus revision, and exact
   build does the evidence support?
2. Can the input be used in CI, release evidence, a public benchmark
   packet, or a reference-workspace report without exposing secrets,
   personal data, raw partner repository names, unsupported licenses,
   or ambiguous corpus lineage?
3. Which owner, backup coverage posture, approval steps, refresh
   cadence, and stale-evidence triggers govern the packet?
4. Which docs, known-limits, compatibility, migration, and support-copy
   surfaces must be checked before claim wording widens?

## Shared identity model

Every intake and rehearsal packet uses the same identity spine:

| Field | Source | Rule |
|---|---|---|
| `workflow_bundle_ref` | `artifacts/qe/workflow_bundle_ids.yaml` | Cite the existing `bundle_id` plus `bundle_revision`; do not mint a lane-local bundle id. |
| `archetype_row_ref` | `artifacts/compat/reference_workspace_rows.yaml` | Cite the existing `archetype_row_id` plus revision when the evidence is archetype-bound. |
| `scoreboard_family_id` | `artifacts/qe/workflow_bundle_ids.yaml` | Use the scoreboard family that will read the packet. |
| `corpus_ref` | `fixtures/benchmarks/corpus_manifest.yaml` | Cite a stable corpus id, a reservation id, or a cleared reference workspace id; never cite a private repository name in public packets. |
| `exact_build_identity_ref` | `docs/build/exact_build_identity_model.md` | Required before any publication rehearsal can be marked publishable. |
| `known_limit_ref` | `docs/product/known_limits_contract.md` | Required whenever the packet narrows scope, platform, corpus, competitor parity, migration, docs, or support export. |

The design-partner intake checklist and the publication rehearsal
checklist intentionally repeat the same approval-step vocabulary so a
bundle can move from partner intake to benchmark publication without a
separate spreadsheet or re-keyed status labels.

## Intake workflow

The intake workflow applies to design-partner repositories, fixture
repositories, benchmark corpora, reference workspaces, trace captures,
and support packets.

1. **Open intake against a bundle.** Choose a
   `workflow_bundle_ref`, optional `archetype_row_ref`, candidate
   `corpus_ref`, and evidence class before reviewing bytes. If no
   bundle exists, the input is exploratory and cannot feed a public
   claim.
2. **Record lineage.** The source class, source owner, source
   revision, license posture, provenance notes, and any transformation
   recipe must be recorded before the input enters CI or release
   evidence.
3. **Run privacy clearance.** The reviewer records the secret scan,
   personal-data scan, path/name redaction result, retention posture,
   export posture, and support-packet redaction outcome. The decision
   is one of `admit_public`, `admit_internal_only`,
   `redact_then_admit`, `exclude_until_replaced`, or
   `withdraw_existing_use`.
4. **Review license and redistribution.** The input may not become a
   public fixture, public benchmark corpus, or public reference
   workspace until license terms, attribution, allowed use, and
   redistribution posture are explicit.
5. **Bind owners and backup coverage.** Intake records the selection
   owner, evidence owner, publication owner, backup owner or active
   backup waiver, and refresh cadence.
6. **Attach review evidence.** Approval evidence uses opaque refs:
   partner approval ticket, redaction report, license review, retention
   review, corpus diff, and publication rehearsal ref. Raw review
   notes and raw partner bytes stay outside this packet.

## Privacy-clearance workflow

The privacy clearance result is binding on downstream packet posture.

| Decision | Effect |
|---|---|
| `admit_public` | The cleared artifact may be cited by public packets using stable ids and redacted labels. |
| `admit_internal_only` | Release evidence may cite the artifact by stable ref, but public packets cite only derived metrics or redacted summaries. |
| `redact_then_admit` | The raw input remains restricted; the sanitised artifact receives a stable corpus or reference-workspace id after review evidence is attached. |
| `exclude_until_replaced` | No CI, release, support, benchmark, or public-proof lane may depend on the raw input. A synthetic or sanitised replacement may open a new intake. |
| `withdraw_existing_use` | Existing packets citing the artifact are narrowed or withdrawn, and dependent claim rows are refreshed in the same change. |

Clearance is not a one-time checkbox. It expires when corpus revision,
fixture bytes, support-export shape, exact-build identity chain,
license terms, partner approval scope, retention posture, or claim-row
binding changes.

## Publication rehearsal workflow

A benchmark, compatibility, migration, or public-proof claim is not
publication-ready until the rehearsal checklist is green for the lane
that will publish it.

The rehearsal checks:

- reproducibility inputs: exact command line, config refs, corpus
  revision, protected metrics, fitness catalog, hardware row,
  lab-image row, environment row, and task script or success criterion;
- exact-build linkage: each packet resolves to the coordinated exact
  build identity and release channel;
- public-proof packet linkage: scoreboard family, packet shape,
  workflow bundle, archetype row, prior-claim diff, freshness envelope,
  and rerun triggers are complete;
- docs and known-limits alignment: docs/help version-match state,
  known-limit notes, release notes, support export, migration notes,
  and claim-manifest rows agree;
- privacy clearance: public packets do not include raw secrets,
  personal data, private repository names, raw trace bodies, raw
  support transcripts, or license-restricted bytes; and
- dry-run owners: benchmark, compatibility, migration, docs/support,
  release evidence, and product scope owners are named with backup
  coverage or an active waiver.

## Claim-widening gate

A packet may widen beta, stable, evaluation, or public marketing
language only when all of these are true:

- the bundle and archetype refs are stable and versioned;
- privacy clearance is `admit_public` or `redact_then_admit` with
  attached evidence;
- the exact build identity, corpus manifest, protected metrics,
  fitness catalog, hardware, environment, docs/help version-match,
  known-limit notes, and claim-manifest rows are current;
- the publication rehearsal checklist for the relevant lane has no
  blocking or retest-pending checks; and
- the public-proof packet result class is compatible with the claim
  wording being widened.

If any condition fails, the correct action is to keep the packet
methodology-only, narrow the claim before publish, quarantine the
packet, or withdraw the existing claim. The packet must not compensate
with broader prose.

## Out of scope

This packet does not recruit partners, run benchmark launches, publish a
website, generate public-proof packets automatically, or declare any
specific archetype certified. It defines the packet shape and review
discipline those future actions must satisfy.
