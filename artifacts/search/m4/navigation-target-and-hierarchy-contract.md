# Navigation Target And Hierarchy Contract Artifact

Stable packet: `nav-contract:stable:launch-language`

Source refs:

- `schemas/search/navigation-targets.schema.json`
- `docs/search/m4/navigation-target-and-hierarchy-contract.md`
- `fixtures/search/m4/navigation-target-and-hierarchy-contract/baseline_stable.json`

Release evidence:

- Definition and declaration targets remain separate relation kinds.
- Implementation navigation with multiple candidates binds to `NavigationDisambiguationSet`.
- References and rename preview preserve all required access kinds, including `route-binding`, `test-only`, `generated`, and `runtime-observed`.
- Hierarchy edges preserve direct, transitive, inferred, framework-generated, and runtime-observed classes.
- Rename preview preserves blocked, generated, readonly, sparse-scope, partially loaded, and redacted candidate truth without source bodies.
- Editor, search, graph, review, AI, support export, and CLI/headless projections preserve the same packet id and closed vocabularies.
