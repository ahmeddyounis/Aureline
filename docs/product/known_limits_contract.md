# Known-limits, exclusions, and downgrade-notes contract

This document is the reviewer-facing contract that makes caveats first-
class release objects. It names the vocabulary every known-limit /
exclusion / downgrade note binds to, the scope the note must declare,
the propagation a note forces on the eleven shared publication
destinations, and the review rubric that decides whether a note is
mandatory, optional, stale, superseded, or promotion-blocking. The goal
is simple: claims cannot outrun the exclusions, support posture, or
downgrade conditions they declare, and the mismatch between claim text
and evidence state is machine-detectable rather than a prose-memory
exercise.

Tooling reads the companion artifacts; this document sets the reader
expectations that bind them together. If this document and the
artifacts disagree, the artifacts are authoritative and this document
is updated in the same change.

Companion artifacts:

- [`/schemas/product/known_limit_note.schema.json`](../../schemas/product/known_limit_note.schema.json)
  — boundary schema for one known-limit note record. Every channel
  that cites a note resolves against this schema.
- [`/artifacts/product/known_limit_classes.yaml`](../../artifacts/product/known_limit_classes.yaml)
  — machine-readable register naming the closed vocabularies, the
  limitation-class rows, the review-rubric rows, the propagation
  rules, the detection rules, and the invariants this contract
  enforces.
- [`/schemas/release/assurance_claim.schema.json`](../../schemas/release/assurance_claim.schema.json)
  — assurance-claim row schema. Carries `known_limit_refs`; this
  contract defines what those refs resolve to.
- [`/schemas/governance/claim_manifest.schema.json`](../../schemas/governance/claim_manifest.schema.json)
  — public-truth claim-manifest schema. Declares `known_limit_note`
  as an `evidence_kind` and `known_limit_missing` as a downgrade
  trigger; this contract is the authoritative source of those notes.
- [`/schemas/qe/public_proof_packet.schema.json`](../../schemas/qe/public_proof_packet.schema.json)
  — public-proof packet schema. Carries a
  `known_limits_and_exclusions` block whose `declared_class` mirrors
  `limitation_class_vocabulary` verbatim.
- [`/artifacts/governance/evidence_freshness_slos.yaml`](../../artifacts/governance/evidence_freshness_slos.yaml)
  — freshness SLOs and stale-propagation profiles. A known-limit
  note's freshness is evaluated under the same metadata rules as
  every other evidence packet.
- [`/artifacts/governance/evidence_rerun_triggers.yaml`](../../artifacts/governance/evidence_rerun_triggers.yaml)
  — rerun-trigger catalog. Every note names at least one trigger so
  a mismatch expires the note immediately even while `stale_after`
  remains in window.
- [`/artifacts/product/replacement_grade_cutlines.yaml`](../../artifacts/product/replacement_grade_cutlines.yaml)
  — cutline rows. Every note that narrows replacement-grade, daily-
  driver, or self-host wording on a cutline lists the cutline in
  `affected_cutline_refs`.
