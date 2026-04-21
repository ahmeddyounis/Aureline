# Build-vs-buy register

This document is the human-readable companion to the canonical
machine-readable register in
[`/artifacts/governance/build_vs_buy_register.yaml`](../../artifacts/governance/build_vs_buy_register.yaml).
It turns the architecture posture from TAD section 4.3 and Appendix Z
into a current governance asset that dependency review, ADRs, and fork
review can cite directly.

Companion artifacts:

- [`/artifacts/governance/build_vs_buy_register.yaml`](../../artifacts/governance/build_vs_buy_register.yaml)
  — canonical machine-readable domain register and scoring rubric.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — selected and admitted third-party dependency rows that trace back to
  this register.
- [`/artifacts/governance/third_party_import_register.yaml`](../../artifacts/governance/third_party_import_register.yaml)
  — copied, bundled, or mirrored third-party bytes that may inherit the
  fork and exit posture from a dependency row.
- [`/docs/governance/dependency_review_policy.md`](../governance/dependency_review_policy.md)
  — admission workflow for dependencies and imports.
- [`/docs/governance/fork_review_policy.md`](../governance/fork_review_policy.md)
  — required review bar for protected-path forks, long-lived patch
  stacks, and other deliberate divergence.

## 1. How to use this register

1. Score a new subsystem proposal with the rubric below before selecting
   an upstream or declaring a build-first posture.
2. Record the selected posture in the matching `domain.*` row in
   `build_vs_buy_register.yaml`.
3. When a dependency or import is chosen for that domain, add or update
   the canonical row in the dependency or import register and cite the
   matching `domain.*` row in `build_vs_buy_refs`.
4. If the proposal requires a local fork, long-lived patch stack, or
   behavioral mirror, apply the fork review policy in the same change.

Current repository posture:

- The shell / renderer / editor-core domain is already fully traced to
  protected-path dependency rows in the dependency register.
- Git has a repo-tooling seed row today, but no product-surface Git
  adapter dependency has been admitted yet.
- The remaining launch-critical domains below are intentionally seeded in
  the build-vs-buy register before their first dependency rows exist, so
  future admissions land against a stable governance frame instead of
  inventing one ad hoc.

## 2. Scoring rubric

Score each field on a `0` to `5` scale.

- `0` means the weakest signal for the stated direction.
- `5` means the strongest signal for the stated direction.

| Field | Higher score means | `0` anchor | `3` anchor | `5` anchor |
|---|---|---|---|---|
| `user_facing_differentiation` | stronger case to build | commodity capability users will not notice as implementation-specific | some differentiated UX above standard compatibility expectations | core product reason to exist or trust boundary users directly judge |
| `hot_path_performance_sensitivity` | stronger case to build | off the hot path and tolerant of indirection | visible in interactive flows but not the tightest budget | directly on the protected path with little latency slack |
| `standard_maturity` | stronger case to reuse | no credible common standard or ecosystem contract | usable standard with notable portability gaps | mature, widely understood standard with strong compatibility value |
| `contributor_depth` | stronger case to reuse | one thinly staffed upstream or little review capacity | healthy but shallow bench | broad contributor and reviewer pool with sustainable maintenance |
| `license_notice_risk` | stronger case to build or avoid | permissive, low-friction notice and redistribution posture | manageable obligations with some review complexity | ambiguous, restrictive, or high-overhead notice posture on a critical path |
| `maintainer_health` | stronger case to reuse | orphaned, erratic, or effectively unmaintained | mixed signals, acceptable but fragile | active maintainers, regular releases, and credible issue response |
| `replacement_feasibility` | stronger case to reuse | swap would be extremely expensive or architecture-shaping | adapter boundary exists but replacement would still be disruptive | clean boundary and realistic replacement path |
| `fork_cost` | stronger case to reuse and upstream-first | low-cost temporary patch or no fork pressure | meaningful carrying cost and review burden | high ongoing rebase, audit, and divergence cost |

Decision bands:

- **Build / own** when build pressure is clearly dominant: the
  differentiation and hot-path scores are high, credible upstreams fail
  protected-path needs, and reuse pressure is not strong enough to
  offset the ownership benefit.
- **Reuse / wrap / integrate** when standards maturity, maintainer
  health, contributor depth, and replacement feasibility dominate.
- **Reuse + extend** or **hybrid** when Aureline should preserve a
  standard contract but add differentiated UX, caching, metadata, or
  safety layers above it.
- **Fork only by exception** when a release-critical gap cannot be
  resolved upstream on the required cadence and the fork review policy is
  satisfied in full.

Protected-path gates:

- A high `license_notice_risk` score is a stop sign until the dependency
  and notice posture are made explicit.
- Protected-path choices must cite both the domain row in this register
  and the canonical dependency or import row that carries the actual
  upstream selection.
- Forks that claim compatibility with a standard or upstream contract may
  not silently drift that contract; the divergence must be reviewed,
  named, and retired or re-ratified on cadence.

## 3. Domain register

### Shell / renderer / editor core

- Row id: `domain.shell_renderer_editor_core`
- Owner: `@ahmeddyounis`
- Default posture: `build`
- Preferred upstream / standard inputs: `wgpu` and windowing primitives,
  text shaping and rasterisation primitives, accessibility bridges
