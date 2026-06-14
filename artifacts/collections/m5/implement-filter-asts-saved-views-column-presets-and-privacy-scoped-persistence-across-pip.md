# M5 Collection Privacy-Scoped Persistence

- Packet: `m5-collection-persistence:stable:0001`
- Label: `M5 Collection Privacy-Scoped Persistence`
- Bindings: 6 (2 incompatible)
- Surfaces: 6 / 6

## Bindings

- **persist:pipeline-run-list:0001** (pipeline_run_list): view `view:pipeline-run-list` scope `workspace/workspace_portable`
  - Pipeline run list filter, saved view, and column preset persisted in workspace scope
  - compatibility=`current` reopen=`restored_exact`
- **persist:review-queue:0001** (review_queue): view `view:review-queue` scope `shared/shared_redacted`
  - Review queue saved view shared across the team after redaction
  - compatibility=`current` reopen=`restored_exact`
- **persist:incident-list:0001** (incident_list): view `view:incident-list` scope `workspace/workspace_portable`
  - Incident list state persisted under an older schema, migrated forward on reopen
  - compatibility=`migratable_forward` reopen=`restored_after_migration`
  - Incompatible: Saved under an earlier collection schema; migrated forward to the current schema on reopen with all filter, view, and column choices preserved
- **persist:graph-list:0001** (graph_list): view `view:graph-list` scope `user/local_only_private`
  - Graph/reference list saved view kept private to the local profile
  - compatibility=`current` reopen=`restored_exact`
- **persist:marketplace-results:0001** (marketplace_results): view `view:marketplace-results` scope `provider_owned/provider_owned`
  - Marketplace results saved view owned by the provider catalog
  - compatibility=`current` reopen=`restored_exact`
- **persist:provider-admin-table:0001** (provider_admin_table): view `view:provider-admin-table` scope `policy_pinned/policy_governed`
  - Provider/admin table state persisted under an incompatible schema; reset to default on reopen
  - compatibility=`incompatible_needs_reset` reopen=`reset_to_default`
  - Incompatible: Saved under an incompatible admin-table schema that cannot be migrated; reset to the default view and disclosed the dropped filter and column choices
