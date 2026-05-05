# AI memory-class examples (YAML)

This directory contains compact YAML fixtures that demonstrate how
delete/export behavior fans out **by `ai_memory_class`**.

These examples are intentionally small and focus on:

- per-class outcomes for delete requests (what clears vs what is preserved);
- per-class inclusion/exclusion for exports (what is exported vs what is disclosed as excluded); and
- the provenance bindings (refs + invalidation fingerprints) that prevent
  unlabeled cross-workspace/provider reuse.

See:

- `docs/ai/memory_class_matrix.md` (matrix)
- `docs/ai/memory_and_reconciliation_contract.md` (full contract)
- `schemas/ai/memory_object.schema.json` (boundary schema)