- Primary concerns: hot-path latency regression, graphics-driver drift,
  platform accessibility parity, limited maintainer depth on critical
  renderer seams
- Current dependency trace:
  `dep.renderer.wgpu`, `dep.renderer.winit`, `dep.text.rustybuzz`,
  `dep.text.swash`, `dep.text.fontdb`, `dep.accessibility.accesskit`,
  `dep.text.noto_class_fallback_font`, `import.fonts.noto_subset`
- Exit or fork strategy: own the latency-critical shell, editor, and
  rendering loop in-repo; keep third-party seams narrow enough to swap or
  retire a fork within one release family

### Syntax grammars

- Row id: `domain.syntax_grammars`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_curate`
- Preferred upstream / standard inputs: Tree-sitter grammar ecosystem
- Primary concerns: grammar maintenance health, license compatibility,
  injection-query drift, per-language patch creep
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: upstream fixes first; carry only minimal,
  time-boxed local patches and retire them once upstream ships

### Language and debug protocols

- Row id: `domain.language_debug_protocols`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_extend`
- Preferred upstream / standard inputs: LSP, DAP, JSON-RPC where required
- Primary concerns: protocol-version skew, vendor-extension sprawl,
  compatibility drift hidden behind richer UX
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: preserve protocol compatibility, add
  Aureline-owned graph and caching layers above the protocol, and avoid
  protocol forks without ADR-backed cause

### Git execution

- Row id: `domain.git_execution`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_wrap`
- Preferred upstream / standard inputs: Git CLI or mature library
  bindings
- Primary concerns: large-repo edge cases, platform packaging and auth
  behavior, provenance of background Git operations
- Current dependency trace: repo-tooling seed only via `dep.repo.git_cli`
- Exit or fork strategy: wrap semantics behind a replaceable adapter, keep
  behavior truthful to Git, and do not clone Git semantics casually

### Search core

- Row id: `domain.search_core`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_integrate`
- Preferred upstream / standard inputs: ripgrep-compatible search engines
- Primary concerns: Unicode and ignore-file parity, scheduling around the
  protected path, result composition and ranking truth
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: keep the search engine behind an adapter, own the
  orchestration and ranking layers, and swap engines rather than forking
  the engine when feasible

### PTY and terminal parsing

- Row id: `domain.pty_terminal_parsing`
- Owner: `@ahmeddyounis`
- Default posture: `hybrid`
- Preferred upstream / standard inputs: PTY libraries, ANSI and VT
  parsers
- Primary concerns: cross-platform shell quirks, integrated render and
  metadata fidelity, policy hooks around command truth
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: keep parser contracts stable and rendering,
  metadata, and policy surfaces Aureline-owned; time-box any divergence

### Remote environment descriptors

- Row id: `domain.remote_environment_descriptors`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_integrate`
- Preferred upstream / standard inputs: Dev Container spec, SSH,
  container runtimes, OCI-compatible metadata
- Primary concerns: standards drift, local-path and mount identity gaps,
  partial feature parity across hosts
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: preserve compatibility adapters and keep any
  Aureline augmentation separate from upstream descriptor formats

### Extension sandbox

- Row id: `domain.extension_sandbox`
- Owner: `@ahmeddyounis`
- Default posture: `build_on_standards`
- Preferred upstream / standard inputs: Wasm, WIT/component model,
  process isolation
- Primary concerns: capability creep, ABI stability, ambient-privilege
  regressions, opaque host coupling
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: version the WIT worlds, keep the sandbox
  replaceable, and refuse private opaque ABI lock-in for core contracts

### Policy and identity plumbing

- Row id: `domain.policy_identity_plumbing`
- Owner: `@ahmeddyounis`
- Default posture: `build_selectively`
- Preferred upstream / standard inputs: OIDC, SCIM, OpenFeature, Rego and
  open policy models
- Primary concerns: offline behavior, inspectable policy bundles,
  enterprise compatibility, avoiding hidden control-plane coupling
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: keep policy and entitlement bundles text-based,
  exportable, and provider-replaceable behind normalized claims

### Update and provenance stack

- Row id: `domain.update_provenance_stack`
- Owner: `@ahmeddyounis`
- Default posture: `reuse_security_patterns`
- Preferred upstream / standard inputs: TUF, Sigstore, SLSA, SPDX,
  CycloneDX
- Primary concerns: mirror continuity, key rotation, offline verification,
  supply-chain inspectability
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: preserve content-addressed internal abstractions,
  reuse proven signing and provenance patterns, and avoid custom trust
  protocols unless a reviewed gap forces one

### Optional collaboration and control-plane services

- Row id: `domain.collaboration_control_plane_services`
- Owner: `@ahmeddyounis`
- Default posture: `build_selectively`
- Preferred upstream / standard inputs: commodity object storage, queues
  and pub-sub, open auth, regional deployment primitives
- Primary concerns: hidden desktop prerequisites, retention and export
  posture, replacement cost, runaway operational coupling
- Current dependency trace: no dependency rows admitted yet
- Exit or fork strategy: isolate service APIs, keep local degradation
  strong, and forbid hosted services from becoming silent prerequisites for
  core local editing, search, Git, or task workflows
