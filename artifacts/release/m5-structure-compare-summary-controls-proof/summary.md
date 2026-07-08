# Structure Rows & Compare Summary Cards

- Packet: `structure-compare-summary-controls:stable:0001`
- Surface: `Structure rows and compare summary cards`
- Structure rows: 5 (1 redacted-hidden)
- Compare summary cards: 2 (64 changed objects total)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Structure rows

- **server.tls** [`artifact:config/app.yaml`]: added (structured_object) — High confidence: matches the config schema
- **server.port** [`artifact:config/app.yaml`]: modified (structured_object) — High confidence: scalar value change
- **config.redacted_credential** [`artifact:config/app.yaml`]: redacted_hidden (redacted_field) — A value changed here; content is withheld
- **package[left-pad]** [`artifact:package/Cargo.lock`]: removed (package_delta) — High confidence: dependency dropped from the lockfile
- **package[serde].source** [`artifact:package/Cargo.lock`]: metadata_only (metadata_field) — Metadata-only: source registry moved, version unchanged

## Compare summary cards

- **application config (YAML)** [`artifact:config/app.yaml`]: 3 changed (+1/-0/~1/meta 0/redacted 1) — risk [redacted_content_present=caution]
- **dependency lockfile** [`artifact:package/Cargo.lock`]: 61 changed (+40/-12/~8/meta 1/redacted 0) — risk [large_change_volume=caution]
