# Notebook document headers and kernel-state strips

- Packet: `m5-notebook-document-header-kernel-state-strip-controls:stable:0001`
- Surface: `M5 notebook document headers and kernel-state strips: canonical .ipynb source, selected kernel origin, busy/queued/offline truth, and no-kernel edit parity across claimed notebook surfaces`
- Notebook document headers: 6 (3 not a settled canonical source)
- Kernel-state strips: 6 (3 not a live kernel)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Notebook document headers

- **analysis.ipynb (local)** — source `local_ipynb`, identity `saved_clean` → `local_document`, export `Paired HTML export is up to date`, deep link `notebook_location`
- **featurize.ipynb (remote)** — source `remote_ipynb`, identity `autosaved` → `remote_document`, export `No paired export for this notebook`, deep link `notebook_location`
- **retrain.ipynb (managed)** — source `managed_workspace_ipynb`, identity `unsaved_changes` → `managed_document`, export `Paired report export is stale until you save`, deep link `kernel_manager`
- **baseline.ipynb (imported)** — source `imported_ipynb`, identity `read_only` → `imported_document`, export `No paired export for an imported notebook`, deep link `docs_anchor`
- **Untitled scratch notebook** — source `scratch_untitled`, identity `conflicted` → `scratch_document`, export `No paired export until the notebook is saved`, deep link `support_bundle`
- **Recovered notebook (unknown source)** — source `unknown_source`, identity `recovered` → `unknown_document`, export `No paired export for an unresolved notebook`, deep link `no_deep_link`

## Kernel-state strips

- **Kernel ready (local)** — execution `idle_ready`, connection `connected_local` → `ready_live`, deep link `kernel_manager`
- **Kernel busy (remote)** — execution `busy_running`, connection `connected_remote` → `busy_live`, deep link `kernel_manager`
- **Kernel queued (container)** — execution `queued_pending`, connection `reconnecting` → `queued_live`, deep link `kernel_manager`
- **Kernel disconnected (SSH)** — execution `disconnected_reconnecting`, connection `disconnected` → `disconnected_recoverable`, deep link `kernel_manager`
- **Kernel interrupted (inspect only)** — execution `interrupted`, connection `connection_lost` → `inspect_only`, deep link `docs_anchor`
- **No kernel selected** — execution `dead_no_kernel`, connection `never_connected` → `no_kernel_editable`, deep link `no_deep_link`
