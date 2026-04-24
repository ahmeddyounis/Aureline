# Docs / help / migration same-change-set workflow, stale-example detection rubric, and late-proof exception path

This document freezes the discipline that keeps public truth moving with
product truth instead of lagging behind until beta or stable hardening.
Three rules travel together:

1. **Same-change-set workflow** — the named product, schema, route,
   migration, claim, and known-limit changes that MUST update
   docs / help / release-note surfaces in the same change train they
   land on.
2. **Stale-example detection rubric** — machine-checkable detectors that
   label docs packs, guided steps, migration guides, screenshots,
   recipes, and generated examples as stale and route each kind to a
   declared downgrade or suppression behavior rather than manual memory.
3. **Late-proof exception packet** — the one-packet path for the rare
   case where proof for a release-bearing claim genuinely cannot land in
   the same change, carrying owner, rationale, impacted claim rows,
   degraded wording, support impact, expiry, and explicit reviewer
   approvals so the gap is visible and time-bounded instead of implicit.

The machine-readable boundaries are:

- [`/artifacts/docs/stale_example_rules.yaml`](../../artifacts/docs/stale_example_rules.yaml)
  — stale-example detection rubric and downgrade map that
  docs-pack publishers, parity audits, support exports, and shiproom
  review projections read.
- [`/schemas/docs/late_proof_exception_packet.schema.json`](../../schemas/docs/late_proof_exception_packet.schema.json)
  — boundary schema for the late-proof exception packet family.
- [`/fixtures/docs/late_proof_exceptions/`](../../fixtures/docs/late_proof_exceptions/)
  — worked examples of approved, in-review, and expired late-proof
  exceptions.

Related contracts this policy layers on instead of replacing:

