# Standards-deviation ADR template

<!--
Copy this file to `docs/adr/NNNN-<slug>.md` using the next unused
four-digit number and a short decision-focused slug. The slug
should name the deviation, not the milestone or task (e.g.
`sarif-propertybag-extensions`, `spdx-sbom-narrowing`, not
`m00-181-sarif`).

This template extends `docs/adr/0000-template.md` with the
additional fields a standards deviation MUST carry so that the
matrix row, the decision register, and the ADR all agree. When a
field differs between the ADR, the decision register, and the
matrix, the matrix wins for tooling and the ADR + decision row
MUST be updated in the same change.
-->

- **Decision id:** `D-XXXX` (must exist in
  `artifacts/governance/decision_index.yaml`)
- **Status:** Proposed | Accepted | Deferred | Narrowed-by-default |
  Superseded by ADR-NNNN
- **Decision date:** `YYYY-MM-DD` (or `pending`)
- **Freeze deadline:** `YYYY-MM-DD` (copied from the register)
- **Owner:** `@handle`
- **Backup owner:** `@handle` or `null` with a cited waiver id
- **Forum:** `architecture_council` | `performance_council` |
  `security_trust_review` | `accessibility_review` |
  `compatibility_ecosystem_review` | `product_scope_review` |
  `release_council` | `shiproom_executive_scope_review`
- **Related requirement ids:** e.g. `PRD-FOO-001` (or `none`)

## Deviation anchors

- **Matrix row ref:**
  `artifacts/governance/standards_matrix.yaml#standard.<slug>`
- **Preferred standard:** family and version from the matrix row
- **Deviation class:** `narrow_with_adr` | `extend_with_adr` |
  `bridge_with_adr` | `temporarily_diverge_with_adr`
- **Affected surfaces:** the specific import surfaces, export
  surfaces, or bridge seams this ADR changes
- **Deviation scope:**
  - **In scope:** ...
  - **Out of scope:** ...

Every deviation ADR MUST select exactly one deviation class. If a
change genuinely spans two classes (for example, a narrow and a
bridge), split it into two decision rows and two ADRs — one per
class — so the matrix can track the postures independently.

## Context

One or two paragraphs describing the problem, the forces at play,
and the protected lanes or public-truth surfaces the deviation
touches. Reference the matrix row, the preferred standard, and any
upstream specification anchor rather than restating the spec.

Call out the failure mode that motivates the deviation: a missing
standard feature, an ambiguity in the spec, a security posture
that the standard does not support, a compatibility window that
the standard cannot honour today, or a consumer constraint that
forces a temporary divergence.

## Decision

State the deviation in a single paragraph. Be concrete:

- For a **narrow**, name exactly which fields, keywords, or
  behaviours Aureline refuses to produce or consume, and under
  what condition.
- For an **extension**, name exactly which fields, keywords, or
  behaviours Aureline adds, where they live on the wire (for
  example, SARIF `propertyBag`, JSON Schema `x-*` extension
  keywords), and how consumers that do not understand them MUST
  behave.
- For a **bridge**, name the custom contract and the adapter
  surface that projects it onto the standard. State the adapter's
  fidelity posture (lossy, lossless-with-metadata, one-way).
- For a **temporary divergence**, name the interim posture, the
  named migration target, and the signal that will fire the
  migration.

## Consequences

Bulleted list of expected consequences. Call out anything that
becomes frozen, anything that becomes permitted, and anything that
now needs a follow-up change. Include at least:

- what the matrix row now claims (`support_class`,
  `import_expectation`, `export_expectation`, `deviation_notes`);
- what evidence paths now apply and where they live;
- which downstream artefacts (release notes, claim manifest,
  compatibility row, docs page, SDK binding) must update in the
  same change;
- whether any backward-compatibility note is now required.

## Alternatives considered

- **Option A.** Short summary. Reason rejected.
- **Option B.** Short summary. Reason rejected.

If the decision has a `default_if_unresolved` narrowing posture in
the decision register, describe what that narrowing would look
like in practice so a reader can tell whether it actually landed
or whether the forum accepted a non-default outcome.

## Evidence plan

Name at least one minimum-evidence path from
`standards_matrix.yaml#evidence_path_classes`, and explain how
this deviation will be proven:

- **Validator harness:** which upstream validator runs, what
  inputs it receives, where its report lives.
- **Example export fixture:** where the committed example lives
  and what it demonstrates.
- **Import round-trip fixture:** parse-then-emit-then-compare
  harness location and golden data.
- **Bridge disclaimer doc:** the narrative page consumers read
  before relying on the bridge.
- **Compatibility note:** the release note / docs entry that names
  the pinned version and compatibility window.
- **Conformance report artifact:** the release-evidence report
  vouching for the deviation on a given release train.

A deviation ADR without a committed evidence plan is not ready to
close.

## Rollback or re-adoption plan

State the exit. Every deviation carries one of:

- **Rollback plan.** What would revert Aureline to a verbatim
  import / export posture? Which lanes, fixtures, and SDK bindings
  change back? Who approves the rollback?
- **Re-adoption plan.** What signal (upstream specification
  ratification, consumer adoption, security posture change) fires
  the re-adoption? What is the committed migration target and by
  what milestone?

"Indefinite" or "until the standard catches up" is not acceptable
— restate the exit as a concrete signal and a named milestone or
forum check-in.

## Compatibility and consumer impact

- **Affected consumers:** who reads / writes the format today
  (hosted review, CI tooling, SDK consumers, extension authors,
  enterprise operators).
- **Upgrade path:** what consumers must do when the deviation
  lands, and what they must do when it is retired.
- **Silent-break guard:** how reviewers verify the deviation does
  not silently drift the standard's contract between releases.

## Source anchors

Point to the doc lines in `.t2/docs/` or the protected-lane
artefacts that motivated this decision.

- `.t2/docs/<doc>.md:<line>` — short quoted phrase.
- Upstream specification anchor (version, section) — short
  quoted phrase.

## Linked artifacts

- Matrix row: `artifacts/governance/standards_matrix.yaml#standard.<slug>`
- Decision register row: `artifacts/governance/decision_index.yaml#D-XXXX`
- ADR parent (if this deviation rides on an earlier ADR):
  `docs/adr/NNNN-<slug>.md`
- RFC (if any): `docs/rfc/NNNN-<slug>.md`
- Affected packages / lanes: `crates/<crate>`,
  `artifacts/<family>/`, `docs/<lane>/...`, `wit/aureline/...`
- Release / docs surfaces that must update in the same change:
  release-notice seed, docs pack manifest, compatibility row seed,
  claim manifest, SDK binding, ... (list the specific files).

## Supersession history

Leave empty on first acceptance. If this ADR supersedes an earlier
deviation ADR on the same matrix row, record
`Supersedes ADR-NNNN (<slug>) — <reason>`. If this ADR is later
superseded, set `Status` to `Superseded by ADR-NNNN` and add a
line here — do not rewrite the body.
