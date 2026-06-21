# Problems / output / execution-evidence qualification: the M5 capstone gate

This packet is the **capstone qualification gate** that binds the M5 Problems-row,
output-channel, execution-evidence-projection, chronology, and fallback-drill lanes
into one promotion model. It answers a single release question: *can a claimed M5
tooling profile still ship its claim when the Problems / output / execution-evidence
causal evidence behind it has drifted, gone stale, or started failing?* Where the
upstream lanes each freeze one slice of the causal system, this packet grades each
**claimed tooling profile** against the whole chain and **auto-narrows** any profile
whose evidence can no longer back its claim.

Each claimed [`tooling_profile`](#tooling-profiles) — the Problems panel, output
channel, terminal runner, debug console, notebook output, pipeline overlay, AI-tool
evidence, and support export — carries one profile qualification graded across the
seven [`certification_dimension`](#certification-dimensions)s the source set treats as
**one causal chain, not seven independent checks**:

- **`problems_correlation`** — Problems rows correlate each finding to its source
  task, run, and owning output channel.
- **`output_channel_identity`** — output channels preserve provider/run/channel
  identity and the content trust class.
- **`evidence_projection_lineage`** — projected overlays preserve their
  run/step/provider/artifact lineage.
- **`causal_link_integrity`** — the structured-versus-heuristic causal chain stays
  unbroken and resolves to one canonical id.
- **`confidence_honesty`** — confidence labels do not overclaim; heuristic origins
  stay labeled.
- **`stale_superseded_handling`** — stale and superseded state stay visible rather
  than implied current.
- **`reopen_to_origin_parity`** — reopen-to-origin resolves to the canonical evidence
  across every surface.

It is the qualification capstone over the
[`m5-execution-evidence`](./m5-execution-evidence.md) **lane matrix**, the
[`m5-problem-records`](./m5-problem-records.md) **Problems row**, the
[`m5-execution-evidence-projections`](./m5-execution-evidence-projections.md)
**projected overlay**, the [`m5-chronology-reuse`](./m5-chronology-reuse.md)
**chronology entry**, the [`m5-output-channels`](./m5-output-channels.md) **output
channel**, and the [`m5-fallback-evidence-drills`](./m5-fallback-evidence-drills.md)
**drill corpus**. Every claimed profile draws its proof from those checked-in lane
support exports (`upstream_lane_refs`) rather than from a private causal model, and
all share one vocabulary — origin class, confidence tier, freshness state, and proof
currency are reused, not re-invented — so About/help, service-health, compatibility,
release, support, and AI surfaces ingest one qualification state instead of restating
tooling claims by hand.

If this doc, the
[`m5-problems-output-evidence-certification.schema.json`](../../schemas/tooling/m5-problems-output-evidence-certification.schema.json)
boundary, the frozen set under
[`/artifacts/tooling/m5-problems-output-evidence-certification/`](../../artifacts/tooling/m5-problems-output-evidence-certification/),
and the perturbation corpus under
[`/fixtures/tooling/m5-problems-output-evidence-certification/`](../../fixtures/tooling/m5-problems-output-evidence-certification/)
disagree, the machine-readable schema plus the checked-in support export
(`artifacts/tooling/m5-problems-output-evidence-certification/support_export.json`)
win, and this doc must update in the same change.

## Companion artifacts

- [`/schemas/tooling/m5-problems-output-evidence-certification.schema.json`](../../schemas/tooling/m5-problems-output-evidence-certification.schema.json)
  — boundary schema for the `m5_problems_output_evidence_certification_packet`
- [`/artifacts/tooling/m5-problems-output-evidence-certification/support_export.json`](../../artifacts/tooling/m5-problems-output-evidence-certification/support_export.json)
  — the canonical qualification packet (source of truth)
- [`/artifacts/tooling/m5-problems-output-evidence-certification/report.md`](../../artifacts/tooling/m5-problems-output-evidence-certification/report.md)
  — the generated qualification report
- [`/artifacts/tooling/m5-problems-output-evidence-certification/waiver-and-downgrade-log.md`](../../artifacts/tooling/m5-problems-output-evidence-certification/waiver-and-downgrade-log.md)
  — the release-visible auto-downgrade log
- [`/fixtures/tooling/m5-problems-output-evidence-certification/`](../../fixtures/tooling/m5-problems-output-evidence-certification/)
  — the perturbation corpus
- `tools/release/problems_output_evidence_certification.py` — validation and
  re-derivation engine
- `crates/aureline-runtime/src/certify_m5_problems_output_and_execution_evidence_truth/`
  — Rust truth source

## Tooling profiles

`problems_panel`, `output_channel`, `terminal_runner`, `debug_console`,
`notebook_output`, `pipeline_overlay`, `ai_tool_evidence`, `support_export`. This is
the same profile vocabulary frozen by the fallback-drill corpus, reused here rather
than re-invented.

## Certification dimensions

`problems_correlation`, `output_channel_identity`, `evidence_projection_lineage`,
`causal_link_integrity`, `confidence_honesty`, `stale_superseded_handling`,
`reopen_to_origin_parity`. Every claimed profile is graded against all seven; a
profile that omits a dimension fails closed.

## The qualification ladder and auto-narrowing

A profile names a `claimed_grade` (`qualified`, `limited`, or `labs_not_claimed`) and
the engine **re-derives** an `effective_grade` from the per-dimension evidence — it is
never trusted from the stored grade:

- **`qualified`** — every dimension's invariant holds and its proof is current and
  reopenable.
- **`limited`** — the honest ceiling for a read-only / overlay profile (an imported or
  pipeline-provider run), which never claims live local authority.
- **`retest_pending`** — honestly labeled proof has aged past the freshness window;
  the lineage stays reopenable until re-verified.
- **`blocked`** — a causal-link, confidence, identity, or reopen invariant is failing,
  a required dimension carries no proof, or a first-party profile leaned on imported
  proof for a live claim.
- **`labs_not_claimed`** — an unadvertised lane that makes no public claim and is
  never widened or narrowed.

A narrowed profile must carry a strictly lower effective grade than its claim, a
recorded [`narrow_trigger`](#narrow-triggers), and a precise narrowed label — never a
generic "unavailable"/"blocked". The waiver-and-downgrade log lists every profile held
below its claim; there are no manual waivers, only automatic narrowing.

### Narrow triggers

`problems_correlation_lost`, `output_channel_identity_flattened`,
`projection_lineage_flattened`, `causal_link_broken`, `confidence_overclaimed`,
`stale_evidence`, `superseded_state_hidden`, `reopen_path_lost`,
`missing_dimension_proof`, `imported_overlay_claims_live`, `upstream_lane_narrowed`.

## Release-evidence rows

The packet carries one explicit `release_evidence_row` per release-bearing integrity
axis — `causal_link_integrity`, `confidence_honesty`, `stale_superseded_handling`, and
`reopenable_evidence_parity` — rolling each axis up across the claimed profiles
(`profiles_holding`/`profiles_claimed` and the weakest effective grade) so a release
evidence packet carries those rows directly rather than re-deriving them.

## Consumer surfaces

The `consumer_surfaces` block asserts that About, help, service-health, compatibility,
release-evidence, support-export, and AI-evidence surfaces all ingest this one
qualification state, and that narrowed profiles are visibly labeled below their claim
on every surface.

## Freshness wired into release automation

The `evidence_freshness` block records the freshness SLO (hours), the last refresh,
and `auto_narrow_on_stale`. The release tool re-derives every profile's effective
grade from the current evidence, so stale or failing causal-link / confidence proof
narrows the affected claim automatically — a claim cannot coast on aged proof.

## Validation

`tools/release/problems_output_evidence_certification.py`:

- `validate` — re-derive each profile's effective grade from the support export and
  fail on any overclaim, drift, missing dimension, or release-evidence-row mismatch.
- `corpus` — re-derive every perturbation fixture against its expected outcome.
- `self-test` — `validate` plus `corpus`.

The Rust truth source re-derives the same effective grade and narrow trigger, so the
checked-in artifacts can never imply a wider claim than the current evidence backs.

## Regenerating the artifacts and fixtures

The Rust builder is the single source of truth. Regenerate the checked-in artifacts
and fixtures from it:

```bash
D=artifacts/tooling/m5-problems-output-evidence-certification
cargo run -p aureline-runtime --example dump_m5_problems_output_evidence_certification support > "$D/support_export.json"
cargo run -p aureline-runtime --example dump_m5_problems_output_evidence_certification report  > "$D/report.md"
cargo run -p aureline-runtime --example dump_m5_problems_output_evidence_certification waiver  > "$D/waiver-and-downgrade-log.md"
# The `corpus` mode prints { index, cases } for the fixtures under
# fixtures/tooling/m5-problems-output-evidence-certification/.
cargo run -p aureline-runtime --example dump_m5_problems_output_evidence_certification corpus
```
