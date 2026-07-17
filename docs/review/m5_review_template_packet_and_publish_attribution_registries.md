# M5 review-template-packet and template-publish-attribution registries

Comment / summary review-template-packet implement lane over the frozen
[M5 review-pack evaluator matrix][matrix] (`m5_review_pack_evaluator_matrix`). It makes the
matrix's `review_template_packet` object class operable by carrying resolved, honest projections
of two registries so review detail, the AI review panel, the review-pack summary, the local-CI
parity strip, provider handoff, and support / export surfaces inherit one canonical comment /
summary template model — pack-authored rationale blocks, checklist text, and bundle manifests all
carrying a template version and a pack digest — rather than a hand-authored parallel prose that has
to be kept consistent.

## Registry-A — review-template-packet

One typed comment / summary template packet per review, carrying the **template-content class** it
binds so each class stays bound to the same review-pack version / digest as human, local, and CI
review:

- `rationale_block` — the pack-authored rationale block the template supplies;
- `checklist_text` — the pack-authored checklist text the template supplies;
- `bundle_manifest` — the bundle manifest the template packet carries;
- `summary_text` — the comment / summary text the packet publishes;
- `user_edited_field` — a field the user edited away from the pack-authored template content (a
  provenance state that must stay visibly separate before publish or export);
- `redacted_field` — a field redacted from the template content before send (a provenance state
  that must stay visibly separate before publish or export).

The `user_edited_field` and `redacted_field` classes are publish-truth-sensitive: they surface
directly in the user-facing preview a packet publishes, so their provenance claim must stay matched
to the template version the content is actually bound to. A packet that cannot name the template
version and pack digest it is bound to, that is a hand-copied per-entry assumption instead of
tracing to the shared registry, or that would flatten the template version / pack digest into
generic review text degrades honestly instead of publishing template-driven content under an
undisclosed template version. The registry reuses the matrix
`m5-review-template-packet.schema.json` domain schema.

## Registry-B — template-publish-attribution

The typed record of exactly which template text a local-draft, publish-now, open-in-provider, or
export flow will send, naming which **attribution binding** it carries so authorship and pack
provenance stay inspectable:

- `field_provenance_binding` — whether each field is pack-authored, generated, user-edited, omitted,
  or redacted;
- `template_version_and_digest_binding` — the template version and pack digest carried through
  publish, export, and reopen;
- `destination_and_redaction_binding` — the destination, the template source, and the redaction
  state that must be shown before send.

The attribution keeps the field provenance and the template version / pack digest so an exported
review / support packet can be read without the live UI having to re-interpret which fields were
pack-authored versus user-edited or redacted. The registry uses the minted
`m5-template-publish-attribution.schema.json` domain schema.

## Acceptance criteria proven by the resolved examples

1. Published, draft, and exported review summaries preserve template version and pack digest instead
   of flattening them into generic review text: the `template_version_and_digest_binding` attribution
   and the `rationale_block` / `checklist_text` / `bundle_manifest` template-content classes stay
   mechanically distinct across the rows and through the narrowed draft (Beta) and export (Preview)
   fixtures, and a packet that drops the template version / pack digest degrades.
2. User edits and redactions stay visibly separate from pack-authored template content: the
   `user_edited_field` and `redacted_field` classes are provenance-tracked and publish-truth-sensitive,
   and a packet whose template-version / authorship-provenance join is not preserved degrades — the
   support-export fixture carries a `user_edited_field` packet alongside pack-authored classes so the
   separation is visible end to end.
3. No publish / export surface can send template-driven review content without showing destination,
   template source, and redaction state first: the `destination_and_redaction_binding` attribution
   carries the destination / template-source / redaction state, and an attribution that would send
   without it degrades honestly across the review, AI-review, provider-handoff, and support / export
   surfaces.

Raw secret values and private endpoints never cross this boundary. The Rust validator in
`crates/aureline-ui` is the authoritative gate; the checked-in combined registries schema
(`schemas/review/m5-review-template-packet-and-publish-attribution-registries.schema.json`)
documents the shape.

[matrix]: ../../crates/aureline-ui/src/m5_review_pack_evaluator_matrix/mod.rs
