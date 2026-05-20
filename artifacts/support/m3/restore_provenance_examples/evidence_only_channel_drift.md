# Restore packet — Evidence only (sync, side-by-side channel/version drift)

Drawn from corpus drill `evidence_only.channel_version_drift`.

| Field | Value |
| --- | --- |
| Source event | `sync` |
| Producer build | `producer:aureline:0.0.0-dev-beta-channel` |
| Schema outcome | `manual_review` |
| Fidelity result | **Evidence only** |
| Prior artifact | **required** — compare and export refs present |
| Visible in | diagnostics, support export, crash recovery |

## Missing dependencies (reopened as placeholders)

| Preserved pane | Dependency | Last-known provenance | Safe actions |
| --- | --- | --- | --- |
| `pane-provider-session-0001` | `missing_provider` | provider session from stable-channel install | reconnect, export evidence, manual review |

## Named exclusions (what was deliberately left out)

- secret material
- delegated approval
- live authority handle
- machine-unique trust anchor

## Reviewer reading

The package was synced from a side-by-side install on a different release
channel and version. The producer build does not match the importer and the
provider connection is unavailable on this channel, so import refuses to claim
authority: the outcome is **Evidence only** under manual review. Channel and
producer drift are surfaced in the packet rather than hidden, and the prior
artifact stays available for compare and export.
