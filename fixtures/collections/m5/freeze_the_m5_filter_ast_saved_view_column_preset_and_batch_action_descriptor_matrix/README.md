# M5 Dense Collection Qualification Matrix Fixtures

## support_export_row_downgrades_on_unidentified_batch_action.json

An auto-downgrade drill fixture for the dense-collection qualification matrix.
Every claimed M5 dense collection surface — pipeline run list, review queue,
incident list, graph list, marketplace results, activity rows, provider/admin
table, query-backed result set, and support/export projection — carries its
filter-AST class, selection-scope class, result-counter class, batch-action scope
class, scope-counter vocabulary, saved-view contract, column-preset contract, and
batch-action descriptors.

The support/export projection row claims `beta`, but its batch-action scope class
is not yet identified (`batch_action_class` is absent). Because a claimed row may
not outrun identified evidence, the row auto-downgrades to `effective` `held`,
records an `unidentified_batch_action` downgrade trigger, and carries a precise
degraded label rather than a generic provider error. Every other row identifies
all four of its filter-AST, selection-scope, result-counter, and batch-action
classes, so its effective qualification equals its claim.

The all-matching review-queue, provider/admin, and query-backed rows keep the
visible count distinct from the all-matching count and disclose provider/policy
narrowing; every declared export, copy, share, mutating, provider-backed, or
destructive batch action previews its scope and emits a scope receipt before
commit; selection survives by stable identity; and no saved view captures a
transient selection or provider cursor.

The fixture validates against
`schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/support_export.json`.
