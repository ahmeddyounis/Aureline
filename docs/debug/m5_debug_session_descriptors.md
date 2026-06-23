# M5 debug-session and attach-target descriptors

This contract materializes two of the governed debugger object families that the
[M5 debug-contracts matrix](./m5_debug_contracts.md) names — the **debug session**
and the **attach target** — as concrete, typed, serde-serializable descriptors. It
is the canonical M5 source every debugger-capable surface reads to explain *what was
launched or attached, against which target, with what current authority and adapter
posture*. Notebook, profiler, incident, support, AI, and core debug surfaces consume
these descriptors directly instead of minting surface-local session objects.

Authoritative product anchors:

- `.t2/docs/Aureline_Technical_Design_Document.md` §7.6.10.1–§7.6.10.4 and §9.46 on
  debug launch/session, breakpoints, variables/watches, evaluate side-effect
  governance, chronology capture, replay, and notebook-debug parity.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §14.5 on debug session headers, frame
  mapping, variables/watch panes, evaluate/REPL review, and dump/source-map truth.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` `FR-DEBUG-001` and the debug
  surface rules on stable breakpoints, variables, stack views, chronology cues, and
  artifact-linked evidence.

This lane composes with the live debug-session lifecycle records already frozen in
`crates/aureline-runtime/src/debug/` and the adapter-negotiation truth in
`crates/aureline-runtime/src/stabilize_debugger_host_and_adapter_negotiation/`; it
keeps the *reviewed descriptor model* those producers emit and every surface reads.

## The two descriptors

The module
[`crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs`](../../crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs)
owns the typed records.

### `AttachTargetDescriptor` — picker-stage truth

The descriptor for the process, container, remote helper, core file, or replay
capture a session attaches to or launches. It carries:

- a stable `descriptor_id` and a `TargetKindClass`;
- the exact `target_identity` — canonical target id, label, opaque process ref,
  working-directory digest, and **build / artifact identity**;
- the `TargetBoundaryClass` — `local`, `remote`, `container`, or `managed` — with a
  computed `boundary_crosses_trust` flag;
- the `TargetMutabilityClass` — `mutable`, `read_only_capture`, or
  `policy_write_protected` — with `permits_mutation`;
- the `TargetPrivilegeClass` — `sandboxed`, `user_standard`, `elevated`, or
  `system` — with `privilege_requires_disclosure`;
- the adapter ref/version and the `AdapterDriftClass`; and
- the negotiated capability refs and the negotiation-evidence proof packet.

### `DebugSessionDescriptor` — active-session truth

The live or post-mortem session. It carries a stable `session_id`, the
`execution_context_id` it routes through, the `DebugEntrypointClass` command, the
`DebugSessionModeClass` result, the `SessionRunStateClass`, the `ReentryPosture`, the
current `AdapterDriftClass`, a `target_descriptor_ref` plus a `TargetIdentityEcho`,
and a derived `holds_live_authority` flag.

## Controlled vocabulary

- **Session mode** (`DebugSessionModeClass`) — `launch`, `attach`, `core_file`,
  `replay`, `inspect_only`. The five stay distinct; only `launch` and `attach` can
  hold live authority.
- **Entrypoint** (`DebugEntrypointClass`) — `launch_target`, `attach_target`,
  `open_core_file`, `open_replay`, `restore_session`, `reattach`, `restart`,
  `open_in_support`. The command half of the command/result pair, routed through one
  execution-context pipeline.
- **Boundary** (`TargetBoundaryClass`) — `local`, `remote`, `container`, `managed`.
  Everything but `local` crosses a trust boundary that must be disclosed.
- **Mutability** (`TargetMutabilityClass`) — `mutable`, `read_only_capture`,
  `policy_write_protected`. Only `mutable` permits mutation.
- **Privilege** (`TargetPrivilegeClass`) — `sandboxed`, `user_standard`, `elevated`,
  `system`. `elevated` and `system` require disclosure.
- **Adapter drift** (`AdapterDriftClass`) — `adapter_current`, `adapter_drifted`,
  `reconnect_required`, `inspect_only_no_adapter`, `unsupported_skew`. A drifted
  adapter still controls the target with a disclosed caveat; reconnect-required,
  inspect-only, and unsupported-skew do not permit live control.
- **Re-entry posture** (`ReentryPosture`) — `initial_entry`, `restored_layout_only`,
  `reattach_required`, `reattached_reacquired_authority`, `opened_in_support`. Only
  an initial entry or an explicit reattach can hold live authority.
- **Run state** (`SessionRunStateClass`) — `running`, `paused`,
  `reconstructed_inspectable`, `awaiting_reattach`, `terminated`. The last three
  forbid live authority.

## Authority derivation

`holds_live_authority` is never asserted directly; it is **derived** from the mode,
the re-entry posture, and the adapter drift together:

```
holds_live_authority =
    mode.mode_holds_live_authority()
    && reentry.implies_live_authority()
    && adapter_drift.permits_live_control()
