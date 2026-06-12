# Help / About truth source seed

The Help / About / provenance / service-health surface is Aureline's first
honest truth surface. A reviewer or end user opens it to answer "what build
is running, how was it installed, what client scope am I in, where do I take
issues, and is anything visibly degraded?" without scanning logs or chasing
a marketing page.

This document is the reviewer-facing landing page for the seed shipped under
`crates/aureline-shell/src/help_about/`. The canonical machine-readable
contract and the wider badge vocabulary land in a later milestone; this seed
deliberately covers the slice of that contract the initial in-product surface
needs.

The beta release-truth attachment now lets Help / About consume the shared
claim-manifest and compatibility-report projection through
`HelpAboutReleaseTruthCard`. When that card is attached, the surface carries
the same manifest rows as service health and activates the community handoff
action without losing the current object or issue context.

The docs / public-proof parity blocker
(`tools/ci/m3/docs_public_proof_gate/`) verifies that the freshness tokens
rendered by this Help / About card remain the same tokens used by the claim
manifest and public-proof packets. Its generated report lives at
`artifacts/docs/m3/public_proof_parity_report.md`.

The optional-surface release attachment adds one more machine source for
Help / About and support-export rows:
[`/artifacts/release/optional_surface_qualification.json`](../../artifacts/release/optional_surface_qualification.json).
When an opt-in capability, optional integration, secondary platform, or preview
surface lacks its own stable qualification packet, Help / About renders that
row's `displayed_label` and `active_narrow_reasons` from the register rather
than inheriting a neighboring Stable claim.

Infrastructure and incident-adjacent DevOps/SRE depth uses the same model:
[`/artifacts/infra/infrastructure-surface-qualification/support_export.json`](../../artifacts/infra/infrastructure-surface-qualification/support_export.json)
is the checked source for infrastructure `displayed_posture`,
`narrow_reasons`, and packet refs, and
[`/docs/help/infrastructure-surface-qualification.md`](./infrastructure-surface-qualification.md)
is the help-facing summary over that same row set.

## What the seed surface guarantees

The surface is a thin projection over upstream truth — no surface in the
shell forks a private copy of build, runtime, or freshness identity:

- **Build identity** is quoted verbatim from
  [`aureline_build_info::BuildIdentityRecord`]. The exact-build identity
  ref, commit, tree state, target triple, and toolchain channel come from
  the same record support exports already attach.
- **Install mode** is derived from the release-channel-class token minted
  by [`aureline_build_info::release_channel_class`] joined to the
  `dirty` bit on the build record. Unrecognized channel tokens light a
  typed `unknown_install_mode` honesty marker rather than silently
  rendering as `stable`.
- **Client scope** projects the shared
  [`crate::badges::target_origin::TargetOriginBadge`] vocabulary from the
  resolved [`aureline_runtime::ExecutionContext`]. Help / About reads the
  same `local_desktop`, `remote_host`, `managed_workspace`, `local_to_remote`,
  `local_to_managed`, and `degraded_trust` tokens that the terminal pane,
  task seed, debug-prep seed, and provider/auth chip already render.
- **Docs and help truth** quotes the source / version-match / freshness
  vocabulary minted by the embedded docs/help boundary card so the help
  shell never invents its own freshness ladder.
- **Service health** renders typed seed-placeholder rows for the local
  runtime, the auth subsystem, the docs/help subsystem, and the update
  channel. Every row carries a `seed_placeholder_awaiting_wiring` state
  until a live subsystem monitor is attached.
- **Provenance** renders typed seed-placeholder rows for signature,
  attestation, checksum, SBOM, and open-advisory state. The full
  machine-readable verifier lands through the about-card hardening path.
- **Community handoff** carries the frozen route classes —
  `public_issue_tracker`, `public_rfc_forum`, `private_security_channel`,
  `private_support_channel` — with stable disclosure copy, destination
  trust-class tokens, data-exit boundaries, auth expectations, and issue
  template refs.

## Honesty marker semantics

`HelpAboutSurface::honesty_marker_present` is a single bit the chrome
mirrors as a visible chip. It lights when **any of** the following hold:

- the install-mode row resolved to `unknown_install_mode`;
- the client-scope badge carries a degraded execution-context field, a
  pending trust posture, or no upstream context was wired;
- the docs/help source row reports a `degraded_cached`, `stale`, or
  `unverified` freshness, or an `incompatible_drift_detected` /
  `pre_release_unverified` / `unknown_target_build` version match.

The seed-placeholder rows for service health and provenance do not light
the global honesty marker on their own. Those rows are in-spec for the initial
seed and labeled with a typed `seed_placeholder_awaiting_wiring` state so
the chrome cannot fabricate "all green" before the live aggregator lands.

## Action vocabulary

Help / About exposes a closed action vocabulary so the chrome and support
exports never invent button-only labels:

| Action class | Availability default | Seed behaviour |
|---|---|---|
| `open_execution_context_inspector` | `live` | Routes to the shared runtime inspector. |
| `copy_context_for_support_export` | `live` | Renders the deterministic plaintext block. |
| `open_release_packet` | `reserved_for_later_milestone` | Reserved until release-packet linkage is attached. |
| `view_provenance_details` | `reserved_for_later_milestone` | Reserved. |
| `open_advisory_history` | `reserved_for_later_milestone` | Reserved. |
| `report_issue_via_community_handoff` | `reserved_for_later_milestone` | Upgrades to `live` when the release-truth card is attached; routes to the matching public or private lane while preserving object and issue context. |

Live action availability narrows in two cases. When the resolved execution
context carries a degraded field, `open_execution_context_inspector`
downgrades to `blocked_by_degraded_context`; when trust is pending it
downgrades to `blocked_by_pending_trust`. The
`copy_context_for_support_export` action is held `live` even on degraded
contexts — that is precisely when support packets need the truth-source
dump.

## Failure drill

Open Help / About while the docs/help source row reports
`freshness_class = stale` and
`version_match_state = incompatible_drift_detected`. The fixture
`fixtures/help/about_cases/failure_drill_stale_docs_source.json` exercises
this drill. Expected behaviour:

- `docs_help_truth.honesty_marker_present == true`;
- `docs_help_truth.freshness_class_token == "stale"` rendered verbatim;
- the global `honesty_marker_present` is true;
- `client_scope.honesty_marker_present` stays false (the client-scope
  upstream is still healthy);
- `copy_context_for_support_export` stays `live`.

## Fixture index

| Fixture | Scenario |
|---|---|
| [`protected_walk_local_dev.json`](../../fixtures/help/about_cases/protected_walk_local_dev.json) | Trusted local desktop, exact-build docs match, no honesty marker. |
| [`failure_drill_stale_docs_source.json`](../../fixtures/help/about_cases/failure_drill_stale_docs_source.json) | Stale `mirrored_official_docs` snapshot lights honesty without collapsing actions. |
| [`missing_execution_context.json`](../../fixtures/help/about_cases/missing_execution_context.json) | Pre-bootstrap surface lights `unknown` boundary cue and surfaces a context-missing marker. |

## Out of scope

The seed deliberately does **not** own:

- the canonical machine-readable About card schema and badge vocabulary
  beyond the UI stubs the initial in-product surface needs;
- live signature / attestation / SBOM / advisory verification (the
  provenance section seeds row scaffolding only);
- live service-health aggregation (the service-health section seeds row
  scaffolding only);
- hosted incident workspaces, advisory publication, or notification-center
  breadth beyond the Help / About surface.
