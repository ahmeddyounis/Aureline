# Native Desktop Integration Report

Status: beta proof seeded

This report records the current native desktop integration truth packet. The
runtime projection is implemented in `aureline_shell::platform_integration` and
serializes through `schemas/platform/desktop_entry_event.schema.json`.

## Covered Entry Flows

| Flow | Current proof |
|---|---|
| system open | `desktop_entry_event_record` preserves literal target, canonical target, owner build/channel, and trust/profile context |
| file association | file-open entry routes through product-owned review when authority widens |
| auth callback | consumed or expired callbacks route to restart/reauth recovery, not silent authority reuse |
| protocol handler | privileged action classes require in-product review before execution |
| recent item and jump-list reopen | exact object reopen or truthful placeholder with locate/cached-context/close actions |
| reveal-in-shell | reveal-only action preserves bound identity and cannot write |
| privacy-safe native notification | OS notification and badge activation reopen exact durable objects with redacted summary payloads |

## Interruption Coverage

The seeded packet validates recovery rows for removable volume loss and return,
network share unavailability, missing workspace roots, credential-store lock,
display-topology drift, wake/resume, sleep-expired callbacks, and network
transition. Each row requires explicit recovery actions and blocks hidden
mutating replay.

## Drill Coverage

Current per-platform drills cover:

- channel precedence and no last-writer-wins handler ownership;
- handler spoof-resistance for browser and protocol returns;
- recent reopen fidelity when the target is stale or missing;
- lock-screen redaction and summary-only notification payloads;
- wake/resume truth with no silent privileged replay.

Fixture anchors:

- `fixtures/platform/m3/native_desktop_contract/packet_summary.json`
- `fixtures/platform/m3/native_desktop_contract/entry_events.json`
- `fixtures/platform/exact_target_reopen_cases/`
- `fixtures/platform/native_lifecycle_cases/`
- `fixtures/ux/m3/notification_privacy/`

## Remaining Follow-Ups

This is a governed proof projection, not a live OS adapter. The live platform
adapters still need to emit these records from real Launch Services, Windows
shell, and xdg/portal callbacks before the beta claim can graduate beyond the
seeded contract.

