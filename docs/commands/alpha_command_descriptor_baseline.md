# Alpha command-descriptor baseline

This baseline publishes the launch-wedge command descriptor registry,
invocation-session packet minimum, disabled-reason cause vocabulary, parity
report, and result baseline.

Machine-readable artifacts:

- `schemas/commands/command_descriptor_alpha.schema.json`
- `schemas/commands/invocation_session_alpha.schema.json`
- `artifacts/commands/alpha_command_registry.yaml`
- `artifacts/commands/alpha_command_parity_report.yaml`
- `fixtures/commands/disabled_reason_alpha/manifest.json`
- `fixtures/commands/invocation_session_alpha/`

The alpha registry is a governed publication over the canonical seeded command
registry in `artifacts/commands/command_registry_seed.yaml`. It does not mint
new command IDs. It freezes the alpha claim set and records how palette, menu,
keybinding, CLI/headless, AI/tool, onboarding/recipe, browser handoff, and voice
descriptor lanes project the same descriptor without widening preview,
approval, disabled-reason, or result semantics.

## Claimed command set

| Command ID | State | Preview | Approval | Automation | Result baseline |
| --- | --- | --- | --- | --- | --- |
| `cmd:workspace.open_folder` | stable | `no_preview_required` | `no_approval_required` | `macro_safe`, `recipe_safe`, `headless_safe` | success invocation with mutation journal evidence |
| `cmd:workspace.clone_repository` | stable | `structured_diff_preview` | `no_approval_required` | `recipe_safe`, `headless_safe` | disabled/provider-unlinked invocation |
| `cmd:workspace.import_profile` | stable | `structured_diff_preview` | `explicit_confirmation_required` | `recipe_safe`, `headless_safe`, `approval_required` | result packet with preview, checkpoint, rollback, and support export refs |
| `cmd:workspace.restore_from_checkpoint` | stable | `structured_diff_preview` | `explicit_confirmation_required` | `headless_safe`, `approval_required` | preview-denied invocation preserving unapplied checkpoint refs |
| `cmd:command_palette.open` | stable | `no_preview_required` | `no_approval_required` | `ui_only` | UI-only descriptor, not surfaced as runnable in CLI |
| `cmd:docs.open_in_browser` | stable | `no_preview_required` | `no_approval_required` | `ui_only` | browser-handoff descriptor row |

## Disabled reasons

The alpha disabled-reason fixture manifest maps these cause families to the
canonical `disabled_reason_code` vocabulary:

| Cause family | Canonical code |
| --- | --- |
| focus | `required_argument_unresolved` |
| selection | `required_argument_unresolved` |
| lifecycle state | `command_retired` |
| missing dependency | `required_provider_unlinked` |
| policy | `policy_blocked_in_context` |
| entitlement | `managed_only_channel_required` |
| remote or host mismatch | `client_scope_excludes_surface` |

Surfaces must keep disabled commands discoverable when the descriptor says the
row matters, and they must carry the typed reason plus a repair hook. A disabled
row with only surface-local prose is non-conforming.

## Cross-surface parity

`artifacts/commands/alpha_command_parity_report.yaml` is the first alpha parity
diff packet. It compares stable command ID, authority class, capability class,
preview class, approval posture, disabled-reason mode, automation support,
result contract, and evidence refs across:

- command palette;
- menu or button;
- keybinding help;
- CLI/headless;
- AI/tool;
- recipe or onboarding-generated lane;
- browser or voice descriptor lane.

Unknown high-risk gaps, preview/approval mismatches, disabled-reason omissions,
authority widening, and result-contract drift are blocking defects for this
alpha packet. The checked-in report has zero blocking findings.

## Discoverability round trips

The alpha registry records discoverability consumers for Start Center cards,
onboarding hints, keymap bridges, help-search results, migration guidance,
voice hints, and browser handoff rows. Each consumer carries:

- the stable command ID;
- the keyboard or intent route;
- the docs/help anchor;
- the invocation or result packet fixture when the row can run;
- `preserves_preview_apply_semantics = true`;
- `typed_reason_required_when_unavailable`;
- an exact reopen ref.

The shell onboarding projection consumes this alpha registry publication and
exports the same round-trip rows so Start Center and help-search consumers do
not restate command truth in free-form prose.

## Non-shell consumer

The `aureline_commands` CLI now emits an `alpha_publication` block from the same
alpha registry artifact used by Rust validation. That makes the alpha command
truth inspectable from a headless consumer before a full product CLI exists.
