# Stable Shell Zoning And Responsive Fallback

This contract freezes the executable shell-slot registry for stable desktop
surfaces. The canonical implementation is
`crates/aureline-shell/src/stabilize_shell_zoning_and_responsive_fallback/`;
the boundary schema is `schemas/ux/shell-slot-registry.schema.json`; the pinned
fixture packet is
`fixtures/ux/m4/stabilize-shell-zoning-and-responsive-fallback/shell_zoning_responsive_fallback_packet.json`.

## Canonical Truth

Stable shell surfaces attach to declared slots only:

- title/context identity;
- activity rail primary routes;
- left sidebar section surfaces;
- main workspace working-set and review surfaces;
- right inspector contextual detail;
- bottom-panel tool panels;
- status-bar recovery and scoped extension items;
- transient command-palette, dialog, and sheet overlays.

Each slot record names its owning shell zone, permitted surface kinds, fallback
order, close and reopen commands, placeholder class, owner contract, and the
rule that private top-level chrome is forbidden.

## Responsive Fallback

The packet emits compact, standard, and expanded ladders for every declared
slot. Ladders preserve slot identity, breadcrumb truth, trust truth, execution
target truth, and keyboard-reachable recovery. Optional detail moves to sheets
or overflow before the editor width drops below the protected floor; main
workspace identity, trust, status, and command entry remain reachable.

## Placeholder Hydration

Warm restore, missing extensions, remote unavailability, provider capability
loss, and display-topology drift render placeholders in the declared slot. The
placeholder keeps breadcrumb, trust, target, and reopen truth and records that
adjacent layout did not collapse silently.

## Diagnostics And Support

Diagnostics and support export should consume
`canonical_shell_zoning_packet()` or `support_export_lines()` from
`aureline-shell` instead of recreating slot ownership locally. The packet audit
must pass before a surface can claim stable shell composition.

## Verification

Regenerate the fixture:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_stabilize_shell_zoning_and_responsive_fallback -- emit-fixtures fixtures/ux/m4/stabilize-shell-zoning-and-responsive-fallback
```

Run the focused replay:

```sh
cargo test -p aureline-shell --test stabilize_shell_zoning_and_responsive_fallback
```
