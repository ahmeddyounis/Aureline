# Fixtures: Project Doctor container boundary truth

This directory contains fixture metadata for the
`project_doctor_container_boundary_truth` packet.

The canonical full corpus is checked in at:

`artifacts/doctor/m5/project-doctor-container-boundary-truth.json`

## Coverage

- **Both M5 workflow surfaces** have container-dependent scenarios:
  `remote_preview` and `incident_workflow`.
- **Every engine class** appears: `docker`, `podman`, `devcontainers_cli`, and
  `managed_cloud`. **Every reachability** appears: `reachable`, `unreachable`,
  and `policy_blocked`. **Every support class** appears: `certified`,
  `supported`, `experimental`, and `unsupported`.
- **Every workspace mode** (`attached_container`, `devcontainer`,
  `remote_managed`) and **every boundary label** (`local`, `remote`, `managed`)
  appear, with the boundary label always consistent with the mode.
- **Every definition source** (`devcontainer_json`, `dockerfile`,
  `compose_file`, `image_reference`, `managed_template`) and both rebuild
  decisions (`rebuild`, `reuse_existing`) appear.
- **Every log availability** appears: `live`, `buffered`, `snapshot`, and
  `unavailable`. Available logs always carry an export-safe time range; the
  redaction posture is always `metadata_safe_default`.
- **Every preflight decision and reason** appears: `proceed_full`/`none`
  (attach-and-reuse on remote preview and incident), `proceed_with_disclosure`
  for `trust_gated_hooks` (preview rebuild with a trust-gated postCreate hook),
  `side_effects_require_review` (incident rebuild with ports + writable mount;
  experimental rebuild with a published port), `unsupported_engine` (managed
  cloud), `blocked_offer_alternative` for `engine_unreachable` (incident, engine
  down) and `policy_blocked` (preview, devcontainers CLI blocked by policy).
- **The non-inheriting preflight gate** is provable: every scenario's published
  `published_preflight_decision` and `published_preflight_reason` equal the
  decision recomputed from its own reachability, support class, and disclosed
  side effects. Tampering with any input (making the engine unreachable, adding a
  trust-gated hook to a `proceed_full` scenario) makes the published decision
  diverge and fails validation.
- **No dead ends, no silent hooks**: every blocked scenario offers a non-empty
  stay-local alternative and at least one diagnostics action, and no scenario
  with a trust-gated hook is published as `proceed_full`.
- **Cross-surface parity**: every scenario renders on `desktop_sheet`,
  `cli_inspect`, `headless_json`, `browser_handoff`, `support_export`, and
  `incident_packet`, carries the locale-invariant `machine_meaning_keys`, and is
  metadata-safe (`redaction_class: metadata_safe_default`,
  `raw_private_material_excluded: true`, `overcapture_excluded: true`).
