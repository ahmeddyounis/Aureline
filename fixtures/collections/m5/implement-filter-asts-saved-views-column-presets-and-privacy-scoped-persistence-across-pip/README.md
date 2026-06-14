# M5 Collection Privacy-Scoped Persistence Fixtures

## persistence_reopen_migrate_and_reset.json

A reopen drill fixture for the privacy-scoped collection persistence packet. It
binds the first real M5 dense surfaces — pipeline run list, review queue, incident
list, graph list, marketplace results, and provider/admin table — to durable,
reopenable persistence records. Each binding reuses a shared `aureline-search`
`SavedCollectionView` object rather than a surface-local serialization blob, and
carries a persisted column preset, the full scope-counter vocabulary, a persisted
schema version, and a compatibility state.

The pipeline, review, graph, and marketplace bindings are `current`: their
persisted filter/view/column state replays exactly on reopen
(`restored_exact`). The incident binding was saved under an earlier schema and is
`migratable_forward`; on reopen it is migrated forward with every filter, view,
and column choice preserved (`restored_after_migration`). The provider/admin
binding was saved under an `incompatible_needs_reset` schema that cannot be
migrated; on reopen it resets to the default view and discloses the dropped
choices (`reset_to_default`). Both incompatible bindings carry a precise,
visible label rather than a generic provider error.

Privacy scope is preserved: the shared review-queue view is `shared_redacted`
(never `local_only_private`), the provider marketplace view is `provider_owned`,
the admin view is `policy_governed`, and the local graph view is
`local_only_private`. No binding persists a transient selection, a provider
cursor, or secret-bearing material, and every portable view carries a portable
filter AST.

The fixture validates against
`schemas/collections/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/support_export.json`.
