# Restore prompt and deep-link entry fixtures

This directory is the seed corpus for the **restore prompt + deep-link entry**
contract:

- Documentation: [`docs/ux/restore_and_deeplink_contract.md`](../../../docs/ux/restore_and_deeplink_contract.md)
- Restore-prompt builder: `crates/aureline-shell/src/restore/mod.rs`
- Deep-link validator: `crates/aureline-shell/src/deeplink/mod.rs`

Fixtures round-trip through the canonical builders so the live shell, the
diagnostics packet, and exported records share one truth.

## Index

| Fixture | Record kind | Scenario covered |
| --- | --- | --- |
| `restore_prompt_recovered_drafts.json` | `restore_prompt_record` | Drafts present after abnormal termination; safe-mode and clear-journal paths visible. |
| `restore_prompt_no_restore_first_launch.json` | `restore_prompt_record` | First launch, nothing to restore; restore-now disabled with typed reason; safe-mode/open-clean stay reachable. |
| `deeplink_admitted_workspace_open.json` | `deep_link_validation_record` | System-default-browser open of a local workspace root; admitted, no review required. |
| `deeplink_review_required_managed_resume.json` | `deep_link_validation_record` | Managed-workspace resume via deep link; admitted but reviewed-sheet required. |
| `deeplink_denied_unknown_origin.json` | `deep_link_validation_record` | `unknown_untrusted` origin; denied with `origin_unverified`. |
