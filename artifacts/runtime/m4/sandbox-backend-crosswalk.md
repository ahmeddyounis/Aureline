# Sandbox Backend Crosswalk

This artifact is the release-evidence crosswalk for stable runtime sandbox
claims. It is consumed with `schemas/runtime/sandbox-profile.schema.json` and
`artifacts/runtime/m4/sandbox_profile_backend_truth_packet.json`.

| Backend class | Enforcement classification | Stable posture |
|---|---|---|
| `macos_desktop` | native process sandbox primitives, scoped filesystem mediation, keychain-backed secret paths, PTY broker, observable backend class | profiles are claimable when the packet row is `enforced`; incomplete rows must narrow or block |
| `windows_desktop` | restricted-token or equivalent native isolation, job/object resource control, credential-manager path, ConPTY mediation, observable backend class | profiles are claimable when the packet row is `enforced`; incomplete rows must narrow or block |
| `linux_desktop` | published package stack using namespaces/filtering, resource control, secret-service path, PTY mediation, observable backend class | only packet-claimed rows are stable; unsupported rows do not widen to unrestricted desktop launch |
| `remote_managed` | remote or managed broker enforcement, target identity proof, signed capability manifest, delegated credential rules, audit parity | remote capability is unavailable if enforcement is lost; it does not become local execution |
| `browser_companion` | no local execution backend; read-mostly inspection plus approved remote/managed handoff | no local shell, kernel, debug attach, scaffold, connector, or AI tool launch from the browser |

Rows marked `unsupported` or `fail_closed` are still truthful stable evidence
when the product displays them as unavailable. They are not stable execution
claims. Rows marked `stricter_downgrade` must name the fallback profile and show
the downgrade in installers, runtime inspectors, command diagnostics, docs/help,
and support exports.
