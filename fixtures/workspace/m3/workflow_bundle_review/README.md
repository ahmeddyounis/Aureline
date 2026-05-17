# Workflow Bundle Review Fixtures

These fixtures exercise the workspace-level workflow-bundle review packet:

- `launch_wedge_install_update_drift_rollback.json` covers a certified launch bundle with install/update diff, local override drift, missing package drift, remove review, rollback checkpoint linkage, mirror posture, diagnostics, CLI, and support export parity.
- `imported_user_pending_review.json` covers an imported-user handoff bundle where migration state remains review-only, unsupported items stay visible, and removal preserves user-owned imported artifacts.

The fixtures intentionally carry only opaque refs, closed vocabulary values, and short support-safe identifiers. They do not include raw settings values, local paths, source content, secrets, or signing material.
