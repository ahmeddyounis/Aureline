# Navigation target fidelity target-accuracy corpus

These fixtures exercise the typed navigation target model frozen in
`docs/navigation/m3/navigation_target_beta_contract.md` and the boundary
schema at `schemas/navigation/navigation_target.schema.json`.

The corpus covers:

- cross-language definitions, declarations, type targets, and references;
- implementation, call/type hierarchy, framework-derived, and runtime-observed edges;
- generated-file boundaries and ambiguous route/doc candidates;
- breadcrumb, bookmark, and history continuity after drift;
- rename-preview conflicts, blocked refs, generated notes, and sparse-scope caveats;
- UI, CLI/headless, AI, review, graph, shell-continuity, and support-export parity.

Fixtures carry opaque refs, typed vocabulary, evidence refs, and export-safe
summaries only. They must not carry raw source bodies, raw paths, provider logs,
URLs, credentials, or private code snippets.
