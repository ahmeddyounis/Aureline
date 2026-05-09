# Proof packet: window display safety guards

This lane validates the runtime safety guards that keep Aureline windows reachable
across common desktop topology drift: display detach/dock and mixed-DPI moves.

Normative sources:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` window/restore rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` desktop support and window topology.
- `docs/ux/window_display_contract.md` topology/adjustment vocabulary.
- `docs/qa/multi_window_verification.md` scenario seed and drill list.

## Evidence

- Fixture-backed geometry coverage:
  - `crates/aureline-shell/src/windowing/display_safety.rs`
  - `fixtures/windowing/topology_cases/`
- Runtime records (developer-local):
  - `.logs/window_display_safety/*.window_display_safety.json`

## Validation

- Unit coverage:
  - `cargo test -p aureline-shell display_safety::tests::recenter_logic_is_stable_across_fixture_cases`
- Live shell drill (manual):
  - `cargo run -p aureline-shell --bin aureline_shell`
  - Dock/undock or change monitor topology, then confirm:
    - windows stay reachable (no fully off-screen primary window),
    - scale changes trigger reflow without input drift,
    - `.logs/window_display_safety/` contains a topology or adjustment record.
