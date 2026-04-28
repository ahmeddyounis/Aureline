# Extension lockfile and recommendation fixtures

These fixtures exercise the continuity contract in:

- [`/docs/ecosystem/extension_lockfile_and_recommendation_contract.md`](../../../docs/ecosystem/extension_lockfile_and_recommendation_contract.md)
- [`/schemas/ecosystem/extension_lockfile.schema.json`](../../../schemas/ecosystem/extension_lockfile.schema.json)
- [`/schemas/ecosystem/extension_recommendation_set.schema.json`](../../../schemas/ecosystem/extension_recommendation_set.schema.json)

The examples are deliberately small but cover the key state vocabulary:
native, compatibility bridge, blocked, manual review required, workspace
owned, profile owned, imported, and admin suggested.

| Fixture | Schema | Coverage |
|---|---|---|
| `native_workspace_lock.json` | `extension_lockfile.schema.json` | Workspace recommendation resolved natively through an approved mirror with permission and publisher-continuity refs. |
| `imported_bridge_and_blocked_lock.json` | `extension_lockfile.schema.json` | Imported bridge row, policy-blocked row, and manual-review placeholder row in one reviewable lockfile. |
| `recommendation_set_mixed_lanes.json` | `extension_recommendation_set.schema.json` | Workspace, profile, imported, and admin-suggested groups projecting to the same continuity states. |

