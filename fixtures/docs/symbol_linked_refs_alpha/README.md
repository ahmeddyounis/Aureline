# Symbol-Linked Docs References Alpha

This fixture suite protects the docs/help search lane that links product
symbols and commands to docs anchors. The fixtures assert that docs results
reuse planned search result IDs, expose exact anchors, disclose source,
version, locality, freshness, project-docs precedence, citation availability,
missing-anchor downgrade state, stale-example provenance, and docs suggestion
publish posture.

| Fixture | Covers |
|---|---|
| `project_symbol_and_stale_example.json` | Project-docs precedence, package-guide downgrade for a missing exact method anchor, citation drawer hooks, stale-example signal, and README suggestion publish-boundary state. |
