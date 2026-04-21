# Action origin, target, route, and exposure taxonomy

This document freezes the vocabulary every Aureline surface uses
when it names **where an action was initiated**, **where it runs**,
**which transport path it crosses to get there**, and **which
exposure the outside world sees as a result**. The four vocabularies
are not independent dialects: together they produce the *route
truth* a later command router, shell adapter, CLI surface, AI
tool-call plane, provider adapter, publish pipeline, browser-handoff
launcher, tunnel exposer, and support-export lane project against
**without minting per-surface route fields**.

This taxonomy is a companion to:

- the ADR-0009 execution-context record and scope taxonomy
  ([`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)),
  which owns `target_class`, `scope_class`, the authority envelope,
  and the execution-context provenance record,
- the ADR-0010 connected-provider, browser-handoff, and
  approval-ticket contract
  ([`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)),
  which owns `provider_actor_class`, the mutation-mode set, the
  browser-handoff packet, the approval ticket, and the high-risk
  gating rules,
- the target-discovery, host-boundary, managed-workspace lifecycle,
  notebook-trust, structured-round-trip, and install-review
  taxonomy
  ([`docs/runtime/target_discovery_and_install_review_taxonomy.md`](./target_discovery_and_install_review_taxonomy.md)),
  which owns `target_discovery_confidence_class`,
  `host_boundary_cue_class`, and
  `managed_workspace_lifecycle_state`,
