# Compatibility row seed

This document is the narrative companion to the first compatibility
qualification seed. It exists so mixed-version support, schema drift,
reference-workspace certification, helper or agent skew, and
provider-linked lanes all project from one shared row model instead of
drifting into ad hoc spreadsheets and release-note prose.

Companion artifacts:

- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  — canonical qualification rows. Each row names a stable `row_id`,
  boundary label, deployment-profile scope, support class, skew window,
  fail posture, owner, evidence state, and next review milestone.
- [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — explicit supported, best-effort, untested, and unsupported skew
  cases for each qualification row.
- [`/artifacts/governance/governance_packet_template.yaml`](../../artifacts/governance/governance_packet_template.yaml)
  — later release-time `compatibility_report` and `claim_manifest`
  packets extend the row ids defined here.

## Why this exists before release packets

The repository already had:

- release-time compatibility reports reserved under `artifacts/release/`,
- exact-build identity as a shared build-truth model,
- command, profile/state, extension, and benchmark artifacts with their
  own schemas or seeded docs, and
- architecture-level compatibility rules in the design docs.

What it did not have was one row model that ties those boundaries
together. Without that row model, the first compatibility report would
need to invent its own ids, the first claim manifest would paraphrase
those ids in prose, and the first certified-archetype packet would
restate the same scope again with slightly different wording.

The seed fixes that now:

- `artifacts/compat/` is the canonical pre-release compatibility seed
  home.
- `artifacts/release/` remains the home for concrete release packets.
- Release packets extend the seeded row ids; they do not replace them.

## Row model

Every compatibility row carries:

- a stable `row_id`,
- a stable `artifact_or_protocol_boundary_label`,
- an explicit interface pair,
- one or more named deployment profiles,
- one support class,
- one skew-window class and a human-readable window description,
- one explicit out-of-window posture,
- one owner reference,
- one review cadence,
- one evidence state, and
- one next review milestone.

This is the minimum needed to let later tooling cite the same row from
compatibility reports, claim manifests, benchmark packets,
deployment-profile artifacts, and certified-archetype packets without
free-text aliasing.

## Status model

The qualification matrix answers: "what boundary exists, what does it
claim, and who owns it?"

The version-skew register answers: "for that row, which skew cases are
supported, best-effort, untested, or unsupported?"

Those are intentionally separate. A row may have:

- `supported` cases that are admissible for release or claim-bearing
  evidence,
- `best_effort` cases that degrade honestly and remain inspectable,
- `untested` cases that are reserved explicitly instead of silently
  promoted, and
- `unsupported` cases that fail closed, go read-only, degrade with a
  typed reason, or are explicitly blocked.

The register is where unsupported and untested diverge. They are never
collapsed into one vague "not ideal" state.

## Seeded row families

| Row id | Boundary label | Purpose | Default out-of-window posture |
|---|---|---|---|
| `compat_row:desktop_benchmark_lab.exact_build_identity` | `boundary.build.exact_build_identity` | Ties benchmark evidence to exact-build identity instead of version strings. | `fail_closed` |
| `compat_row:command_plane.command_descriptor_schema` | `boundary.commands.command_descriptor` | Keeps palette, CLI, menu, AI-tool, and invocation-session consumers on one command contract. | `fail_closed` |
| `compat_row:release_identity.exact_build_propagation` | `boundary.build.exact_build_identity_propagation` | Keeps binaries, symbols, docs/help, support bundles, and release evidence on one build identity. | `fail_closed` |
| `compat_row:state.profile_layout_schema` | `boundary.profile.portable_profile_and_layout_restore` | Makes profile/schema drift explicit across restore, layout, and support/export surfaces. | `degraded` |
| `compat_row:extension_host.sdk_wit_permission_window` | `boundary.extensions.host_contract_and_permission_vocab` | Seeds WIT, SDK/runtime, permission-vocabulary, bridge, and helper-family compatibility rows. | `explicitly_unsupported` |
| `compat_row:launcher.local_helper_contracts` | `boundary.runtime.launcher_local_sidecars` | Seeds local helper or sidecar compatibility before helper lanes land. | `fail_closed` |
| `compat_row:remote.attach_envelope_and_drift` | `boundary.remote.agent_attach_envelope` | Seeds remote-agent attach, reconnect, and staged-upgrade truth. | `degraded` |
| `compat_row:tooling.task_event_envelope` | `boundary.tooling.task_event_envelope` | Reserves one compatibility row for canonical task-event envelopes even before the dedicated schema export lands. | `fail_closed` |
| `compat_row:provider.service_api_and_browser_handoff` | `boundary.provider.service_api_family` | Covers provider-linked lanes, browser handoff, and future optional service-API families. | `read_only` |
| `compat_row:certification.launch_archetype_matrix` | `boundary.certification.launch_archetype_matrix` | Links certified-archetype and launch-language claims to current reference-workspace evidence. | `degraded` |

## How later artifacts use these rows

### Compatibility reports

Release-time compatibility reports should add:

- release-specific evidence refs,
- migration notes,
- release verdicts, and
- per-row freshness timestamps.

They should not rename row families, collapse multiple boundaries into
one prose paragraph, or carry compatibility status without a `row_id`.

### Claim manifests

Claim manifests should cite:

- `row_id`,
- `artifact_or_protocol_boundary_label`, and
- the release evidence that currently backs the claim.

That makes support-class wording, caveat text, and validity windows
traceable back to one seeded compatibility row.

### Certified-archetype packets

Certified-archetype or launch-language packets should cite the same
`row_id` values the compatibility report and claim-manifest lanes cite.
That keeps "Certified", "Supported", and "Experimental" wording
mechanically aligned with the same source rows and freshness windows.

### Deployment-profile artifacts

Install-topology, boundary, and deployment-profile artifacts should cite
the row id plus the named deployment profiles already attached to the
row. That avoids profile-local aliases such as "desktop compatible" or
"enterprise supported" with no stable backing id.

## Change discipline

- Add a new row when a new boundary becomes claim-bearing.
- Update the existing row when the same boundary gains a new skew
  window, deployment-profile scope, or evidence state.
- Do not create a new spreadsheet or report-local alias for a boundary
  that already has a `row_id`.
- If a release packet or docs page needs wording that the row model
  cannot express, extend the row model here first and update the release
  packet second.
