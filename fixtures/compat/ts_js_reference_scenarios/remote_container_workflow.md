# Remote/container TS workflow attribution (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:remote_container_identity_and_attribution`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.vite`
  - `framework_pack:typescript_web.npm_pnpm_toolchain`

## Scenario goal

Prove that a TS/JS workflow remains attributable when execution occurs in
a remote/container target:

- toolchain identity is explicit (no “whatever node is installed” claims);
- package-manager posture is explicit (registry source, auth source, egress);
- config root/workspace root decisions remain inspectable; and
- preview routes and tunnels preserve route truth and disclosure.

## Required truth and disclosures

- Remote attach, endpoint, and preview truth is captured via:
  - `docs/remote/attach_tunnel_port_forward_contract.md`
  - `docs/verification/target_and_host_boundary_packet.md`
- Any package-manager work required by the remote workflow still uses the
  same plan contract (no “remote exception”):
  - `docs/execution/package_manager_and_lockfile_safety_contract.md`

## Evidence hooks

- Worked attach/tunnel/handoff fixtures exist:
  - `fixtures/remote/attach_cases/`

## Known-limit expectations

- Certified archetype promotion requires at least one remote mode. If the
  remote mode is narrower than local mode (capability narrowing, blocked
  routes, restricted egress), certification requires a known-limit note that
  names the narrowing and recovery path.

