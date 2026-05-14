# AI mutation wedge alpha fixtures

This corpus proves the first review-first AI mutation evidence packet:

- `docs_backed_review_pre_apply.json` mints an evidence packet before apply, keeps route/spend/approval lineage, and exports reconstructible docs-pack citations plus inference/confidence markers.
- `applied_after_approval.json` preserves the same lineage after approval and records rollback plus mutation-journal refs.
- `tainted_context_rejected.json` fences policy-disallowed terminal output, explains why it cannot authorize the mutation, and preserves route/spend/approval truth after rejection.

The fixtures are metadata-only. They do not include raw prompts, raw diff bodies, raw source paths, raw URLs, credential material, exact token counts, or exact cost amounts.