```

So a replayed view, a core-file inspection, a restored-but-not-reattached layout, a
reconnect-required adapter, or an unsupported skew can never claim live control. The
freeze gate re-derives the flag for every session and rejects any drift.

## Contract rules (frozen invariants)

The canonical set computes each invariant's `holds` flag from the built descriptors;
an inconsistent edit flips an invariant and fails the freeze gate.

- **`descriptors.session_modes_distinct`** — launch, attach, core-file, replay, and
  inspect-only appear as five distinct session modes, never one generic session.
- **`descriptors.inspect_only_modes_hold_no_live_authority`** — core-file, replay,
  and inspect-only sessions never hold live authority.
- **`descriptors.restore_never_reacquires_authority_silently`** — a
  restored-layout-only or reattach-required session holds no live authority; only an
  explicit reattach reacquires it.
- **`descriptors.live_authority_derived_from_mode_posture_drift`** — each session's
  live-authority flag equals the derivation above.
- **`descriptors.attach_identity_preserved_picker_to_session`** — each session
  resolves its attach target and echoes the target identity, mutability, privilege
  class, boundary, and adapter drift unchanged from the picker descriptor.
- **`descriptors.adapter_drift_first_class`** — drift, reconnect-required,
  inspect-only, and unsupported-skew all appear and require disclosure.
- **`descriptors.every_session_routes_execution_context`** — every session carries a
  non-empty execution-context id and a typed entrypoint.
- **`descriptors.run_state_authority_consistent`** — no reconstructed,
  awaiting-reattach, or terminated run state is paired with held live authority.
- **`descriptors.build_artifact_identity_preserved`** — build / artifact identity is
  preserved from the attach target into the session's identity echo.

## First consumers

- core debugger session header, call stack, and launchers;
- notebook debug surface (kernel bridge, frame-to-cell linkage);
- profiler / trace / replay workspace;
- incident / crash review;
- support export / escalation packets; and
- AI context and tool-call evidence.

## Checked-in artifacts

- Descriptor module:
  [`crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs`](../../crates/aureline-debug/src/m5_debug_session_descriptors/mod.rs)
- Boundary schema:
  [`schemas/debug/m5_debug_session_descriptors.schema.json`](../../schemas/debug/m5_debug_session_descriptors.schema.json)
- Published fixture:
  [`fixtures/debug/m5_debug_session_descriptors/canonical_set.json`](../../fixtures/debug/m5_debug_session_descriptors/canonical_set.json)
- Evidence note:
  [`artifacts/debug/m5_debug_session_descriptors.md`](../../artifacts/debug/m5_debug_session_descriptors.md)
- Freeze gate:
  [`crates/aureline-debug/tests/m5_debug_session_descriptors.rs`](../../crates/aureline-debug/tests/m5_debug_session_descriptors.rs)

## Regenerating

```sh
cargo run -p aureline-debug --example dump_m5_debug_session_descriptors \
  > fixtures/debug/m5_debug_session_descriptors/canonical_set.json
cargo test -p aureline-debug
```
