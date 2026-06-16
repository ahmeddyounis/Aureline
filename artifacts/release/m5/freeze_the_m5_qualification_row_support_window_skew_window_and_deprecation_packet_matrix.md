# M5 Qualification/Skew Matrix Artifact Companion

This file is the artifact-level companion document for the checked-in M5 qualification-row, support-window, skew-window, and deprecation-packet matrix.

- **Canonical JSON**: `artifacts/release/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.json`
- **Schema**: `schemas/compat/m5-qualification-and-skew.schema.json`
- **Typed consumer**: `crates/aureline-release/src/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix/mod.rs`
- **Validation capture**: `artifacts/release/captures/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix_validation_capture.json`
- **Companion doc**: `docs/m5/freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix.md`
- **Generator**: `tools/regenerate_m5_qualification_and_skew_matrix.py`

The matrix is the single source of truth for M5 qualification-row scope, per-dimension qualification posture, declared skew windows, support windows, deprecation packets, claim-publication linkage, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 tools/regenerate_m5_qualification_and_skew_matrix.py` from the repository root after changing the rows.