- [`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../docs/reviewed_pack_and_late_copy_policy.md)
  — reviewed-pack and late-copy workflow for trust, legal, policy,
  recovery, support, and compatibility copy. Late-copy packets govern
  wording; this policy governs proof cadence and example freshness.
- [`/docs/docs/docs_pack_manifest_contract.md`](../docs/docs_pack_manifest_contract.md)
  — docs-pack manifest that already owns the pack-level source,
  version, locale, freshness, publishable-state, and stale-example
  label vocabulary.
- [`/docs/governance/claim_manifest_contract.md`](../governance/claim_manifest_contract.md)
  — claim-row publication contract for canonical public copy,
  effective-posture downgrade, and channel binding.
- [`/docs/governance/public_surface_truth_map.md`](../governance/public_surface_truth_map.md)
  — canonical source-of-truth map and drift-blocking rules. This
  policy names which of those drift severities MUST land in the same
  change set.
- [`/docs/governance/change_budget_workflow.md`](../governance/change_budget_workflow.md)
  — freeze-era protected-change policy. Late-proof exceptions live
  inside that budget envelope and cite its limits explicitly.
- [`/docs/governance/evidence_freshness_policy.md`](../governance/evidence_freshness_policy.md)
  — freshness rules that still govern the evidence and owner rows
  this policy references.
- [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — promotion gates that already reserve a same-change-set release
  bundle and a late-proof exception slot.

## Why freeze this now

The reviewed-pack contract closed how wording on a release-bearing
surface binds to reviewed source. The docs-pack manifest closed how a
pack's source / version / freshness / stale-example axes render without
interpretation drift. The claim-manifest contract closed how public
truth projects across docs, Help / About, service health, support
exports, release packets, CLI / help, evaluation artifacts, and
public-proof packets. The source-of-truth map named the owner artifacts
public surfaces MUST trace back to.

What remained implicit was the operational cadence that keeps those
rows from drifting in practice:

- *Which* product / schema / route / migration / claim / known-limit
  changes force a same-change-set update on which docs / help /
  release-note surfaces. Shiproom has been enforcing these pairings by
  memory.
- *How* a stale example is detected by tooling instead of by a reviewer
  noticing, and which downgrade or suppression action each kind of
  stale example (docs-pack example, guided step, migration guide,
  screenshot, recipe, generated example) triggers.
- *Where* proof that cannot land in the same train is recorded so the
  resulting degraded wording is visible, time-bounded, and approved
  rather than an unlogged "we'll update the docs later".

This policy closes that gap. It gives docs, release, support, and
public-truth lanes one same-change-set pairing table, one stale-example
rubric, and one late-proof exception packet family instead of a mix of
shiproom folklore, freeze-exception prose, and PR-checklist memory.

## Scope

Frozen at this revision:

- the named product / schema / route / migration / claim / known-limit
  change classes whose in-train pairing is required on specific
  docs / help / release-note surfaces;
- the stale-example detection rubric, including the artifact kinds
  covered, the detectors that fire, the downgrade / suppression actions
  each detector maps to, and the override / waiver semantics;
- one `late_proof_exception_packet` shape carrying owner, rationale,
  impacted claim rows, degraded wording, support impact, expiry, and
  required reviewer set; and
- reviewer, rollback, and reversal rules shared across docs, migration,
  support, release notes, CLI / help, evaluation, and public-proof
  lanes.

Out of scope until a superseding decision row opens:

- a full docs publishing system (pipeline, CMS integration, review
  tooling UI);
- automated emission of exception packets from release tooling;
- localization workflow automation and translation-memory integration;
  and
- replacing the reviewed-pack, claim-row, compatibility-row,
  destination-descriptor, or docs-pack manifest contracts.

## Same-change-set workflow

A change lands "in the same change set" when the docs / help /
release-note update rides the same commit train (branch, release-
candidate tag, or out-of-band hotfix) as the change that made the prior
wording untrue, and both enter the release packet or late-copy record
together. A change "in the same reviewed-pack version" is a stricter
sibling rule frozen by the reviewed-pack-and-late-copy policy; this
policy governs the coarser commit-train cadence.

### Surface vocabulary

Surfaces subject to same-change-set pairing are the release-bearing
surfaces the reviewed-pack policy already lists:

- docs pane
- docs browser
- Help / About
- service health
- migration notes
- support export
- release notes
- CLI / help
- evaluation artifacts
- public-proof packets

Marketing copy, blog posts, and external-press material are explicitly
out of scope; they ride on the public-truth claim-manifest projection,
not this policy.

### Pairing table

Every row below names a change class, the owner artifact the change
lives in, and the docs / help / release-note surfaces that MUST carry a
matching update in the same change set. If the paired update cannot
ride the same train, a late-proof exception packet (below) is the only
admissible path; silent divergence is non-conforming.

| Change class | Owner artifact | Required same-change-set updates |
|---|---|---|
| Product behavior change on a claim-bearing workflow | the crate / feature that owns the behavior | docs pane, release notes, and — if the change narrows or widens an effective-claim posture — the `claim_row` canonical copy |
| Schema version bump (additive-minor or breaking) on a public schema | the schema file under `/schemas/**` | docs pane section on that schema, release notes schema-change line, migration notes entry if the bump is breaking, and — for claim-bearing schemas — the paired `claim_row` evidence link |
| Route change (URL, boundary, handoff, handoff reason) on a `destination_descriptor` | `artifacts/docs/destination_descriptor_seed.yaml` or a later owner | Help / About disclosure, service-health route row, docs pane / docs browser footer, and the `help_status_badge_record` consumers that resolve it |
| Migration step (required, optional-but-recommended, known-gotcha) | the migration-notice pack | migration notes, release notes "what's new / what's breaking" section, support export migration entry, and — when a `compat_row` narrows — the paired compatibility-row field |
| Claim row change: posture downgrade, canonical-copy edit, evidence-link edit, lifecycle transition | `schemas/governance/claim_manifest.schema.json` consumers | every cross-channel destination the row declares (docs pane, Help / About, service health, support export, release notes, CLI / help, evaluation artifacts, public-proof packets) |
| Known-limit or exclusion note: added, withdrawn, or reworded | the claim row's `known_limits` / `exclusion_notes` slot | docs pane known-limits section, release notes known-limits list, support export known-limit block, and the paired claim-row row |
| Compatibility scope change: support-window language, skew window, deviation text | `compat_row` | migration notes, release notes compatibility section, support export compatibility block, and the paired `claim_row` if a claim cites the compat row |
| Service-health state change: plane state, contract state, operator-visible boundary | the service-health owner | service-health view, Help / About boundary disclosure, support export header, and the paired `claim_row` if one covers the plane |
| Lifecycle transition: promotion, demotion, policy-disable, quarantine, deprecation | the capability-lifecycle row | docs pane capability state, release notes lifecycle section, CLI / help lifecycle chip, and the paired `claim_row` effective posture |

### Detection and enforcement

Pairings are enforced by the contract-validation lane
(`.github/workflows/contract_validation.yml`, see the control-artifact
index row `contract_artifact_validation_lane`) and by shiproom
dashboard review:

1. A commit train that changes an owner artifact row in a change class
   above MUST include the paired surface update OR a
   `late_proof_exception_packet` in `proposed` or `approved` status
   that cites the change and the train.
2. A release packet that fails to show the pairing or an approved
   exception is non-conforming and the release row defaults to the
   narrower outcome (narrow the claim, route to known-limit, or hold
   the train) per the launch decision register defaults.
3. Shiproom dashboard projects any open or expired exception packet
   against the release packet so the gap is never implicit.

### What does *not* require same-change-set pairing

- Renames, typo fixes, markup-only churn, and internal-comment edits on
  docs that do not change the authoritative wording a claim row, route,
  or migration step cites.
- Internal-only evidence packet refreshes that do not change effective
  claim posture, support window, compat window, or known-limit text.
- Tooling / build / test changes that do not touch a public schema,
  route, claim row, migration step, lifecycle transition, or
  service-health state.

These changes may still update docs, but the cadence is not governed
here.

## Stale-example detection rubric

The rubric lives in
[`/artifacts/docs/stale_example_rules.yaml`](../../artifacts/docs/stale_example_rules.yaml)
as the authoritative machine-readable register. This section is the
narrative companion and MUST be updated together with the YAML.

### Artifact kinds covered

- **Docs-pack examples.** Every example enumerated by a
  `docs_pack_manifest_record.example_summary` entry with an
  `example_label_class`. The docs-pack manifest contract already
  closes the label and reason vocabulary; this rubric extends it with
  detectors and paired downgrade actions.
- **Guided steps.** Step lists inside onboarding, tour, and first-run
  surfaces that instruct a user to run a command, open a route, or
  accept a provider handoff.
- **Migration guides.** Prose and inline snippets inside migration-
  notice packs and release-notes migration blocks.
- **Screenshots.** Bitmap or vector captures that render a UI element,
  chip, chrome, or route. Screenshots are stale when the rendered
  surface has moved out from under them.
- **Recipes.** Automation-recipe bodies, AI-surface recipes, and
  recipe-response surfaces that encode a runnable sequence.
- **Generated examples.** Examples produced by a generator (reference
  corpora, generated-reference packs, OpenAPI-derived snippets,
  schema-driven examples).

### Detector-to-action map

The YAML register binds each detector to exactly one downgrade or
suppression action. The action classes are:

| Action class | Behaviour |
|---|---|
| `label_stale_with_reason` | Render the example with a typed stale disclosure on the primary surface; the docs-pack manifest `example_summary` flips the entry to `stale_example` with the stale reason the detector declared. |
| `label_needs_review` | Render with a typed "pending reviewer re-verification" disclosure; the entry flips to `needs_review_example` until a reviewer relabels it `stable_example` or `stale_example`. |
| `quarantine_example` | Hide from user surfaces but retain in the manifest / pack body for parity audits; the entry flips to `quarantined_example`. |
| `suppress_from_surface` | Hide the example from the rendering surface entirely (guided step is skipped, screenshot is not displayed, recipe is not offered). The authoring pack still carries it so export preview can show why. |
| `block_pack_publishable` | Escalate to the docs-pack `publishable_state` gate: the pack moves to `blocked` with the `stale_examples_exceed_threshold` blocking reason (or the more specific reason the detector declared). |
| `require_late_proof_exception_packet` | When the detector fires on a release-bearing surface inside a train that cannot carry a same-change-set fix, the update may only proceed under an approved `late_proof_exception_packet` that names the detector, the impacted claim row, the degraded wording, and the expiry. |

Tooling reads the YAML and emits detector events against the docs-pack
manifest, the parity-audit projection, and the support export preview.
A detector event that cannot be paired with an action is a validation
failure and the pack / guide / recipe / screenshot is not publishable.

### Override / waiver semantics

Any suppression or downgrade MAY be overridden by an explicit
`stale_example_review_override` record inside the rubric that names the
reviewer, the reason, and the expiry. An override that would cause a
release-bearing surface to render stale text as current truth is only
admissible under a paired `late_proof_exception_packet`; a standalone
override cannot silently widen truth.

### Interaction with the reviewed-pack policy

The stale-example rubric is about example and step freshness. The
reviewed-pack policy is about wording on release-bearing surfaces. A
stale example whose fix is blocked on reviewed source changes MUST
ride a late-copy packet (for the corrected wording) and MAY ride a
late-proof exception packet (for the degraded wording). The two packet
families compose: late-copy governs what the text says, late-proof
governs what proof backs it and when the proof will land.

## Late-proof exception packet

Normal cadence requires that claim-bearing proof (evidence packets,
benchmark runs, compatibility reports, migration evidence, docs-pack
refresh, support-export preview) lands in the same change train as the
wording it backs. The late-proof exception packet is the one-packet
path when proof cannot land in time but the wording still must move.
The boundary schema is
[`/schemas/docs/late_proof_exception_packet.schema.json`](../../schemas/docs/late_proof_exception_packet.schema.json).

### When a late-proof exception is required

A packet MUST be opened when all of these are true:

1. a same-change-set pairing row above applies to the change;
2. the paired update would ride this train; but
3. at least one piece of required proof (benchmark corpus re-run, fresh
   compat report, migration evidence, docs-pack refresh, support-export
   preview) cannot land in the same train.

A packet MUST NOT be opened to widen a claim. Widening always requires
proof first.

### Packet requirements

Every `late_proof_exception_packet` carries:

- stable `packet_id` and `opened_at` timestamp;
- owner DRI and backup owner;
- rationale (free-form text) and a closed `cause_class` from the
  vocabulary in the schema;
- linked claim-row refs, linked compatibility-row refs, and linked
  docs-pack manifest refs whose proof is missing or stale;
- the `degraded_wording` block naming the surfaces that will render
  narrowed, stale, or preview wording while the packet is active, the
  binding-state label each surface flips to, and the text digests
  so silent edits can be detected;
- the `support_impact` block naming the support paths that must be
  aware of the degraded state;
- an `expiry_at` timestamp and a declared `reversal_class`;
- the required reviewer set (at minimum: docs / public-truth owner,
  release owner, plus the proof owner whose artifact is missing, plus
  the claim / compat / docs-pack owner whose row is impacted); and
- status (`proposed`, `approved`, `active`, `proof_landed`, `expired`,
  `withdrawn`, `rejected`).

### Reviewer requirements

| Cause class | Minimum required reviewers |
|---|---|
| `benchmark_proof_deferred` | docs / public-truth owner, release owner, benchmark owner |
| `compatibility_proof_deferred` | docs / public-truth owner, release owner, compatibility owner |
| `migration_evidence_deferred` | docs / public-truth owner, release owner, migration owner |
| `docs_pack_refresh_deferred` | docs / public-truth owner, release owner, docs-pack publisher |
| `support_export_preview_deferred` | docs / public-truth owner, release owner, support owner |
| `claim_evidence_alignment_deferred` | docs / public-truth owner, release owner, and the claim-row owner |

Packets may add extra reviewers (incident commander, legal / policy
owner, security / trust owner) but may not omit the minimum set for
the declared cause class.

### Expiry, rollback, and reversal

Every packet MUST declare how the exception ends:

- `restore_prior_wording` — when the proof never lands, the surface
  reverts to the prior reviewed wording and the claim narrows.
- `land_proof_and_drop_exception` — proof lands before expiry; the
  packet moves to `proof_landed` and the surface flips to
  `reviewed_current`.
- `withdraw_claim_and_route_to_known_limit` — the claim is withdrawn
  and the surface renders the known-limit path instead.
- `supersede_with_new_exception` — an approved successor packet takes
  over; the prior packet moves to `superseded` and the successor
  inherits the impacted rows and expiry budget.

A packet that remains `active` past `expiry_at` without a successor or
reversal is non-conforming and the release row defaults to the
narrower outcome per the launch decision register defaults. Permanent
"temporary" late-proof exceptions are non-conforming.

### Shiproom review linkage

Open and expired late-proof exception packets project into the
shiproom dashboard alongside waivers and freeze exceptions (see
`artifacts/release/shiproom_dashboard_seed.yaml`). The release packet
quotes the packet ids rather than narrating the gap ad hoc; reviewers
see one inspectable list rather than meeting-notes memory.

## Change discipline

- Adding a new same-change-set pairing row is additive-minor and
  requires this document and `artifacts/docs/stale_example_rules.yaml`
  to update together when the row touches an example / step / screenshot
  surface.
- Adding a detector, action class, or cause class bumps
  `stale_example_rules.schema_version` or
  `late_proof_exception_packet_schema_version`. Repurposing an existing
  value is breaking and requires a new decision row.
- Retiring a row requires a supersession note naming the successor and
  the milestone it lands under.
- Milestone and task identifiers stay out of every user-facing string
  this policy produces; the pairing, detector, and exception vocabulary
  describes *what* changed, not *which* planning row it came from.
