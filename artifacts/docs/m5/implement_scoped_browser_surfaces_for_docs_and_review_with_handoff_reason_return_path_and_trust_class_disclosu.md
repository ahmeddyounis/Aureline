# Scoped Browser Surfaces (docs and review)

- Packet: `packet:m5:scoped_browser:retry_backoff_handoffs`
- Session: scoped browser: reviewing the networking retry backoff change
- Promotion: `stable` (0 findings)
- Surfaces: 3 | Degradations: 1

## Surfaces

- [docs_reading] `surface:docs:tokio_retry_guide` (Exponential backoff guidance (mirrored docs)) — trust `signed_mirror_verified` — mirrored_official_docs / compatible_minor_drift / warm_cached / mirrored_pack / medium
  - Handoff reason: [exact_anchor_unavailable_locally] the exact backoff anchor is only on the upstream page; the inline peek could not resolve it locally
  - Return path: [back_to_inline_peek] Back to the retry_with_backoff peek
  - Capability: available_explicit | Captured/live: captured_snapshot | Cited: true
- [review] `surface:review:retry_backoff_thread` (Review thread: retry/backoff change) — trust `live_provider_handoff` — review_host / exact_build_match / authoritative_live / managed / medium
  - Handoff reason: [review_thread_requires_hosted_view] the review thread and its inline comments live on the hosted review surface
  - Return path: [back_to_review_panel] Back to the review panel
  - Capability: available_explicit | Captured/live: live | Cited: true
- [light_edit] `surface:light_edit:retry_doc_comment` (Light edit: retry doc comment) — trust `first_party_authoritative` — project_docs / exact_build_match / authoritative_live / local / high
  - Handoff reason: [light_edit_requires_scoped_editor] the typo fix opens a scoped editor surface over the local doc comment
  - Return path: [back_to_workspace] Back to the workspace
  - Capability: not_required_local | Captured/live: live | Cited: true

## Degradations

- [mirror_offline_snapshot/advisory]: the docs mirror was last synced two days ago; the docs surface is served from the cached snapshot
