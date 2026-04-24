# Launch decision register

This document is the human-readable companion to the canonical
launch decision register. The register exists so that every
launch-critical decision has one owner, one deadline, one narrower
default, one affected-artifact list, and one linked premise — and so
that no launch-critical ambiguity survives the milestone-zero review
cycle as background scope.

Companion artifacts:

- [`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
  — machine-readable register. Tooling reads this file; the narrative
  below describes the same rows.
- [`/artifacts/governance/decision_defaults.yaml`](../../artifacts/governance/decision_defaults.yaml)
  — narrower-default automation: posture vocabulary, auto-application
  rules, reviewer-guidance classes, and report-projection rules.
- [`/schemas/governance/decision_register.schema.json`](../../schemas/governance/decision_register.schema.json)
  — schema the register conforms to.
- [`/schemas/governance/decision_defaults.schema.json`](../../schemas/governance/decision_defaults.schema.json)
  — schema the defaults file conforms to.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — rolling architecture-ADR decision index. Launch-register rows
  cite rolling-index rows via `linked_decision_index_rows`; the two
  registers share no duplicate status fields.
- [`/docs/governance/decision_workflow.md`](./decision_workflow.md)
  — linkage, narrowing, and supersession rules for the rolling index.
  The launch register reuses the same posture vocabulary and the
  same append-only history discipline.
- [`/docs/governance/commitment_and_rebaseline_policy.md`](./commitment_and_rebaseline_policy.md)
  — commitment classes, phase budgets, and rebaseline rules.

**One register, one decision id.** Architecture-pack sections and
milestone-scorecard rows cite the launch-register `LR-NNNN` id
directly. They do not mint a parallel status field. When a launch
row's rolling-index anchor (`D-NNNN`) closes, the launch row closes
through the same record; when the launch row fires its narrower
default, every downstream artifact picks the narrowed scope up
verbatim from `narrower_default.description`.

## Why this register exists

The rolling decision index (`artifacts/governance/decision_index.yaml`)
tracks architecture and contract decisions and closes through ADRs
under `docs/adr/`. That register is the right home for "what does the
buffer do on save," "what does the RPC envelope look like," "which
windowing API do we pick."

The launch register handles a different shape: decisions whose
ambiguity is **product-claim ambiguity**, not
contract-shape ambiguity. "What languages do we commit to at
launch." "What do we claim publicly about compatibility." "What is
the spend envelope on the AI provider." "Which deployment profiles
ship as launch-bearing." These decisions need the same freeze-date
discipline and the same narrower-default automation as architecture
decisions, but they close through product-scope review packets or
release-council rulings rather than ADRs. The launch register is
their canonical home.

## How rows move

Every launch-register row uses the same six-state vocabulary as the
rolling index, and fires the same narrower-default posture set on
`applies_on`:

- **`open`** — the decision is recognised but no forum has picked it
  up. Every seeded row begins here.
- **`deciding`** — the forum has opened an ADR / RFC / scope-review
  packet and is actively running down the option space.
- **`decided`** — the closing artifact has landed and is linked. For
  `decision_vehicle_class: adr_or_rfc` rows, `linked_adr` points at
  the ADR. For `sponsor_scope_decision` and `release_decision` rows,
  `linked_adr` stays null and `decision_history` carries an `accept`
  entry naming the scope-review packet or release-council ruling.
- **`deferred`** — the forum has accepted that the decision cannot
  land at the target milestone and has restated `freeze_by_date`.
- **`narrowed_by_default`** — `applies_on` passed while the row was
  still open or deciding; the row's `narrower_default.posture` fired
  and the row is now bound by `narrower_default.description`.
  Reopening a wider scope mints a **new** `LR-NNNN` id; the narrowed
  row survives as audit.
- **`superseded`** — a later `LR-NNNN` replaces this one. The
  original row is not deleted; its `decision_history` preserves the
  transition.

Posture vocabulary (identical to the rolling index):

- **`narrow`** — reduce committed scope to the row's narrowed
  statement. The default for product-scope rows.
- **`defer`** — push the decision to a later milestone with a
  restated freeze. The default for release-scope rows that
  explicitly require a later ruling.
- **`freeze_lane`** — block dependent work on the affected lane
  until the ADR lands. The default for accessibility and
  compatibility-bridge rows whose downstream lanes must not ride
  an unclosed posture.
- **`rebaseline`** — trigger a milestone rebaseline review. Not
  currently seeded; reserved for decisions whose narrowing would
  force broader scope cuts across the milestone.

Automation rules in
[`/artifacts/governance/decision_defaults.yaml`](../../artifacts/governance/decision_defaults.yaml)
name when each posture fires automatically (no forum sign-off
required) versus when it holds `deciding` until a named forum signs
off. When a row names `requires_approval_from`, the posture does
not auto-fire; the row holds until the forum meets.

## Reviewer guidance: four shapes of closing

Every row's `decision_vehicle_class` field routes it to one of four
closing shapes. Reviewers opening a row for the first time start by
reading this class.

### `adr_or_rfc` — architecture / contract

The row closes when an ADR lands under `docs/adr/` (or an RFC lands
under `docs/rfc/` and then converts to an ADR). This is the default
shape for launch-register rows whose content is architecture or
contract-bearing — command parity, docs publication model, identity
modes, route-disclosure semantics, compatibility-bridge scope,
onboarding scope, theme and accessibility certification, durable-
attention model, embedded webview / auth policy.

When a reviewer opens one of these rows: route it to the named
forum's next meeting. If the forum agrees on a direction, open an
ADR directly. If the decision needs written exploration, open an
RFC and set `current_state = deciding`.

Examples: LR-0004, LR-0005, LR-0008, LR-0009, LR-0010, LR-0013,
LR-0014, LR-0015, LR-0016.

### `sponsor_scope_decision` — product / sponsor commitment

The row closes by an explicit sponsor- or product-scope-review
ruling recorded in a scope-review packet under the evidence owner's
scope-review home. No ADR is required. This is the shape for
decisions whose content is launch commitment (what we bundle, what
we claim, which providers we integrate) rather than contract shape.

When a reviewer opens one of these rows: attach it to the owning
forum's next scope-review meeting. Close the row by landing the
scope packet and appending an `accept` entry to `decision_history`;
`linked_adr` stays null.

Examples: LR-0001 (language/framework bundle), LR-0002 (deployment
profiles launch set), LR-0003 (open/local vs managed boundary),
LR-0006 (AI provider / spend envelope), LR-0012 (public-proof
language).

### `release_decision` — release council ruling

The row closes by a release-council ruling recorded in the release
evidence lane (`artifacts/release/`). This is the shape for
signing-quorum, release-channel, and support-window decisions whose
content is a release posture the council owns end-to-end.

When a reviewer opens one of these rows: route it to the next
release-council agenda; the council closes it as part of the
release-posture sign-off.

Examples: LR-0007 (exact-build / signing quorum), LR-0011 (release
channels / support windows).

### `narrowing_only_governance_note` — narrower-default outcome stands

The row does not require a decision record. If the row is still
open on `applies_on`, the narrower default fires automatically and
the row moves to `narrowed_by_default`. This class is reserved for
rows whose explicit launch posture is "the narrow outcome is the
launch posture, period." When no row currently resolves to this
class, the class sits in the register schema and in
`decision_defaults.yaml` as the fallback shape for any future row.

Reviewer action: confirm the posture in
`narrower_default.description` is the intended launch posture; if
not, escalate to the owning forum **before** `applies_on` to change
the class or the posture.

## When the default narrowing should auto-apply

The register distinguishes two cases:

- **Auto-fires on `applies_on` without further approval** — the
  row's `narrower_default.requires_approval_from` list is empty.
  On the freeze date, tooling sets `current_state = narrowed_by_default`
  (for `narrow` posture) or the equivalent posture-specific state and
  appends a dated history entry. Downstream artifacts pick up the
  narrowed scope verbatim. Reopening wider scope mints a new `LR-NNNN`.
- **Holds `deciding` until a named forum signs off** — the row's
  `narrower_default.requires_approval_from` names one or more forums.
  The default does not fire automatically; the row holds at
  `deciding` until one of the named forums meets and rules. Freeze
  exceptions for dependent lanes follow the rulebook in
  [`decision_workflow.md`](./decision_workflow.md).

As a rule of thumb: `sponsor_scope_decision` rows auto-fire by
default, because a product-claim row that misses its freeze really
does default to the narrow public posture. `release_decision` rows
hold for release-council sign-off, because the council is the only
body that can ratify a release-bearing change (including a narrow
one). `accessibility_certification` and `compatibility` rows with
`freeze_lane` posture hold for their specialist council.

## The sixteen seeded launch decisions

Dates, owners, affected artifacts, and linked premises live in the
register. The table below is a navigational index only.

| Decision id | Title | Owner forum | Vehicle | Default posture |
|-------------|-------|-------------|---------|-----------------|
| LR-0001 | Launch language and framework bundle | product_scope_review | sponsor_scope_decision | narrow |
| LR-0002 | Deployment profiles launch set | product_scope_review | sponsor_scope_decision | narrow |
| LR-0003 | Open / local versus managed boundary | product_scope_review | sponsor_scope_decision | narrow |
| LR-0004 | Command-parity scope across palette, menubar, CLI, and automation | product_scope_review | adr_or_rfc | narrow |
| LR-0005 | Docs and source-version publication model | product_scope_review | adr_or_rfc | narrow |
| LR-0006 | AI provider, model, and spend envelope | product_scope_review | sponsor_scope_decision | narrow |
| LR-0007 | Exact-build identity and signing quorum | release_council | release_decision | narrow (holds for release council) |
| LR-0008 | Identity modes and offline-entitlement posture | security_trust_review | adr_or_rfc | narrow |
| LR-0009 | Route-disclosure semantics for outbound traffic | security_trust_review | adr_or_rfc | narrow |
| LR-0010 | Compatibility-bridge scope | compatibility_ecosystem_review | adr_or_rfc | freeze_lane (holds for compat council) |
| LR-0011 | Release channels and support windows | release_council | release_decision | defer (holds for release council) |
| LR-0012 | Benchmark, migration, and public-proof language | product_scope_review | sponsor_scope_decision | narrow (holds for scope + release) |
| LR-0013 | Onboarding stable scope | product_scope_review | adr_or_rfc | narrow |
| LR-0014 | Theme and accessibility certification rows | accessibility_review | adr_or_rfc | freeze_lane (holds for a11y review) |
| LR-0015 | Durable-attention model | architecture_council | adr_or_rfc | narrow |
| LR-0016 | Embedded docs, webview, and auth policy | security_trust_review | adr_or_rfc | narrow |

## Linkage to architecture pack and scorecard

Architecture-pack sections cite launch-register rows by `LR-NNNN`
through the row's `architecture_pack_refs` field. Milestone scorecard
rows cite launch-register rows by `LR-NNNN` through the row's
`scorecard_lane_refs` field, resolved against
`ownership_matrix.yaml#scorecard_lane_index`. Neither artifact
invents a parallel status field; both read `current_state` from the
register.

When a launch row is also anchored in the rolling decision index,
the launch row names the `D-NNNN` in `linked_decision_index_rows`.
Closing the `D-NNNN` (landing the ADR) is the event that closes the
launch row; the launch row copies the closing artifact into
`linked_adr` and appends an `accept` entry to its own
`decision_history`.

## Sample freeze report

Tooling projects the register into
[`/fixtures/governance/decision_register/sample_freeze_report.yaml`](../../fixtures/governance/decision_register/sample_freeze_report.yaml)
using `report:unresolved_past_deadline_as_narrow_scope` from
`decision_defaults.yaml`. The report shows, for a hypothetical
report date past `freeze_by_date`, what the launch posture would
be if every still-open row fired its narrower default: explicit
narrow-scope outcomes, not `TBD`. The projection is deterministic
from the register and the defaults file.

## Adding a new launch-register row

1. Append a row to
   [`/artifacts/governance/decision_register.yaml`](../../artifacts/governance/decision_register.yaml)
   using the next unused `LR-NNNN` id.
2. Fill every required field, including `narrower_default.posture`,
   `narrower_default.description`, `public_claim_posture_while_open`,
   `affected_artifacts`, and at least one linked assumption or
   dependency.
3. Set `decision_vehicle_class` based on the reviewer-guidance
   classes above.
4. Add a navigational entry to the table above.
5. If an ADR, scope-review packet, or release-council agenda opens
   at the same time, set `linked_adr` or `linked_rfc` or append the
   first `decision_history` entry to record the opening.
6. If the row narrows or supersedes an existing `LR-NNNN`, use the
   supersession discipline from
   [`decision_workflow.md`](./decision_workflow.md): mint a new id,
   set the old row's `current_state = superseded`, and populate
   `superseded_by` on the old row. Do not edit the old row's body.

Out of scope at this revision: sponsor approval meetings themselves
and post-milestone-zero product strategy. Those events are
recorded through scope-review packets and release-council rulings;
this register is the canonical home for the launch-gate decisions
they consume.
