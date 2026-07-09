# Experiment run rows and environment fingerprint cards

- Packet: `m5-experiment-run-row-environment-fingerprint-controls:stable:0001`
- Surface: `M5 experiment run rows and environment fingerprint cards: run origin, commit/workspace revision, execution origin, outcome, and captured-environment truth across claimed notebook and data surfaces`
- Experiment run rows: 6 (3 not first-party)
- Environment fingerprint cards: 6 (4 not reliably captured)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Experiment run rows

- **Baseline sweep (notebook)** — origin `notebook_cell`, status `succeeded` → `local_run`, revision `commit:9f3ac21`, deep link `run_object`
- **Nightly featurize (script)** — origin `script_task`, status `running` → `local_run`, revision `commit:1c7de40`, deep link `notebook_location`
- **Weekly retrain (scheduled)** — origin `scheduled_task`, status `queued` → `managed_run`, revision `workspace:main@rev-204`, deep link `run_object`
- **Attached external eval** — origin `manual_attach`, status `failed` → `manually_attached`, revision `commit:unknown-attached`, deep link `docs_anchor`
- **Imported baseline (last quarter)** — origin `imported_run`, status `stale` → `imported_run`, revision `commit:imported-a13f`, deep link `dataset_catalog_anchor`
- **Unlabeled run** — origin `unknown_origin`, status `canceled` → `origin_unknown`, revision `workspace:unresolved`, deep link `no_deep_link`

## Environment fingerprint cards

- **Interpreter fingerprint** — scope `interpreter`, state `captured_complete` → `captured`, deep link `run_object`
- **Kernel spec fingerprint** — scope `kernel_spec`, state `captured_partial` → `partially_captured`, deep link `notebook_location`
- **Package fingerprint** — scope `packages`, state `pinned` → `pinned`, deep link `dataset_catalog_anchor`
- **Accelerator fingerprint** — scope `hardware_accelerator`, state `drifted` → `uncaptured`, deep link `docs_anchor`
- **OS / platform fingerprint** — scope `os_platform`, state `captured_missing` → `uncaptured`, deep link `no_deep_link`
- **Container image fingerprint** — scope `container_image`, state `unavailable` → `uncaptured`, deep link `docs_anchor`
