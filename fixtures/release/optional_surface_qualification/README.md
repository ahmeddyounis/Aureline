# Optional-surface qualification fixture cases

Negative fixtures for `ci/check_optional_surface_qualification.py`. Each JSON
file is a complete optional-surface qualification register that is structurally
valid except for one targeted flaw, paired in [`cases.json`](./cases.json) with
the check id its flaw must trip.

The gate runs every case during `--check` and fails when a case marked
`rejected` validates clean or trips a different check than expected. The Rust
contract test also parses every case and asserts the typed model rejects the
structural cases.

| Case | Flaw | Expected check id |
|---|---|---|
| `absent_packet_rendered_stable.json` | A surface with no packet renders qualified | `surface.qualified_without_packet` |
| `breached_packet_on_qualified.json` | A qualified surface rides a breached packet | `surface.qualified_on_stale_packet` |
| `claim_label_ceiling_mismatch.json` | A surface's claim label disagrees with the stable claim manifest | `ceiling.claim_label_mismatch` |

To regenerate a case after a deliberate register shape change, copy the
canonical register, reintroduce the single flaw, recompute the `summary` and
`publication` blocks when needed, and confirm the gate still trips the expected
check id.
