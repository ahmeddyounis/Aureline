# Requests for Comment

Requests for Comment (RFCs) are the exploratory proposal form used when a
decision needs a written investigation before the forum can accept an ADR.
RFCs live alongside ADRs but serve a different role:

- An **RFC** proposes and explores an option space. It can end with a
  recommendation but does not, by itself, close a decision.
- An **ADR** records the chosen outcome. Every accepted ADR resolves one
  decision id in the register.

A change does **not** need both: small decisions can land as an ADR
directly with an `Alternatives considered` section. RFCs are for
decisions where the forum wants a working document before committing.

Companion artifacts:

- [`/docs/adr/`](../adr/) — decision records.
- [`/docs/governance/decision_backlog.md`](../governance/decision_backlog.md)
  — seeded decision list.
- [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  — machine-readable register.

## When to open an RFC

Open an RFC when:

- a decision spans multiple protected lanes and a single ADR would be
  too narrow (e.g. a cross-cutting subscription envelope affects
  buffer, VFS, and RPC at once);
- a decision must be exposed to contributors or a decision forum for
  review before acceptance;
- narrowing or re-baselining authority (see
  [`/docs/governance/dri_map.md`](../governance/dri_map.md)) requires
  a written case, not a one-paragraph ADR.

A change that touches a protected lane but has an obvious, unambiguous
right answer may skip straight to an ADR.

## Workflow

1. **Open or claim a decision row.** The RFC must reference an existing
   decision id in `decision_index.yaml`. If no row exists, open one in
   the same change and set its status to `deciding`.
2. **Copy the template.** Copy [`0000-template.md`](./0000-template.md)
   to `NNNN-slug.md` using the next unused four-digit number.
3. **Draft the proposal.** Fill every required section. Leave the
   `Recommendation` section empty until the RFC is ready to close.
4. **Review.** RFCs follow the ownership routes in
   [`/CODEOWNERS`](../../CODEOWNERS) plus the forum named on the
   decision row.
5. **Close the RFC.** An RFC closes in one of four ways, all recorded
   in the RFC frontmatter and mirrored on the decision row:
    - **Accepted** — a companion ADR lands that resolves the decision.
      `linked_rfc` on the decision row points at this RFC.
    - **Rejected** — the forum declines the proposal. A short rationale
      is appended; the decision row remains open or is closed via a
      separate ADR.
    - **Withdrawn** — the author pulls the RFC. Decision row remains
      open.
    - **Superseded** — a later RFC replaces this one. Add
      `Superseded by RFC-NNNN` to the status; do not rewrite the body.

## Metadata discipline

RFCs, ADRs, and the decision register share the metadata defined in
[`/schemas/governance/decision_index.schema.json`](../../schemas/governance/decision_index.schema.json).
An RFC links to the register row rather than restating fields. Adding a
new field requires a schema change, not an ad-hoc frontmatter key in a
single RFC.
