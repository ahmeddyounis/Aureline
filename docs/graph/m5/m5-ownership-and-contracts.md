# M5 ownership-and-contracts packet

This document describes the canonical packet that carries the **M5 ownership and contract
descriptors** — the honest answer the M5 graph surfaces give to *who owns this, and what kind of
source produced that answer?* Where the [workset-scope packet](m5-workset-scope.md) answers *what
slice am I looking at?*, the [topology-identity packet](m5-topology-identity.md) answers *which
exact graph object is this?*, the [impact-query packet](m5-impact-query.md) answers *is this empty
impact answer safe?*, and the [graph-governance matrix](m5-graph-governance.md) freezes *which depth
claim a lane may publish*, this packet answers the question review hints, explainer cards,
onboarding context, AI ownership suggestions, and support all ask of an ownership answer: **is this
curated truth, policy-derived truth, imported provider metadata, or merely a heuristic guess?**

It is the user-facing companion to the governed artifact at
`artifacts/graph/m5/m5-ownership-and-contracts.json` and the typed model in the `aureline-graph`
crate (`m5_ownership_and_contracts`).

## Why source kind matters

An ownership answer is dangerous when its source is hidden: a heuristic reviewer guess presented as
a bare "owner" reads as declared truth, and an imported provider contact can quietly overwrite a
curated team annotation. This packet refuses to flatten those states. Every descriptor carries an
[`OwnershipSourceClass`](#source-classes), and a non-curated descriptor must say where its answer
came from.

### Source classes

| Class | Meaning | Authority |
| --- | --- | --- |
| `curated` | Curated first-party metadata from a maintainer annotation | authoritative |
| `policy_derived` | Ownership derived from a policy such as a CODEOWNERS rule | authoritative |
| `imported` | Metadata imported from a connected provider | inferred/imported |
| `heuristic` | Ownership inferred by a heuristic or AI producer — a hint, not declared truth | inferred/imported |

Source class is a **precedence**: `curated` outranks `policy_derived`, which outranks `imported`,
which outranks `heuristic`. The `authoritative_descriptor` query returns the highest-precedence
descriptor for a subject and role, so curated truth wins over a derived hint.

Only `curated` is exempt from carrying a `source_reason`; `policy_derived`, `imported`, and
`heuristic` descriptors **must** carry an explicit reason, so an inferred or imported answer never
reads as curated first-party truth.

## Roles stay distinct

Owner, reviewer, maintainer, support contact, and change-control links are separate
[`OwnershipRole`](#roles) values rather than one generic owner field:

| Role | Meaning |
| --- | --- |
| `owner` | The accountable owner of the subject |
| `reviewer` | A designated reviewer for changes |
| `maintainer` | A maintainer responsible for ongoing upkeep |
| `support_contact` | A support contact for questions |
| `change_control` | A change-control link such as a CODEOWNERS rule or change policy |

A `change_control` descriptor carries its link distinctly in `change_control_url`; no other role
may carry that field.

## What this packet covers

Each entry in `descriptors` is an `OwnershipDescriptor` carrying:

- the `subject_id` it attaches to (in the shared topology identity space) and the `subject_kind`;
- the distinct `role` and the `source_class` that produced the answer, plus — for any non-`curated`
  class — an explicit `source_reason`;
- a redaction-aware `party_label`, a `freshness` and `confidence` token, and a `visibility` scope;
- for the `change_control` role, a `change_control_url`;
- the descriptor ids it `supersedes`; and
- an `export_permalink` that embeds the canonical descriptor id.

### Inference never overwrites curated truth

The headline guardrail: a `heuristic` or `imported` descriptor **may not supersede** a `curated` or
`policy_derived` descriptor merely because it is newer or easier to compute. A supersede link from
an inferred or imported descriptor to authoritative truth fails validation
(`InferenceOverwritesCurated`).

### Visibility never widens past its scope

Each descriptor declares a [`OwnershipVisibility`](#visibility): `public`, `internal`, or `private`.
Private (policy-scoped) ownership is shown only in-product to authorized users and is **never
exported**. Each consumer binding declares a `max_visibility` ceiling, and a binding may not carry a
descriptor more restricted than its ceiling. The support-export binding may carry only export-safe
(`public` or `internal`) descriptors, and the export projection redacts every `private` descriptor
entirely.

### Carried beyond one panel

Each of `review_hint`, `explainer_card`, `onboarding_context`, `ai_ownership_suggestion`, and
`support_export` carries exactly one `OwnershipConsumerBinding`, stamped with the active snapshot
and scope, and each binding **preserves source-class labels** so an inferred reviewer is never
flattened into a curated owner downstream. The `support_export` binding **must carry every
export-safe descriptor and no private one**, so support and enterprise review can cite ownership
without a private dashboard lookup.

## Guardrails proven

- A non-curated descriptor with no `source_reason` fails validation (`MissingSourceReason`).
- A `change_control` descriptor with no `change_control_url`, or any other role that carries one,
  fails validation (`ChangeControlWithoutLink`, `NonChangeControlWithLink`).
- An `imported` or `heuristic` descriptor that supersedes a `curated` or `policy_derived` descriptor
  fails validation (`InferenceOverwritesCurated`); a descriptor superseding itself or an undeclared
  id fails (`SelfSupersede`, `UnresolvedSupersedesRef`).
- A binding that flattens source labels fails validation (`SourceLabelsNotPreserved`); a binding
  that carries a descriptor beyond its visibility ceiling fails (`VisibilityExceedsBinding`).
- A descriptor permalink that is empty or does not embed its id fails validation
  (`UnsafeDescriptorPermalink`).
- A binding not stamped with the active snapshot or scope fails validation
  (`SnapshotBindingMismatch`, `ScopeIdMismatch`); an export-safe descriptor not carried by the
  support-export binding fails (`ExportSafeDescriptorMissingFromSupportExport`), and a `private`
  descriptor carried by it fails (`PrivateDescriptorInSupportExport`).

## Upstream provenance

The packet binds to the canonical graph-depth governance matrix
(`artifacts/graph/m5/m5-graph-governance.json`), the workset-scope packet
(`artifacts/graph/m5/m5-workset-scope.json`), and the topology-identity packet
(`artifacts/graph/m5/m5-topology-identity.json`) whose node identity space its subjects reuse.

<a id="help-surface"></a>
<a id="ownership-badge"></a>

## Help surface and ownership badge

Help and docs surfaces narrow from this one packet: the **ownership badge** reflects the source
class of an ownership answer (curated, policy-derived, imported, or heuristic), and the help surface
explains each role and source class rather than re-describing ownership state by hand.
