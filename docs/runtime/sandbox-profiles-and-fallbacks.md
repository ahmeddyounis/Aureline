# Runtime Sandbox Profiles And Fallbacks

This contract is the stable source for runtime isolation claims. Runtime
surfaces read the packet at
`artifacts/runtime/m4/sandbox_profile_backend_truth_packet.json` and the schema
at `schemas/runtime/sandbox-profile.schema.json`; they do not invent profile,
backend, fallback, or approval-lineage wording locally.

## Stable Profiles

| Profile ID | Runtime family | Filesystem | Network | Secrets | Child processes | Trust requirement |
|---|---|---|---|---|---|---|
| `read_only_analyzer_v1` | parsers, analyzers, import validators | workspace/workset read-only | none by default | none | denied | untrusted or trusted |
| `repo_task_v1` | build, test, lint, formatter, repo tasks | workspace read plus declared output sinks | declared network class only | brokered handles only | derived envelopes only | trusted workspace or explicit override |
| `interactive_terminal_v1` | local/remote PTY | user-selected runtime paths | PTY boundary with policy-aware egress | selected projections shown in inspector | user-driven PTY children | trusted workspace for repo shell mutation |
| `debug_attach_v1` | debugger/profiler attach | target-scoped source and symbol stores | target contract only | target-needed projections | claimed platform only | trusted workspace plus approval |
| `notebook_kernel_v1` | notebook kernels and REPLs | notebook/data/export sinks | declared endpoint classes | session-scoped projections | kernel contract only | trusted notebook/workspace and trust class |
| `bootstrap_scaffold_v1` | template/scaffold/bootstrap hooks | side worktree before apply | bootstrap/mirror network only | explicit projections only | no daemons; derived envelopes only | trusted template and preview/apply approval |
| `ai_tool_mutator_v1` | AI and recipe mutators | reviewed target and export sinks | approved tool/endpoint set | ticket-named projection set | nested derived envelopes only | reviewed plan and approval ticket |
| `data_connector_v1` | DB/cloud/infra connectors | connector cache and export path | declared endpoint or tunnel only | connection-scoped handles | adapter contract only | trusted connector and endpoint approval |

## Approval Binding

Approval tickets and capability envelopes are separate objects. Tickets record
who approved the action, for which actor, surface, action class, target,
workspace scope, sandbox profile, capability hash, policy epoch, expiry, revoke
state, and audit lineage. Envelopes record what the runtime can actually do.
Every approved envelope references the ticket that authorized it.

Remembered approvals are not bearer credentials. Destructive, networked,
provider-backed, remote, secret-bearing, privileged, or trust-changing actions
must mint a fresh short-lived ticket at use time. Target drift, policy drift,
version drift, authority drift, sandbox/capability drift, expiry, or revocation
forces reapproval before execution.

Runtime inspectors, command diagnostics, support exports, and approval-history
rows must show the same lineage: actor, issuing surface, action class, scope,
target, profile, capability hash, policy epoch, expiry, revoke state, audit refs,
and the revalidation trigger. Raw secret bodies and raw command bodies are not
included.

## Backend Fallback

Claimed stable execution can only publish one of these outcomes:

| Outcome | Meaning |
|---|---|
| `enforced` | The requested profile is enforced by the published backend. |
| `stricter_downgrade` | A narrower profile is enforced and disclosed. |
| `unsupported` | The capability is unavailable on that backend. |
| `fail_closed` | Launch is refused instead of widening authority. |

Unsupported or incomplete backends never fall back to ambient full-user
execution on a lane marketed as sandboxed or isolated. Browser companion
surfaces have no local-device execution backend; mutating or credentialed work
must use an approved remote or managed backend with the same ticket and envelope
semantics.