- the command-descriptor and invocation-session packet contract
  ([`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md)),
  which owns `issuing_surface`, `authority_class`,
  `capability_scope_class`, `execution_intent`, and the
  invocation-session envelope every command crosses.

This taxonomy does not redefine those contracts; it names the four
vocabularies every later surface needs so the phrase *"where did
this action run, and who could see it?"* resolves to one packet
every surface quotes. If this document disagrees with any of the
ADRs or contracts above, the ADR / contract wins and this document
MUST be updated in the same change.

Live networking, live provider adapters, live tunnel exposers, live
browser launchers, and live publish pipelines are explicitly out of
scope at this milestone; this taxonomy freezes the labels those
later lanes will honour without inventing parallel route fields.

## Artifacts this taxonomy points at

- [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml)
  — machine-readable matrix that binds every frozen
  `action_origin_class`, `action_target_class`,
  `action_route_class`, `action_exposure_class`,
  `route_change_reason_code`, and `authority_linkage_class` to its
  minimum packet fields, its admissible pairings with the other
  vocabularies, its required redaction class, its default
  export-inclusion posture, and at least one conformance test.
- [`/fixtures/runtime/route_taxonomy_examples/`](../../fixtures/runtime/route_taxonomy_examples/)
  — worked packet fixtures exercising read-only, mutating,
  approval-gated, browser-mediated, wrong-target, and
  route-changed flows.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  — re-exports `target_class`, the scope / authority envelope,
  and the execution-context id every route packet quotes.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — re-exports `destination_class`, `provider_actor_class`, and
  the browser-handoff packet every `browser_handoff_route`
  references.
- [`/schemas/integration/approval_ticket.schema.json`](../../schemas/integration/approval_ticket.schema.json)
  — re-exports the approval-ticket record every
  `approval_gated_route` links to.

## Who reads this taxonomy

- **Command authors** (palette, menu, keybinding help, CLI help,
  AI-tool surface, automation-recipe surface) projecting a single
  route-truth row per invocation rather than re-deriving the
  route per surface.
- **Shell and CLI authors** rendering the "where does this run?"
  disclosure on every confirmation, preview, and transcript line
  without inventing a surface-local label.
- **AI tool-call and companion authors** explaining to the user
  why a tool call landed locally, landed on a remote agent,
  opened a browser tab, or queued a deferred publish.
- **Provider-adapter and publish-pipeline authors** explaining
  which route an action took when it became externally visible,
  which authority admitted the route, and which route-change
  reason the user saw.
- **Support, docs, governance, and evidence-packet authors**
  quoting route truth from the canonical packet rather than
  re-deriving it from logs.

## Canonical owner

The **route-truth emitter** — the surface that owns the
`invocation_session_packet_record` for the action, i.e. the
command router once it lands, and the shell / CLI / AI / recipe
dispatcher in the interim — is the canonical owner of the route
packet. Every other surface **quotes** the packet by
`invocation_session_id` and MUST NOT re-derive its payload.

The route packet is the cross-surface companion to the
invocation-session packet: the invocation-session packet carries
`command_id`, `authority_class`, `capability_scope_class`,
`enablement_decision`, `preview_posture`, `approval_posture`,
`execution_intent`, and `outcome`; the route packet carries
`action_origin_class`, `action_target_class`, `action_route_class`,
`action_exposure_class`, the authority-linkage ref, the
route-change reason, and the export / redaction guidance. Both
packets carry the same `invocation_session_id`; audit, support, and
replay surfaces cross-walk them by id.

## Shared rules every vocabulary obeys

1. **One route-truth packet per invocation.** Every action a
   user, CLI, AI tool call, automation recipe, extension host,
   remote agent, managed control plane, scheduler, or provider
   callback could observe emits exactly one
   `action_route_truth_record` bound to an
   `invocation_session_id`. Downstream surfaces quote the packet;
   they do not re-derive it.
2. **Closed vocabularies.** Every field named below draws from a
   closed vocabulary. Free-form strings in an
   `action_origin_class`, `action_target_class`,
   `action_route_class`, `action_exposure_class`,
   `route_change_reason_code`, or `authority_linkage_class`
   slot are non-conforming.
3. **Typed denial over silent fallback.** When the surface cannot
   resolve origin, target, route, exposure, or authority-linkage
   it emits the `*_unknown_*` token, routes to a repair hook, and
   denies any mutation. A silent "assumed local" or "assumed
   read-only" fallback is non-conforming.
4. **Origin, target, route, and exposure are orthogonal.** The
   same origin can cross different routes to different targets
   with different exposure classes. A `cli_invocation_local`
   origin can produce a `local_rpc_route` to a
   `local_host_target` with `local_only_mutation` exposure, or
   the same origin can ride a `browser_handoff_route` to a
   `system_browser_target` with
   `provider_visible_mutation` exposure. Packet shape MUST keep
   the four fields independently addressable.
5. **Unknown is a first-class value, not a silence.** Unknown
   origin, unknown target, unknown route, unknown exposure, and
   unknown authority-linkage are all typed classes with
   `*_unknown_*` tokens and repair hooks. Silently treating an
   unknown as a known is non-conforming. A surface that cannot
   tell whether a route is local or remote emits
   `heuristic_unknown_route` and `route_unknown_requires_review`
   rather than guessing.
6. **Authority ticket or approval linkage is always named.**
   Every non-read-only route carries an
   `authority_linkage_class` pointing to the approval ticket
   (ADR-0010), browser-handoff packet (ADR-0010), publish-
   evidence packet, remote-agent attach ticket, or supervisor
   repair ticket that admitted the route.
   `no_authority_required_read_only` is legal only when the
   route is genuinely read-only; every mutation, browser handoff,
   publish, or tunnel exposure MUST carry a linkage.
7. **Raw material never rides.** Raw URLs, raw callback bodies,
   raw delegated-token bytes, raw webhook payloads, raw tunnel
   handshake bytes, raw publish-pipeline tokens, and raw
   execution-context env bodies MUST NOT appear in the route
   packet. Opaque refs (`approval_ticket_ref`,
   `browser_handoff_packet_ref`, `publish_evidence_packet_ref`,
   `tunnel_session_ref`, `provider_callback_correlator_ref`)
   cross. Resolution happens at the narrowest projection
   boundary (ADR-0004, ADR-0007).
8. **Route truth is consistent across UI, CLI, support, and
   machine-readable diagnostics.** The same packet is quoted by
   the in-product disclosure copy, the CLI transcript, the
   support export, the mutation-journal entry, the evidence
   packet, and the replay artifact. Surface-local paraphrases
   of the frozen tokens are forbidden; surfaces MAY translate
   the tokens for a locale but MUST carry the canonical token on
   the underlying envelope.

## Vocabulary 1: action origin class

An origin class names **where the action was initiated from**.
Origin is independent of target: a `cli_invocation_local` origin
may run locally, remotely, through a managed control plane, or
through a browser handoff.

### `action_origin_class` (frozen)

| Token                                     | Definition                                                                                                                                                      | Example                                                                                |
|-------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------|
| `user_keystroke_local`                    | Action initiated by a human keystroke, click, or gesture on a local Aureline surface (palette, menu, context menu, in-editor gesture).                          | User presses F12 to go-to-definition.                                                  |
| `user_voice_or_gesture_local`             | Action initiated by a local accessibility-assistive input channel (voice, switch device, eye-tracker) on a local surface.                                       | Voice command "open terminal" dispatched locally.                                      |
| `cli_invocation_local`                    | Action initiated by the local `aureline` CLI, including subcommands, `--help`, and scripted invocations on the local shell.                                     | `aureline env inspect` run in the local terminal.                                      |
| `ai_tool_call_local`                      | Action initiated by an AI tool call resolved against the local Aureline instance (AI surface hosted in-product).                                                | AI assistant requests `editor.format_document` on the local editor.                    |
| `ai_tool_call_managed`                    | Action initiated by an AI tool call landing on a managed workspace (the AI call originates in the managed control plane or the AI-sandbox boundary).            | Managed AI tool call arrives at the managed workspace to apply a patch.                |
| `recipe_or_automation_local`              | Action initiated by a local automation recipe, scaffold, or scheduled task step replaying a prior invocation envelope.                                          | Project scaffold runs `git.init` followed by `editor.open_folder`.                     |
| `extension_host_local`                    | Action initiated by a local extension adapter (language service, linter, refactor extension) on behalf of the user.                                             | Rust-analyzer extension requests `editor.rename_symbol`.                               |
| `remote_agent_request`                    | Action initiated by an Aureline remote-agent attach service acting on a user session that crosses the remote-agent boundary.                                    | Remote-agent-bound tool call dispatches `terminal.broadcast_line` on the attach.       |
| `managed_control_plane_request`           | Action initiated by the managed-workspace control plane (prebuild warmer, idle-suspend controller, quota enforcer) acting on the workspace.                     | Control plane triggers `workspace.pause_for_idle` on a ready instance.                 |
| `provider_callback_inbound`               | Action initiated by a typed provider callback (OAuth callback, publish-success callback, webhook) that completes a previously minted browser-handoff packet.     | OAuth callback arrives after the user completed the browser sign-in.                   |
| `webhook_inbound`                         | Action initiated by an inbound provider webhook (check-run completion, PR review requested, release published) bound to a connected-provider record.             | GitHub webhook for a completed CI run arrives on the inbox.                            |
| `scheduled_trigger`                       | Action initiated by a pre-declared schedule (cron-like, idle-rollover, policy-epoch-roll) rather than a user keystroke or provider event.                       | Idle-rollover timer moves the managed workspace to `idle_suspended`.                   |
| `supervisor_repair_origin`                | Action initiated by the supervisor or recovery controller running a typed repair (capsule drift, capability downgrade, policy restore).                         | Supervisor runs `managed_workspace.retry_warming` after `recovering`.                  |
| `import_or_restore_session`               | Action initiated while importing a profile, restoring a snapshot, replaying a transcript, or applying a rollback checkpoint. Origin is the importer, not a user.| Profile import replays `workspace.open_folder` during onboarding restore.              |
| `unknown_origin_class`                    | Origin could not be resolved to one of the frozen classes; surface MUST route to a repair hook rather than silently attribute the action to the local user.      | Origin-resolver crashed mid-dispatch; packet minted with repair hook.                  |

Rules (frozen):

1. Every packet MUST carry exactly one `action_origin_class`.
   Silent blanks are forbidden.
2. `ai_tool_call_local` and `ai_tool_call_managed` MUST be
   distinguishable; collapsing them into one "ai" origin is
   non-conforming because they cross different host boundaries.
3. `unknown_origin_class` MUST pair with a `repair_hook_ref`; a
   packet that renders `unknown_origin_class` without a repair
   hook is non-conforming.
4. Adding an origin class is additive-minor; repurposing an
   existing origin class is breaking and requires a new decision
   row.

## Vocabulary 2: action target class

A target class names **where the action actually runs**. Target
class quotes the ADR-0009 `target_class` enum where the target is
an execution target, and extends it with the route-specific
targets (browser tab, system browser, native-OS callback, publish
pipeline, tunnel) that are not execution targets in the ADR-0009
sense but still need a typed label.

### `action_target_class` (frozen)

| Token                              | Definition                                                                                                                                                | ADR-0009 target_class it quotes (if any) |
|------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------|
| `local_host_target`                | Action runs on the local host kernel. The canonical local-execution target.                                                                               | `local_host`                             |
| `user_mode_sandbox_target`         | Action runs in a user-mode sandbox on the local kernel (seccomp, AppContainer, macOS Sandbox).                                                            | `local_host` + sandbox profile           |
| `container_local_target`           | Action runs in a local container (image-pinned, host-backed volumes).                                                                                     | `container_local`                        |
| `devcontainer_target`              | Action runs in a declared devcontainer bound to the workspace.                                                                                            | `devcontainer`                           |
| `remote_ssh_target`                | Action runs on an SSH remote (route dependency, reachability, clock-skew flags apply).                                                                    | `remote_ssh`                             |
| `remote_agent_target`              | Action runs on an Aureline remote-agent attach; client scope is `remote_agent`.                                                                           | `remote_agent`                           |
| `managed_workspace_target`         | Action runs on a managed workspace instance.                                                                                                              | `managed_workspace`                      |
| `notebook_kernel_local_target`     | Action runs on a local notebook kernel.                                                                                                                   | `notebook_kernel_local`                  |
| `notebook_kernel_remote_target`    | Action runs on a remote notebook kernel.                                                                                                                  | `notebook_kernel_remote`                 |
| `ai_sandbox_target`                | Action runs inside the AI sandbox / tool-call runtime; tainted-context fences apply.                                                                      | `ai_sandbox`                             |
| `connected_provider_target`        | Action mutates a connected provider (code host, issue tracker, CI, docs portal, release publisher) over its API surface.                                  | n/a (provider endpoint)                  |
| `embedded_webview_target`          | Action renders into or mutates through an embedded webview inside the product (docs preview, rich OAuth prompt inside the trust boundary).                | n/a (in-product webview)                 |
| `system_browser_target`            | Action hands off to the system default browser via a `browser_handoff_packet`; runs outside the product boundary.                                         | n/a (external browser tab)               |
| `native_os_callback_target`        | Action is received by a native-OS callback surface (deep-link handler, protocol handler, OS-level auth sheet).                                            | n/a (OS-level callback)                  |
| `publish_release_target`           | Action lands on a publish / release pipeline target (release publisher, package registry, signed-artifact publisher).                                     | n/a (publish endpoint)                   |
| `tunnel_exposed_target`            | Action lands on or through a reverse / forward tunnel exposing a local port to an external endpoint.                                                      | n/a (tunnel endpoint)                    |
| `bridged_helper_target`            | Action runs on a compatibility-bridged helper (extension compatibility bridge, external helper, legacy language-server bridge).                           | n/a (bridged helper)                     |
| `unknown_target_class`             | Target could not be resolved to one of the frozen classes. Surface MUST deny launch and route to repair.                                                  | n/a                                      |

Rules (frozen):

1. Every packet MUST carry exactly one `action_target_class`.
   Silent blanks are forbidden.
2. A packet whose `action_target_class` quotes an ADR-0009
   `target_class` MUST round-trip to the same id via the
   execution-context record; a surface that renders
   `local_host_target` for an action whose execution-context
   record resolves to `remote_ssh` is non-conforming.
3. A packet whose `action_target_class` is
   `connected_provider_target` MUST pair with a
   `connected_provider_record_id` and the acting
   `provider_actor_class` (ADR-0010). A generic "provider"
   target without an actor class is forbidden.
4. A packet whose `action_target_class` is `system_browser_target`
   MUST reference a `browser_handoff_packet_ref` (ADR-0010). Raw
   URL launches from a protected surface are forbidden.
5. A packet whose `action_target_class` is `unknown_target_class`
   MUST deny launch and MUST carry a `repair_hook_ref`.
6. Adding a target class is additive-minor; repurposing one is
   breaking.

## Vocabulary 3: action route class

A route class names **which transport path the action crosses**
from origin to target. Route is orthogonal to origin and target: a
`user_keystroke_local` origin targeting a `connected_provider_target`
might ride an `approval_gated_route` (local publish with approval
ticket) or a `browser_handoff_route` (open-in-provider) depending
on the mutation mode.

### `action_route_class` (frozen)

| Token                                    | Definition                                                                                                                                                                                     | Example                                                                                  |
|------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------|
| `in_process_route`                       | Action is dispatched entirely inside the local process (no RPC, no IPC). Typical for read-only UI state.                                                                                        | Palette surfaces a disabled-with-reason without leaving the shell.                       |
| `local_rpc_route`                        | Action crosses a local RPC or IPC boundary to an in-product subsystem (workspace VFS, execution resolver, secret broker) on the local host.                                                    | Save-dirty-buffer dispatches through the workspace VFS RPC.                              |
| `remote_rpc_route`                       | Action crosses a remote-agent or SSH RPC boundary to land on a remote target.                                                                                                                  | `task.run.cargo_test` dispatched to a remote-SSH host.                                   |
| `managed_control_plane_route`            | Action crosses the managed-workspace control-plane API (provision, warm, pause, resume, retire, snapshot, quarantine).                                                                         | Control-plane `resume_from_idle` command reaches the managed workspace.                  |
| `remote_agent_attach_route`              | Action rides an Aureline remote-agent attach session; the attach ticket admits the route.                                                                                                      | AI tool call lands on the remote-agent attach.                                           |
| `approval_gated_route`                   | Action is admitted by an ADR-0010 `approval_ticket_record`; ticket is spent at dispatch and referenced on the packet.                                                                           | `git.push_branch` spends a second-party-review ticket and pushes.                        |
| `browser_handoff_route`                  | Action hands off to the system browser via a typed `browser_handoff_packet` rather than running locally.                                                                                       | `open_in_provider` on a PR that the product cannot publish locally.                      |
| `embedded_webview_route`                 | Action rides an in-product embedded webview (docs preview, in-product rich prompt) whose content is hosted inside the product boundary.                                                         | Docs-help surface renders a rich `docs_or_portal_web` anchor inline.                     |
| `native_callback_route`                  | Action routes through a native-OS callback (deep link, protocol handler, OS-level auth sheet); return is received by the native-callback target.                                                | OS-level passkey sheet returns a signed assertion to the product.                        |
| `provider_webhook_return_route`          | Action is admitted by an inbound provider-webhook or callback return bound to a connected-provider record.                                                                                     | Webhook for a completed CI run updates the review overlay.                               |
| `publish_pipeline_route`                 | Action traverses a release / publish pipeline (signing, attestation, publisher, registry).                                                                                                     | Release-publish command runs the pipeline and attaches evidence.                         |
| `tunnel_exposed_route`                   | Action traverses a reverse / forward tunnel exposing a local resource externally.                                                                                                              | `share_local_preview_over_tunnel` exposes a local port to an external URL.               |
| `mirror_or_private_registry_route`       | Action runs through a mirror, private registry, or artifact proxy rather than the canonical provider host.                                                                                     | Extension-install fetches a signed bundle from the workspace's mirror.                   |
| `bridged_helper_route`                   | Action routes through a compatibility bridge (extension compatibility bridge, external helper, legacy-adapter) rather than a first-class Aureline target.                                       | Legacy language-server helper receives `format_document` over the bridge.                |
| `heuristic_unknown_route`                | Route could not be resolved to one of the frozen classes (resolver unavailable, mid-flight route change not yet confirmed); surface MUST deny mutation and route to repair.                     | Route resolver disabled; packet names heuristic_unknown_route and denies.                |

Rules (frozen):

1. Every packet MUST carry exactly one `action_route_class`.
   Silent blanks are forbidden.
2. `approval_gated_route` MUST pair with an
   `approval_ticket_ref` in `authority_linkage_ref`; the ticket
   id MUST match a live ADR-0010 approval ticket whose
   `action_class` admits the route.
3. `browser_handoff_route` MUST pair with a
   `browser_handoff_packet_ref`; raw URL launches are forbidden.
4. `publish_pipeline_route` MUST pair with a
   `publish_evidence_packet_ref`; a publish without evidence is
   non-conforming.
5. `tunnel_exposed_route` MUST pair with a `tunnel_session_ref`
   and a declared external reachability; a tunnel that silently
   widens its reachability is non-conforming.
6. `heuristic_unknown_route` MUST pair with a `repair_hook_ref`
   and MUST deny any mutating action; read-only analysis MAY
   continue only if the packet's
   `action_exposure_class` is `no_side_effect_local_read`.
7. Adding a route class is additive-minor; repurposing one is
   breaking.

## Vocabulary 4: action exposure class

An exposure class names **what outside world sees the action**. A
route can be admitted, authorised, and executed safely and still
create an exposure the user needs to see; exposure is the
"what becomes visible externally" truth independent of route.

### `action_exposure_class` (frozen)

| Token                                     | Definition                                                                                                                                                  | Example                                                                                 |
|-------------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------------------------------------|
| `no_side_effect_local_read`               | Action has no observable side effect outside the local process. No outside party can see the action ran.                                                    | `search.find_in_workspace` against the local index.                                     |
| `local_only_mutation`                     | Action mutates local workspace state only. Mutation-journal entry visible to the local user; no provider, network, or publish footprint.                    | `editor.format_document` saves a formatted file locally.                                |
| `workspace_visible_mutation`              | Action is visible to other Aureline sessions attached to the same workspace (shared worktree, managed workspace, multi-client sync).                        | `workspace.rename_root` touches a managed workspace shared with a colleague's session.  |
| `provider_visible_mutation`               | Action is visible to a connected provider. Provider surfaces, reviewers, assignees, or audit logs can see the mutation.                                     | `git.push_branch` makes a branch visible on the code host.                              |
| `publicly_visible_publish`                | Action publishes artefacts that are visible beyond a single provider (public registry, external package index, public release).                             | `release.publish_to_public_registry` pushes a signed artefact.                          |
| `cross_tenant_visible`                    | Action crosses a tenant boundary (shared infrastructure, cross-org provider link, managed-policy cross-tenant flow); visible in more than one tenant scope. | Policy-injected service identity applies a mutation seen in two tenants.                |
| `third_party_callback_visible`            | Action results in an inbound callback that third parties can observe (webhook delivery, outbound notification, external event subscriber).                  | Publish emits a webhook observable on every configured subscriber.                      |
| `browser_session_visible`                 | Action becomes visible inside the user's browser session (cookies, storage, OAuth state) but does not persist outside the session.                          | OAuth device-code flow completes in the user's browser session.                         |
| `tunnel_exposed_public`                   | Action exposes a local resource over a reverse / forward tunnel whose reachability is declared-public.                                                      | Local preview exposed via an externally-reachable tunnel URL.                           |
| `exposure_unknown_requires_review`        | Exposure could not be resolved; surface MUST deny the action and route to repair rather than guess.                                                         | Packet missing exposure class; surface denies with repair hook.                         |

Rules (frozen):

1. Every packet MUST carry exactly one `action_exposure_class`.
   Silent blanks are forbidden.
2. `no_side_effect_local_read` is the only class that MAY pair
   with `no_authority_required_read_only`. Every other class
   MUST pair with a named authority linkage.
3. An action whose `action_exposure_class` escalates from a
   read-only class to a mutating / provider-visible class
   mid-invocation MUST mint a new packet (the route-change
   case, §Vocabulary 5) rather than silently overwrite the
   exposure label.
4. `publicly_visible_publish`, `cross_tenant_visible`,
   `tunnel_exposed_public`, and `third_party_callback_visible`
   MUST be rendered inline on every confirmation, preview,
   and approval surface; tooltip-only disclosure is
   non-conforming.
5. Adding an exposure class is additive-minor; repurposing one
   is breaking.

## Vocabulary 5: route-change reason codes

A route-change reason names **why the route that ran was not the
canonical one** the user or caller proposed. Route changes
include wrong-target detection, approval-required escalations,
policy narrowing, browser-handoff escalations, managed-control-plane
fallbacks, and repair-hook interventions.

### `route_change_reason_code` (frozen)

| Token                                       | Meaning                                                                                                                                                       |
|---------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `canonical_no_route_change`                 | The route taken matches the route the caller proposed; no change is present. Packet MUST still carry this token rather than leaving the field null.           |
| `route_escalated_to_approval_required`      | Route was upgraded to `approval_gated_route` because the action matched an approval posture (high-risk, publish-class, admin-class).                          |
| `route_escalated_to_browser_handoff`        | Route was upgraded to `browser_handoff_route` because the requested mutation is not reachable through the local product.                                      |
| `route_escalated_to_publish_pipeline`       | Route was upgraded to `publish_pipeline_route` because the action publishes signed artefacts (release, registry).                                             |
| `route_escalated_to_tunnel_exposure`        | Route was upgraded to `tunnel_exposed_route` because the action exposes a local resource externally.                                                          |
| `route_changed_target_unreachable`          | Route changed because the originally requested target is unreachable; fallback target (if any) is named.                                                      |
| `route_changed_target_reassigned`           | Route changed because the target was reassigned (remote-agent migration, managed-workspace successor image, tenant switch).                                    |
| `route_changed_policy_narrowed`             | Route changed because admin policy narrowed the admissible route set (forced `open_in_provider`, forbade a route class, raised an approval floor).             |
| `route_changed_authority_escalation`        | Route changed because the required authority escalated (step-up authenticator required, cross-tenant approval required).                                      |
| `route_changed_wrong_target_detected`       | Resolver detected that the originally-resolved target was not the intended target (toolchain digest drift, capsule drift, wrong managed workspace bound).     |
| `route_changed_approval_withdrawn`          | Route changed because an in-flight approval ticket was revoked or expired; in-flight action fails closed.                                                     |
| `route_changed_freshness_floor_unmet`       | Route changed because the target or publisher freshness floor was unmet; action deferred or denied.                                                           |
| `route_changed_host_mismatch`               | Route changed because the requested host / tenant did not match the grant's `canonical_host` / `tenant_or_org_scope`.                                         |
| `route_unknown_requires_review`             | Route could not be confirmed; surface MUST deny mutation and route to a typed review. Pairs with `heuristic_unknown_route`.                                   |

Rules (frozen):

1. Every packet MUST carry exactly one
   `route_change_reason_code`. When the route was not changed,
   the packet MUST carry `canonical_no_route_change` rather
   than leaving the field null.
2. A route-change reason that upgrades the route class (for
   example, `route_escalated_to_approval_required`) MUST carry
   the matching `authority_linkage_ref` naming the ticket or
   packet that admitted the new route.
3. `route_changed_wrong_target_detected` MUST carry both the
   originally-resolved target (under a `prior_target_ref` slot)
   and the corrected target; dropping the prior target is
   non-conforming.
4. `route_changed_approval_withdrawn` MUST fail closed; the
   packet's `outcome_class` (from the invocation-session
   envelope) MUST render `denied_by_approval` or
   `denied_authority_withdrawn`.
5. Adding a reason code is additive-minor; repurposing one is
   breaking.

## Vocabulary 6: authority-linkage class

An authority-linkage class names **which authority object admits
the route**. Every non-read-only route carries exactly one linkage;
read-only local analysis may use
`no_authority_required_read_only`.

### `authority_linkage_class` (frozen)

| Token                                       | What it references                                                                                                                                                 |
|---------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `no_authority_required_read_only`           | The route is read-only local analysis; no authority object is required. Legal only when `action_exposure_class = no_side_effect_local_read`.                       |
| `local_user_keystroke_authority`            | The route is admitted by the local user's immediate keystroke / gesture, constrained by workspace trust state (ADR-0001).                                          |
| `approval_ticket_linked`                    | Admitted by an ADR-0010 `approval_ticket_record`; the ticket id is named in `authority_linkage_ref.approval_ticket_ref`.                                           |
| `browser_handoff_packet_linked`             | Admitted by an ADR-0010 `browser_handoff_packet_record`; packet id is named in `authority_linkage_ref.browser_handoff_packet_ref`.                                 |
| `publish_evidence_packet_linked`            | Admitted by a publish-evidence packet (signing attestation, release-council attestation); evidence ref is named.                                                   |
| `remote_agent_attach_ticket_linked`         | Admitted by a remote-agent attach ticket; attach ticket id is named.                                                                                               |
| `managed_control_plane_token_linked`        | Admitted by a managed control-plane token (provision, warm, pause, resume, retire, snapshot, quarantine). Token ref is named; raw token bytes never cross.         |
| `policy_injected_service_identity_linked`   | Admitted by an ADR-0010 policy-injected service identity; identity id and policy epoch are named.                                                                  |
| `supervisor_repair_ticket_linked`           | Admitted by a supervisor repair ticket (capsule drift recovery, capability downgrade rollback, managed workspace reactivation).                                    |
| `authority_missing_denied`                  | No admissible authority object is live; surface MUST deny the action with a typed repair hook.                                                                     |

Rules (frozen):

1. Every packet MUST carry exactly one
   `authority_linkage_class`. Silent blanks are forbidden.
2. `no_authority_required_read_only` is legal only when the
   packet's `action_exposure_class` is
   `no_side_effect_local_read` and the `action_route_class` is
   `in_process_route` or `local_rpc_route` against a
   read-only authority.
3. A packet whose `authority_linkage_class` is
   `authority_missing_denied` MUST render the action's
   invocation-session `outcome_class` as
   `denied_by_authority` (or the matching disabled-reason on
   the invocation packet) and MUST carry a
   `repair_hook_ref`.
4. Adding a linkage class is additive-minor; repurposing one is
   breaking.

## Packet shape

The canonical route-truth packet shape is
`action_route_truth_record`. It is a companion record to the
invocation-session packet (ADR-0033 / command-descriptor contract)
and shares `invocation_session_id`.

```yaml
action_route_truth_record:
  schema_version: 1
  packet_id: "<opaque>"
  invocation_session_id: "<opaque>"           # cross-walks invocation-session packet
  command_id: "<opaque>"                       # quoted from invocation-session packet
  command_revision_ref: "<opaque>"

  # Vocabulary 1 — where the action was initiated
  action_origin_class: "<action_origin_class>"
  action_origin_ref:
    origin_actor_class: "<provider_actor_class | null>"   # when provider callback or webhook
    origin_subject_ref: "<opaque | null>"
    origin_session_ref: "<opaque | null>"

  # Vocabulary 2 — where the action runs
  action_target_class: "<action_target_class>"
  action_target_ref:
    execution_context_id: "<opaque | null>"               # ADR-0009 id when target is an execution target
    connected_provider_record_id: "<opaque | null>"        # when connected_provider_target
    browser_handoff_packet_ref: "<opaque | null>"          # when system_browser_target or native_os_callback_target
    publish_evidence_packet_ref: "<opaque | null>"         # when publish_release_target
    tunnel_session_ref: "<opaque | null>"                  # when tunnel_exposed_target
    bridged_helper_ref: "<opaque | null>"                  # when bridged_helper_target
    managed_workspace_instance_ref: "<opaque | null>"      # when managed_workspace_target

  # Vocabulary 3 — which route is taken
  action_route_class: "<action_route_class>"
  host_boundary_cue_stack:                                 # re-exports taxonomy Vocabulary 2
    - "<host_boundary_cue_class>"
  target_discovery_packet_ref: "<opaque | null>"           # re-exports taxonomy Vocabulary 1 when applicable

  # Vocabulary 4 — what exposure the action creates
  action_exposure_class: "<action_exposure_class>"
  exposure_disclosure_summary: "<required human-legible paragraph>"

  # Vocabulary 5 — whether the route changed and why
  route_change_reason_code: "<route_change_reason_code>"
  prior_target_ref: "<opaque | null>"                      # required on route_changed_*
  prior_route_class: "<action_route_class | null>"         # required on route_changed_*

  # Vocabulary 6 — which authority admits the route
  authority_linkage_class: "<authority_linkage_class>"
  authority_linkage_ref:
    approval_ticket_ref: "<opaque | null>"
    browser_handoff_packet_ref: "<opaque | null>"
    publish_evidence_packet_ref: "<opaque | null>"
    remote_agent_attach_ticket_ref: "<opaque | null>"
    managed_control_plane_token_ref: "<opaque | null>"
    policy_injected_service_identity_ref: "<opaque | null>"
    supervisor_repair_ticket_ref: "<opaque | null>"

  # Policy / envelope / redaction
  policy_context:
    identity_mode: "<identity_mode>"                        # ADR-0001
    policy_epoch: <integer>                                 # ADR-0008
    trust_state: "<trust_state>"                            # ADR-0001
    execution_context_id: "<opaque | null>"                 # ADR-0009
  redaction_class: "<redaction_class>"                      # ADR-0007 / ADR-0010
  export_inclusion_posture: "<export_inclusion_posture>"    # see §Export / redaction guidance

  # Repair and audit
  repair_hook_ref: "<opaque | null>"                        # required when origin / target / route / exposure / linkage is unknown_* or authority_missing_denied
  audit_event_refs:                                         # see §Audit events
    - "<opaque>"
  evidence_refs:
    - "<opaque>"
  freshness_class: "<freshness_class>"                      # ADR-0011
  minted_at: "<monotonic_timestamp>"
```

## Export / redaction guidance

Route-truth packets cross many surfaces. The redaction class
controls what each surface may include; raw tokens / raw URLs /
raw callback bodies / raw webhook payloads / raw tunnel handshakes
are **never** carried on any surface.

### `export_inclusion_posture` (frozen)

| Posture                            | Default inclusion                                                                                                                                                    |
|------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `metadata_safe_default`            | `action_origin_class`, `action_target_class`, `action_route_class`, `action_exposure_class`, `route_change_reason_code`, `authority_linkage_class`, `packet_id`, `invocation_session_id`, `policy_context` counts, `exposure_disclosure_summary`, `freshness_class`, `redaction_class`. Raw ids hashed / aliased per ADR-0007. |
| `operator_only_restricted`         | `metadata_safe_default` plus non-hashed linkage refs (`approval_ticket_ref`, `browser_handoff_packet_ref`, `publish_evidence_packet_ref`, `tunnel_session_ref`, `managed_control_plane_token_ref`, `supervisor_repair_ticket_ref`). Raw URLs / raw tokens still excluded.                                                  |
| `broadened_capture_opt_in`         | Above plus `prior_target_ref` and `prior_route_class` for route-changed cases, and execution-context env hashes. Explicit opt-in recorded on the provenance record; ADR-0007 redaction scan runs before packaging.                                                                                                        |

Per-surface inclusion (frozen defaults):

| Surface                              | Default posture                       | Notes                                                                                                                                               |
|--------------------------------------|---------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| Product UI (route-truth disclosure)  | `metadata_safe_default`               | `exposure_disclosure_summary` rendered inline; frozen tokens rendered with their canonical labels; surface-local paraphrases forbidden.             |
| CLI transcript                       | `metadata_safe_default`               | Transcript line carries frozen tokens; raw URLs and raw tokens never included.                                                                      |
| Support bundle                       | `operator_only_restricted`            | Operator-class bundle; raw URLs / raw callback bodies excluded even under this class.                                                               |
| Mutation-journal entry               | `metadata_safe_default`               | Entry names packet_id + invocation_session_id + linkage refs only; raw material excluded.                                                           |
| Save manifest (ADR-0006)             | `metadata_safe_default`               | Same as mutation-journal entry.                                                                                                                     |
| Evidence packet (publish lane)       | `operator_only_restricted`            | Evidence references `publish_evidence_packet_ref` and frozen tokens; signer identity and signing-authority class included; raw tokens excluded.     |
| Replay / timeline capture            | `metadata_safe_default`               | Imported and replayed frames carry packet ids only; raw material excluded at capture and may not be promoted on replay.                             |
| AI context capture                   | `metadata_safe_default`               | Frozen tokens plus `exposure_disclosure_summary`; raw URLs / raw tokens / raw callback bodies never captured.                                       |
| Crash dump                           | `broadened_capture_opt_in`            | Opt-in only; redaction scan precedes packaging; denied by default for any route whose exposure class is `publicly_visible_publish`, `cross_tenant_visible`, or `tunnel_exposed_public`. |
| Clipboard projection                 | `metadata_safe_default`               | Frozen tokens copyable; raw URLs / raw tokens follow ADR-0007 reveal-on-demand posture.                                                             |

Admin policy MAY narrow further (drop `operator_only_restricted`
fields, pin the posture to `metadata_safe_default`, forbid
`broadened_capture_opt_in`). It MAY NOT widen beyond the frozen
exclusion rules.

## Mapping notes (UI, CLI, docs, support, machine-readable diagnostics)

1. **In-product disclosure copy.** Every preview, approval, and
   confirmation surface renders:
   - the `action_target_class` (and the
     `exposure_disclosure_summary`),
   - the `action_route_class` (plus the
     `route_change_reason_code` when it is not
     `canonical_no_route_change`),
   - the `authority_linkage_class` (plus the humanised linkage
     ref when non-null).
2. **CLI transcript.** The canonical CLI envelope includes a
   `[route] origin=<...> target=<...> route=<...>
   exposure=<...> authority=<...> reason=<...>` line per
   invocation. The same tokens appear on `aureline env inspect`,
   `aureline doctor --explain`, and `aureline cmd --dry-run`.
3. **Docs.** Docs references the frozen tokens by their exact
   spelling (no cute labels, no translations) and links from
   each token to this document. Surface-local paraphrases MUST
   quote this document as the source of truth.
4. **Support export.** The support-bundle row includes frozen
   tokens under `operator_only_restricted` by default; counts
   of route-change reason codes aggregate over a window.
5. **Machine-readable diagnostics.** Every observable event
   (audit stream, metrics emitter, replay artifact) carries the
   frozen tokens and the packet_id; readers MUST NOT invent
   parallel tokens. A tool that cannot parse the frozen
   vocabulary is expected to emit `unknown_*` rather than
   silently mapping it to a known token.

## Audit events (frozen)

Every observable route-truth action emits a structured event on
the `action_route_truth` stream. Events carry
`invocation_session_id`, `packet_id`, the four vocabulary
classes, the `route_change_reason_code`, the
`authority_linkage_class`, the typed reason where relevant, and
the policy context. Events MUST NOT carry raw URLs, raw tokens,
raw callback bodies, or raw webhook payloads.

| Event id                                   | Fires when                                                                                                                                 |
|--------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| `action_route_packet_minted`               | A route-truth packet is minted against an invocation-session id.                                                                           |
| `action_route_packet_updated`              | A route-truth packet was updated mid-invocation (route change, wrong-target detection, exposure re-assessment). New packet_id is minted.    |
| `action_route_changed`                     | Route changed from the proposed route class. Names `prior_route_class` and `route_change_reason_code`.                                     |
| `action_route_target_corrected`            | Wrong-target detected; corrected target and prior target named.                                                                            |
| `action_route_exposure_upgraded`           | Exposure class escalated (for example, `local_only_mutation` → `provider_visible_mutation`); new packet minted.                            |
| `action_route_authority_missing_denied`    | Route denied because no admissible authority object is live.                                                                                |
| `action_route_unknown_requires_review`     | Route could not be confirmed; surface routes to review.                                                                                    |
| `action_route_export_posture_widened`      | `export_inclusion_posture` widened beyond `metadata_safe_default`; records broadened_capture opt-in.                                        |
| `action_route_schema_version_bumped`       | This taxonomy's `action_route_truth_schema_version` was bumped.                                                                            |

## Worked scenarios (coverage)

The fixtures in
[`/fixtures/runtime/route_taxonomy_examples/`](../../fixtures/runtime/route_taxonomy_examples/)
cover:

- **Read-only local** — a local search dispatched in-process with
  `no_side_effect_local_read` exposure and
  `no_authority_required_read_only` linkage.
- **Local mutating** — a local editor save crossing a local RPC
  route with `local_only_mutation` exposure and
  `local_user_keystroke_authority` linkage.
- **Approval-gated provider publish** — a code-host push crossing
  an `approval_gated_route` to a `connected_provider_target`
  with `provider_visible_mutation` exposure and
  `approval_ticket_linked` linkage.
- **Browser-mediated OAuth return** — a `provider_callback_inbound`
  origin arriving through a `provider_webhook_return_route` to an
  `embedded_webview_target`, closing out a previously-minted
  `browser_handoff_packet`.
- **Wrong-target detected** — the resolver mints a packet
  against one managed-workspace instance, detects drift
  mid-invocation, and emits a route-updated packet with
  `route_changed_wrong_target_detected` naming both the prior
  and corrected targets.
- **Route changed to browser handoff** — a user-proposed local
  publish is upgraded to `browser_handoff_route` because the
  mutation is not reachable through the local product; linkage
  carries the `browser_handoff_packet_ref` and the original
  `prior_route_class` is preserved.

Together these fixtures cover every **read-only**, **mutating**,
**approval-gated**, and **browser-mediated** acceptance point
named in the deliverables plus at least one wrong-target /
route-changed case.

## Where related decisions live

- Execution-context record, target-identity taxonomy, scope /
  authority envelope:
  [`docs/adr/0009-execution-context-and-scope.md`](../adr/0009-execution-context-and-scope.md)
  and
  [`docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md).
- Connected-provider actor classes, browser-handoff packet,
  approval ticket, high-risk gating, grant-resolution reasons,
  denial posture, audit events:
  [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
- Target-discovery confidence, host-boundary cues, managed-
  workspace lifecycle, notebook-trust ladder, structured
  round-trip risk, install-review summary slots:
  [`docs/runtime/target_discovery_and_install_review_taxonomy.md`](./target_discovery_and_install_review_taxonomy.md).
- Command-descriptor and invocation-session packet contract:
  [`docs/commands/command_descriptor_contract.md`](../commands/command_descriptor_contract.md).
- Identity modes and workspace trust:
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- RPC transport (carries route-truth packets between processes):
  [`docs/adr/0004-rpc-transport-and-schema-toolchain.md`](../adr/0004-rpc-transport-and-schema-toolchain.md).
- Subscription envelope and authority-class matrix:
  [`docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`](../adr/0005-subscription-envelope-and-invalidation-semantics.md).
- Save / cache / mutation-journal contract (carries
  `authority_linkage_ref`):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- Secret broker and redaction defaults:
  [`docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md).
- Settings resolver and policy-epoch semantics:
  [`docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md).
- Capability lifecycle and dependency markers (freshness classes,
  downgrade rule):
  [`docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md).

## Future lanes that consume this taxonomy

These lanes will read this taxonomy mechanically rather than mint
parallel route vocabularies:

- Command router, shell adapter, and CLI surface (route-truth
  disclosure on every dispatch).
- AI tool-call plane (route-truth disclosure on every tool call;
  AI-initiated mutations without a linked authority ticket deny
  with `authority_missing_denied`).
- Connected-provider adapters (review, issue, CI, release, docs,
  identity, AI, managed-registry, importer) binding
  `connected_provider_target` + `approval_gated_route` /
  `browser_handoff_route` / `publish_pipeline_route`.
- Browser-handoff launcher and callback receiver (every launch
  mints a `browser_handoff_route` packet; every callback
  validates against the packet before mutating local state).
- Publish pipeline and release publisher (every publish mints a
  `publish_pipeline_route` packet; evidence packet ref attached).
- Tunnel exposer (every exposure mints a `tunnel_exposed_route`
  packet and declares public reachability inline).
- Managed-workspace control plane (every control-plane action
  mints a `managed_control_plane_route` packet with the
  control-plane token ref).
- Support-export, mutation-journal, replay, and evidence-packet
  exporters (quote packets by id rather than re-deriving route
  truth).

## Change management

- Adding a new token to `action_origin_class`,
  `action_target_class`, `action_route_class`,
  `action_exposure_class`, `route_change_reason_code`,
  `authority_linkage_class`, `export_inclusion_posture`, or the
  audit-event set is **additive-minor**: bump the taxonomy's
  `action_route_truth_schema_version`, extend the YAML matrix
  at
  [`/artifacts/runtime/action_origin_target_labels.yaml`](../../artifacts/runtime/action_origin_target_labels.yaml),
  and extend the relevant boundary schema when one lands.
- **Repurposing** any existing token (reusing an existing
  `action_route_class` for a different transport, reusing an
  existing `action_exposure_class` for a different exposure,
  collapsing two `route_change_reason_code` values into one) is
  breaking and requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- **Renaming** a token is done through an alias mechanism
  (mirrors the ADR-0008 alias discipline); never by mutating the
  canonical id.
- **Relating a token to an ADR-0009 / ADR-0010 / ADR-0011 /
  ADR-0012 vocabulary** must go through the other ADR or
  contract first; this taxonomy re-exports those vocabularies,
  it does not mint parallel ones.
