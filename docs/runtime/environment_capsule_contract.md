# Environment-capsule, environment-diff, and toolchain-identity contract

This document freezes the shared runtime contract Aureline uses to
answer **"what was the environment this execution ran against?"**
and **"what changed in that environment between two executions?"**
with one inspectable truth object instead of ad hoc environment
dumps. It exists so terminal, task, test, debug, AI/apply,
refactor-preview, and hosted-review replay surfaces cannot each
invent their own environment record, their own diff shape, or their
own redaction posture for secret-bearing or policy-bearing fields.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document MUST be updated in the same change.

## Companion artifacts

- [`/schemas/runtime/environment_capsule.schema.json`](../../schemas/runtime/environment_capsule.schema.json)
  — boundary schema for the `environment_capsule_record`, the
  `environment_capsule_export_manifest_record`, and the
  `environment_capsule_audit_event_record` every capsule-emitting
  surface, capsule consumer, and support / evidence / replay
  exporter reads.
- [`/schemas/runtime/environment_capsule_alpha.schema.json`](../../schemas/runtime/environment_capsule_alpha.schema.json)
  and [`/artifacts/templates/workspace_template_seed.yaml`](../../artifacts/templates/workspace_template_seed.yaml)
  — alpha seed schema and first workspace-template manifest for
  launch-bundle capsule refs. These artifacts feed Start Center and
  validation tooling without redefining the full capsule body.
- [`/schemas/runtime/environment_diff_packet.schema.json`](../../schemas/runtime/environment_diff_packet.schema.json)
  — boundary schema for the `environment_diff_record` every rerun,
  reattach, debug-attach, apply, refactor-preview, and hosted-
  review replay flow emits when it needs to say "here is what
  changed since the last capsule".
- [`/fixtures/runtime/environment_cases/`](../../fixtures/runtime/environment_cases/)
  — concrete capsule and diff fixtures covering terminal, run,
  test, debug, refactor-preview, and hosted-review replay surfaces.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — execution-context record the capsule is referenced from via
  `environment_capsule_ref`. The capsule body quoted here is what
  the `environment_capsule_ref` resolves against; the two schemas
  share target-identity, toolchain-identity,
  compatibility-fingerprint, drift-state, authority-envelope, and
  redaction-class vocabularies.
