# Implement public-interface diff reports, compatibility windows, support-class caveats, and successor/deprecation packets for changed stable-facing M5 contracts

This document is the human-readable companion to the canonical M5 public-interface diff-report register checked in at `artifacts/release/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.json` and described by the schema at `schemas/compat/m5-public-interface-diff-reports.schema.json`.

## Purpose

M5 introduces new stable-facing artifact families and helper/agent/provider boundaries, and it *changes* contracts that already shipped. Where the M5 qualification/skew matrix (`schemas/compat/m5-qualification-and-skew.schema.json`) freezes the *static* qualification row, support window, and deprecation packet each stable-facing family holds, and the stable version-windows freeze the per-surface version floor/ceiling, this register speaks for the **change**: every stable-facing M5 contract that M5 touched gets one diff report. A reader can tell, from one row, what changed at a contract, whether the change is additive/behavioral/breaking, whether the reader and writer sides were reviewed compatible, the compatibility window the contract now lives in, the support class and caveats it publishes, and — for a deprecated contract — the successor (replacement path), alias map, removal horizon, migration guide, and rollback implications. Docs, Help/About, release-center, service-health, CLI inspect, support, upgrade-notes, and export surfaces ingest the same rows so the current-vs-replacement contract and current support window resolve from one packet.

## Structure

Each report binds one changed contract to:

- **Contract kind** — `schema` (a wire/state schema), `cli_headless_output` (a CLI/headless output contract), `exported_packet` (an exported truth/support packet), `sdk_runtime_contract` (an SDK/runtime contract), or `compatibility_bridge` (a mixed-version bridge) — and the **change class** (`additive`, `behavioral`, or `breaking`).
- **Public-interface diff** — the `added`, `removed`, and `changed` surface elements, plus the reader-side and writer-side compatibility review (`compatible`, `breaking`, or `unreviewed`). A producer-side schema update is never treated as sufficient: both sides carry a review posture.
- **Compatibility window** — the version floor, current, and ceiling, the compatibility posture (`fully_compatible`, `backward_compatible`, `forward_compatible`, or `breaking`), and whether the support window is `within_window` or `support_ended`.
- **Support-class caveat** — the support class (`fully_supported`, `supported_with_caveats`, `limited`, or `unsupported`) and the caveats that narrow the marketed claim.
- **Successor/deprecation packet** — for a deprecated contract: the deprecation status (`deprecated`, `superseded`, `removal_scheduled`, or `removed`), the owner, the successor (replacement path), the old→new alias map, the removal checkpoint and horizon, the migration guide, the rollback implications, and whether the removal is overdue.
- **Claim linkage** — the stable claim the contract backs, the report state earned, the active narrowing reasons, and the effective label after narrowing.

## Narrowing rules

- A changed contract carries a Stable (or LTS) support claim only when its diff report is current, its reader/writer compatibility review is complete, any breaking change is governed by a **complete, in-horizon** deprecation packet (owner, successor, removal horizon, migration, and rollback implications), its compatibility window is open, its proof packet is current within its freshness SLO, the owner has signed off, and its backing claim holds. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A breaking change whose diff carries **no** deprecation packet narrows on `breaking_change_unpacketed`; a packet that omits any required field narrows on `deprecation_packet_incomplete`; a packet whose removal checkpoint has passed narrows on `removal_overdue`. A managed breaking change with a complete, in-horizon packet — successor named, old surface retained as an alias — still holds, published with caveats.
- A change whose reader/writer compatibility review is missing on either side narrows on `reader_writer_review_missing`, so a producer-side update never promotes on its own.
- A contract whose compatibility/support window has ended narrows on `support_window_ended`, even when the diff and packet are otherwise complete.
- The diff and the support claim are distinct. A change can be backward compatible — so the diff is clean — yet still narrow its published support claim because the diff report's proof packet went stale (`evidence_stale`) or was never captured (`evidence_missing`). This narrows the marketed surface predictably instead of over-claiming.
- A `limited` report and a `supported_with_caveats` class must each record at least one caveat. A contract held provisionally rides an active, unexpired waiver; an expired waiver narrows it.

## Consumption

Downstream Help/About, release-center, service-health, CLI inspection, upgrade-notes, and support-export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts`) rather than cloning status text, so every surface renders one source of truth for the current-vs-replacement contract and its support window.

## Freshness

The register is checked in with an `as_of` date and a per-report proof packet freshness SLO. A report whose proof packet breaches its SLO narrows automatically before publication; the frozen CI validation capture at `artifacts/release/captures/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces. Regenerate with `python3 tools/regenerate_m5_public_interface_diff_reports.py` from the repository root after changing the reports.
