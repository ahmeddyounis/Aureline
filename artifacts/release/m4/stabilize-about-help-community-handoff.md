# About/Help Community Handoff Release Evidence

## Evidence

- Help/About packet implementation:
  `crates/aureline-shell/src/help_about/mod.rs`
- Unit tests:
  `crates/aureline-shell/src/help_about/tests.rs`
- Canonical schema:
  `schemas/help/community-handoff-packet.schema.json`
- Fixture corpus:
  `fixtures/help/m4/stabilize-about-help-community-handoff/`
- Help docs:
  `docs/help/m4/stabilize-about-help-community-handoff.md`
- Community boundary docs:
  `docs/community/m4/stabilize-about-help-community-handoff.md`

## Acceptance Mapping

- Build facts, service-health refs, and provenance refs are projected into each
  handoff packet from the Help/About surface.
- Destination classes cover public, official-authenticated, community,
  vendor-managed, and local-only continuity.
- Public issue, security disclosure, docs feedback, RFC discussion, community
  support, and vendor/private support packets preserve exact origin anchors and
  return paths.
- Repro-packet preview redacts local paths, usernames, hostnames, tokens,
  extension inventory, deployment profile, and selected diagnostics.
- Browser-blocked and offline fixtures retain drafted text, attachments,
  redaction settings, and target class with retry/export/open-later actions.

## Verification

Run:

```sh
cargo test -p aureline-shell help_about
```
