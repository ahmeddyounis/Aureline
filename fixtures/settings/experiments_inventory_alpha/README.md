# Experiments Inventory Alpha Fixtures

These fixtures exercise the settings-owned experiments inventory contract.
They are copy-safe examples for CLI inspection, diagnostics, support exports,
and downgrade/import warnings. The canonical source remains
`artifacts/governance/experiments_inventory_alpha.yaml`.

Protected states covered:

- visible lifecycle rows for Labs, Preview, Beta, Stable, Deprecated,
  DisabledByPolicy, and Retired;
- policy-disable and kill-switch precedence with preserved local data;
- saved-artifact dependency warnings for profile, workspace, saved/export, and
  migration packet flows.
