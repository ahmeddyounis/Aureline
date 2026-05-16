# Protocol handler ownership audit

This artifact is the reviewer-facing projection of the desktop-entry
ownership audit packet that lives in
[`crates/aureline-install/src/ownership_audit/`](../../../crates/aureline-install/src/ownership_audit/).
It freezes which channel/build owns each OS-level handoff surface
(file association, protocol handler, default-browser callback,
deep-link intent, recent-item registration) across the side-by-side,
portable, managed, and air-gapped layouts the install-topology alpha
packet claims.

The audit references the install topology alpha packet at
[`fixtures/install/topology_alpha/install_topology_alpha_packet.json`](../../../fixtures/install/topology_alpha/install_topology_alpha_packet.json)
and projects through the same surface and support-export wrappers as
the topology packet, so About, update, diagnostics, install review,
CLI, and support-export rows report the same ownership truth.

## Audit rows

| Audit row id | Layout | Owning build | Surface | Verdict | Disclosure |
| --- | --- | --- | --- | --- | --- |
| `ownership.windows.stable.preview.file_association` | Stable + Preview side-by-side | Windows per-user Stable | File association | Selected owner (Stable) | Selection never last-writer-wins; channel-owner summary; handler-owner change previewed before commit |
| `ownership.windows.stable.preview.protocol_handler` | Stable + Preview side-by-side | Windows per-user Stable | Protocol handler | Selected owner (Stable) | Per-channel suffixed scheme; selection never last-writer-wins; handler-owner change previewed before commit |
| `ownership.windows.stable.preview.default_browser_callback` | Stable + Preview side-by-side | Windows per-user Stable | Default-browser callback | Selected owner (Stable) | Per-channel suffixed scheme; selection never last-writer-wins |
| `ownership.windows.stable.preview.deep_link_intent` | Stable + Preview side-by-side | Windows per-user Stable | Deep-link intent | Selected owner (Stable) | Per-channel suffixed scheme; selection never last-writer-wins |
| `ownership.windows.preview.side_by_side.protocol_handler` | Stable + Preview side-by-side | Windows Preview side-by-side | Protocol handler | Candidate only | Per-channel suffixed scheme; channel-owner summary; handler-owner change previewed before commit |
| `ownership.windows.portable.beside_stable.file_association` | Stable + Portable beside | Windows Portable Stable | File association | Not registered | Portable does not steal installed ownership |
| `ownership.windows.portable.beside_stable.protocol_handler` | Stable + Portable beside | Windows Portable Stable | Protocol handler | Not registered | Portable does not steal installed ownership |
| `ownership.windows.portable.beside_stable.default_browser_callback` | Stable + Portable beside | Windows Portable Stable | Default-browser callback | Not registered | Portable does not steal installed ownership |
| `ownership.windows.managed.beside_stable.file_association` | Stable + Managed beside | Windows Managed Stable | File association | Admin policy owned | Managed owner shown but not overrideable; channel-owner summary |
| `ownership.windows.managed.beside_stable.protocol_handler` | Stable + Managed beside | Windows Managed Stable | Protocol handler | Managed fleet owned | Per-channel suffixed scheme; managed owner shown but not overrideable |
| `ownership.windows.displaced_preview.protocol_handler` | Stable + Preview side-by-side | Windows per-user Stable | Protocol handler | Displaced owner | Channel-owner summary; handler-owner change previewed before commit; pivots to topology stale-handler diagnostic |
| `ownership.airgap.bundle.protocol_handler` | Air-gapped bundle only | Air-gap bundle | Protocol handler | Not registered | Not applicable |
| `ownership.linux.mirror.recent_item` | Stable only | Linux per-machine Stable (mirror) | Recent-item registration | Selected owner (Stable) | Not applicable (recent items do not dispatch) |

## Deep-link route check parity

Every audit row whose handoff surface dispatches a route must list the
deep-link route checks the validator applies and assert the in-product
invocation runs the same family:

- `origin_trust` — no `unknown_untrusted` admission.
- `reviewed_sheet_preview` — boundary-raising routes reopen the
  reviewed entry-flow sheet before dispatch.
- `target_scope` — target/workspace scope check before execution.
- `single_use_replay` — single-use replay posture (denies
  double-consumption).
- `handler_ownership_verification` — handler ownership is verified
  before dispatch.

Every dispatching row in the audit fixture lists all five checks and
sets `in_product_invocation_uses_same_checks=true`. Recent-item
registration does not dispatch and is recorded with an empty list and
`false`, which the validator allows for non-dispatching surfaces.

## Coexistence outcomes

Side-by-side rows (`stable_and_preview_side_by_side`,
`stable_and_portable_beside`, `stable_and_managed_beside`,
`three_channel_mixed`) MUST set `silent_steal_blocked=true` and name
at least one real disclosure token. The audit makes these outcomes
verbatim:

- **Stable + Preview**: the user or admin selects the default owner;
  shared schemes use per-channel suffixes; an owner change is staged
  through install review with explicit acknowledgement before commit.
- **Stable + Portable**: portable never claims host-global ownership;
  deep links continue to route to the installed Stable owner.
- **Stable + Managed**: the policy or ring owner is shown in About,
  install review, and support export; the user surface cannot
  override the managed owner silently.
- **Air-gapped bundle**: the bundle row carries
  `not_registered` for protocol-handler ownership; admission still
  runs through the same trust/preview/scope checks if an admin opens
  the build manually.

## Displaced-owner diagnostic

The audit includes the displaced-owner row
`ownership.windows.displaced_preview.protocol_handler` which sets
`owner_verdict=displaced_owner` and references the topology
stale-handler diagnostic
`install.handler.diagnostic.windows.displaced_stable_owner` so a
reviewer can pivot from the audit row to the diagnostic and the
support-export reference without reading installer logs.

## Verification

```bash
cargo test -p aureline-install
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- validate
cargo run -q -p aureline-shell --bin aureline_shell_ownership_audit -- support-export
```

The command set covers fixture validation, deep-link route-check
parity, portable / managed / displaced-owner posture, and the
metadata-safe support-export wrapper that mirrors the surface
projection.
