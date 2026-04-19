# ADR 0009 — Execution-context object model, workset / scope vocabulary, and authority projection

- **Decision id:** D-0015 (see `artifacts/governance/decision_index.yaml#D-0015`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-08-15
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** `TOOL-ENV-006`, `TOOL-CTX-002`

## Context

Every surface that launches code (`task`, `terminal`, `test`, `debug`,
notebook kernel, scaffolding action, AI tool call, `doctor --repair`
probe), explains a launch (`"why this toolchain?"`, support bundle,
replay capture, evidence packet, mutation journal, provenance
inspector), scopes an operation to a subset of a repository (search,
graph, indexing, rename, `aureline workset`, sparse checkout), or
projects environment / workspace / policy state across a process or
RPC boundary has to agree on the same answer to five questions: which
target is execution bound for, which toolchain / runtime was
activated, which environment capsule provided inputs, which workset
or slice the operation is scoped to, and which authority owns each
field a consumer renders. The PRD
(`.t2/docs/Aureline_PRD.md:1501-1520`, section 5.35) and the TAD
(`.t2/docs/Aureline_Technical_Architecture_Document.md:2283-2366`,
sections 16.1 / 16.1.1 / 16.1.2) both freeze a single execution-
context model, a single environment capsule shape, and a single
provenance record as first-order product contracts. The PRD and TAD
(`.t2/docs/Aureline_PRD.md:1106`,
`.t2/docs/Aureline_Technical_Architecture_Document.md:1845-1866`,
section 12.6) similarly freeze a workset / scope taxonomy
(`Root`, `Workset`, `Slice`, `Lens`, `Expansion plan`) that every
monorepo, multi-repo, and sparse-checkout flow has to share. This
ADR closes the stable vocabulary for those contracts before the
terminal, task, test, debug, AI, and export surfaces each invent
their own shape.

The freeze matters now, ahead of the execution-plane crates, the
task / test / debug surfaces, the AI tool-call plane, the
remote-agent attach path, and the support-export lane landing: if
those lanes proliferate before a shared execution-context and
workset / scope vocabulary is frozen, each will invent its own
target identity shape, its own toolchain / activator decision
record, its own environment capsule record, its own workset
descriptor, and its own `"why this ran"` inspector; launch,
provenance, replay, doctor-repair, and remote-attach work would
then land with incompatible assumptions about which target was
addressed, which toolchain resolved, which activators fired,
which slice the operation scoped to, and which authority owns each
projected field.

The execution-context resolver rides alongside the ADR-0001 identity
modes (managed, self-hosted, account-free local) whose trust state
gates activator execution and narrows declared network / egress
posture, the ADR-0004 RPC transport (execution-context records and
workset descriptors cross the wire as typed payloads; raw secret
material and raw environment bodies never do), the ADR-0005
subscription envelope (execution-context views ride the shared
envelope with authority class `execution` for authoritative frames
and `derived_knowledge` for projections; the authority-class matrix
frozen there is the canonical ownership vocabulary this ADR points
at), the ADR-0006 VFS save contract (target-identity, root identity,
and mount / alias layers re-export the five-layer filesystem
identity record; save manifests record the authority envelope a
launch observed), the ADR-0007 secret broker (secrets projected
into an execution target cross as `credential_alias` handles only,
never as raw bytes), and the ADR-0008 settings resolver (the
`execution` authority surfaces an effective-setting view whose
`resolved_scope` taxonomy includes `workspace`,
`folder_or_module_override`, `language_override`, and the
workset / slice projection described below). This ADR does not
redefine those contracts; it defines the execution-context-specific
and workset-specific fields they refer to.

Full execution-plane feature implementation (task runner, test
runner, debug-adapter orchestration, terminal host, notebook kernel
controller, remote-agent attach service, AI tool-call plane,
Project Doctor probe runners) is explicitly out of scope at this
milestone; this freeze establishes the vocabulary and invariants
those later lanes will honour.

## Decision

Aureline freezes a single **execution-context contract** and a
single **workset / scope contract**: every launch-capable or
scope-narrowing surface reads one stable **execution-context
record** shape, names exactly one **target identity** and one
**toolchain / runtime identity**, references one **environment
capsule**, carries one **workset / scope descriptor**, quotes one
**execution-context provenance record**, and tags each field it
projects with one **authority class** from the shared matrix so
reviewers can tell which fields are **authoritative**, which are
**projected** from a canonical owner, and which are **stale** at
the moment of rendering. No surface may invent a parallel
execution-context shape, a copy-only workset vocabulary, or a
private provenance format.

All rules below are stated in terms of contract, vocabulary, and
record names rather than specific crates so adapter changes are
hygiene, not re-litigation.

### Execution-context record (frozen)

Every surface that quotes or renders a launch reads one record
shape. The record is the only truth that flows between the resolver
and any consumer (UI `"why this toolchain?"` inspector, CLI
`aureline env inspect`, `aureline doctor --explain`, support bundle,
replay artifact, evidence packet, mutation-journal renderer, AI
tool-call plane). The record carries **references**, not raw
environment bodies; raw env bodies, command lines, and secret values
cross only through the explicit **broadened capture** projection
(see "Process-boundary constraints").

Required fields:

- `execution_context_id` — monotonic stable id allocated at resolve
  time. Copied into every event the launch emits (task events, test
  events, debug sessions, terminal transcripts, AI tool-call frames)
  so every downstream record round-trips to the same context.
- `schema_version` — `execution_context_schema_version` the record
  conforms to. Distinct from the setting-definition and
  subscription schema versions.
- `invocation_subject` — typed invocation descriptor carrying
  `command_id`, `surface` (`task`, `test`, `debug`, `terminal`,
  `notebook_kernel`, `scaffolding`, `ai_tool_call`, `doctor_repair`,
  `import_probe`, `replay_probe`), `actor_class` (one of
  `user_keystroke`, `user_command`, `session_override`,
  `workspace_migration`, `extension_api`, `ai_apply`,
  `scheduled_task`, `imported_profile`, `admin_policy_injector`),
  and `workspace_id` / `profile_id`.
- `target_identity` — frozen target descriptor (see "Target
  identity (frozen)").
- `toolchain_identity` — frozen toolchain / runtime descriptor
  (see "Toolchain / runtime identity (frozen)").
- `environment_capsule_ref` — stable reference to the environment
  capsule consumed by this launch (see "Environment capsule
  reference (frozen)"). The capsule itself is content-addressed;
  the record quotes the capsule's `capsule_id`, `capsule_hash`,
  and `resolved_schema_version` without copying its body.
- `workset_scope` — frozen workset / scope descriptor (see
  "Workset / scope vocabulary (frozen)"). Every execution declares
  exactly one scope; ambient unscoped execution is forbidden on
  protected surfaces.
- `trust_state` — one of `restricted`, `trusted` (ADR-0001). Writes
  to this field from surfaces other than the workspace-trust gate
  are forbidden; execution-context renderers read, never assign.
- `identity_mode` — one of `account_free_local`, `self_hosted_org`,
  `managed_convenience` (ADR-0001). Copied from the active workspace
  authority.
- `policy_epoch` — integer policy epoch at resolve time. Consumers
  that render a launch from a later epoch MUST surface the epoch
  delta; they MAY NOT silently accept a stale epoch as current.
- `activator_decisions` — ordered list of typed activator records
  (see "Activator decision class (frozen)"). Every detected
  activator appears with a disposition (`applied`, `blocked`,
  `ignored`, `unsupported`, `degraded`); silent omission is
  forbidden.
- `override_delta` — typed per-run override record
  (`working_directory_override`, `arguments_override`,
  `env_var_delta_classes` as a list of classes — never raw values,
  `launch_profile_overrides`, `temporary_secret_handle_classes`).
  Raw env values and raw command lines never live here; the
  classes name what changed.
- `cache_disposition` — one of `cold`, `warm`, `prebuild_reused`,
  `capsule_reused`, `rejected_drift`, `rejected_policy`,
  `rejected_trust`. Paired with `cache_key_ref` when a cache was
  consulted and with `invalidation_reason` when a cache was
  rejected.
- `provenance_record_ref` — reference to the execution-context
  provenance record (see "Execution-context provenance record
  (frozen)"). The record is the authoritative answer to
  "why this execution context?"; the execution-context record
  quotes its handle so consumers do not re-derive provenance
  locally.
- `authority_envelope` — per-field authority tag list (see
  "Canonical-owner versus projected-field table (frozen)"). Every
  field a surface renders is tagged as `authoritative`,
  `projected_from_<class>`, or `stale` so reviewers can tell which
  fields are the canonical owner's own state and which are
  projections.
- `degraded_fields` — ordered list of typed `degraded_field_record`
  entries (see "Degraded-field taxonomy (frozen)"). Empty means
  fully resolved; non-empty forces every surface that reads this
  record to render a visible honesty marker rather than claiming a
  field is authoritative when it is not.

No surface may invent additional top-level fields. Adding a new
top-level field is an additive-minor schema change (bump
`execution_context_schema_version`); repurposing an existing field
is breaking and requires a new decision row.

### Target identity (frozen)

Every execution targets exactly one **execution target**. Surfaces
that render a launch, a result, or a support bundle MUST quote the
target descriptor; surfaces MUST NOT synthesise an ambiguous
target from the command line alone.

Target-class enum:

- `local_host` — the running host process' operating system
  boundary.
- `ssh_remote` — remote host reached over SSH (including Aureline
  remote-agent SSH attach).
- `container_local` — OCI container on the local host (Docker,
  Podman, containerd).
- `devcontainer` — `.devcontainer/devcontainer.json`-defined
  target, whether local or hosted.
- `remote_workspace_vm` — remote hosted VM target (Codespaces-style,
  workspace-service-managed).
- `prebuild_runtime` — prebuilt runtime instance reused from a
  prebuild snapshot; pairs with `capsule_reused` cache disposition.
- `managed_workspace` — managed-identity-mode hosted workspace
  instance.
- `notebook_kernel_local` / `notebook_kernel_remote` — notebook
  kernel targets.
- `ai_sandbox` — AI tool-call sandbox target (bounded execution
  for AI tool calls).

Target descriptor fields:

- `target_class` — one of the frozen classes above.
- `canonical_target_id` — stable identifier within the class
  (container id, VM id, SSH host key fingerprint, managed
  workspace id). Surfaces quote this id; they MAY NOT substitute
  a human-friendly label.
- `mount_identity` — five-layer filesystem-identity record
  (ADR-0006). Re-exported unchanged so target mounts share the
  save / watcher / identity vocabulary.
- `route_dependency` — typed network-route description when the
  target is remote (`direct`, `tunneled`, `managed_proxy`,
  `policy_mirror`); `null` for local targets.
- `reachability_state` — one of `reachable`, `warming`,
  `degraded`, `unreachable`, `policy_blocked`. A surface that
  renders a launch against an `unreachable` target MUST NOT claim
  the launch succeeded.
- `capability_envelope_ref` — reference to the target's declared
  capability envelope (re-exports the ADR-0006 root-capability
  envelope where applicable; extends it with execution-target-
  specific flags like `supports_pty`, `supports_gpu`,
  `supports_network_egress`).

Target rules:

- Writing to a target that does not match the resolved
  `canonical_target_id` is forbidden; the resolver fails closed
  with `target_identity_mismatch` rather than silently retargeting.
- Target-class-demotion is narrowing-only: a managed-workspace
  target MAY degrade to `degraded` reachability, but the resolver
  MAY NOT silently rewrite a managed target as a local-host
  target.
- Remote and container targets declare both the **requested**
  target artifact (image ref, devcontainer file) and the
  **materialised** runtime instance (container id, VM id);
  surfaces render both so users can distinguish "declared
  environment" from "actual place work ran" (TAD 16.1.2).

### Toolchain / runtime identity (frozen)

Every execution resolves exactly one toolchain / runtime identity.
The identity is an alias into the toolchain registry; surfaces
MUST NOT invent synthetic toolchain labels.

Toolchain-class enum:

- `interpreter` — Python / Ruby / Node.js / Deno / Bun / PHP /
  Lua / other interpreter runtimes.
- `compiler_toolchain` — Rustup / cargo, Go, .NET, JVM
  (Gradle / Maven / SBT), C / C++ (clang / gcc / MSVC).
- `package_manager_runner` — npm / pnpm / yarn / uv / Poetry /
  Conda / Bundler / cargo-script task runners.
- `containerised_runtime` — toolchain surfaced through a container
  or devcontainer target.
- `notebook_kernel_runtime` — Jupyter / ipykernel / other notebook
  runtimes.
- `language_server_process` — language-server toolchain (clangd,
  rust-analyzer, pyright, etc.) when the execution target is a
  language server.
- `debug_adapter_runtime` — DAP-compatible debug-adapter process.
- `test_runner_runtime` — pytest, cargo test, go test, jest, vitest,
  nextest, etc.
- `build_driver_runtime` — cargo, go build, bazel, make, ninja,
  gradle, msbuild, etc.
- `ai_tool_runtime` — the AI tool-call plane's in-target executor.

Toolchain descriptor fields:

- `toolchain_class` — one of the frozen classes above.
- `toolchain_id` — stable id in the toolchain registry
  (dotted lower-snake, pattern `^[a-z][a-z0-9_]*(?:\.[a-z][a-z0-9_]*)+$`;
  for example, `python.cpython`, `rust.cargo`, `node.pnpm`).
- `resolved_version` — canonical version string as reported by the
  toolchain (no surface-local munging).
- `executable_identity` — five-layer filesystem-identity record
  (ADR-0006) for the resolved executable. Symlinks, shims, and
  wrappers converge on one `canonical_filesystem_object`.
- `activation_strategy` — one of `ambient_path`, `env_manager_shim`
  (asdf, mise, nvm, rbenv, pyenv), `venv_activation` (Python venv,
  Poetry, uv, Conda), `nix_shell` / `nix_flake` / `direnv`,
  `devcontainer_build`, `oci_image_ref`, `explicit_override`,
  `fallback_resolution`. Ambient-path resolution is a declared
  strategy, not a silent default.
- `wrapper_provenance` — list of wrapper / shim records
  (each with `wrapper_kind`, `wrapper_filesystem_identity`, and
  `wrapper_version`). Silent wrapper insertion is forbidden.
- `extension_pack_refs` — ordered list of extension-pack handles
  consumed by the toolchain.
- `known_unsupported_gaps` — ordered list of typed unsupported-gap
  records (reason code, affected-feature class). The resolver MAY
  NOT silently drop a feature; it surfaces a gap record so the
  `"why this toolchain?"` inspector can explain the limitation.
- `degraded_fallback_flag` — boolean; `true` when the resolver
  fell back to a lower-confidence toolchain. Pairs with a
  `degraded_field_record` with `reason = toolchain_fallback`.

Toolchain rules:

- A toolchain id is stable once the toolchain leaves
  `experimental` lifecycle; aliases redirect, they do not rename.
- The resolver records every wrapper / shim in `wrapper_provenance`;
  hiding shims is a `provenance_gap` degraded-field event.
- Ambient-path resolution is a last-resort strategy; when a repo
  declares an env-manager or venv strategy, the resolver MUST NOT
  silently downgrade to ambient path.

### Environment capsule reference (frozen)

The environment capsule is the authoritative declarative bundle of
inputs that produced this execution (TAD 16.1.1). The
execution-context record references the capsule; it does not copy
the capsule's body. This preserves content-addressability so
inspectors, support bundles, and replay artifacts can fetch the
capsule once and compare fingerprints.

Capsule reference fields:

- `capsule_id` — stable capsule id allocated by the capsule author.
- `capsule_hash` — content-addressed digest of the capsule body.
- `resolved_schema_version` — capsule-envelope schema version.
- `workspace_template_ref` — reference to the workspace template
  that hydrated this capsule, when present.
- `prebuild_snapshot_ref` — reference to the prebuild snapshot
  that warmed this capsule, when present.
- `drift_state` — one of `in_sync`, `stale_inputs`,
  `generator_changed`, `manually_diverged`, `unknown_lineage`
  (re-exports ADR-0006 drift state for the capsule's declarative
  inputs).
- `compatibility_fingerprint` — commit / tree identity, capsule
  hash, platform / arch, policy epoch, extension lock digest, and
  critical toolchain digests. Prebuild reuse compares this
  fingerprint; mismatches downgrade visibly rather than silently
  serving stale tools (TAD 16.1.1).

Capsule rules:

- A capsule referenced by an execution-context record MUST have a
  resolvable `capsule_id`; unresolvable references emit a
  `capsule_unresolved` degraded-field event.
- Workspace templates, prebuild snapshots, and capsule bodies are
  open, diffable, mirrorable artifacts; enterprise policy MAY add
  signing or approval requirements, but MAY NOT replace the
  capsule record with an opaque vendor-only blob.
- Prebuild snapshots are accelerators, not authorities: when
  `drift_state` is anything other than `in_sync`, the resolver
  downgrades `cache_disposition` to `rejected_drift` and surfaces
  the degraded field.

### Execution-context provenance record (frozen)

Every launch emits one provenance record. The record is the
versioned, inspectable answer to "why this execution context?" and
is the only provenance record every downstream surface reads
(`"why this toolchain?"` inspector, `aureline env inspect`,
`aureline doctor --explain`, support bundles, replay artifacts,
Project Doctor probes, mutation-journal generation). No surface
may invent a private explanation format (TAD 16.1.2).

Required fields:

- `provenance_record_id` — stable id allocated at resolve time.
- `execution_context_id` — round-trip id of the enclosing
  execution-context record.
- `recorded_at` — monotonic stamp of the resolver's decision.
- `resolver_version` — version of the resolver that produced this
  record.
- `invocation_subject` — copied from the execution-context record.
- `target_identity` — copied from the execution-context record,
  both **requested** and **materialised** forms where they differ.
- `baseline_context` — host baseline fingerprint, shell / login
  snapshot class (`default_login`, `non_login`, `policy_shell`,
  `custom_startup`), locale / proxy / trust-store lineage, and
  baseline timestamp.
- `activator_decisions` — ordered list of activator records with
  reason codes and evidence refs. Ignored or blocked activators
  appear with explicit reason codes and suggested follow-up
  actions; silent disappearance is a `provenance_gap` event.
- `override_delta` — copied from the execution-context record
  (env-var classes, not raw values).
- `policy_and_trust_decisions` — workspace-trust state,
  approval-ticket ref when present, policy bundle epoch, sandbox
  profile class, egress posture class.
- `cache_reuse_disposition` — `cache_key_ref`, hit / miss,
  prebuild / capsule reuse decision, drift markers,
  invalidation reason.
- `degraded_or_unresolved_fields` — missing tools, blocked
  activators, fallback executable, unsupported source kinds, and a
  typed `confidence_level` (`high`, `medium`, `low`).
- `redaction_class` — ADR-0007 redaction class applied on export
  (`metadata_and_hashes_only` by default; raw environment bodies,
  command lines, and secret values require explicit
  `broadened_capture` opt-in).

Provenance rules:

- The same record powers UI explainers, CLI inspectors, support
  bundles, replay capture, and Project Doctor (no feature may
  invent a private explanation format, TAD 16.1.2).
- Ignored or blocked activators MUST appear with explicit reason
  codes and repair-candidate hooks, not silently omitted.
- Remote, template, capsule, and prebuild paths MUST name both the
  requested source artifact and the materialised runtime instance.
- Provenance exports default to metadata, hashes, and class labels;
  raw environment bodies, command lines, or secret values require
  explicit broadened capture and carry the `broadened_capture`
  redaction class.

### Activator decision class (frozen)

The resolver records every detected activator's disposition as a
typed record so the `"why this toolchain?"` inspector and
`doctor --explain` can render the decision chain without re-parsing
logs.

Activator-class enum:

- `env_manager_shim` (asdf, mise, nvm, rbenv, pyenv, goenv,
  `.tool-versions`-driven).
- `python_venv_activation` (`venv`, `virtualenv`, Poetry, uv,
  Hatch, Pipenv).
- `conda_activation`.
- `rustup_toolchain_override` (`rust-toolchain.toml`).
- `go_toolchain_override` (`go.mod` `toolchain` directive).
- `jdk_version_activation` (`sdkman`, `asdf-java`, project JDK
  override).
- `nodejs_version_activation` (`nvm`, `fnm`, Volta, `.nvmrc`).
- `nix_shell_or_flake` (`nix develop`, `shell.nix`, `flake.nix`).
- `direnv_activation` (`.envrc`).
- `devcontainer_hook` (devcontainer `postCreate`, `postStart`,
  `onCreate`, Compose bring-up).
- `workspace_bootstrap_script` (repo-declared `bootstrap`,
  `setup.sh`, repo Makefile targets).
- `policy_injected_environment` (admin-policy-injected env bundle).

Disposition enum:

- `applied` — activator ran; its effect is present in the resolved
  environment.
- `blocked` — activator was detected but not run (trust state,
  policy, missing capability). Carries a typed reason code and a
  repair hook.
- `ignored` — activator was detected but explicitly skipped
  (for example, a per-run override opted out). Carries the
  opt-out source.
- `unsupported` — activator is recognised but not executable on
  the current target / host (for example, `devcontainer_hook` on a
  local-host target without container support).
- `degraded` — activator ran in a degraded mode (for example,
  `nix_shell_or_flake` fell back to impure evaluation because a
  required flag was unavailable).

Activator-decision rules:

- Repo-defined lifecycle hooks (devcontainer `postCreate`,
  Compose bring-up, Nix / direnv activation, bootstrap scripts)
  are **trust-gated actions**; in restricted mode or
  policy-denied contexts they remain visible suggestions with
  repair guidance, not silent no-ops or hidden execution
  (TAD 16.1.1).
- `blocked` and `ignored` activators MUST carry a reason code and
  a repair-candidate hook the Project Doctor surface can consume.
- A missing activator is recorded as an `unresolved_activator`
  degraded-field event rather than silently dropped.

### Workset / scope vocabulary (frozen)

Every execution and every scope-narrowing operation declares
exactly one **scope descriptor** so the terminal, task, test,
debug, search, graph, AI, and export surfaces describe the same
slice. The scope descriptor is a discriminated union; every record
names exactly one `scope_class`.

Scope-class enum:

- `current_root` — single workspace root currently focused in the
  surface (the PRD-level default for most interactive flows).
  Minimum fields: `workspace_id`, `root_id` (five-layer identity).
- `named_workset` — a user-named intentional scope over one or
  more roots (PRD `.t2/docs/Aureline_PRD.md:1106`:
  "users should be able to open **worksets** that intentionally
  scope a monorepo to a smaller slice"). Minimum fields:
  `workset_id`, `workset_name`, `member_refs`
  (ordered list of root / folder / module refs),
  `membership_policy` (`explicit_list`, `glob_pattern`,
  `dependency_graph_reachable`).
- `sparse_slice` — a subset of a root expressed as an inclusion /
  exclusion pattern set (sparse checkout, partial-enumeration
  lens). Minimum fields: `root_id`, `include_patterns`,
  `exclude_patterns`, `dependency_closure_enabled`,
  `missing_members_policy` (`manifest_known`, `cached`,
  `unavailable`).
- `full_workspace` — the entire set of roots the workspace knows
  about, including transitively referenced roots. Minimum fields:
  `workspace_id`, `included_root_ids` (ordered list).
- `policy_limited_view` — a scope whose effective membership is
  narrowed by admin policy (for example, tenant-scoped review
  workspace, classified-directory exclusion). Minimum fields:
  `underlying_scope_ref` (pointer to the pre-narrowing
  scope descriptor), `policy_ref`,
  `visible_member_refs`, `hidden_member_count`
  (integer; the exact hidden list is never exposed outside the
  policy-admin surface, preventing information leak).
- `review_workspace` — a review-scoped surface (change set /
  review bundle) whose scope is the review's affected member set.
  Minimum fields: `review_id`, `member_refs`.
- `companion_surface` — a companion-surface scope (docs preview,
  agent companion). Minimum fields: `companion_id`, `member_refs`.

Partial-truth labels (frozen, re-exported from TAD 12.6):

- `loaded` — producer has authoritative knowledge of this member.
- `manifest_known` — producer knows the member exists via a
  manifest but has not loaded its contents.
- `cached` — producer has a cached view; freshness class applies.
- `unavailable` — producer cannot resolve this member (scope
  excluded, entitlement missing, offline unreachable).

Lens and expansion-plan fields (frozen):

- `lens_class` — one of `search_result_lens`,
  `graph_neighbourhood_lens`, `review_overlay_lens`,
  `ai_context_lens`, `companion_focus_lens`, or `none`. A lens is
  an overlay on an underlying scope; it narrows presentation, not
  authority. The underlying scope's membership is still the
  authority for save, rename, and policy decisions.
- `expansion_plan` — ordered list of typed expansion steps
  (`load_manifest`, `warm_index`, `materialise_snapshot`,
  `fetch_cached_view`, `refresh_stale_cache`). Expansion plans
  are declarative; execution consults the plan to decide what to
  load on demand.

Composition rules:

- An execution-context record declares exactly one `workset_scope`
  descriptor. Ambient unscoped execution is forbidden on protected
  surfaces.
- A scope MAY be narrowed by admin policy into a
  `policy_limited_view`; narrowing MUST NOT silently broaden the
  underlying scope (for example, admin policy cannot widen a
  sparse slice into a full workspace).
- A lens narrows presentation only; the surface MUST NOT use a
  lens to evade the underlying scope's authority or policy
  membership.
- A `full_workspace` scope is the only scope allowed to omit
  root-level membership enumeration; every other scope names its
  members explicitly.
- Scope membership is evaluated against the five-layer
  filesystem-identity record (ADR-0006); alias-converged roots
  count once, even when multiple presentation paths reach the
  same canonical filesystem object.

### Canonical-owner versus projected-field table (frozen)

Execution-context fields are tagged by authority so reviewers can
tell which surface owns each field and which surfaces project it.
Authority classes reuse the matrix frozen in ADR-0005 (the
`authority_class` enum on the subscription envelope). This ADR
does not mint new authority classes; it binds execution-context
fields to existing ones.

| Field                                         | Canonical owner (`authority_class`) | Projection rules                                                                                                                 |
|-----------------------------------------------|-------------------------------------|----------------------------------------------------------------------------------------------------------------------------------|
| `invocation_subject`                          | `execution`                         | Authoritative at the invoking surface; projected elsewhere carries `projected_from_execution`.                                   |
| `target_identity.canonical_target_id`         | `execution`                         | Authoritative. Never inferred by a projecting surface.                                                                           |
| `target_identity.mount_identity`              | `workspace_vfs`                     | Authoritative at the VFS; execution-context readers project with `projected_from_workspace_vfs`.                                 |
| `target_identity.route_dependency`            | `execution`                         | Authoritative at the execution target. Remote reachability projections MUST carry `freshness` labels per ADR-0005.               |
| `toolchain_identity.*`                        | `execution`                         | Authoritative at the execution-context resolver.                                                                                 |
| `environment_capsule_ref.*`                   | `workspace_vfs`                     | Capsule body lives in the workspace; the execution-context record quotes the reference and projects with `projected_from_workspace_vfs`. |
| `workset_scope`                               | `workspace_vfs`                     | Workspace authority owns root / workset / slice membership; execution-context renderers project.                                 |
| `trust_state`                                 | `policy_entitlement`                | Authoritative at the workspace-trust gate. Execution-context readers project with `projected_from_policy_entitlement`.           |
| `identity_mode`                               | `policy_entitlement`                | Authoritative at the workspace authority (ADR-0001). Projected fields MUST NOT claim `authoritative`.                            |
| `policy_epoch`                                | `policy_entitlement`                | Authoritative at the policy authority. Stale epoch projections surface a visible delta.                                          |
| `activator_decisions`                         | `execution`                         | Authoritative at the resolver. Replay / support-bundle readers project with `projected_from_execution`.                          |
| `override_delta`                              | `execution`                         | Authoritative at invoke time. Export projections carry class labels, never raw values.                                           |
| `cache_disposition` / `cache_key_ref`         | `derived_knowledge`                 | Cache indexes are derived; reuse decisions quote input digests and surface drift.                                                |
| `provenance_record_ref`                       | `execution`                         | Authoritative at the resolver.                                                                                                   |
| AI-plane summaries of any field               | `provider_overlay`                  | Provider overlays MAY NOT overwrite authoritative execution-context fields; they project and label.                              |
| Search / graph views of the execution context | `derived_knowledge`                 | Derived; every projection carries `producer_refs` and a freshness label per ADR-0005.                                            |

Projection rules:

- Every field a consumer renders is tagged with an
  `authority_envelope` entry naming the canonical authority class
  (authoritative), the projecting class (`projected_from_<class>`),
  or `stale` when the projection is older than the last-known
  authoritative update. Surfaces MUST render this tag on every
  surface that shows the field.
- A projection MUST carry a `freshness` label from the ADR-0005
  frozen set; a projection that cannot prove freshness is labelled
  `stale` with a stale-reason code.
- Overlay authorities (`provider_overlay`) MAY NOT overwrite an
  authoritative field; they MAY propose a value that the canonical
  owner accepts or rejects, and the projection carries the
  overlay's origin.
- Derived authorities (`derived_knowledge`) MUST quote every input
  digest in their `producer_refs` so consumers can compare the
  projection against the canonical owner's current value.

### Degraded-field taxonomy (frozen)

Honest degradation is the rule; silent completeness is forbidden.
Every unresolved, stale, or partially resolved field emits a
`degraded_field_record` the consuming surface renders.

Degraded-field reason codes:

- `toolchain_fallback` — resolver fell back to a lower-confidence
  toolchain; pair with `degraded_fallback_flag` on the toolchain
  record.
- `activator_blocked_by_trust` — activator was detected but
  blocked because the workspace is in `restricted` trust.
- `activator_blocked_by_policy` — activator was blocked by admin
  policy.
- `activator_unsupported_on_target` — activator is recognised but
  not runnable on the current target class.
- `capsule_unresolved` — capsule reference could not be resolved
  (missing artifact, broken hash).
- `capsule_drift_detected` — capsule fingerprint does not match
  the prebuild snapshot's fingerprint.
- `target_unreachable` — target identity resolved but reachability
  is `unreachable` or `degraded`.
- `policy_epoch_stale` — consumer projection is older than the
  current policy epoch.
- `trust_state_unresolved` — trust gate has not yet evaluated the
  workspace.
- `workset_member_unavailable` — one or more declared members of
  the workset are `unavailable` (scope excluded, entitlement
  missing, offline unreachable).
- `provenance_gap` — one or more expected provenance fields are
  missing (for example, a wrapper insertion was not recorded).
- `confidence_low` — resolver marked its confidence as `low` for
  one or more fields.
- `remote_agent_scope_mismatch` — remote-agent attach-time scope
  does not match the workspace the host expects.

Degraded-field rules:

- A consumer surface that reads an execution-context record with
  non-empty `degraded_fields` MUST render a visible honesty marker
  on every affected field.
- A surface MAY NOT promote a degraded field to authoritative by
  re-projecting it; the projection carries the degraded state
  forward.
- The resolver emits a typed audit event for every
  degraded-field record (see "Audit events (frozen)").

### Fail-closed rules (frozen)

The resolver applies four invariants that together forbid silent
drift:

1. A surface MUST NOT render an execution-context field without a
   populated authority tag. A missing tag is a conformance bug.
2. A scope MAY be narrowed (sparse, policy-limited, session
   override); a scope MAY NOT be silently widened. Widening
   requires an explicit user / migration / approval action and is
   recorded in the mutation journal (ADR-0006).
3. A toolchain identity or target identity that does not match
   the canonical record is denied with `target_identity_mismatch`
   or `toolchain_identity_mismatch`; the resolver does not
   silently retarget.
4. An execution that would require a capability the target cannot
   supply (for example, GPU access on a `container_local` target
   without GPU passthrough) is denied with
   `capability_dependency_unmet` or visibly downgraded with a
   `capability_dependency_degraded` record; silent best-effort
   execution is forbidden.

### Audit events (frozen)

Every observable resolver action emits a structured event on the
`execution_context` audit stream. Events carry
`execution_context_id`, `target_class`, `toolchain_id`,
`workset_scope.scope_class`, `identity_mode`, `trust_state`,
`policy_epoch`, `actor_class`, and degraded-field reason where
relevant. Events MUST NOT carry raw environment bodies, raw
command lines, or raw secret values (ADR-0007 redaction applies).

| Event id                                                   | Fires when                                                                                                |
|------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| `execution_context_resolved`                               | A launch's execution-context record is produced.                                                          |
| `execution_context_reused`                                 | A cache / capsule / prebuild is reused; carries the cache-disposition reason.                             |
| `execution_context_rejected`                               | A proposed launch is denied (typed reason: `target_identity_mismatch`, `toolchain_identity_mismatch`, `trust_denied`, `policy_denied`, `capability_dependency_unmet`). |
| `execution_context_degraded`                               | One or more fields resolved in a degraded state.                                                          |
| `activator_applied` / `_blocked` / `_ignored` / `_unsupported` / `_degraded` | An activator decision is recorded.                                                                        |
| `capsule_reused` / `_rejected_drift` / `_unresolved`       | A capsule reference resolves, fails compatibility, or is missing.                                         |
| `target_reachability_changed`                              | Target reachability transitions (`reachable`, `warming`, `degraded`, `unreachable`, `policy_blocked`).    |
| `workset_scope_narrowed` / `_widened`                      | A scope is narrowed (allowed) or widened (requires explicit action).                                      |
| `policy_epoch_rolled`                                      | Policy epoch advances; any in-flight projections are marked stale.                                        |
| `provenance_record_emitted`                                | The resolver produces a provenance record.                                                                |
| `provenance_export_broadened`                              | A support-export path opted into `broadened_capture` redaction for this record.                           |
| `execution_context_schema_version_bumped`                  | The execution-context schema version advances.                                                            |

### Process-boundary constraints (frozen)

1. The execution-context resolver runs in the host process
   alongside the workspace authority; remote-agent execution
   brokers carry an attach-time **scope binding** and MUST NOT
   serve launches bound to a different workspace or a different
   agent scope.
2. Execution-context records and workset descriptors cross the
   RPC boundary as typed payloads (ADR-0004). Raw environment
   bodies, raw command lines, and raw secret values do not;
   projected fields carry classes, hashes, and references only.
3. Extensions reach execution-context records through the
   extension-SDK surface; they MUST NOT crawl `provenance_record`
   artifacts directly.
4. AI tool calls receive execution-context records and workset
   descriptors as typed payloads; raw env / command / secret
   material is never projected into the AI plane unless an
   explicit `broadened_capture` opt-in is recorded on the
   provenance record.
5. Crash dumps MUST NOT inherit in-flight execution-context
   records that carry `credential_alias` handles or
   `broadened_capture` payloads; ADR-0007 redaction applies.
6. Mutation-journal entries that reference an execution
   (task launch, test run, debug session, AI tool call) name
   `execution_context_id`, `target_class`,
   `toolchain_id`, `workset_scope.scope_class`, and
   `provenance_record_id` without embedding raw env / command /
   secret material.
7. Support-bundle exports default to metadata, hashes, and class
   labels; broadened capture requires an explicit opt-in recorded
   in the provenance record.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- The full task / test / debug runner implementations (process
  orchestration, event pumps, adapter drivers). This ADR freezes
  the vocabulary those runners will render; the runners themselves
  land under later rows.
- The live execution-context resolver implementation (crate,
  cache strategy, incremental re-resolution, notification
  plumbing). This ADR freezes the contract; the crate lands under
  a later row.
- The remote-agent attach service and the managed-workspace
  hosting fabric. This ADR freezes the target-class set, the
  reachability states, and the scope-binding rule; the services
  themselves land under later rows.
- The terminal host, notebook kernel controller, and scaffolding
  engine. This ADR freezes the execution-context record they
  consume; the hosts themselves land under later rows.
- The AI tool-call plane sandbox and approval ticket fabric. This
  ADR reserves the `ai_sandbox` target class, the
  `ai_tool_call` surface, and the `ai_tool_runtime` toolchain
  class; the plane itself lands under later rows.
- Project Doctor's probe runner. This ADR freezes the provenance
  record Project Doctor consumes; probe authoring lands under
  later rows.
- Per-toolchain activator grammar (how each env manager / venv /
  flake is detected and run). This ADR freezes that activator
  decisions are typed, recorded, trust-gated, and explainable;
  the detection grammar lands under the toolchain lane.
- The experiment / kill-switch fabric for launching an AI tool
  call under a managed rollout. This ADR reserves the
  `policy_epoch` and `activator_decisions` fields; the rollout
  fabric lands under a later row.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/execution_context_tradeoff_rows.yaml`.
Headline summary:

| Axis                                          | Chosen stack                                                                                                          | Best rejected alternative                                                  | Why chosen wins                                                                                                           |
|-----------------------------------------------|-----------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------|
| **Execution-context identity**                | One stable `execution_context_id` with a frozen record shape and round-trip through every downstream event.          | Per-surface launch descriptors.                                            | Per-surface descriptors guarantee drift between task, test, debug, terminal, AI, replay, and support.                     |
| **Target identity**                           | Frozen target-class enum + canonical id + mount-identity re-export of ADR-0006.                                      | Free-form target strings ("my-container").                                 | Free-form strings collapse the "wrong machine / wrong container" hazard the TAD explicitly flags (16.1.2).                |
| **Toolchain / runtime identity**              | Dotted lower-snake id + resolved-version + executable-identity + declared activation strategy.                       | Version string alone.                                                      | Version alone cannot distinguish `python.cpython` via venv from via Conda from via `ambient_path`; support and AI will drift. |
| **Environment capsule reference**             | Reference-only (capsule id + hash + schema version + drift state); capsule body stays content-addressed.             | Inline capsule body inside every execution-context record.                 | Inline bodies defeat content-addressed reuse and explode RPC / replay payloads.                                           |
| **Workset / scope vocabulary**                | Discriminated union over `current_root`, `named_workset`, `sparse_slice`, `full_workspace`, `policy_limited_view`, plus review / companion. | Single `scope_id: string` field.                                           | A single opaque string hides sparse-checkout truth, admin-policy narrowing, and the partial-truth labels TAD 12.6 freezes.|
| **Authority projection**                      | Per-field `authority_envelope` tagging using the ADR-0005 authority-class matrix.                                    | Rely on consumer heuristics to decide authority.                           | Heuristics drift; the matrix is already the canonical ownership vocabulary for reactive subscriptions.                    |
| **Degraded-field taxonomy**                   | Typed reason codes + honesty markers; no silent completeness.                                                        | Best-effort rendering when a field is unresolved.                          | Best-effort rendering is exactly the "silent stale tools" failure mode TAD 16.1.1 forbids.                                |
| **Provenance record**                         | One versioned record powering UI / CLI / support / replay / Project Doctor.                                          | Per-feature explanation formats.                                           | Per-feature formats produce contradictory "why this ran?" answers across UI, CLI, and support.                            |
| **Secret / env projection**                   | Class labels and `credential_alias` handles only; raw env / command / secret require `broadened_capture` opt-in.     | Default-broadened capture in support bundles.                              | Default-broadened export leaks secrets and raw env bodies exactly where bundles are shared outside the host.              |
| **Schema of record**                          | Rust types in the eventual execution-context crate; JSON Schema export at `schemas/runtime/execution_context.schema.json`. | External IDL + codegen at this milestone.                                  | No second-language consumer yet; the JSON Schema export reserves a clean integration point. Mirrors ADR 0004–0008.        |

Each row carries reopen triggers in the YAML. A benchmark finding
that execution-context resolution cannot meet its latency budget,
a conformance finding that two surfaces disagree on target identity
or scope membership, a support finding that a degraded field is
not typed, a security finding that a projection leaked raw env
data, or a roadmap step that mints a new target class reopens the
relevant row.

### Execution-context fixtures

A small corpus of execution-context fixtures lives under
`fixtures/runtime/execution_context_examples/`. They are short,
reviewable scenarios (local task launch, remote-agent SSH attach,
devcontainer launch, sparse-workset scope, policy-limited review
view, degraded activator, rejected prebuild drift, capsule-reuse
hit) used by the terminal, task, test, debug, AI, export,
doctor-repair, and replay lanes to anchor the target-class enum,
the toolchain-class enum, the scope-class enum, the authority
envelope, the degraded-field taxonomy, and the provenance record
shape to concrete inputs and observable outcomes. They are not a
test suite; they are the language the ADR's tables refer to.

## Consequences

- **Frozen:** the execution-context record shape, including
  `execution_context_id`, `schema_version`, `invocation_subject`,
  `target_identity`, `toolchain_identity`,
  `environment_capsule_ref`, `workset_scope`, `trust_state`,
  `identity_mode`, `policy_epoch`, `activator_decisions`,
  `override_delta`, `cache_disposition`, `provenance_record_ref`,
  `authority_envelope`, and `degraded_fields`.
- **Frozen:** the target-class enum (`local_host`, `ssh_remote`,
  `container_local`, `devcontainer`, `remote_workspace_vm`,
  `prebuild_runtime`, `managed_workspace`,
  `notebook_kernel_local`, `notebook_kernel_remote`,
  `ai_sandbox`) and the target-descriptor field set.
- **Frozen:** the toolchain-class enum (`interpreter`,
  `compiler_toolchain`, `package_manager_runner`,
  `containerised_runtime`, `notebook_kernel_runtime`,
  `language_server_process`, `debug_adapter_runtime`,
  `test_runner_runtime`, `build_driver_runtime`,
  `ai_tool_runtime`) and the toolchain-descriptor field set.
- **Frozen:** the environment-capsule reference shape
  (`capsule_id`, `capsule_hash`, `resolved_schema_version`,
  `workspace_template_ref`, `prebuild_snapshot_ref`,
  `drift_state`, `compatibility_fingerprint`) and the rule that
  prebuild snapshots are accelerators, never authorities.
- **Frozen:** the provenance-record shape
  (`provenance_record_id`, `execution_context_id`, `recorded_at`,
  `resolver_version`, `invocation_subject`, `target_identity`,
  `baseline_context`, `activator_decisions`, `override_delta`,
  `policy_and_trust_decisions`, `cache_reuse_disposition`,
  `degraded_or_unresolved_fields`, `redaction_class`) and the
  rule that one record powers every explainer surface.
- **Frozen:** the activator-class enum, the activator-disposition
  enum (`applied`, `blocked`, `ignored`, `unsupported`,
  `degraded`), and the rule that repo-defined lifecycle hooks are
  trust-gated and explainable.
- **Frozen:** the scope-class enum (`current_root`,
  `named_workset`, `sparse_slice`, `full_workspace`,
  `policy_limited_view`, `review_workspace`, `companion_surface`),
  the partial-truth labels (`loaded`, `manifest_known`, `cached`,
  `unavailable`), the lens-class set, and the expansion-plan
  shape.
- **Frozen:** the authority-projection table and the rule that
  every rendered field carries an authority-envelope tag from
  the ADR-0005 matrix.
- **Frozen:** the degraded-field reason codes and the honest-
  degradation rule.
- **Frozen:** the audit-event id set
  (`execution_context_resolved`, `_reused`, `_rejected`,
  `_degraded`, `activator_applied` / `_blocked` / `_ignored` /
  `_unsupported` / `_degraded`, `capsule_reused` / `_rejected_drift` /
  `_unresolved`, `target_reachability_changed`,
  `workset_scope_narrowed` / `_widened`, `policy_epoch_rolled`,
  `provenance_record_emitted`, `provenance_export_broadened`,
  `execution_context_schema_version_bumped`).
- **Frozen:** the schema of record is Rust types in the eventual
  execution-context crate; the boundary schema lives at
  `schemas/runtime/execution_context.schema.json`; no external
  IDL or codegen toolchain at this milestone. This mirrors
  ADR 0004, ADR 0005, ADR 0006, ADR 0007, and ADR 0008.
- **Permitted:** adding a new target class, toolchain class,
  activator class, scope class, lens class, authority-envelope
  tag, audit-event id, or degraded-field reason code is
  additive-minor with a schema bump and a row in the registry;
  repurposing an existing value is breaking and requires a new
  decision row.
- **Permitted:** admin policy MAY narrow a workset scope into a
  `policy_limited_view`. Policy MAY NOT silently widen a scope
  or silently downgrade a target class; any widening / retarget
  lands through a declared policy bundle that itself redacts to
  its class.
- **Follow-up:** the task / test / debug / terminal / notebook /
  AI / remote-agent / Project Doctor lanes instrument every
  resolver event and respect every frozen target / toolchain /
  scope / authority-envelope / degraded-field rule before
  claiming execution-plane guarantees.
- **Follow-up:** the mutation-journal lane adds
  `execution_context_id`, `target_class`, `toolchain_id`,
  `workset_scope.scope_class`, and `provenance_record_id` to
  every execution-related entry once the live resolver lands;
  the record fields named above are already reserved.
- **Follow-up:** the support-export lane's broadened-capture
  opt-in lands as a typed policy gate tied to the
  `provenance_export_broadened` audit event.
- **Ratifies:** the ADR-0005 subscription envelope's authority
  class `execution` now refers to the execution-context and
  provenance records frozen here. The ADR-0004 frozen error
  taxonomy's `policy` and `validation` classes absorb
  `target_identity_mismatch`, `toolchain_identity_mismatch`, and
  `capability_dependency_unmet` as typed subcodes. The ADR-0006
  save manifest's target-identity fields cover the workspace
  side of launches; the execution side rides this ADR. The
  ADR-0007 secret broker owns every `credential_alias` handle
  projected into a target; no raw secret ever crosses into an
  execution-context record. The ADR-0008 settings resolver's
  `resolved_scope` taxonomy covers the same workspace / folder /
  language override scopes execution-context readers project.

## Alternatives considered

- **Per-surface launch descriptors.** Rejected: per-surface
  descriptors guarantee drift between task, test, debug,
  terminal, notebook, AI, replay, and support. The
  `"why this execution context?"` promise cannot hold when each
  surface invents its own shape.
- **Free-form target strings.** Rejected: a `target: "my-container"`
  string collapses the "wrong machine / wrong container / wrong
  route" hazard TAD 16.1.2 explicitly names. Typed target class +
  canonical id + mount identity is the only auditable posture.
- **Version-string-only toolchain identity.** Rejected: a bare
  `"python 3.11"` cannot distinguish a venv from a Conda env from
  ambient path. Without activation strategy and executable
  identity, support and AI tool calls drift silently.
- **Inline capsule bodies.** Rejected: inline bodies defeat
  content-addressed reuse and explode RPC / replay payloads.
  Reference-only keeps capsules diffable and mirrorable.
- **Opaque `scope_id` string.** Rejected: one opaque string hides
  sparse-checkout truth, admin-policy narrowing, partial-truth
  labels (`loaded`, `manifest_known`, `cached`, `unavailable`),
  and the review / companion surface scopes.
- **Consumer-heuristic authority resolution.** Rejected: without
  per-field authority tags, consumers invent local rules for
  which field they trust. The ADR-0005 authority-class matrix is
  already the canonical ownership vocabulary for reactive
  subscriptions; binding execution-context fields to it keeps
  ownership honest.
- **Best-effort rendering when a field is unresolved.** Rejected:
  best-effort rendering is exactly the "silent stale tools"
  failure mode TAD 16.1.1 forbids. Typed degraded-field records
  are the only honest posture.
- **Per-feature provenance formats.** Rejected: per-feature
  formats produce contradictory "why this ran?" answers across
  UI, CLI, replay, and support. One versioned record is the
  only posture that survives TAD 16.1.2.
- **Default-broadened support-bundle capture.** Rejected: default
  broadened capture leaks secrets and raw env bodies exactly
  where bundles are shared outside the host. Metadata + hashes +
  class labels are the safe default; broadened capture requires
  an explicit opt-in recorded in the provenance record.
- **External IDL + generator for the execution-context
  envelope.** Rejected: same argument ADR 0004, ADR 0005,
  ADR 0006, ADR 0007, and ADR 0008 make — an IDL without a
  second-language consumer costs more than it buys; the JSON
  Schema export reserves the integration point.
- **Defer to a later milestone.** Rejected: the default-if-
  unresolved narrowing on `D-0015` ("freeze the product to a
  single-target local-host, single-toolchain, single-scope
  `current_root` launch model with no typed activator
  decisions, no environment capsule, no workset / slice, no
  admin-policy narrowing, no degraded-field taxonomy, no
  provenance record, and no authority-projection envelope")
  would block the terminal, task, test, debug, AI, remote-agent,
  notebook, scaffolding, Project Doctor, and support-export
  lanes exactly when later work needs the frozen vocabulary.

The `D-0015` `narrow` default-if-unresolved posture would have
locked the resolver to a single-target, single-toolchain,
single-scope model with no activator typing, no capsule, no
workset / slice, no policy narrowing, no degraded-field taxonomy,
no provenance record, and no authority projection until an ADR
landed. Accepting this ADR replaces that narrowing with the
frozen execution-context record, target-class enum, toolchain-
class enum, capsule-reference shape, scope-class enum,
authority-projection table, degraded-field taxonomy, and
provenance-record shape above; the narrowing default does not
apply.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1106` — "users should be able to open
  **worksets** that intentionally scope a monorepo to a smaller
  slice".
- `.t2/docs/Aureline_PRD.md:1501` — "5.35 Execution context".
- `.t2/docs/Aureline_PRD.md:1520` — "execution-context rules".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1845` —
  "12.6 Workspace slices, worksets, and multi-repo composition".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2283` —
  "16.1 One execution-context model".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2312` —
  "16.1.1 Environment definition, workspace template, and
  prebuild architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2342` —
  "16.1.2 Execution-context provenance inspector, resolver
  explainability, and Project Doctor architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2346` —
  "Provenance field / Minimum content / Why it matters" table.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:2357` —
  "Explainability rules".

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0015`
- RFC: none.
- Tradeoff register (machine form):
  `artifacts/architecture/execution_context_tradeoff_rows.yaml`.
- Workset / scope matrix (machine form):
  `artifacts/runtime/execution_scope_matrix.yaml`.
- Boundary schema (machine form):
  `schemas/runtime/execution_context.schema.json`.
- Execution-context fixtures:
  `fixtures/runtime/execution_context_examples/`.
- Companion vocabulary document:
  `docs/runtime/execution_context_vocabulary.md`.
- Identity modes this contract rides:
  `docs/adr/0001-identity-modes.md`.
- Transport boundary records cross:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`.
- Reactive-truth contract the authority-class matrix lives in:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`.
- VFS save / identity contract target mounts re-export:
  `docs/adr/0006-vfs-save-cache-identity.md`.
- Secret broker every projected credential resolves against:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`.
- Settings resolver whose scope taxonomy this ADR projects:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`.
- Affected lanes: `governance_lane:docs_public_truth`,
  `governance_lane:shell_command_system`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
