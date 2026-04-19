# Target-discovery, host-boundary, managed-workspace lifecycle, notebook-trust, structured-round-trip, and install-review taxonomy

This document freezes the vocabulary every runtime, ecosystem,
notebook, and managed-workspace surface uses when it names **which
target was discovered and how confidently**, **which host /
isolation boundary a projected fact crossed**, **which lifecycle
state a managed workspace occupies**, **which rung of the
notebook-trust ladder a notebook sits on**, **which structured
round-trip risk a rich-document preview carries**, and **which
slots an install / update review packet MUST fill**.

The six vocabularies are not independent dialects: they are the
companion labels execution, notebook, ecosystem, managed-runtime,
and install-review surfaces project against the already-frozen
execution-context record (ADR-0009), the capability-lifecycle /
dependency-marker record (ADR-0011), the extension-manifest /
effective-permission seed (ADR-0012), the identity-mode envelope
(ADR-0001), and the subscription envelope (ADR-0005). This
taxonomy does not redefine those contracts; it names the fields
they will refer to so later notebook, managed-workspace, and
install-review packets do not mint parallel vocabularies.

If this document disagrees with any of ADR-0001, ADR-0004,
ADR-0005, ADR-0006, ADR-0007, ADR-0008, ADR-0009, ADR-0010,
ADR-0011, or ADR-0012, the ADR wins and this document MUST be
updated in the same change.

## Artifacts this taxonomy points at

