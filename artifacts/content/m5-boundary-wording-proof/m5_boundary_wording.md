# Boundary Wording: Hosted/Local/Self-hosted/Commercial Honesty

- Catalog: `m5-boundary-wording-catalog:stable:0001`
- Label: `Hosted/Local/Self-hosted/Commercial Boundary Wording Across M5 Surfaces`
- Reference locale: `en`
- Entries: 14
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## Boundary claims by concept

- `concept.byok_provider` — surfaces: settings
  - `entry.byok_provider.settings` — Bring your own provider key to use your own model account directly. [term: byok; actual: byok; surface: settings; claim: states_boundary]; identity: not_required; network: required; data: optional; export: retained; rollback: retained; alternatives: byok, export, rollback
- `concept.cloud_sync` — surfaces: account_upgrade_prompt, help_about, onboarding, settings
  - `entry.cloud_sync.settings` — Managed cloud sync keeps the workspaces you choose in sync across devices. [term: managed; actual: managed_optional; surface: settings; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.cloud_sync_compatibility
  - `entry.cloud_sync.onboarding` — Turn on managed cloud sync now, or stay fully local and decide later. [term: managed; actual: managed_optional; surface: onboarding; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.cloud_sync_compatibility
  - `entry.cloud_sync.account_upgrade` — Add managed cloud sync to this account, or keep your local-only setup. [term: managed; actual: managed_optional; surface: account_upgrade_prompt; claim: widens_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.cloud_sync_compatibility
  - `entry.cloud_sync.help_about` — Managed cloud sync is optional; local-only editing is always available. [term: managed; actual: managed_optional; surface: help_about; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.cloud_sync_compatibility
- `concept.hosted_build_farm` — surfaces: marketplace
  - `entry.hosted_build_farm.marketplace` — The hosted build farm runs heavy builds on managed infrastructure. [term: hosted; actual: managed_required; surface: marketplace; claim: widens_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, self_hosted, export, rollback; support: support.metadata.hosted_build_farm_compatibility
- `concept.local_indexing` — surfaces: help_about, settings
  - `entry.local_indexing.settings` — Local-only indexing builds your code index on this device with no account. [term: local_only; actual: local_independent; surface: settings; claim: states_boundary]; identity: not_required; network: not_required; data: local_only; export: retained; rollback: retained; alternatives: local_only, export, rollback
  - `entry.local_indexing.help_about` — Indexing is local only: nothing is uploaded and no sign-in is required. [term: local_only; actual: local_independent; surface: help_about; claim: states_boundary]; identity: not_required; network: not_required; data: local_only; export: retained; rollback: retained; alternatives: local_only, export, rollback
- `concept.managed_policy_pack` — surfaces: release_notes
  - `entry.managed_policy_pack.release_notes` — Org policy packs now require a managed workspace; self-hosted policy packs stay supported. [term: managed; actual: managed_required; surface: release_notes; claim: narrows_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: self_hosted, export, rollback; support: support.metadata.managed_policy_pack_compatibility
- `concept.premium_models` — surfaces: account_upgrade_prompt, help_about, marketplace
  - `entry.premium_models.marketplace` — Premium hosted models are a paid add-on; local and BYOK models stay free. [term: premium; actual: commercial_paid; surface: marketplace; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.premium_models_compatibility
  - `entry.premium_models.account_upgrade` — Upgrade for premium hosted models, or keep using local and BYOK models. [term: premium; actual: commercial_paid; surface: account_upgrade_prompt; claim: widens_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.premium_models_compatibility
  - `entry.premium_models.help_about` — Premium models require a paid plan; the editor works fully with local models. [term: premium; actual: commercial_paid; surface: help_about; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback; support: support.metadata.premium_models_compatibility
- `concept.self_hosted_runner` — surfaces: settings
  - `entry.self_hosted_runner.settings` — Self-hosted runners let you run builds on infrastructure you operate. [term: self_hosted; actual: self_hostable; surface: settings; claim: widens_boundary]; identity: optional; network: required; data: optional; export: retained; rollback: retained; alternatives: self_hosted, export, rollback; support: support.metadata.self_hosted_runner_compatibility
- `concept.trial_window` — surfaces: account_upgrade_prompt
  - `entry.trial_window.account_upgrade` — Your premium trial is time-limited; local and BYOK paths continue after it ends. [term: trial; actual: commercial_paid; surface: account_upgrade_prompt; claim: states_boundary]; identity: required; network: required; data: optional; export: retained; rollback: retained; alternatives: local_only, byok, self_hosted, export, rollback

## Copy-parity lint

No cross-surface boundary drift detected.
