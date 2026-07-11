# Output trust banners and output provenance chip groups

- Packet: `m5-output-trust-banner-output-provenance-chip-group-controls:stable:0001`
- Surface: `M5 output trust banners and output provenance chip groups: plain-text, sanitized-rich, trusted-local-active, and isolated-remote-active trust classes, stale-output honesty, and copy/export choice across claimed notebook outputs`
- Output trust banners: 6 (5 not live)
- Output provenance chip groups: 6 (4 not a current pinned lineage)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-11T00:00:00Z)

## Output trust banners

- **Trusted local active output** — trust `trusted_output`, freshness `live_output` → `trusted_local_active`, representation `rendered_rich`, deep link `output_viewer`
- **Sanitized rich output (stale)** — trust `sanitized_output`, freshness `stale_output` → `sanitized_rich`, representation `rendered_rich`, deep link `notebook_location`
- **Isolated remote active output (cached)** — trust `sandboxed_output`, freshness `cached_output` → `isolated_remote_active`, representation `rendered_rich`, deep link `output_viewer`
- **Raw output shown as plain text (superseded)** — trust `raw_active_output`, freshness `superseded_output` → `plain_text`, representation `raw_source`, deep link `notebook_location`
- **Blocked output (cleared)** — trust `blocked_output`, freshness `cleared_output` → `blocked_content`, representation `redacted_representation`, deep link `support_bundle`
- **Unknown-trust output (no output)** — trust `unknown_trust`, freshness `no_output` → `unknown_content`, representation `raw_source`, deep link `docs_anchor`

## Output provenance chip groups

- **Cell-produced output (fully resolved)** — kind `produced_by_cell`, state `provenance_complete` → `cell_produced` / `fully_resolved`, deep link `notebook_location`
- **Run-produced output (execution count pinned)** — kind `produced_by_run`, state `execution_count_pinned` → `run_produced` / `lineage_pinned`, deep link `notebook_location`
- **Imported output (partial lineage)** — kind `imported_output`, state `provenance_partial` → `imported_origin` / `partially_resolved`, deep link `docs_anchor`
- **Restored output (execution count drifted)** — kind `restored_output`, state `execution_count_drifted` → `restored_origin` / `lineage_drifted`, deep link `support_bundle`
- **External output (lineage missing)** — kind `external_output`, state `provenance_missing` → `external_origin` / `unresolved`, deep link `docs_anchor`
- **Unknown-origin output (stale lineage)** — kind `unknown_provenance`, state `provenance_stale` → `unknown_origin` / `resolution_stale`, deep link `no_deep_link`
