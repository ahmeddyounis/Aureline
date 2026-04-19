# Architecture Decision Records

Architecture Decision Records (ADRs) are the canonical, human-readable log of
architecture decisions for protected-lane behaviour. They exist so that
architecture cannot be decided implicitly in code.

Companion artifacts:

- [`/docs/rfc/`](../rfc/) — deeper proposal form used when a decision needs a
  written exploration before it can be accepted.
- [`/docs/governance/decision_backlog.md`](../governance/decision_backlog.md)
  — the seeded list of decisions that still need to land, their owners, and
  their freeze deadlines.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — machine-readable register of every decision row. Tooling reads this;
  this ADR folder is the narrative form.
- [`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json)
  — schema for the register.

## When a change needs an ADR

An ADR is required before broad implementation of any change that:

- modifies or constrains a protected lane listed in
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  (renderer, buffer, VFS, text, RPC, telemetry, shell / command system,
  benchmark lab, release evidence, docs / public truth, support / export,
  design-system seeds, accessibility / input review, governance packets);
- locks in a shared contract (RPC envelope, subscription envelope,
  schema versioning, identity modes, release posture, build-identity rules);
- narrows the committed scope of a protected lane for the current milestone
  (see the narrowing authority table in
  [`/docs/governance/dri_map.md`](../governance/dri_map.md)).

Anything narrower than a protected-lane-visible behaviour change can skip
the ADR requirement.

## Workflow

1. **Pick a decision row.** Either the change resolves an open row in
   `decision_index.yaml`, or a new row is opened in that file in the same
   change. Every ADR MUST reference exactly one decision id.
2. **Copy the template.** Copy [`0000-template.md`](./0000-template.md) to
   `NNNN-slug.md` using the next unused four-digit number. Keep the filename
   slug short and decision-focused; do not put milestone IDs or task IDs in
   the filename.
3. **Fill every required section.** `Status`, `Context`, `Decision`,
   `Consequences`, `Alternatives considered`, `Source anchors`, and
   `Linked artifacts` are required. If a section does not apply, say so
   in one line rather than deleting the heading.
4. **Link the ADR from the decision register.** Set `linked_adr` on the
   decision row to the relative path of the new ADR and update the row
   status.
5. **Review and merge.** ADRs follow the ownership routes in
   [`/CODEOWNERS`](../../CODEOWNERS); protected-lane ADRs additionally
   require sign-off from the forum named on the decision row (architecture
   council, performance council, security / trust review, etc.).
6. **Never rewrite a merged ADR.** Superseding an ADR means landing a new
   one that sets the earlier ADR's `Status` to `Superseded by NNNN` and
   adds a `superseded_by` entry on the decision row. The original stays
   for history.

## Metadata discipline

ADRs, RFCs, and the decision register share the same metadata fields so
shiproom automation does not have to invent new ones. Fields live in the
register (`decision_index.yaml`); the ADR links to the row rather than
restating them. Adding a new metadata field requires a schema change in
[`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json),
not an ad-hoc field in a single ADR.