- [`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml)
  — machine-readable matrix that binds every frozen
  `managed_workspace_lifecycle_state` to its minimum-field set,
  its admissible transitions, its activation-budget summary shape,
  its required host-boundary cue set, and its conformance tests.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — boundary schema that re-exports `target_class`,
  `reachability_state`, and the scope / authority envelope every
  target-discovery packet quotes.
- [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — boundary schema that re-exports the five orthogonal axes
  (`lifecycle_state`, `support_class`, `release_channel`,
  `freshness_class`, `client_scope`) every install-review row
  quotes.

## Who reads this document

- **Execution authors** (target resolver, launch-review sheet,
  env-inspect / doctor explainers) projecting discovered-target
  rows without re-deriving truth.
- **Notebook-kernel authors** (local / remote kernel controllers,
  notebook-trust gate, cell-apply review, notebook export) naming
  a notebook's rung on the trust ladder and its structured
  round-trip posture.
- **Managed-workspace authors** (prebuild warmer, idle-suspend
  controller, snapshot / hibernation orchestrator,
  `aureline env inspect`, support-bundle exporter) naming which
  lifecycle state is active, which activation-budget slice was
  consumed, and which host-boundary cues rode across.
- **Ecosystem / install-review authors** (manifest registry,
  install / update review sheet, permission inspector,
  mirror / private-registry adapter, claim-manifest publisher)
  filling the frozen install-review summary slots so reviewers
  see the same fields across every surface.
- **Support, docs, and governance writers** who quote packets
  from the canonical owners above; support exports are
  downstream, they do not re-derive truth.

## Shared rules every vocabulary obeys

1. **One owner per packet.** Target-discovery, host-boundary,
   managed-workspace lifecycle, notebook-trust, structured
   round-trip, and install-review packets each have exactly one
   canonical owner (named below). Downstream surfaces quote the
   packet by id; they do not re-derive its payload.
2. **Typed denial over silent fallback.** A surface that cannot
   fill a required field denies with a typed reason and routes to
   a repair hook rather than fall back to a generic "ready" /
   "available" / "trusted" / "installed" chip. Silent widening is
   forbidden.
3. **Axis orthogonality is preserved.** The five ADR-0011 axes,
   the ADR-0009 scope / authority envelope, the ADR-0012
   declared-vs-effective diff, and the five taxonomies named here
   stay separately addressable even when a UI chip collapses them.
4. **Projection carries freshness.** Every projected row names an
   ADR-0011 `freshness_class`; a row whose canonical owner was
   not reached for re-verification renders non-`authoritative_live`.
5. **Raw material never crosses.** Raw environment bodies, raw
   command lines, raw secret bytes, raw policy-bundle bytes,
   raw notebook cell bodies beyond the tainted fence, and raw
   install artifact bytes never appear in any of these packets.
   Handles, digests, refs, and typed counts do.

## Vocabulary 1: target-discovery confidence

A target-discovery packet is the typed answer to "which
execution target did the resolver find, and how confidently?".
Every launch-capable surface (task, test, debug, terminal,
notebook-kernel, scaffolding, AI tool-call, doctor-repair,
import-probe, replay-probe) emits exactly one
`target_discovery_packet_record` per resolve. The packet
**quotes** the ADR-0009 `target_class` enum; it does not mint a
parallel target vocabulary.

**Canonical owner:** `execution_context_resolver`.

### `target_discovery_confidence_class` (frozen)

| Token                      | Definition                                                                                                                                                                | Example                                                                                       |
|----------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------|
| `canonical_declared`       | Resolver found a declarative capsule / devcontainer / lockfile / policy pin that names the target exactly. The canonical id is the declared id.                           | `.devcontainer/devcontainer.json` pinned to a digest-addressed image.                         |
| `canonical_materialised`   | Resolver did not find a declaration but the materialised instance (running container id, VM id, SSH host key) round-trips to a canonical id.                             | Attaching to a running `devcontainer` whose image is still the digest the workspace pins.     |
| `probed_consistent`        | Resolver probed the target and the probe results match the declared capability envelope and toolchain fingerprints.                                                       | Probe on an SSH remote confirms the architecture and toolchain digests the capsule pins.      |
| `probed_divergent`         | Resolver probed the target and one or more signals disagreed (toolchain digest drift, capability-envelope flag mismatch, route-dependency mismatch). Surface MUST flag.  | Container is up but the `python3` digest differs from the declared critical-toolchain digest. |
| `inferred_from_ambient`    | No declaration or probe was authoritative; resolver inferred from ambient signals (PATH, host locale, login shell snapshot).                                              | No capsule present; ambient `python3` on PATH inferred as the interpreter.                    |
| `unresolved_requires_user` | Resolver could not decide between two or more plausible targets without user input; surface MUST prompt rather than pick silently.                                        | Two `python` toolchains resolve to different versions; user must pick.                        |
| `resolver_unavailable`     | Resolver itself is unreachable, stale, or in emergency-disable. Surface MUST deny launch with `execution_context_rejected`.                                               | Resolver crashed mid-resolve; no target rendered.                                              |

**Rules (frozen):**

1. A target-discovery packet MUST carry exactly one confidence
   class. Silent blanks are forbidden.
2. `probed_divergent`, `inferred_from_ambient`, and
   `unresolved_requires_user` MUST also emit a non-empty
   `divergence_or_inference_reasons` list naming the typed
   causes (toolchain digest drift, capsule drift state,
   capability envelope mismatch, ambient PATH only, locale
   divergence, activator-produced degraded environment, route
   mismatch, etc.).
3. `canonical_declared` / `canonical_materialised` packets MUST
   NOT carry divergence reasons; their purpose is to assert the
   declared or running identity is authoritative.
4. Adding a confidence class is additive-minor and bumps the
   taxonomy's `target_discovery_schema_version`.

## Vocabulary 2: host-boundary cues

A host-boundary cue names **why a projected fact is not the
rendering surface's own fact**: it crossed a trust, isolation,
route, identity, or kernel boundary. Cues are orthogonal to
target class — a single target can present several cues at once.

**Canonical owner:** the authority that sits **inside** the
boundary (for example, the remote-agent attach service for
`remote_agent_boundary`; the notebook-kernel controller for
`notebook_kernel_boundary`). Cues are quoted, not re-derived, by
downstream surfaces.

### `host_boundary_cue_class` (frozen)

| Token                              | Definition                                                                                                                                                                    | Example                                                                                  |
|------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------|
| `local_host_boundary`              | Projected fact is from the local host kernel that also renders it. Not a remote; included for completeness so every packet carries exactly one cue.                            | Task launched on the developer's laptop.                                                 |
| `user_mode_sandbox_boundary`       | Fact came from a user-mode sandbox on the same kernel (seccomp, AppContainer, macOS Sandbox). Capability envelope narrows; filesystem identity may alias.                      | Scaffolding runner on a sandboxed helper.                                                |
| `container_kernel_boundary`        | Fact crossed a container kernel namespace (cgroup / net-ns / mnt-ns). Capability envelope follows the image manifest, not the host.                                            | Task invoked inside `container_local` target.                                            |
| `devcontainer_boundary`            | Fact crossed a declared devcontainer boundary. Mount identity layer follows the devcontainer; activator decisions follow the devcontainer lifecycle hooks.                    | Debug adapter running inside `.devcontainer/devcontainer.json`.                          |
| `remote_ssh_boundary`              | Fact was produced on an SSH remote. Route dependency, reachability, and clock-skew flags all apply.                                                                            | Test runner on `ssh://build-host-1`.                                                     |
| `remote_agent_boundary`            | Fact came from an Aureline remote-agent attach service. Client scope is `remote_agent`; remote-scope mismatch absorbs into a dependency marker.                                | AI tool call landed on the remote-agent attach.                                          |
| `managed_workspace_boundary`       | Fact came from a managed workspace (cloud-hosted Aureline runtime); `managed_only_channel` / `managed_admin_surface` axes may apply.                                           | Prebuild warming on a managed workspace instance.                                        |
| `notebook_kernel_boundary`         | Fact came from a notebook kernel (local or remote). Cell-authority rules and notebook-trust ladder apply in addition to base host rules.                                       | Cell output from a `notebook_kernel_remote` target.                                      |
| `ai_sandbox_boundary`              | Fact came from an AI sandbox / tool-call runtime. Tainted-context fences and AI evidence-packet rules apply.                                                                   | AI tool invoked a script inside the AI sandbox.                                          |
| `browser_handoff_return_boundary`  | Fact returned from a browser-handoff (ADR-0010 approval ticket). Content is `untrusted_external` by default and inherits the tainted fence.                                    | OAuth callback payload returned to the composer.                                         |
| `bridged_host_boundary`            | Fact came from a compatibility-bridged host (extension compatibility bridge, external helper). Bridge profile and compatibility notes apply.                                   | Legacy language-server helper running under a compatibility bridge.                      |

**Rules (frozen):**

1. Every projected fact MUST carry exactly one `host_boundary_cue_class`.
   A packet with no cue is non-conforming. A local-host fact uses
   `local_host_boundary`.
2. A packet MAY additionally carry a **cue stack** naming every
   intermediate boundary when the fact crossed more than one
   (e.g. `remote_ssh_boundary` over `managed_workspace_boundary`).
   The stack is ordered outermost-to-innermost.
3. A cue other than `local_host_boundary` MUST pair with the
   ADR-0005 authority envelope tag `projected_from_execution` (or
   `projected_from_provider_overlay` for provider-linked facts)
   and an ADR-0011 `freshness_class`.
4. Adding a cue class is additive-minor. Repurposing a cue
   (reusing an existing token for a different boundary) is
   breaking and requires a new decision row.

## Vocabulary 3: managed-workspace lifecycle states

A managed-workspace lifecycle packet names **which state a
managed workspace instance occupies**. Managed workspaces are
the union of prebuild warmers, cloud-hosted runtimes, and
long-lived dev environments whose runtime identity and
activation budget are first-class.

The full matrix — minimum fields, admissible transitions,
activation-budget summary shape, host-boundary cue requirements,
and conformance tests per state — lives in
[`/artifacts/runtime/managed_workspace_lifecycle.yaml`](../../artifacts/runtime/managed_workspace_lifecycle.yaml).
The frozen state vocabulary is repeated here so surface authors
can cite it inline.

### `managed_workspace_lifecycle_state` (frozen)

| Token                      | Definition                                                                                                                                                                | Example                                                                                     |
|----------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------|
| `undeclared`               | No managed-workspace instance is claimed for this workspace. Launches target a local or SSH remote.                                                                       | Fresh clone with no managed binding.                                                        |
| `provisioning`             | Control plane is allocating compute, volumes, and network for a new instance. No launch admissible yet.                                                                   | Just-pressed "Create managed workspace"; image is pulling.                                  |
| `warming`                  | Instance is up but capsule / prebuild warmers are still running. Reachability is `warming`.                                                                               | Devcontainer postCreate activators running.                                                 |
| `ready`                    | Instance is reachable, capsule is in-sync, activators applied, activation budget not yet exhausted. Normal launch target.                                                 | Open-editor on a freshly warmed managed workspace.                                          |
| `idle_suspended`           | Instance suspended after idle window; filesystem preserved, compute released. Resume requires a reactivation step visible to the user.                                    | Laptop closed for 2h; managed workspace auto-paused.                                        |
| `snapshot_paused`          | Instance paused and snapshotted by user or admin. Filesystem snapshotted; resume restores from the snapshot.                                                              | User paused the instance to take a break; snapshot pinned.                                  |
| `hibernated`               | Long-idle snapshot kept in cold storage; reactivation requires re-pull. Activation budget for reactivation counted separately from cold-start.                            | 14-day idle; hibernation policy engaged.                                                    |
| `recovering`               | Control plane detected a failure (volume drift, network partition, capsule drift) and is restoring. Reachability is `degraded`; launches deny.                            | Control plane restarting the instance after a node failure.                                 |
| `quarantined`              | Instance disabled by policy or kill-switch. Renders as a tombstone with a repair hook; launches deny with `kill_switch_tripped`.                                          | Emergency-disable bundle quarantined the instance.                                          |
| `retiring`                 | Instance is scheduled for retirement. New launches denied; running sessions allowed to drain within the drain window.                                                     | End-of-life window for a deprecated image; sessions draining.                               |
| `retired`                  | Instance removed. Surfaces that still reference it render a tombstone with a typed migration hint.                                                                        | Historic references on a review bundle that predates retirement.                            |

**Rules (frozen):**

1. A managed-workspace packet MUST name exactly one state.
2. State transitions are rule-bound (see the YAML matrix); silent
   state changes MUST emit the matching
   `managed_workspace_state_changed` audit event with the typed
   reason.
3. `quarantined` and `retired` are absorbing: a packet in either
   state MUST carry a repair hook (managed-access request,
   migration hint, ticket reference). Null repair hooks are
   forbidden.
4. Every state other than `undeclared` MUST pair with an
   `activation_budget_summary_record` (see §Example packet
   schemas below).

## Vocabulary 4: notebook-trust ladder (reserved slots)

A notebook-trust packet names **how far up the trust ladder a
notebook currently sits** and **which ladder rungs its cells,
outputs, metadata, and attachments obey**. The ladder exists so
mixed-trust notebooks (imported public notebook, ingested gist,
AI-generated cells, partially reviewed cells) stay reviewable
without collapsing into a single "trusted / untrusted" bit.

**Canonical owner:** `notebook_trust_gate` (reserved). Until the
notebook lane lands, extension authors, AI authors, and import
authors MUST NOT mint parallel trust bits; they reserve packet
fields under the rung vocabulary below.

### `notebook_trust_rung` (frozen)

| Token                                | Definition                                                                                                                                                                                            |
|--------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `untrusted_tainted`                  | Notebook or cell is untrusted and tainted (imported from untrusted source, AI-generated without review, pulled through a browser-handoff-return surface). Outputs render behind a tainted fence.      |
| `untrusted_quarantined_for_review`   | Notebook has been accepted into a review workspace for evaluation. Cells are not executed; outputs are not re-rendered; only the reviewer's preview renders.                                          |
| `structural_only_trusted`            | Notebook structure (cell graph, metadata, markdown) is trusted; code cells and outputs remain untrusted. Allows navigation, search, and outline rendering without executing code.                    |
| `selective_cell_trust`               | Individual cells are marked trusted; siblings are not. Cell-level authority rides the rung; the notebook-level rung aggregates using the dependency-marker downgrade rule.                            |
| `fully_trusted_user`                 | User opted in to trust the whole notebook (workspace-trusted and user-confirmed). Activators that require trust may fire; outputs may re-render live.                                                 |
| `fully_trusted_workspace_policy`     | Admin policy pins the notebook to trusted (for curated workspace notebooks). Trust survives re-open; still narrowed by capability-lifecycle / dependency-marker rules.                                |
| `trust_revoked_pending_review`       | Notebook was trusted but a revocation event (policy change, signature mismatch, provenance drift) demoted it. Outputs render with a revocation notice; user action required to revisit the rung.      |

**Reserved packet fields (slots):**

- `notebook_trust_rung` — one of the tokens above.
- `per_cell_trust_overrides` — list of `{cell_id, rung, reason}`;
  present when rung is `selective_cell_trust` or
  `trust_revoked_pending_review`.
- `trust_ladder_evidence_refs` — list of evidence refs pointing to
  signatures, provenance anchors, import-source records, or AI
  evidence packets backing the rung.
- `trust_ladder_disclosure_summary` — human-legible paragraph the
  review sheet / notebook header renders; a surface that hides
  this summary is non-conforming.
- `revocation_event_ref` — nullable; non-null on
  `trust_revoked_pending_review`.
- `irreversibility_flag` — boolean; true when applying the rung
  performs an action (kernel attach, filesystem write, outbound
  call) that cannot be silently reverted by closing the notebook.
- `disclosure_flag_set` — list of ADR-0007 redaction hints and
  ADR-0011 `freshness_class` markers the surface MUST render
  alongside the rung.

**Rules (frozen):**

1. A notebook surface that hides the trust rung, hides the
   per-cell overrides, or renders `fully_trusted_*` without an
   evidence ref is non-conforming.
2. The rung NEVER widens silently. A move up the ladder
   (e.g. `structural_only_trusted` → `fully_trusted_user`)
   requires a user or admin action and emits the
   `notebook_trust_rung_changed` audit event.
3. AI-authored or browser-handoff-return cells MUST enter at
   `untrusted_tainted`; an AI flow that writes cells at a higher
   rung is non-conforming.
4. Re-opening a notebook after a policy-epoch roll (ADR-0008)
   that invalidates a signed-trust pin MUST demote to
   `trust_revoked_pending_review` rather than silently inherit
   the prior rung.

## Vocabulary 5: structured round-trip risk (reserved slots)

Rich-document surfaces (notebooks, structured outputs, diagramic
cells, design exports, rich clipboard flows) preview changes
whose **round-trip through export / import / copy / AI-apply**
may lose structure, hide formatting, or silently rewrite data.
This vocabulary names the risk so previews cannot silently
repaint a lossy action as lossless.

**Canonical owner:** the renderer that owns the structured form
(notebook renderer, diagram renderer, design-export renderer).
Downstream surfaces quote the packet; they do not re-derive risk.

### `structured_round_trip_risk_class` (frozen)

| Token                             | Definition                                                                                                                                                                                                                                   |
|-----------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `lossless_roundtrip`              | Round-trip preserves every declared structural field and attachment. No chip required.                                                                                                                                                       |
| `lossy_metadata_only`             | Non-essential metadata (timestamps, author hints, display-only annotations) is dropped. Surface renders a metadata-loss note.                                                                                                                 |
| `lossy_output_representation`     | Output representations (rendered images, mime-alternate pairs, rich text alt-forms) are normalised. Cell inputs preserved; reviewer sees a representation-loss chip.                                                                          |
| `lossy_structural`                | Structural elements (cell order, grouping, attachments, linked ids) change. Surface MUST show a structural-loss chip and enumerate the affected fields before allowing commit.                                                                |
| `lossy_irreversible`              | Round-trip performs an irreversible action (external side effect, remote mutation, signature re-sign). Surface MUST require typed confirmation (ADR-0024 shell interaction-safety) and emit an irreversibility flag.                          |
| `round_trip_unavailable`          | Surface cannot determine round-trip risk (renderer unreachable, schema-version mismatch, unsupported attachment class). Must deny the lossy path rather than guess.                                                                           |
| `round_trip_policy_blocked`       | Admin policy blocks the round-trip (egress-narrowed export, managed-only content class, redaction-required attachment). Surface renders a typed policy-block chip rather than a generic error.                                                |

**Reserved packet fields (slots):**

- `structured_round_trip_risk_class` — one of the tokens above.
- `preview_representation_class` — one of
  `full_fidelity_preview`, `normalised_preview`,
  `summary_only_preview`, `tombstone_preview`; names what the
  preview surface is actually rendering.
- `irreversibility_flag` — boolean; true when commit triggers an
  action that cannot be reverted by closing the preview.
- `disclosure_flag_set` — list of typed disclosure flags
  (`representation_normalised`, `attachment_dropped`,
  `signature_resigned`, `external_side_effect`, `remote_mutation`,
  `redaction_applied`, `policy_narrowed`).
- `affected_fields` — ordered list of typed field paths the
  round-trip alters; present whenever the class is
  `lossy_metadata_only`, `lossy_output_representation`, or
  `lossy_structural`.
- `reversal_hint_ref` — nullable repair ref (undo path, reverse
  action, clipboard restore); null only when `irreversibility_flag`
  is true.

**Rules (frozen):**

1. A surface that applies a structured change MUST render the
   risk class BEFORE commit. Post-hoc disclosure is forbidden.
2. `lossy_irreversible` MUST require a typed confirmation that
   quotes the ADR-0024 permission prompt vocabulary.
3. A structured round-trip whose class is
   `round_trip_unavailable` or `round_trip_policy_blocked` MUST
   deny commit. Silent downgrade to a lossy class is forbidden.
4. The class interacts with the notebook-trust ladder: an
   `untrusted_tainted` or `untrusted_quarantined_for_review`
   notebook MUST NOT render a `lossless_roundtrip` class for any
   outbound export until the rung lifts.

## Vocabulary 6: install-review taxonomy

An install-review packet carries every typed field an install /
update review sheet, permission inspector, claim-manifest entry,
and support-export row must fill for an extension, bundle, or
managed-workspace install. It rides ADR-0012's manifest /
effective-permission / publisher-continuity / policy-pack seed
without redefining it.

**Canonical owner:** `install_review_packet_emitter` (bound to
the extension registry or bundle installer).

### Reserved install-review summary slots (frozen field set)

Every install-review packet MUST fill the following slots. A
review surface that hides a slot denies commit with the
ADR-0011 `review_disclosure_incomplete` denial reason; a support
export that drops a slot is non-conforming.

- `install_review_subject_class` — one of
  `extension_install`, `extension_update`,
  `extension_downgrade`, `bundle_install`, `bundle_update`,
  `mirror_adapter_install`, `managed_workspace_binding_install`,
  `emergency_disable_apply`, `publisher_continuity_migration`.
- `install_review_disposition_class` — one of
  `proposed`, `awaiting_user_confirmation`,
  `awaiting_admin_confirmation`, `approved`, `denied`,
  `deferred_pending_freshness`, `deferred_pending_provider_link`,
  `expired_superseded`.
- `manifest_row_ref` — ADR-0012 manifest row the install
  references.
- `effective_permission_summary_ref` — ADR-0012 effective-
  permission summary row (required in every non-trivial install).
- `declared_vs_effective_diff_visible` — boolean; MUST be true on
  every non-trivial review. False denies commit.
- `transitive_capability_inheritance_ref` — ADR-0012 closure ref;
  drill-down required per ADR-0011 projection rules.
- `publisher_continuity_row_ref` — ADR-0012 publisher row
  (including `lineage_state` and `trust_tier`).
- `mirror_and_private_registry_continuity_ref` — continuity row
  quoted; null only when `registry_source_class` is
  `vendored_local`.
- `policy_pack_narrowing_refs` — ordered list of applied
  ADR-0012 `policy_pack_constraint_row` refs.
- `capability_lifecycle_row_refs` — list of ADR-0011 lifecycle
  rows the subject binds (parent and closure members).
- `dependency_marker_refs` — ADR-0011 live dependency markers
  attached to the binding.
- `compatibility_label_packet_ref` — see §Example packet schemas.
- `activation_budget_summary_ref` — see §Example packet schemas.
- `host_boundary_cue_stack` — cue stack (Vocabulary 2).
- `target_discovery_packet_ref` — pointer to the most recent
  target-discovery packet for the install target (Vocabulary 1).
- `notebook_trust_rung` — present iff subject surfaces touch a
  notebook (Vocabulary 4); null otherwise.
- `structured_round_trip_risk_packet_ref` — present iff subject
  performs a structured round-trip (Vocabulary 5); null
  otherwise.
- `irreversibility_flag_set` — ordered list of typed flags
  (`signature_resign`, `mirror_snapshot_pinned`,
  `publisher_migration_committed`, `emergency_disable_committed`,
  `managed_binding_pinned`, `workspace_trust_escalated`).
- `disclosure_flag_set` — typed disclosure list
  (`declared_effective_diff`, `policy_narrowed`,
  `provider_link_required`, `freshness_floor_unmet`,
  `managed_only_channel_required`, `kill_switch_tripped`,
  `publisher_quarantined`, `structured_round_trip_risk_present`,
  `notebook_trust_rung_below_fully_trusted`).
- `redaction_class` — one of `metadata_and_hashes_only`,
  `support_bundle_scoped`, `broadened_capture`; applied on
  export per ADR-0007.
- `review_event_refs` — ordered list of audit events (see below).

### Install-review audit events (frozen)

The reserved audit-event ids emitted on the `install_review`
stream (a companion stream to ADR-0012's `extension_trust`):

- `install_review_packet_opened`
- `install_review_packet_confirmed`
- `install_review_packet_denied`
- `install_review_packet_deferred_pending_freshness`
- `install_review_packet_deferred_pending_provider_link`
- `install_review_packet_superseded`
- `install_review_disclosure_incomplete`
- `install_review_disclosure_completed`
- `install_review_irreversibility_confirmed`
- `install_review_schema_version_bumped`

**Rules (frozen):**

1. A review packet MUST carry every slot named above; silent
   omission is non-conforming. Null is only admissible where the
   slot definition above allows it.
2. A review surface MUST render `irreversibility_flag_set` and
   `disclosure_flag_set` inline. Tooltips alone are
   non-conforming.
3. A review that changes `install_review_disposition_class`
   emits the matching audit event; silent state changes are
   non-conforming.
4. Adding a subject class, disposition class, flag, or audit
   event id is additive-minor and bumps the install-review
   schema version.

## Example packet schemas

The two example packets below show how the compatibility-label
vocabulary and the activation-budget summary bind into the
surrounding taxonomy. Both are companion shapes under the
install-review and managed-workspace-lifecycle packets; they are
not new record families.

### `compatibility_label_packet_record` (example)

The compatibility label is how every install-review and
managed-workspace binding declares the envelope a consumer will
run in. Fields (frozen shape; the eventual ecosystem crate's
Rust types are the schema of record):

```yaml
compatibility_label_packet_record:
  schema_version: 1
  packet_id: "<opaque>"
  subject_ref: "<extension_identity | bundle_id | managed_binding_id>"
  declared_identity_mode_set:
    - account_free_local
    - self_hosted_org
    - managed_convenience   # re-exports ADR-0001
  declared_target_class_set:
    - local_host
    - devcontainer
    - managed_workspace     # re-exports ADR-0009
  declared_host_contract_family: wasm_component_model  # ADR-0012
  declared_artifact_transport_family: wasm_signed_artifact
  declared_platform_arch_set: ["linux-x86_64", "darwin-arm64"]
  declared_toolchain_bindings:
    - toolchain_class: interpreter
      toolchain_id: python.cpython
      version_range: ">=3.11,<3.13"
  declared_capability_binding_refs: ["cap:lifecycle:row:editor.lsp"]
  declared_freshness_floor_class: warm_cached
  declared_client_scope_set: [desktop_product, cli]
  compatibility_bridge_notes: null
  redaction_class: metadata_and_hashes_only
  freshness_class: authoritative_live
```

Rules: every field is a **declared** value; the effective
compatibility is computed by intersecting this packet with the
host's actual target-discovery packet (Vocabulary 1), the active
identity mode (ADR-0001), the capability-lifecycle rows
(ADR-0011), and the policy-pack narrowings (ADR-0012). An
install-review surface MUST render the intersection, not the
declared set alone.

### `activation_budget_summary_record` (example)

The activation-budget summary names how much of a managed
workspace's budgeted activation time, cold-start quota, prebuild
warming quota, and reactivation quota has been consumed. Fields:

```yaml
activation_budget_summary_record:
  schema_version: 1
  packet_id: "<opaque>"
  managed_workspace_instance_ref: "<opaque>"
  managed_workspace_lifecycle_state: ready
  budget_window:
    window_kind: rolling_seven_day   # or billing_cycle, instance_lifetime
    window_start: "2026-04-12T00:00:00Z"
    window_end:   "2026-04-19T00:00:00Z"
  budget_slices:
    cold_start_seconds:
      budgeted: 600
      consumed: 120
      remaining: 480
    warming_seconds:
      budgeted: 1800
      consumed: 900
      remaining: 900
    ready_seconds:
      budgeted: 28800
      consumed: 14400
      remaining: 14400
    idle_suspend_events:
      budgeted: 40
      consumed: 12
      remaining: 28
    hibernation_reactivation_events:
      budgeted: 4
      consumed: 1
      remaining: 3
  degradation_markers:
    - reason: warming_slice_exceeded
      since: "2026-04-18T11:22:00Z"
  host_boundary_cue: managed_workspace_boundary
  freshness_class: warm_cached
  redaction_class: metadata_and_hashes_only
```

Rules: every live managed-workspace packet MUST carry this
summary (see Vocabulary 3). A budget slice whose `consumed`
crosses `budgeted` MUST emit a `degradation_marker` with a typed
reason; silently continuing is non-conforming. When the instance
enters `quarantined`, `retiring`, or `retired`, the budget
freezes at its last-observed values.

## Cross-surface coverage

This taxonomy spans five required contexts:

- **Local execution**: Vocabulary 1 + Vocabulary 2
  (`local_host_boundary`); Vocabulary 6 on ambient installs.
- **Remote execution**: Vocabulary 1 + Vocabulary 2
  (`remote_ssh_boundary` or `remote_agent_boundary`); Vocabulary 6
  on remote-agent bundles.
- **Managed workspaces**: Vocabulary 3 + Vocabulary 2
  (`managed_workspace_boundary`); activation-budget summary
  required; install-review packet (Vocabulary 6) on managed
  binding installs.
- **Notebooks**: Vocabulary 2 (`notebook_kernel_boundary`) +
  Vocabulary 4 (`notebook_trust_rung`) + Vocabulary 5
  (structured round-trip). Mixed-trust notebooks MUST carry
  per-cell overrides.
- **Extension / bundle installs**: Vocabulary 6 (install-review)
  + Vocabulary 2 (cue stack for the install target) +
  Vocabulary 1 (target discovery) + ADR-0012's manifest and
  effective-permission rows.

## Where related decisions live

- Execution-context record and scope vocabulary:
  [`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md).
- Capability lifecycle and dependency markers (five axes,
  downgrade rule, denial posture):
  [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md).
- Extension manifest, effective permission, publisher continuity,
  and policy-pack seed (install-review base vocabulary):
  [`docs/adr/0012-extension-manifest-permission-publisher-policy.md`](../adr/0012-extension-manifest-permission-publisher-policy.md).
- Identity modes and workspace trust:
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- Subscription envelope and authority class matrix:
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Secret broker and redaction defaults:
  [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md).
- Settings resolver and policy-epoch semantics:
  [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md).
- Connected-provider browser-handoff and approval-ticket
  vocabulary:
  [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
- Execution-context cross-surface vocabulary companion:
  [`docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md).
- Shell interaction-safety contract (typed confirmation and
  irreversibility semantics this taxonomy relies on):
  [`docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md).

## Future lanes that consume this vocabulary

These lanes will read this taxonomy mechanically rather than
mint parallel labels:

- Execution resolver and launch-review UI (target-discovery and
  host-boundary cues).
- Notebook kernel controller, notebook-trust gate, cell-apply
  review, and notebook export (notebook-trust ladder,
  structured round-trip risk).
- Managed-workspace control plane, prebuild warmer, idle-suspend
  controller, and activation-budget reporter (managed-workspace
  lifecycle, activation-budget summary).
- Extension registry, install / update review sheet, permission
  inspector, claim-manifest publisher, and mirror adapter
  (install-review summary slots, compatibility-label packet).
- AI tool-call plane, browser-handoff-return surface, and
  AI-apply review (boundary cues and tainted-fence interplay
  with notebook trust and structured round-trip risk).

## Change management

- Adding a token to any of the six vocabularies, adding a
  reserved slot, adding a compatibility-label declared-set member,
  or adding an activation-budget slice is additive-minor:
  bump the taxonomy's schema version, extend the YAML matrix,
  and extend the relevant boundary schema.
- Repurposing any existing token (reusing a
  `managed_workspace_lifecycle_state` for a different
  transition, reusing a `notebook_trust_rung` for a different
  rung, collapsing two `host_boundary_cue_class` values into
  one) is breaking and requires a new decision row.
- Renaming a packet id, a subject class, a lifecycle state, or a
  trust rung is done through an alias mechanism (mirrors the
  ADR-0008 alias discipline); never by mutating the canonical id.
