# Package-Manager, Registry, And Lockfile-Safety Contract

This document freezes the execution-facing contract for package-manager
plans before live package integrations exist. It composes with the
package-action contract rather than replacing it: the package UI,
command router, dependency ledger, CI/provider importers, support
exports, and policy surfaces all read one package-change plan family
before install, update, remove, audit, or fix flows can write files or
touch the network.

Machine-readable companions:

- [`/schemas/execution/package_change_plan.schema.json`](../../schemas/execution/package_change_plan.schema.json)
  defines `package_change_plan_record`.
- [`/schemas/execution/registry_source.schema.json`](../../schemas/execution/registry_source.schema.json)
  defines `registry_source_record`.
- [`/fixtures/execution/package_manager_cases/`](../../fixtures/execution/package_manager_cases/)
  contains worked package-manager plans and registry-source records for
  local, container, CI/provider, managed, and support projections.

Related contracts:

- [`/docs/package/package_action_contract.md`](../package/package_action_contract.md)
  owns the package-action, review-packet, registry-auth, script-risk,
  lockfile-impact, transitive-impact, and rollback vocabularies used by
  package UI actions.
- [`/docs/commands/invocation_result_and_parity_contract.md`](../commands/invocation_result_and_parity_contract.md)
  owns command result packets that package changes must link after
  preview, denial, apply, rollback, or scheduled execution.
- [`/docs/execution/run_and_attempt_contract.md`](./run_and_attempt_contract.md)
  owns run / attempt identity for execution-shaped package work.
- [`/docs/execution/task_event_and_evidence_contract.md`](./task_event_and_evidence_contract.md)
  owns task-event, evidence-link, provider-overlay, imported-CI, and
  support-bundle linkage.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  owns generated-artifact posture used by structured lockfiles,
  resolved manifests, codegen outputs, and sidecar downloads.

Normative source material includes `.t2/docs/Aureline_PRD.md`,
`.t2/docs/Aureline_Technical_Architecture_Document.md`,
`.t2/docs/Aureline_Technical_Design_Document.md`, and the package /
extension install-review governance in the planning documentation. If
those sources disagree with this document or its schemas, the upstream
source wins and all companion artifacts update together.

## Scope

Frozen here:

- one package-change plan shape for `install`, `update`, `remove`,
  `audit`, and `fix` flows;
- one registry-source record that preserves source, mirror, auth,
  network, freshness, and remapping truth before write or network
  actions occur;
- one review-sheet field set for lifecycle scripts, postinstall hooks,
  native builds, mirror usage, auth source, network egress, generated
  artifact fallout, package-manager caches, sidecar downloads, and
  workspace scripts;
- no-hidden-mutation guards for lockfiles, caches, sidecar downloads,
  workspace scripts, and registry remapping across local, container,
  remote, managed, CI/provider, and support projections; and
- linkage to dependency ledgers, policy blocks, support exports,
  command result packets, package-action records, package-review
  packets, run / attempt records, and network attribution records.

Out of scope:

- invoking package-manager CLIs or language-specific resolvers;
- building package-manager UI;
- parsing real lockfiles or package-manager cache layouts; and
- implementing vulnerability, license, or advisory intelligence.

## Package-Change Plan Minimum

Every pre-apply package flow emits one `package_change_plan_record`.
Audit-only flows emit the same record with read-only posture so support
and CI can compare it with mutating plans.

| Field family | Required truth |
| --- | --- |
| Plan identity | Stable plan id, action class, package-manager family, issuing surface, actor, command, and change-visibility class. |
| Target context | Local, container, remote, managed, CI/provider, or support projection; execution-context ref; target identity; workspace scope; trust state; policy epoch. |
| Manifest scope | Manifest scope class, manifest locator, member/slice ref, package coordinate refs, and manifest-delta class. |
| Registry source | Registry-source record ref plus source class, mirror/remapping class, auth source, and network-egress class. |
| Advisory / license delta | Advisory and license delta classes, affected package count, severity/finding refs, and suppression or waiver refs where relevant. |
| Lockfile impact | Lockfile-impact class, prior/proposed lockfile snapshot refs, resolver identity, entry count buckets, and generated-artifact posture. |
| Review sheet | Script, postinstall, native build, mirror, auth, egress, generated-artifact, cache, sidecar, and workspace-script fields. |
| Rollback / checkpoints | Rollback posture, lockfile checkpoint, workspace snapshot, cache checkpoint, generated-artifact checkpoint, and rollback blocker note. |
| Linkage | Dependency-ledger refs, policy-block refs, support-export refs, command-result packet refs, package-action/review-packet refs, run/attempt refs, evidence refs, and network attribution refs. |

Package changes MUST NOT be represented by a generic “dependency
update” label when any of these fields is known. The plan's
`change_visibility_classes` must name the visible consequences:
manifest/lockfile delta, script/native-build delta, advisory/license
delta, registry/auth delta, read-only audit, or blocked unknowns.

## Review Sheet

The review sheet is the user/admin/support-facing projection of the
plan. It is schema-bound so CLI, CI, provider, hosted-review, support,
and package UI surfaces can compare the same packet.

Required review-sheet fields:

