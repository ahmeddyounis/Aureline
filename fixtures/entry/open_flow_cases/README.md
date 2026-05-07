# Entry open-flow disambiguation + parity fixtures

This directory is the seed corpus for the open-flow disambiguation and parity packet:

- Contract: `artifacts/entry/open_flow_truth.md`
- Map: `artifacts/entry/open_route_command_map.yaml`

Each `*.yaml` file is one worked case proving that:

- Start Center, main menu, command palette, protocol handlers, and deep links can all invoke the same **open-route vocabulary**; and
- multi-root widening and cross-window “open in new window” behavior are explicit, named choices (never silent guesses).

## Index

| Fixture | Scenario focus |
| --- | --- |
| `open_local_file_parity.yaml` | open local file parity across Start Center/menu/palette/OS handoff |
| `open_workspace_manifest_parity.yaml` | open workspace manifest parity across Start Center/menu/palette/deep link |
| `open_folder_scope_consequence_disambiguation.yaml` | replace current vs open new window vs add root (explicit multi-root truth) |
| `remote_attach_vs_clone_disambiguation.yaml` | remote attach/resume vs clone disambiguation for remote targets |
| `import_handoff_parity.yaml` | import handoff parity + required review posture |

