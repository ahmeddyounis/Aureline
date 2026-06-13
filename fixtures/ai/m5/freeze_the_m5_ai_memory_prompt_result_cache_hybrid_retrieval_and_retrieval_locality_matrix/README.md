# M5 AI Recall Matrix Fixtures

## held_code_understanding_matrix.json

A matrix fixture where the code-understanding surface is held pending upstream
embedding-generation labeling. Composer assist, docs/browser recall, semantic
search, and support/export remain Stable; managed/offline remains Beta.
Demonstrates that a held surface uses `not_applicable` evidence with no required
evidence packet refs and carries the `upstream_dependency_narrowed` downgrade
trigger. The fixture validates against
`schemas/ai/freeze-the-m5-ai-memory-prompt-result-cache-hybrid-retrieval-and-retrieval-locality-matrix.schema.json`.