| Field | Rule |
| --- | --- |
| `lifecycle_scripts` | Names whether package lifecycle scripts are absent, sandboxed, unsandboxed pending consent, policy blocked, or unknown. Script descriptor refs are opaque; raw script bodies are forbidden. |
| `postinstall_hooks` | Names postinstall hook posture separately from generic lifecycle scripts so postinstall risk cannot be hidden in a lockfile update. |
| `native_builds` | Names whether no build, local toolchain build, container build, prebuilt binary, provider-managed build, or unknown native work is expected. |
| `mirror_usage` | Names direct, mirror-pinned, offline-bundle, provider-injected, or remapped registry use, and cites upstream and mirror refs where known. |
| `auth_source` | Names no-auth, broker handle, delegated identity, policy-injected credential, managed service identity, mTLS, device flow, raw-secret observed, unsupported, or unknown. |
| `network_egress` | Names no-network, public internet, organization-approved external, mirror-only, provider-managed, deny-all blocked, or unknown egress. |
| `generated_artifact_fallout` | Names whether generated outputs are absent, lockfile-only, cache-only, sidecar downloads, generated code/docs/config, or unknown. |
| `package_manager_cache` | Names cache read/write posture so package-manager caches cannot mutate invisibly. |
| `sidecar_downloads` | Names sidecar downloader posture and count buckets. |
| `workspace_scripts` | Names workspace script execution posture separately from package lifecycle scripts. |

The review sheet MUST render before the first write or outbound network
attempt for mutating plans. Read-only projections from CI/provider or
support exports MUST preserve the same field names and enum values even
when the originating system only supplied partial evidence.

## No-Hidden-Mutation Rules

Every plan carries `no_hidden_mutation_guards`. All guard booleans are
schema-const `true` so a plan cannot claim compliance while leaving a
class unreviewed.

Required guards:

- `lockfile_diff_precomputed` — prior/proposed lockfile refs or an
  explicit unknown/review-required class are present before apply.
- `package_cache_writes_declared` — cache writes, cache reads, or
  cache bypasses are declared before the package manager runs.
- `sidecar_downloads_declared` — sidecar downloads are declared even
  when performed by scripts, build tools, or provider runners.
- `workspace_scripts_declared` — workspace scripts and package-manager
  scripts are separated and reviewed.
- `registry_remapping_declared` — direct-to-mirror, public-to-private,
  provider-injected, offline-bundle, or config override remapping is
  visible.
- `auth_source_declared` — auth source differences cannot be hidden
  behind the registry source label.
- `network_egress_declared` — endpoint class and network egress class
  are available before the network path is opened.
- `generated_artifact_fallout_declared` — generated or derived files
  are classified before they appear as ordinary source edits.
- `target_context_bound_before_network` — local/container/remote/
  managed/CI/support target context is resolved before egress.
- `cross_surface_comparison_ready` — local, CI, provider, and support
  projections can compare the same plan fields.

If any guard cannot be satisfied, the plan resolves to
`blocked_pending_review` or `denied_by_policy_or_trust` and the
surface links the policy block or denial packet instead of applying.

## Registry Source

A `registry_source_record` is the source-truth companion to package
change plans. It preserves:

- registry source class and ecosystem;
- public/private/mirror/offline/vendored/VCS source refs;
- upstream and mirror identity refs;
- freshness, revocation, and content-trust refs;
- auth source and credential-handle refs;
- network egress class, route class, proxy/trust refs, and network
  attribution refs;
- registry remapping class, config-source ref, prior/proposed source
  refs, and review requirement; and
- visibility booleans requiring the source to be visible before write
  and before network.

Raw registry URLs, hostnames, IPs, tokens, certificate material,
absolute paths, lockfile bodies, and manifest bodies never cross this
boundary. Records carry opaque refs, class labels, timestamps, counts,
and reviewable summaries only.

## Lockfiles And Generated Artifacts

Lockfiles and resolved manifests are generated or derived artifacts in
the architecture vocabulary. A package-change plan therefore never
claims a lockfile mutation is a source edit unless a round-trip-safe
editor has been explicitly declared by the owning artifact contract.

Rules:

- Install, update, remove, and fix plans MUST carry a non-read-only
  lockfile-impact class or an explicit `unknown_requires_review`
  posture before apply.
- Audit-only plans MUST use an inspection-only lockfile class and a
  read-only rollback posture.
- Resolver identity and environment factors MUST be cited through refs
  when lockfile output is proposed.
- Generated artifact fallout MUST include generated code, docs, config,
  cache, and sidecar outputs when those are expected side effects.
- Regenerate-or-review is preferred when lockfile round-trip safety is
  unproven.

## Linkage And Attribution

Package changes are attributable outside the package UI only if the
plan links outward. Every plan therefore includes a `linkage` block
with typed refs for:

- dependency ledgers and package inventory rows;
- policy blocks, authority tickets, approval tickets, and waivers;
- command result packets and rollback handles;
- package-action records and package-review packets;
- run, attempt, task-event, evidence-link, and network-attribution
  records; and
- support exports and redaction/export review refs.

Support exports, CI/provider projections, hosted review, admin audit,
and CLI/headless output MUST use these refs rather than reconstructing
package truth from UI prose or raw package-manager logs.

## Fixture Coverage

The fixture corpus covers:

- a local install with sandboxed lifecycle script and lockfile entries;
- a container update where registry remapping and mirror-only egress are
  visible before write/network;
- a CI/provider fix plan that links advisory/license deltas and command
  result packets;
- a managed remove plan blocked because rollback/checkpoint evidence is
  missing;
- a support audit projection that is read-only but comparable to local
  and provider plans; and
- a registry-source record for a managed mirror with policy-injected
  auth and source remapping.

Removing one of these coverage classes is a breaking change to the
pre-implementation contract corpus.
