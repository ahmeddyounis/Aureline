# Protected small-project fixture repositories

This directory holds small, locally stored fixture source trees intended for
daily dogfooding of the core open/edit/save/restore loop.

The canonical list and recipes live in:

- `artifacts/milestones/m1/dogfood_matrix.yaml`

Validate that the fixture set and matrix stay consistent by running:

`python3 ci/check_m1_dogfood_matrix.py --repo-root .`

