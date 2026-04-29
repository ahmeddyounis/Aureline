# Project Doctor probe and explanation cases

These fixtures seed the focused Project Doctor probe catalog and
finding-explanation contracts.

| File | Record kind | Purpose |
|---|---|---|
| `probe_catalog_toolchain_read_only.yaml` | `probe_catalog_entry_record` | read-only execution-context probe admitted for Doctor |
| `probe_catalog_cache_repair_promoted.yaml` | `probe_catalog_entry_record` | mutating cache repair blocked in Doctor and promoted to repair |
| `finding_explanation_extension_crash_loop.yaml` | `doctor_explanation_packet_record` | crash-loop finding explains evidence and links safe mode, bisect, repair, help, bundle, and escalation refs |
| `finding_explanation_helper_attach_escalation.yaml` | `doctor_explanation_packet_record` | remote-helper finding explains managed approval and escalation handoff |

The manifest lists the governing docs, schemas, and reviewer
assertions.
