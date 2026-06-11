# M5 conformance scorecards, validators, and reference-workspace linkage

This document describes the canonical packet that freezes the **M5 conformance**
lane: one conformance/compatibility scorecard per marketed M5 artifact family, the
validator diagnostics an author must clear, and the owner, archetype, and
reference-workspace linkage a support claim rides on. It is the user-facing companion
to the governed artifact at
`artifacts/ecosystem/m5/m5-conformance-and-validators.json` and the typed model in the
`aureline-ecosystem` crate (`m5_conformance_and_validators`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per marketed artifact family and the
[`M5 marketplace fact-views`](m5-marketplace-fact-views.md) project that truth into the
storefront, this packet answers a different question: **is the family's support claim
actually backed by current conformance or compatibility evidence?** First-party or
bridge-backed status is not enough — a claim must name an owner, link an archetype and
a reference workspace, cite conformance and compatibility evidence, and pass the
validators. The new M5 artifact families ride the same conformance, compatibility, and
reference-workspace evidence model that already governs stable extension claims.

## What this packet covers

The packet carries one record family: a **conformance scorecard** for each marketed
artifact family. A scorecard reproduces the support-claim fact set —
`package_kind`, `runtime_origin`, `conformance_label`, `compatibility_label`,
`claimed_support_class`, `effective_support_class`, and `evidence_freshness` — and
links the evidence a claim rides on:

- **Who owns it?** An `owner_ref` for the team or maintainer accountable for the claim.
- **What is it certified against?** An `archetype_ref` and one or more
  `reference_workspace_refs`, plus a `conformance_ref` and a `compatibility_ref` to the
  qualifying evidence.
- **What must the author fix?** A list of `validator_diagnostics`, each with a stable
  `code`, a `domain`, a `severity`, a `message`, and a concrete `remediation`.

## The conformance label vocabulary

Every scorecard carries one `conformance_label` from a single stable vocabulary that
marketplace badges, docs badges, release evidence, and support exports all consume:

- `native` — conforms natively on the target runtime.
- `bridge` — conforms through a compatibility bridge or local-model host.
- `partial` — conforms only partially; some conformance cases are unmet.
- `unsupported` — does not conform on the target; carries no positive claim.
- `retest_pending` — a prior result has lapsed and the family must be re-tested before
  it claims again.

The label caps the support class a family may publish: `native` permits full support,
`bridge` and `partial` cap at best-effort, and `unsupported` and `retest_pending`
permit no claim.

## Validator diagnostics are actionable

Each diagnostic is actionable by construction. The `domain` —
`schema`, `capability`, `compatibility`, `permission`, `provenance`,
`reference_workspace`, `activation_budget`, or `metadata` — tells an author *what* to
fix, the `severity` (`info`, `warning`, `error`) tells them *how urgently*, and the
`remediation` tells them *how*. The validator report is exported flat (one row per
diagnostic, tagged with its scorecard and package kind) for issue reports, release
evidence, and enterprise evaluation, alongside the machine-readable scorecard rows.

## Disposition and effective support are recomputed, not stored by hand

The `certification_disposition` a scorecard publishes — `certified`,
`conditionally_certified`, or `uncertified` — and its `certification_signals` are
**not** authored. They are recomputed from the scorecard's facts as the widest
`min_disposition` over every detected signal:

- **Review-class signals** narrow to `conditionally_certified`: a non-native runtime
  (`non_native_runtime`) and a validator warning (`validator_warning`).
- **Guardrail-class signals** force `uncertified`: stale or unknown evidence
  (`evidence_not_current`), a missing owner (`owner_missing`), an unlinked archetype
  (`archetype_unlinked`), an unlinked reference workspace
  (`reference_workspace_unlinked`), a missing conformance or compatibility ref
  (`conformance_evidence_missing`), a validator failure (`validator_failure`), a
  retest-pending label (`retest_pending`), or an unsupported label (`unsupported`).

The `effective_support_class` is the weakest of the `claimed_support_class` and every
ceiling — the conformance label, the runtime origin, the compatibility label, and the
evidence freshness — and is forced to `unsupported` when the scorecard is
`uncertified`. So an unbacked claim collapses to no claim: a first-party or
bridge-backed family can never publish a support claim without a current, owned,
evidence-linked scorecard. The stored disposition, signal set, and effective support
class must each equal their recomputation, or `validate()` fails.

## Worked examples

The corpus carries one scorecard per marketed artifact family and exercises every
conformance label and disposition:

- The **first-party framework pack** and the **signed recipe pack** are `certified`:
  native, current, owned, and evidence-linked, with full and community support
  respectively.
- The **docs pack** and the **local-model pack** are `conditionally_certified`: a stale
  docs anchor (a warning) and a partial local-model runtime with unexercised
  capabilities keep them backed but flagged.
- The **template artifact** is `uncertified` because its conformance evidence is stale;
  its full-support claim is withdrawn until evidence is refreshed.
- The **bridge-backed package** is `uncertified` because the bridge adapter fails three
  conformance cases (a validator error); its best-effort claim is withdrawn until the
  failures are fixed.
- The **side-loaded package** is `uncertified` and `unsupported`: unsigned, unowned, and
  unlinked, it carries no conformance evidence and no claim.
- The **mirrored-registry variant** is `uncertified` because it is `retest_pending`; its
  claim is held until the suite is re-run on the target, even though it shares the
  first-party pack's owner, archetype, and reference workspace.

## Source of truth

The checked-in artifact, schema, and fixtures are canonical for the M5 conformance and
validator lane. Marketplace badges, docs badges, release evidence, and support exports
should project from the scorecard rows and the flat validator report rather than
re-describing conformance, support, or validator state by hand.
