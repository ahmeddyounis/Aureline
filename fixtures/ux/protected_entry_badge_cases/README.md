# Protected-entry badge parity cases

Worked parity audit cases for the protected-entry badge cluster defined in:

- `/docs/ux/protected_entry_badge_parity.md`
- `/artifacts/ux/protected_entry_badge_matrix.yaml`
- `/docs/ux/command_diagnostics_contract.md`
- `/schemas/commands/diagnostic_projection.schema.json` (`protected_entry_badge_record`)

These cases exist to prevent target/origin/route badges from drifting across
entry points that cross host boundaries or ownership domains (terminal, tasks,
debug, remote attach, provider entry, embedded docs links, browser handoff).

## Case shape

Each YAML document:

- references one or more `archetype_rows` in
  `/artifacts/ux/protected_entry_badge_matrix.yaml`;
- names the entry surfaces under audit; and
- binds each surface to an existing `protected_entry_badge_record` fixture (or
  a closely related governing fixture) rather than embedding hand-authored copy.

## Cases

- `palette_route_parity_across_entries.yaml`
  — identical `palette_invocation_route` token shown across multiple protected
  entry surfaces.
- `browser_only_continuation_embedded_docs_link.yaml`
  — embedded docs link triggers a typed browser handoff; the badge and handoff
  packet remain inspectable in screenshots, support exports, and logs.
- `provider_entry_local_fallback_degraded.yaml`
  — provider-backed entry remains truthful under local fallback + degraded/stale
  posture; the downgrade is explicit (not implied by missing badges).
- `wrong_target_recovery_remote_attach.yaml`
  — wrong-target recovery preserves protected-entry target/origin truth and
  makes reapproval requirements inspectable.
- `policy_limited_entry_badge_retained.yaml`
  — policy-limited protected entry keeps its badge cluster inspectable even
  when disabled, using export-safe wording and typed denial reasons.

