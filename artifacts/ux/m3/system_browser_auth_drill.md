# System-Browser Auth Drill

This drill verifies that claimed auth rows default to system-browser
handoff and that embedded auth exceptions remain explicit, bounded, and
support-exportable.

## Seeded Passing Case

Source row:

[`fixtures/ux/m3/embedded_boundary/rows.json`](../../../fixtures/ux/m3/embedded_boundary/rows.json)

The auth confirmation row records:

- `surface_family_token = embedded_auth_confirmation`
- `auth_flow_class_token = system_browser`
- `auth_provider_domain_label = github.com`
- `auth_reason_code = system_browser_primary`
- `system_browser_auth_default = true`
- `browser_handoff_packet_ref = id:browser-handoff:auth:github`
- `native_approval_owner_token = host_product_native`

The matching event log contains `auth_handoff_default_recorded` and
`browser_handoff_declared` events with the same browser handoff packet ref.

## Failure Drill

The fixture replay test mutates the seeded auth row so
`system_browser_auth_default = false`. The validator must emit:

`identity_row_not_system_browser_default`

The event-log drill removes the browser handoff event from a row with a
handoff packet ref. The validator must emit:

`missing_browser_handoff_event`

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_embedded_boundary_toolkit -- validate
cargo test -p aureline-shell --test embedded_boundary_toolkit_fixtures
```

The support export remains metadata-only; raw embedded auth content,
cookies, secrets, and raw URLs are excluded by construction.