- [`/artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
  — seeded claim-manifest rows that cite `known_limit_note` evidence
  kinds.
- [`/artifacts/release/assurance_claim_rows.yaml`](../../artifacts/release/assurance_claim_rows.yaml)
  — assurance-claim rows whose `known_limit_refs` list resolves
  through this contract.

## Why this contract exists

The assurance-claim matrix already binds every row to a declared and
effective claim class, a closed proof-class breakdown, explicit
exclusion refs, a docs-version match floor, and downgrade triggers. The
claim manifest already binds each public-truth row to requirement ids,
evidence packets, channel bindings, support-window state, and
known-limit refs. What was missing:

- a **first-class shape** for one known-limit / exclusion / downgrade
  note so every channel resolves the same note rather than minting
  channel-local caveat copy;
- a **closed review rubric** that states when the note is mandatory,
  optional, stale, superseded, or promotion-blocking so reviewers do
  not relitigate the decision for every claim row;
- a **support-window alignment block** so support-window or support-
  class language cannot promise more than the linked known-limit and
  evidence state allow;
- **machine-detectable** conditions for missing, stale, or superseded
  notes so promotion review fires on the rules rather than on prose
  memory.

This contract closes those gaps without writing any end-user note. The
notes themselves land in the docs, release-notes, claim-manifest, and
public-proof-packet channels as downstream work; this contract freezes
the shape and the rules that govern them.

## Note record shape

Every known-limit / exclusion / downgrade note is one record conforming
to `schemas/product/known_limit_note.schema.json`. The record binds:

- a stable `known_limit_id` quoted verbatim by docs, Help/About,
  release notes, claim manifests, compatibility reports, benchmark
  packets, marketplace / discovery surfaces, and support exports;
- exactly one `limitation_class` drawn from the closed vocabulary
  (see below);
- exactly one `severity_class` and exactly one `review_rubric_class`;
- a `promotion_blocking` flag forced to `true` by a
  `severity_class = blocking` and by `review_rubric_class =
  promotion_blocking`, and forced to `false` by `review_rubric_class
  = optional`;
- the scope the note narrows, split across `affected_workflow_bundle
  _refs`, `affected_archetype_row_refs`, `affected_persona_refs`,
  `affected_cutline_refs`, `affected_deployment_profiles`, and
  `affected_release_channels`;
- an `explicit_exclusions` list whose contents are mandatory for
  competitor-parity, corpus, migration-path, and support-export
  narrowing classes;
- a `downgrade_triggers` list drawn from the shared downgrade-trigger
  vocabulary, extended with three known-limit-specific triggers
  (`known_limit_note_missing`, `known_limit_note_stale`,
  `known_limit_note_superseded`);
- a typed `workaround` block naming a
  `workaround_availability_class` plus an opaque `workaround_ref`;
- a typed `support_window_alignment` block pinning a
  `support_class_floor`, a `support_class_ceiling`, a
  `support_window_state`, and a `support_window_ref`;
- a freshness envelope with `captured_at`, `stale_after`,
  `cadence_class`, `proof_class`, and `rerun_trigger_refs`;
- a `mandatory_publication_destinations` list drawn from the shared
  publication-destination vocabulary;
- a `binding_claim_row_refs` list naming the claim-manifest rows the
  note binds to;
- an `owner_ref` plus `evidence_owner_ref` lane or handle.

Raw marketing copy, raw vendor / account / hostname identifiers, raw
reviewer notes, and raw credential material never cross this boundary.
The record carries opaque refs, typed vocabulary, UTC timestamps, and
enumerated classes only.

## Vocabulary

### Limitation class

| Value | Meaning | Required scope fields | Claim-class ceiling |
|---|---|---|---|
| `scope_caveat_noted` | Scope caveat. Does not narrow a claim class by itself. | (none required) | `certified` |
| `platform_narrowed` | One or more deployment profiles are excluded or narrowed. | `affected_deployment_profiles` | `limited` |
| `workflow_narrowed` | One or more workflow bundles are excluded or narrowed. | `affected_workflow_bundle_refs` | `supported` |
| `persona_narrowed` | A P0 persona is excluded or narrowed. | `affected_persona_refs` | `experimental` |
| `corpus_narrowed` | Benchmark or migration corpus scope is narrower than the claim implies. | `explicit_exclusions` | `limited` |
| `competitor_parity_narrowed` | Head-to-head parity is intentionally narrow or quarantined. | `explicit_exclusions` | `limited` |
| `docs_drift_active` | Docs / help version match is broken or stale on a bound surface. | (none required) | `evidence_stale` |
| `support_export_narrowed` | `support_export` cannot redact / handoff the narrowed scope honestly. | `explicit_exclusions` | `limited` |
| `migration_path_limited` | Migration evidence is absent, partial, or quarantined. | `explicit_exclusions` | `limited` |

`no_limits_declared` is reserved for packet-level declarations on
public-proof packets and is never a valid class on a known-limit note.
A packet that declares `no_limits_declared` MUST carry an empty notes
list (schema-enforced in `schemas/qe/public_proof_packet.schema.json`).

### Severity class

| Value | Meaning |
|---|---|
| `minor` | Scope caveat only; no claim-class narrowing required. |
| `moderate` | Narrows a bound claim to `limited` or `evidence_stale` on at least one destination. |
| `major` | Narrows a bound claim to `experimental`, `retest_pending`, or `exception_recorded`. |
| `blocking` | Forbids `certified`, `supported`, or replacement-grade wording until cleared or waived. |

`severity_class = blocking` MUST pair with
`review_rubric_class = promotion_blocking` (schema-enforced).

### Review rubric class

| Value | When it applies | Effect on bound claim rows | Promotion-blocking |
|---|---|---|---|
| `mandatory` | Limitation class requires the note on every publishing claim row. | Claim MUST cite the note; wording narrows to `claim_class_ceiling`. | No |
| `optional` | Limitation class is `scope_caveat_noted` with `minor` severity. | Claim MAY cite the note; omission admissible. | No |
| `stale` | Freshness envelope has expired or a rerun-trigger mismatch has fired. | Claim narrows to `evidence_stale` on every destination. | Yes |
| `superseded` | A successor note has landed; only the successor is citable. | Claim rows migrate `known_limit_refs` in the same change. | No (migration only) |
| `promotion_blocking` | `severity_class = blocking`, or the class forbids the declared claim class at its scope. | Wording narrows to `claim_class_ceiling`; release train is blocked. | Yes |

### Note state

`draft` → `in_review` → `active` → (`superseded` | `withdrawn` |
`quarantined`). `draft` and `in_review` states carry null `activated
_at`, null `withdrawn_at`, and null `superseded_by_known_limit_ref`.
`active` carries a non-null `activated_at`. `superseded` names a
successor via `superseded_by_known_limit_ref`. `withdrawn` carries a
non-null `withdrawn_at`. `quarantined` narrows every bound claim row to
`limited` or narrower until cleared.

### Workaround availability

| Value | Meaning | `workaround_ref` |
|---|---|---|
| `available_in_product` | A user-visible setting, command, or repair hook exists. | Non-null |
| `available_via_recipe` | The workaround is a documented CLI / headless recipe. | Non-null |
| `available_via_policy_change` | The workaround requires an admin policy change. | Non-null |
| `unavailable_pending_work` | Work is tracked but not yet landed. | Null |
| `unavailable_by_design` | No workaround is admissible; the limit is a scope boundary. | Null |

### Downgrade triggers

The first twelve values are re-exported verbatim from
`schemas/release/assurance_claim.schema.json#downgrade_trigger`. The
three known-limit-specific triggers are additive and named by this
contract:

- `known_limit_note_missing` — a bound note is not cited by a claim
  row that projects through a mandatory destination for the note.
- `known_limit_note_stale` — the note's freshness has expired or a
  rerun-trigger mismatch has fired.
- `known_limit_note_superseded` — the claim row cites a note whose
  `note_state = superseded` and the successor has not been migrated
  in.

## Propagation rules

A known-limit note binds to at least one publication destination. The
full propagation matrix lives in
[`/artifacts/product/known_limit_classes.yaml#propagation_rules`](../../artifacts/product/known_limit_classes.yaml);
the narrative summary below names the invariants.

- **`docs_site`**: mandatory whenever the note binds to any docs-site
  claim row. `stale` and `promotion_blocking` render `stale_text` and
  a `narrowed_summary`. `superseded` redirects the anchor id to the
  successor note's body.
- **`migration_notes`**: mandatory when `limitation_class` is
  `migration_path_limited` or `competitor_parity_narrowed`. `stale`
  and `promotion_blocking` force a `migration-evidence-missing` or
  `migration-evidence-stale` trigger against
  `evidence_rerun_triggers.yaml`.
- **`help_about`**: mandatory whenever the note binds to any Help /
  About claim row. `stale` degrades the freshness badge to `stale`.
  `promotion_blocking` renders `limited_text`. `superseded` rewrites
  the badge to the successor note id.
- **`service_health`**: mandatory when the note narrows a service-
  contract state. `stale` downgrades to `stale`;
  `promotion_blocking` downgrades to `contract_mismatch`.
- **`support_export`**: mandatory when
  `support_window_alignment.support_window_state` is `narrowed` or
  `expired`, or when `limitation_class = support_export_narrowed`.
  The support-handoff card cites the note id and narrows the
  `support_class` wording to the note's ceiling. A support export
  that cannot render the narrowed posture fails closed and is a
  release blocker.
- **`release_packet`**: mandatory whenever the note binds to any
  release-packet claim row. The packet carries the note id, the
  `downgrade_triggers` list, and the narrowed posture in its claim-
  evidence-link block.
- **`release_notes`**: mandatory when `severity_class` is `moderate`
  or higher. `stale` renders `stale_text`. `promotion_blocking`
  renders the `narrowed_summary` and routes to the `known_limit`
  anchor.
- **`cli_help`**: mandatory when the note binds to a CLI-surface
  claim row. Renders a one-line note id plus a resolvable anchor into
  the `docs_site` body.
- **`evaluation_artifact`**: mandatory when `limitation_class` is
  `corpus_narrowed` or `competitor_parity_narrowed`. `stale`
  quarantines the artifact until refresh; `promotion_blocking`
  quarantines it until the evidence lands.
- **`marketplace_discovery`**: mandatory when `limitation_class` is
  `persona_narrowed`, `competitor_parity_narrowed`, or
  `support_export_narrowed`. `stale` degrades the row to
  `unverified`; `promotion_blocking` removes it from public
  discovery until cleared.
- **`public_proof_packet`**: mandatory whenever the note binds to any
  public-proof packet. A packet whose bound note is `stale`,
  `superseded`, or `promotion_blocking` MUST carry the corresponding
  downgrade trigger in `active_downgrade_reasons` and MUST NOT
  declare `result_class = pass_full_proof`.

`docs_site` and `help_about` are mandatory for every note whose
`review_rubric_class` is `mandatory`, `promotion_blocking`, `stale`,
or `superseded` (schema-enforced). Public-facing wording narrowed by
an active note cannot skip the two destinations every end user reads.

## Support-window alignment

Every known-limit note carries a `support_window_alignment` block with
`support_class_floor`, `support_class_ceiling`, `support_window_state`,
and `support_window_ref`. The block bounds the support-class wording
any bound claim row may project:

- A claim row projecting `certified` wording while the note's
  `support_class_ceiling` is `supported` is non-conforming; promotion
  review fails closed.
- A note whose `support_window_state = expired` MUST pair with
  `review_rubric_class` in {`promotion_blocking`, `stale`,
  `superseded`} or `note_state` in {`superseded`, `withdrawn`,
  `quarantined`} (schema-enforced). Current-window wording cannot
  promise more than the linked evidence state allows.
- A note with `support_window_state` in {`narrowed`, `expired`} MUST
  list `support_export` in `mandatory_publication_destinations`
  (schema-enforced) so support handoffs see the narrowed posture.

The `support_window_ref` is an opaque pin into the support-window row
the note aligns to; raw support-window text lives in
`artifacts/support/` and the docs known-limits pages rather than in
this record.

## Review rubric

### When is a note mandatory?

A note's `review_rubric_class` is `mandatory` when the
`limitation_class` forces the citation on every publishing claim row.
The canonical cases are:

- `platform_narrowed` with any claim row that projects through a
  destination the excluded deployment profile serves;
- `workflow_narrowed` with any claim row that names the affected
  bundle;
- `persona_narrowed` with any claim row whose `canonical_copy` names
  the affected persona;
- any class whose `severity_class` is `moderate` or higher.

### When is a note optional?

A note is `optional` only when `limitation_class = scope_caveat_noted`
and `severity_class = minor`. A reviewer MAY still cite it to make the
scope caveat explicit; omission is admissible.

### When is a note stale?

Freshness is evaluated from metadata alone under
`evidence_freshness_slos.yaml#metadata_evaluation_contract`. A note is
`stale` when:

- `captured_at + stale_after` is in the past, OR
- any `rerun_trigger_refs` entry has fired (a rerun-trigger mismatch
  expires the note immediately even while `stale_after` remains in
  window).

A `stale` note forces every bound claim row's `effective_claim_class`
to `evidence_stale` and forces every rendering destination to render
`stale_text` until the note refreshes or is superseded.

### When is a note superseded?

A note is `superseded` when a successor note has landed and the
current note's `note_state = superseded` names the successor via
`superseded_by_known_limit_ref`. No claim row publishes while its
`known_limit_refs` cites the superseded row; migration to the successor
lands in the same change as the supersede.

### When is a note promotion-blocking?

A note is `promotion_blocking` when:

- `severity_class = blocking`, OR
- `limitation_class` is one whose required evidence is absent at a
  scope that forbids the declared claim class — canonical cases are
  `persona_narrowed` on a replacement-grade cutline, `corpus_narrowed`
  or `competitor_parity_narrowed` on a benchmark claim,
  `docs_drift_active` on a certified claim, `support_export_narrowed`
  on any supported-or-better wording, and `migration_path_limited` on
  replacement-grade wording that names a competing tool.

A `promotion_blocking` note narrows the effective claim class to the
limitation-class `claim_class_ceiling`, raises the corresponding
downgrade trigger on every bound claim row, and blocks the release
train until the note is cleared or an `exception_recorded` packet
lands and is cited.

## Machine detection

`artifacts/product/known_limit_classes.yaml#detection_rules` names the
six detectors a later validator reads to drive promotion review without
re-parsing prose bodies:

- `detector:missing_required_note_on_claim_row` — a claim row projects
  through a mandatory destination for a bound note but does not cite
  the note id.
- `detector:stale_note_freshness_expired` — a note's freshness
  envelope has expired or a rerun-trigger mismatch has fired.
- `detector:superseded_note_still_cited` — a claim row cites a note
  whose `note_state = superseded`.
- `detector:support_window_narrower_than_claim` — a claim row
  projects support-class wording stronger than the bound note's
  `support_class_ceiling` allows.
- `detector:promotion_blocking_note_on_high_bar_claim` — a claim row
  declares `certified`, `supported`, or `replacement_grade_candidate`
  and is bound to a `promotion_blocking` note.
- `detector:mandatory_publication_destination_skipped` — a publication
  destination renders a claim row bound to a note whose
  `mandatory_publication_destinations` includes that destination, but
  the rendered copy does not cite the note id or its narrowed
  projection.

Each detector maps to exactly one downgrade trigger and exactly one
effect on the release train (`narrow_to_...`, `block_promotion_until
_...`, or `no_render_until_...`). Promotion review reads the detector
output; the underlying prose bodies are not required.

## Change discipline

- **Add a new `limitation_class`, `severity_class`,
  `review_rubric_class`, `workaround_availability_class`, or
  `downgrade_trigger` value** by opening a decision row in
  `artifacts/governance/decision_index.yaml`, updating
  `schemas/product/known_limit_note.schema.json` and
  `artifacts/product/known_limit_classes.yaml` in the same change,
  and bumping the document-level `known_limit_note_schema_version`
  const. Tightening the wording on an existing value is additive-
  minor and lands without a decision row.
- **Add a new note** by creating a record conforming to
  `schemas/product/known_limit_note.schema.json` with a stable
  `known_limit_id`, a limitation class from the vocabulary, the
  required scope fields populated for that class, a review rubric,
  a severity class, a workaround block, a support-window alignment
  block, a freshness envelope, and at least one mandatory
  publication destination. Notes that narrow any already-published
  claim row update that claim row's `known_limit_refs` in the same
  change.
- **Narrow or withdraw a note** by moving the `note_state` through
  the superseded, withdrawn, or quarantined transition. `superseded`
  MUST name a successor via `superseded_by_known_limit_ref` and
  every bound claim row MUST migrate its `known_limit_refs` in the
  same change. `withdrawn` MUST carry a `withdrawn_at` timestamp.
- **Refresh a note** by updating `captured_at`, `stale_after`, and
  `rerun_trigger_refs` under the same `known_limit_id`. A refresh is
  additive-minor unless the refresh also changes the
  `limitation_class`, `severity_class`, or `support_class_ceiling`,
  in which case the change opens a decision row.

## What this contract does not do

- It does not write every end-user known-limit note. Writing the
  individual notes that cover docs, release notes, claim manifests,
  compatibility reports, benchmark packets, marketplace / discovery
  surfaces, and support exports is downstream work.
- It does not implement the runtime detection pipeline. The detector
  rules in the machine file are expressed so a later validator can
  drive promotion review from them.
- It does not replace the assurance-claim matrix or the claim
  manifest. Those contracts keep their vocabularies and their
  channel-binding rules; this contract freezes the shape that the
  `known_limit_refs` list on every claim row resolves to.
- It does not guarantee that every published claim row will carry
  every possible caveat at the first stable cut. The review rubric
  is the decision, not the coverage mandate.