- [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — cross-surface execution-context vocabulary the capsule quotes
  by field name.
- [`/docs/execution/context_inspector_packet.md`](../execution/context_inspector_packet.md)
  — execution-context inspector, snapshot, and provenance-diff
  packet. Snapshots embed `environment_capsule_ref` by reference;
  diff entries on `environment_capsule_ref` resolve their detail
  against the `environment_diff_record` defined here.
- [`/docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md)
  — filesystem-identity record (five layers) every capsule uses
  for target mounts, executable identity, and wrapper provenance.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — secret-class vocabulary, credential-alias handle discipline,
  and redaction defaults capsule secret-projection markers project.
- [`/docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  — authoritative ADR for execution-context, workset / scope, and
  the `environment_capsule_ref` reference shape. The ADR freezes
  the reference; this contract freezes the full body that
  reference resolves against.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support-bundle contract. Support exports consume the capsule's
  declared `support_bundle` export path rather than re-minting
  toolchain-identity or target-identity shapes.
- [`/docs/runtime/target_discovery_and_install_review_taxonomy.md`](./target_discovery_and_install_review_taxonomy.md)
  — build-target discovery and install-review taxonomy. Build-
  target discovery artefacts consume the capsule's
  `toolchain_identity` field set verbatim.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — diagnostics, supportability, and export-redaction posture;
  "why this environment?" and "what changed in the environment?"
  treated as in-product contracts rather than afterthoughts.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — execution-context provenance, environment-capsule
  content-addressing, authority-class matrix, subscription
  envelope, and redaction-default definitions.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — execution-context resolver, target identity, toolchain
  identity, environment-capsule reference, activator decision, and
  override-delta shapes.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — "why this toolchain?" / "why this environment?" inspector
  surface copy rules the capsule's declared export paths mirror.
- `.t2/docs/Aureline_Milestones_Document.md`
  — shared environment-capsule and environment-diff posture named
  as a release-blocking contract during the foundations phase.

If this document disagrees with those sources, those sources win
and this contract plus the companion schemas update in the same
change.

## Scope

Frozen at this revision:

- one `environment_capsule_record` shape shared by terminal, task,
  test, debug, AI/apply, refactor-preview, and hosted-review
  replay surfaces so environment truth does not fork by execution
  surface;
- one `environment_diff_record` shape shared by rerun, reattach,
  debug-attach, apply, refactor-preview, hosted-review replay,
  import-probe, and `aureline doctor --explain` flows so a
  reviewer comparing two capsules always reads the same
  per-layer status record;
- a closed `capsule_location_class` vocabulary
  (`workspace_local`, `prebuild_snapshot`, `devcontainer_image`,
  `container_local_ephemeral`, `remote_ssh_host`,
  `remote_workspace_vm`, `managed_workspace_control_plane`,
  `notebook_kernel_local`, `notebook_kernel_remote`,
  `ai_sandbox_ephemeral`, `hosted_review_replay`) that projects
  onto the existing target-class vocabulary without renaming it;
- a closed `capsule_origin_class` vocabulary
  (`workspace_template_hydration`, `prebuild_snapshot_reuse`,
  `container_image_bake`, `declarative_flake_or_shell`,
  `workspace_file_defaults`, `ad_hoc_per_run_override`,
  `hosted_review_replay_capture`, `imported_bundle_hydration`,
  `policy_injected_overlay`) so "why this environment?" answers
  always have a typed answer;
- a closed `declarative_input_kind` vocabulary (fourteen entries)
  naming the declarative inputs a capsule may cite by reference
  and content digest;
- a closed `secret_projection_mode` vocabulary (`alias_only`,
  `alias_plus_class_label`, `redacted_placeholder`,
  `not_projected`, `broadened_capture`) for per-secret-class
  projection markers;
- a closed `capsule_policy_marker` vocabulary covering trust gate,
  sandbox profile, egress posture, activator gate, secret-
  projection rule, export-redaction rule, and capsule-admission
  rule citations;
- a closed `export_sink_class` vocabulary (`support_bundle`,
  `evidence_packet`, `replay_capture`, `mutation_journal_entry`,
  `profile_export`, `ai_tool_call_seed`, `hosted_review_replay`,
  `crash_envelope`, `optional_sync_payload`,
  `project_doctor_finding`, `context_snapshot_export`) with typed
  per-field admissions the capsule declares before a sink emits;
- per-capsule `consuming_surfaces` bindings to the twelve surface
  classes the contract names (terminal, task, test, debug,
  notebook_kernel, scaffolding, ai_tool_call, refactor_preview,
  hosted_review_replay, doctor_repair, import_probe, replay_probe);
- a closed `diff_context` vocabulary (`rerun`, `reattach`,
  `debug_attach`, `apply`, `refactor_preview`,
  `hosted_review_replay`, `import_probe`, `doctor_explain`) and a
  seventeen-value `diff_layer` vocabulary covering every top-level
  field of the capsule body;
- a closed `redaction_limited_reason` vocabulary so a diff entry
  that cannot be explained without broadened capture names exactly
  why it is limited;
- a closed `secret_projection_delta_kind` vocabulary
  (nine entries) and a closed `declarative_input_delta_kind`
  vocabulary (six entries) so diff consumers render typed deltas
  rather than improvised prose;
- an `environment_capsule_audit_event` vocabulary
  (`environment_capsule_resolved`, `environment_capsule_reused`,
  `environment_capsule_rejected_drift`,
  `environment_capsule_degraded`, `environment_capsule_exported`,
  `environment_capsule_export_broadened`,
  `environment_capsule_secret_projection_denied`,
  `environment_capsule_schema_version_bumped`) so every observable
  capsule-lifecycle transition emits one of a frozen set.

Out of scope at this revision:

- the concrete capsule-resolver Rust crate, capsule-on-disk
  layout, capsule cache eviction tuning, or content-addressing
  store. This contract is the shape those implementations compose
  against.
- admin-policy authorship for secret-projection rules,
  export-redaction rules, or capsule-admission rules. Those flow
  in from the admin policy bundle at runtime; this contract names
  the references rather than redefining them.
- provider-specific credential binding or OAuth flows. The ADR-0007
  credential-broker contract owns those; this contract projects
  their secret-class labels and credential-alias counts only.
- the final terminal host, task runner, test runner, debug adapter,
  AI/apply plane, refactor-preview workbench, or hosted-review
  replay viewer UX microcopy. Copy resolves against the closed
  vocabularies named here.

## Shared vocabulary

Every capsule-emitting surface, capsule consumer, and support /
evidence / replay / mutation-journal / crash exporter reads and
emits the fields below using the exact names the schema exports.
Aliases, cute labels, or private renames on protected surfaces are
forbidden.

### Capsule identity

- **`capsule_id`** — dotted lower-snake id allocated by the capsule
  author (for example, `caps.aureline.devcontainer.v3`).
- **`capsule_hash`** — content-addressed digest of the canonical
  capsule body bytes. MUST equal
  `compatibility_fingerprint.capsule_hash` on the same record.
- **`resolved_schema_version`** — capsule-envelope schema version
  at resolve time. Kept as a string for forward compatibility with
  capsule-author-declared schemas that diverge from the Aureline
  runtime envelope version.
- **`captured_at`** — monotonic ISO-8601 UTC timestamp.
- **`workspace_id`**, **`profile_id`** — workspace and profile
  identifiers copied from the workspace authority.

### Target identity

Re-exported verbatim from the execution-context schema. The
capsule embeds the target identity record so support bundles,
replay artefacts, and build-target discovery artefacts read one
target shape:

- `target_class`, `canonical_target_id`, `requested_target_ref`,
  `materialised_instance_ref`, `mount_identity` (ADR-0006
  five-layer filesystem-identity record), `route_dependency`,
  `reachability_state`, `capability_envelope_ref`.

### Toolchain / runtime identity

Re-exported verbatim from the execution-context schema. This field
set is the single toolchain-identity shape the contract promises to
share across execution-context, support-bundle, build-target
discovery, and capsule consumers:

- `toolchain_class`, `toolchain_id`, `resolved_version`,
  `executable_identity`, `activation_strategy`,
  `wrapper_provenance`, `extension_pack_refs`,
  `known_unsupported_gaps`, `degraded_fallback_flag`.

Silent wrapper insertion is forbidden; every wrapper is recorded
with its kind, filesystem identity, and version.

### Capsule location class

Eleven classes are frozen:

- **`workspace_local`** — capsule materialised inside the local
  workspace (ambient path, venv, direnv, rustup override). Target
  class is typically `local_host`.
- **`prebuild_snapshot`** — capsule warmed from a prebuild
  snapshot. Target class is typically `local_host`,
  `devcontainer`, or `prebuild_runtime`.
- **`devcontainer_image`** — capsule baked into an OCI image /
  devcontainer build.
- **`container_local_ephemeral`** — capsule materialised into a
  one-shot local container not backed by a prebuild.
- **`remote_ssh_host`** — capsule materialised on an SSH remote.
- **`remote_workspace_vm`** — capsule materialised on a remote
  workspace VM (not an SSH remote).
- **`managed_workspace_control_plane`** — capsule materialised by
  a managed-workspace control plane the product does not host.
- **`notebook_kernel_local`** — capsule materialised for a local
  notebook kernel process.
- **`notebook_kernel_remote`** — capsule materialised for a
  remote notebook kernel.
- **`ai_sandbox_ephemeral`** — capsule materialised for an AI /
  apply sandbox bounded for the lifetime of one tool call.
- **`hosted_review_replay`** — capsule captured by a hosted review
  replay. The capsule is frozen to the replay's captured inputs and
  MUST NOT mutate at read time.

`capsule_location_class` MUST be consistent with the target class:
for example, `managed_workspace_control_plane` pairs with the
`managed_workspace` target class; `ai_sandbox_ephemeral` pairs with
the `ai_sandbox` target class; `hosted_review_replay` pairs with
any target class that the replay captured, and the capsule carries
`drift_state: manually_diverged` if the replay was taken from a
post-capture edit.

### Capsule origin class

Nine origins are frozen. Every capsule names exactly one:

- **`workspace_template_hydration`** — a workspace template
  expanded the capsule. `workspace_template_ref` MUST be
  non-null.
- **`prebuild_snapshot_reuse`** — a prebuild snapshot warmed the
  capsule. `prebuild_snapshot_ref` MUST be non-null.
- **`container_image_bake`** — the capsule came from a container
  image (devcontainer build, OCI image ref, etc.).
- **`declarative_flake_or_shell`** — a Nix flake or shell declared
  the capsule.
- **`workspace_file_defaults`** — project files
  (`rust-toolchain.toml`, `requirements.txt`, `.envrc`,
  `.nvmrc`, JDK version file, etc.) drove capsule resolution.
- **`ad_hoc_per_run_override`** — a per-run override (launch
  profile, CLI flag, session override) drove capsule resolution
  without a durable declarative input.
- **`hosted_review_replay_capture`** — the capsule is the capture
  taken by a hosted review replay. `declarative_inputs` MUST name
  the `review_replay_capsule_ref`.
- **`imported_bundle_hydration`** — an imported capsule bundle
  hydrated the capsule.
- **`policy_injected_overlay`** — an admin policy overlay
  contributed the capsule (for example, a managed capsule bound to
  a tenant).

### Declarative inputs

Every capsule lists the declarative inputs it was assembled from.
Each input is cited by reference and content digest; raw file
bodies never cross this boundary. The fourteen admissible input
kinds are: `workspace_template_ref`, `prebuild_snapshot_ref`,
`devcontainer_config_ref`, `nix_flake_ref`, `nix_shell_ref`,
`rust_toolchain_file_ref`, `python_requirements_ref`,
`node_lockfile_ref`, `jdk_version_file_ref`, `direnv_envrc_ref`,
`oci_image_ref`, `review_replay_capsule_ref`,
`policy_overlay_ref`, and `ad_hoc_session_override_ref`. An input
kind this contract does not name is an additive-minor extension to
the schema, not a surface-local freeform string.

### Drift and compatibility

- **`drift_state`** — ADR-0006 drift-state vocabulary
  (`in_sync`, `stale_inputs`, `generator_changed`,
  `manually_diverged`, `unknown_lineage`). A capsule whose drift
  state is `generator_changed` or `manually_diverged` MAY be
  reused only when the consuming surface has explicitly opted in
  through a recorded decision.
- **`compatibility_fingerprint`** — commit / tree identity,
  capsule hash, platform / arch, policy epoch, extension lock
  digest, and critical toolchain digests. A mismatch downgrades
  the execution-context cache disposition to `rejected_drift` and
  does not silently reuse the capsule.

### Trust, identity, and policy epoch

`trust_state`, `identity_mode`, and `policy_epoch` are projected
from the workspace authority. The capsule does not mint these
values; it records the values in force when the capsule resolved.
A capsule marked `restricted` MUST NOT declare an export path with
`redaction_class: none`.

### Secret projection markers

A capsule lists one marker per secret class the capsule considered
at resolve time. Each marker carries:

- **`secret_class`** — ADR-0007 secret-class vocabulary
  (`ai_provider_token`, `code_host_token`,
  `package_registry_token`, `database_credential`,
  `ssh_key_material`, `client_certificate`,
  `signing_key_material`, `provider_session`, `device_secret`,
  `ephemeral_operation_token`).
- **`projection_mode`** — one of `alias_only`,
  `alias_plus_class_label`, `redacted_placeholder`,
  `not_projected`, or `broadened_capture`. Raw values enter the
  capsule only when `projection_mode` is `broadened_capture` and
  an `broadened_capture_approval_ticket_ref` is recorded alongside
  the marker.
- **`credential_alias_count`** — number of credential-alias
  handles bound for this secret class. Raw secret values are never
  inlined.
- **`alias_policy_ref`** — opaque ref into the credential-broker
  policy rule that minted these aliases (null when the secret is
  not projected).
- **`raw_value_present`** — always false unless the enclosing
  export manifest declares `redaction_class: broadened_capture`
  with a recorded approval ticket.
- **`injected_delta_class`** — env-var delta class label this
  secret projection produced, when applicable (re-export of the
  execution-context `env_var_delta_class` vocabulary; raw env
  names and values are not inlined).

A capsule that claims `projection_mode: not_projected` documents
that the secret class was considered and deliberately left unbound;
it MUST NOT claim `not_projected` when a credential alias was
actually bound.

### Policy markers

A capsule lists one marker per policy kind cited:

- **`policy_kind`** — one of `trust_gate`, `sandbox_profile`,
  `egress_posture`, `activator_gate`, `secret_projection_rule`,
  `export_redaction_rule`, `capsule_admission_rule`.
- **`policy_ref`** — opaque policy-rule reference.
- **`decision`** — `applied`, `blocked`, `narrowed`, or
  `broadened_with_approval`.
- **`approval_ticket_ref`** — ADR-0007 approval ticket ref when
  `decision` is `broadened_with_approval`; null otherwise.

Empty policy markers are admissible only for capsules whose
`trust_state` is `trusted`, `identity_mode` is
`account_free_local`, and `declarative_inputs` carry no
`policy_overlay_ref`.

### Declared export paths

Every capsule MUST declare at least one export path. An export
path names the sink class, the redaction class, and a per-field
admission table the sink uses at emission time. An export path is
a declarative seed, not a sink implementation: a sink resolves
field admissions against its own redaction defaults at emission
time and MUST refuse a request that would widen admission beyond
what the capsule declared. Eleven sink classes are admissible:
`support_bundle`, `evidence_packet`, `replay_capture`,
`mutation_journal_entry`, `profile_export`, `ai_tool_call_seed`,
`hosted_review_replay`, `crash_envelope`, `optional_sync_payload`,
`project_doctor_finding`, `context_snapshot_export`.

Per-field admission takes one of six values:

- **`emitted_as_is`** — admissible only for fields whose value is
  already a class label, opaque id, hash, count, or
  ADR-0006 filesystem-identity record under the sink's default
  redaction class.
- **`emitted_as_hash`** — admissible when the sink should receive
  a digest rather than the underlying value.
- **`emitted_as_class_label`** — admissible when the sink should
  receive only the class label (for example, the secret class
  rather than the credential alias count).
- **`emitted_as_alias_only`** — admissible for secret projections;
  the sink receives the credential alias handle and no class badge
  or count.
- **`redacted_placeholder`** — the sink receives a typed
  placeholder (for example, "a provider session is present but not
  projected").
- **`withheld`** — the field is not admitted at all.

`emitted_as_is` is forbidden on any field whose value would
include raw env bodies, raw command lines, raw secret bytes, or
raw absolute paths outside the ADR-0006 filesystem-identity record.

### Consuming surfaces

Every capsule MUST bind to at least one surface class from the
frozen list:

- **`terminal`** — terminal session seed.
- **`task`** — task launch.
- **`test`** — test runner launch.
- **`debug`** — debug attach / launch seed.
- **`notebook_kernel`** — notebook kernel seed.
- **`scaffolding`** — scaffolding engine.
- **`ai_tool_call`** — AI tool-call sandbox.
- **`refactor_preview`** — refactor-preview workbench.
- **`hosted_review_replay`** — hosted review replay viewer.
- **`doctor_repair`** — Project Doctor repair probe.
- **`import_probe`** — import probe.
- **`replay_probe`** — replay probe.

A capsule with no declared consuming surface is non-conforming.

### Authority envelope and degraded fields

Re-exported from the execution-context schema. Every rendered
field SHOULD carry an authority-envelope tag; every unresolved
field MUST emit a `capsule_degraded_field_record` with a typed
reason from the closed vocabulary (the thirteen execution-context
reasons plus the four capsule-specific reasons
`declarative_input_missing`,
`declarative_input_content_digest_mismatch`,
`secret_projection_denied`, and `export_path_unavailable`).

## Rules surfaces follow

1. **One environment-capsule record shape.** Every execution
   surface reads from the capsule record. No surface mints a
   private environment descriptor; no surface invents a private
   capsule shape.
2. **Content-addressed capsule hash.** `capsule_hash` is the
   digest of the canonical capsule body. A capsule record whose
   `capsule_hash` and
   `compatibility_fingerprint.capsule_hash` disagree is
   non-conforming.
3. **No raw env bodies, no raw command lines, no raw secret
   values.** Every field a surface renders is a class label, an
   opaque id, a hash, a count, or an ADR-0006 filesystem-identity
   record. Broadened capture requires a recorded approval ticket
   and is opt-in per export path.
4. **Secret-bearing and policy-bearing entries are typed.** Every
   secret-class projection emits a marker with a frozen
   `projection_mode`; every policy citation emits a marker with a
   frozen `policy_kind` and `decision`. Surfaces never claim a
   secret or policy is absent by omission; they record
   `not_projected` or the absence of a policy marker explicitly.
5. **Export paths are declared, not invented.** Every capsule
   declares at least one export path. A sink that would emit the
   capsule without a matching declared export path is
   non-conforming. A sink MUST NOT widen field admissions beyond
   what the declared path authorised.
6. **Fail closed.** Target-identity mismatches, toolchain-identity
   mismatches, declarative-input content-digest mismatches, and
   unmet capability dependencies deny or visibly downgrade; silent
   best-effort reuse is forbidden.
7. **Honest degradation.** Every unresolved, stale, or partially
   resolved field emits a degraded-field record the consuming
   surface renders. A projecting surface MUST NOT re-render a
   degraded field as authoritative.
8. **One diff shape.** Every rerun, reattach, debug-attach,
   apply, refactor-preview, hosted-review replay, import-probe,
   and doctor-explain flow emits one `environment_diff_record`.
   Per-feature diff formats are forbidden.
9. **Diff entries never leak raw bodies.** A diff entry that
   cannot be explained without broadened capture records
   `status: redaction_limited` and a typed
   `redaction_limited_reason`. `redaction_limited` MUST NOT hide
   a visible token change; it is admissible only when the
   difference is in a field the capsule does not expand at the
   default redaction class.
10. **Audit every observable transition.** Every resolver action
    a user, administrator, support engineer, or governance
    reviewer could ask about emits one of the frozen
    capsule-audit events. Audit events never carry raw env /
    command / secret material.

## Re-exported field sets

The following field sets are re-exported verbatim from
`schemas/runtime/execution_context.schema.json` and are the
single cross-surface truth. Execution-context, support-bundle,
build-target discovery, capsule, and diff consumers all read the
same shape:

- `filesystem_identity_record` (ADR-0006 five-layer identity).
- `target_identity`, `target_class`, `reachability_state`,
  `route_dependency_class`, `capability_envelope_ref`.
- `toolchain_identity`, `toolchain_class`, `activation_strategy`,
  `wrapper_kind`, `wrapper_provenance_entry`,
  `unsupported_gap_record`.
- `capsule_drift_state`, `compatibility_fingerprint`.
- `authority_envelope_tag`, `authority_envelope_entry`.
- `secret_class` (ADR-0007 secret-class vocabulary).
- `env_var_delta_class` (for `injected_delta_class`).
- `redaction_class` (ADR-0007 redaction-class vocabulary).

Silent renaming, forking, or private widening of any of these
field sets is non-conforming.

## Diff semantics

An `environment_diff_record` compares two capsule bodies layer by
layer. The layers are frozen: `capsule_identity`,
`target_identity`, `toolchain_identity`, `capsule_location_class`,
`capsule_origin_class`, `declarative_inputs`, `drift_state`,
`compatibility_fingerprint`, `trust_state`, `identity_mode`,
`policy_epoch`, `policy_markers`, `secret_projection_markers`,
`declared_export_paths`, `consuming_surfaces`,
`authority_envelope`, and `degraded_fields`.

Per-layer status is one of: `preserved`, `changed`, `added_on_b`,
`removed_on_b`, `degraded_on_a`, `degraded_on_b`, `unknown_on_a`,
`unknown_on_b`, or `redaction_limited`. Layers that carry
structured deltas (`secret_projection_markers` and
`declarative_inputs`) emit typed delta entries alongside the
layer status so surfaces render typed deltas rather than
improvised prose:

- `secret_projection_delta_kind` covers nine cases including
  `projection_mode_changed`, `credential_alias_count_changed`,
  and `broadened_capture_opened_on_b`. Raw secret values never
  appear; only class labels, counts, and mode changes project.
- `declarative_input_delta_kind` covers six cases including
  `content_digest_changed` and `origin_filesystem_identity_changed`.
  Raw file bodies never appear; only kind / ref / digest changes
  project.

Eight diff contexts are admissible: `rerun`, `reattach`,
`debug_attach`, `apply`, `refactor_preview`,
`hosted_review_replay`, `import_probe`, and `doctor_explain`.
Every diff names exactly one context.

## Redaction posture

- Default redaction class on every export path is
  `metadata_and_hashes_only`. A capsule may declare an
  `optional_sync_payload` path whose redaction class is
  `metadata_and_hashes_only` only; `broadened_capture` on sync is
  refused at the schema.
- `broadened_capture` requires an ADR-0007 approval ticket
  recorded alongside the declared export path, the realised
  export manifest, and any per-secret-class projection marker
  whose `raw_value_present` is true.
- A capsule whose `trust_state` is `restricted` MUST NOT declare
  an export path with `redaction_class: none`; the schema allows
  only `metadata_and_hashes_only` and (with an approval ticket)
  `broadened_capture` under restricted trust.
- Sinks MUST refuse field admissions that widen beyond the
  declared export path. A manifest that declares a sink and
  admission set the capsule did not authorise is non-conforming.

## Seeded capsule-case coverage

The companion fixtures exercise one capsule and one diff per
execution surface the contract binds:

- **Terminal session capsule.** Local host terminal seed under
  account-free-local identity mode with a declarative direnv
  envrc and no secret projection; diff case is a `rerun` that
  flips `drift_state` from `in_sync` to `stale_inputs` after the
  user edits the envrc.
- **Task run capsule.** Local host `cargo test` under a
  `workspace_file_defaults` origin (rust-toolchain.toml) with an
  applied rustup activator; diff case is a `rerun` where the
  `resolved_version` advanced after a rustup update.
- **Test run capsule.** Remote SSH host test-runner-runtime
  capsule with an `ssh_key_material` projection in
  `alias_plus_class_label` mode; diff case is a `reattach` where
  the credential alias rotated.
- **Debug attach capsule.** Devcontainer debug-adapter capsule
  with `prebuild_snapshot_reuse` origin and `ai_provider_token`
  projection under `not_projected` mode; diff case is a
  `debug_attach` comparing the debug seed to the task execution
  it attaches into, showing a drift on `policy_epoch`.
- **Refactor-preview capsule.** AI sandbox ephemeral capsule with
  `ad_hoc_per_run_override` origin and a
  `secret_projection_denied` degraded field for a blocked
  provider session; diff case is an `apply` comparing the
  preview capsule to the workspace capsule and marking one
  layer `redaction_limited` because the apply has not been
  approved for broadened capture.
- **Hosted-review replay capsule.** Hosted review replay capture
  against a frozen devcontainer image with
  `hosted_review_replay_capture` origin; diff case is a
  `hosted_review_replay` comparing the replay capsule against
  the original capture and showing a `changed` layer on
  `declarative_inputs` (one input's content digest advanced
  post-capture), marking `drift_state` `manually_diverged`.

The important property is not the exact fixtures. The property is
that every capsule-emitting surface, every diff consumer, every
export sink, every secret-class projection, and every policy
citation resolves one frozen vocabulary — not a per-subsystem
invention — so terminal, task, test, debug, AI/apply,
refactor-preview, and hosted-review replay surfaces read the same
environment truth.

## Change management

- Adding a new `capsule_location_class`, `capsule_origin_class`,
  `declarative_input_kind`, `secret_projection_mode`,
  `capsule_policy_marker.policy_kind`, `export_sink_class`,
  `export_field_admission.admission`,
  `consuming_surface_class`, `diff_context`, `diff_layer`,
  `diff_status`, `secret_projection_delta_kind`,
  `declarative_input_delta_kind`,
  `redaction_limited_reason`, `capsule_degraded_reason` beyond the
  execution-context set, or `capsule_audit_event_id` is
  additive-minor: bump `environment_capsule_schema_version` (or
  `environment_diff_schema_version` for the diff packet), add a
  row in the relevant matrix or fixture, and extend the schema.
- Repurposing any existing value is breaking and requires a new
  decision row.
- Renaming a `capsule_id`, `policy_ref`, or `input_ref` is done
  through the registry's alias mechanism (mirrors the ADR-0008
  alias discipline); it is never done by mutating the existing
  canonical id.

## Out of scope

- The concrete capsule-resolver Rust crate, the capsule-on-disk
  content store, capsule eviction tuning, or prebuild-snapshot
  packaging. The vocabulary freeze lands here; the runtime crates
  are the schema of record when they land.
- The final UX microcopy for "why this environment?" and "what
  changed in the environment?" inspectors. Copy lives with the
  UX style guide; this contract pins the closed sets that copy
  resolves against.
- Admin-policy authorship for secret-projection rules,
  export-redaction rules, or capsule-admission rules. This
  contract assumes those values flow in from the admin policy
  bundle at runtime and names the references rather than
  redefining them.
- Provider-specific credentials. The ADR-0007 credential-broker
  contract owns those; this contract projects their secret-class
  labels and credential-alias counts only.
