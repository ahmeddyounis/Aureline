# Path-map selector examples

These fixtures show how support, backup, clear-data, and portability
tools select the same path-level state families from
[`/artifacts/state/path_level_seed_map.yaml`](../../../artifacts/state/path_level_seed_map.yaml)
without interpreting path strings or inventing per-tool taxonomies.

Each example:

- quotes `path_row_id` values from the seed map;
- names the selector fields used by the tool;
- lists protected rows that the tool must skip, preview, or route
  through an authority-specific flow; and
- avoids raw paths, raw secrets, hostnames, or user content.

| Fixture | Tool flow | Key coverage |
|---|---|---|
| [`support_bundle_selector.yaml`](./support_bundle_selector.yaml) | support bundle | Metadata defaults, opt-in redaction, and raw-secret exclusion. |
| [`backup_selector.yaml`](./backup_selector.yaml) | backup/restore | Durable truth, selected recovery state, retained evidence, and derived-cache exclusion. |
| [`clear_data_selector.yaml`](./clear_data_selector.yaml) | clear data / low disk | Rebuildable cache clear plus protected local-history, recovery, policy, and credential denial. |
| [`portability_selector.yaml`](./portability_selector.yaml) | profile/workspace/offboarding export | Portable profile rows, workspace-shared rows, tenant offboarding rows, and excluded credentials. |
