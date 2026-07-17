# M5 Review-Template-Packet and Template-Publish-Attribution Registries

- Packet: `m5-review-template-packet-and-publish-attribution-registries:stable:0001`
- Label: `M5 comment / summary review-template-packet and template-publish-attribution registries binding one typed template packet per review to the active review pack — the pack-authored rationale blocks, the checklist text, the bundle manifests, the template version, and the pack digest, each bound to the same review-pack version / content digest as human, local, and CI review — so published, draft, and exported review summaries preserve template version and pack digest instead of flattening them into generic review text across local draft, publish-now, open-in-provider, and export, with canonical / accessible / audit resolution-form coverage, and a machine-readable template-publish-attribution (field-provenance-binding, template-version-and-digest-binding, or destination-and-redaction-binding) that surfaces whether each field is pack-authored, generated, user-edited, omitted, or redacted, keeps user edits and redactions visibly separate from pack-authored template content, and never sends template-driven review content without first showing destination, template source, and redaction state across review, AI-review, provider-handoff, and support / export surfaces`
- Consumer surfaces: 6
- Template-content classes: rationale_block, checklist_text, bundle_manifest, user_edited_field, redacted_field, summary_text, template_packet_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **review_detail**: `stable`
  - Owner: Review-detail owner
  - Scope: Review detail resolves the active review pack to one typed comment / summary template packet — the pack-authored rationale blocks, the checklist text, the bundle manifests, the template version, and the pack digest — bound to the same review-pack version and content digest as human, local, and CI review, and proves the publish-attribution binding for the draft (which fields are pack-authored, generated, user-edited, omitted, or redacted); a packet that cannot name the template version and pack digest it is bound to and an attribution that would let user-edited or redacted text read as pack-authored template content degrade honestly instead of flattening the template into generic review text across local draft, publish-now, open-in-provider, and export
  - Template-packet entries: 2 / Publish-attribution entries: 2
- **ai_review_panel**: `stable`
  - Owner: AI-review owner
  - Scope: The AI review panel resolves the template-version-and-digest binding and the field-provenance attribution while keeping the bound template version / pack digest and whether each field is pack-authored, generated, user-edited, omitted, or redacted visible; a packet operating with a template version / pack digest that cannot be named and a resolution-form gap on an attribution are caught before a green summary can present the draft as clean pack-authored content, and AI review can never publish template-driven content under a different or undisclosed template version
  - Template-packet entries: 2 / Publish-attribution entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the field-provenance class while keeping the template version / pack digest and the template attribution bound to the export, and reports the destination-and-redaction state; a packet that is a hand-copied per-entry assumption and an attribution on an unclassified binding degrade honestly so the template version, pack digest, and authorship provenance are never dropped on export or reopen
  - Template-packet entries: 2 / Publish-attribution entries: 1
- **review_pack_summary**: `stable`
  - Owner: Review-pack-summary owner
  - Scope: The review-pack summary resolves the bundle manifests and checklist text and the destination-and-redaction state — destination, template source, and redaction state shown — bound to the registry so template-driven review content can never be sent without first showing where it goes, which template authored it, and what was redacted; an unstated template version / pack digest on a packet is caught before it can drift
  - Template-packet entries: 2 / Publish-attribution entries: 1
- **local_ci_parity_strip**: `stable`
  - Owner: Local-CI-parity owner
  - Scope: The local-CI parity strip renders the same resolved template-packet and publish-attribution truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied table; the user-edited / redacted field labels and the field-provenance binding stay inspectable off-renderer so user edits and redactions never read as pack-authored template content
  - Template-packet entries: 1 / Publish-attribution entries: 1
- **provider_handoff**: `stable`
  - Owner: Provider-handoff owner
  - Scope: The provider handoff feed carries the same resolved template-packet and publish-attribution truth into browser / provider handoff and reopened draft-only review state, so a dropped template version / pack digest, undisclosed template attribution, user-edited or redacted text shown as pack-authored, or a send without destination and redaction state is visible in evidence — a field-provenance change, a template-version-and-digest change, or a destination-and-redaction change — rather than hidden behind a green summary
  - Template-packet entries: 1 / Publish-attribution entries: 1
