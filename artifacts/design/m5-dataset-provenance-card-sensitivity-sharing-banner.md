# Dataset provenance cards and sensitivity / sharing banners

- Packet: `m5-dataset-provenance-card-sensitivity-sharing-banner-controls:stable:0001`
- Surface: `M5 dataset provenance cards and sensitivity/sharing banners: source, snapshot, sample/redaction, sensitivity, and local-versus-remote location truth across claimed data lanes`
- Dataset provenance cards: 6 (3 not local)
- Sensitivity / sharing banners: 6 (1 include a raw payload)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Dataset provenance cards

- **Customer events (tracked)** — source `tracked_dataset`, provenance `provenance_complete` → `remote_data` / `provenanced`, sensitivity `internal`, deep link `dataset_catalog_anchor`
- **Feature matrix (local file)** — source `local_file`, provenance `version_pinned` → `local_data` / `pinned`, sensitivity `public_safe`, deep link `notebook_location`
- **Pricing snapshot (remote)** — source `remote_snapshot`, provenance `provenance_partial` → `remote_data` / `partially_provenanced`, sensitivity `confidential`, deep link `dataset_catalog_anchor`
- **Synthetic eval set** — source `synthetic_data`, provenance `provenance_missing` → `local_data` / `unprovenanced`, sensitivity `public_safe`, deep link `docs_anchor`
- **Redacted PII sample** — source `redacted_sample`, provenance `access_restricted` → `local_data` / `unprovenanced`, sensitivity `regulated`, deep link `docs_anchor`
- **Unlabeled input** — source `unknown_source`, provenance `version_drifted` → `location_unknown` / `unprovenanced`, sensitivity `unknown_sensitivity`, deep link `no_deep_link`

## Sensitivity / sharing banners

- **Public-safe metadata share** — sensitivity `public_safe`, scope `summary_plus_metadata` → `metadata_safe`, deep link `docs_anchor`
- **Internal raw-payload share** — sensitivity `internal`, scope `raw_payload_included` → `raw_exposed`, deep link `docs_anchor`
- **Confidential evidence share** — sensitivity `confidential`, scope `evidence_included` → `evidence_scoped`, deep link `dataset_catalog_anchor`
- **Regulated redacted share** — sensitivity `regulated`, scope `redacted_share` → `redacted`, deep link `docs_anchor`
- **Production-like blocked share** — sensitivity `production_like`, scope `share_blocked` → `blocked`, deep link `docs_anchor`
- **Unknown-sensitivity summary share** — sensitivity `unknown_sensitivity`, scope `summary_only` → `metadata_safe`, deep link `no_deep_link`
