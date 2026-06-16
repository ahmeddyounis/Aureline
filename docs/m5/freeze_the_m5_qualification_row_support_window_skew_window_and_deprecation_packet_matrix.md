# Freeze the M5 qualification-row, support-window, skew-window, and deprecation-packet matrix

This document is the human-readable companion to the canonical M5 qualification/skew matrix checked in at `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json` and described by the schema at `schemas/compat/m5-qualification-and-skew.schema.json`.

## Purpose

No M5 stable-facing surface may claim support, parity, or certification without a machine-readable qualification row, a declared skew window, explicit downgrade behavior, and a current claim-publication linkage. This matrix is the single source of truth those claims reuse: docs, release notes, CLI inspect, in-product badges, support exports, certification reports, and shiproom dashboards all ingest the same rows. Support means named matrix rows with freshness, caveats, skew behavior, and downgrade rules — not anecdotal success on one environment.

## Structure

The matrix contains:

- **Qualification rows** — one per M5 stable-facing family or boundary (`notebook`, `ai_provider`, `remote_helper`, `companion`, `ecosystem`, `managed_service`, `toolchain_runtime`). Each row binds the family to the stable claim it backs and the lifecycle label it effectively publishes after narrowing.
- **Qualification row cells** — one cell per dimension, per family: `platform`, `deployment_profile`, `archetype_bundle`, `toolchain_envelope`, and `client_scope`. Every dimension is an explicit, inspectable truth; the row must cover every dimension exactly once. A cell is `qualified`, `limited` (qualified with a recorded caveat), `retest_pending`, `stale`, `waived`, or `missing`.
- **Skew window** — the supported skew class (`lockstep_only`, `bounded_skew`, `backward_compatible`, `forward_compatible`, `unsupported_skew`), a version floor/ceiling, the negotiated fields, and the behavior a peer outside the window triggers (`fail_closed`, `reconnect_required`, `reinstall_required`, `coordinated_upgrade_only`, `block_boundary`).
- **Support window** — the support class (`full_support`, `maintenance_only`, `security_only`, `limited`, `end_of_life`), the supported-since date, and an optional end-of-support date.
- **Deprecation packet** — the deprecation status (`active`, `deprecated`, `successor_available`, `removal_scheduled`, `removed`), an optional successor ref, removal date, and migration ref.
- **Narrowing reasons** — the closed set of reasons a family drops below the cutline. A non-holding cell, an unsupported skew, an ended support window, or a staged removal must name its narrowing reason.
- **Stop rules** — closed conditions that gate promotion. Every narrowing reason has a corresponding rule.
- **Promotion verdict** — `proceed` or `hold`, computed from the firing stop rules.

## Narrowing rules

- A family carries a Stable (or LTS) qualification claim only when every dimension is qualified (or `limited` with a recorded caveat, or `waived` under an unexpired waiver), its peer is inside the supported skew window, the deprecation packet is `active`, the support window is open, the proof packet is current within its freshness SLO, and the owner has signed off.
- A family that loses any of those must drop **below** the cutline rather than inherit an adjacent qualified family. The published label is a hard ceiling: it may never exceed the claim's canonical label.
- A `limited` row holds the claim but must record at least one compatibility caveat. A family held provisionally rides an active, unexpired waiver; an expired waiver narrows it.
- A boundary in an `unsupported_skew` class narrows and names `skew_window_exceeded`; its declared unsupported-skew behavior (for example `reinstall_required` or `reconnect_required`) tells the client how to recover. A family whose support window is `end_of_life` narrows and names `support_window_ended`. A family whose deprecation packet is `removal_scheduled` (or `removed`) narrows and names `deprecation_scheduled`, and must declare its successor and migration packet.

## Consumption

Downstream docs, Help/About, CLI inspection, in-product badges, and support-export surfaces should ingest `support_export_projection()` from the typed model (`aureline_release::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix`) rather than cloning status text, so every surface renders one source of truth.

## Freshness

The matrix is checked in with an `as_of` date and a per-row proof packet freshness SLO. A row whose proof packet breaches its SLO narrows automatically before publication; the frozen CI validation capture at `artifacts/release/captures/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix_validation_capture.json` records the summary, promotion verdict, negative drills, and fixture cases the gate enforces.
