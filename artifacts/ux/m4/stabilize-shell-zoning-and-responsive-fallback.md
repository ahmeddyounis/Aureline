# Stable Shell Zoning Evidence

Evidence packet:
`fixtures/ux/m4/stabilize-shell-zoning-and-responsive-fallback/shell_zoning_responsive_fallback_packet.json`

Boundary schema:
`schemas/ux/shell-slot-registry.schema.json`

Executable owner:
`crates/aureline-shell/src/stabilize_shell_zoning_and_responsive_fallback/`

## Result

The packet audit passes:

- every claimed stable surface occupies a declared slot;
- every declared slot maps to the canonical shell zone vocabulary;
- compact, standard, and expanded fallback ladders preserve slot identity,
  breadcrumbs, trust, execution-target truth, and keyboard recovery;
- warm restore and dependency-loss fixtures preserve placeholders in place;
- private top-level chrome, duplicate sidebars, and floating global buttons are
  denied for stable shell rows.

## Consumers

Shell composition, diagnostics, Help/About, docs/help, extension admission, and
support export should consume the Rust packet or emitted JSON fixture directly.
They should not recreate slot ownership, fallback order, or placeholder classes
in local models.

## Verification

```sh
cargo test -p aureline-shell --test stabilize_shell_zoning_and_responsive_fallback
```
