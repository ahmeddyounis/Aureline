# M5 navigation-content component consumer fixtures

Mirror of `artifacts/release/m5-navigation-content-component-consumer-proof/`. Proves the
shell/explorer, search/graph, review, request/data, help center, and support/export +
release-packet consumers all reuse the same frozen navigation-content component families
(tab strip, breadcrumbs, tree view, list view, table/grid, panel header) and vocabulary.
Regenerate with `GEN_NAV_CONTENT_CONSUMER_ARTIFACTS=1 cargo test -p aureline-shell generate_nav_content_consumer_artifacts`.
