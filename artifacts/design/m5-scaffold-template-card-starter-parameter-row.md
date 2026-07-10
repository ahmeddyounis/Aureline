# Scaffold template cards and starter parameter rows

- Packet: `m5-scaffold-template-card-starter-parameter-row-controls:stable:0001`
- Surface: `M5 scaffold template cards and starter parameter rows: starter source, support, host boundary, parameter source precedence, and portability truth across claimed project-entry surfaces`
- Scaffold template cards: 6 (4 not governed first-party)
- Starter parameter rows: 6 (3 not portable)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-09T00:00:00Z)

## Scaffold template cards

- **React SPA starter** — source `first_party_starter`, support `officially_supported` → `first_party_template` / `fully_supported`, host `Runs locally in this workspace; no remote provisioning`, deep link `template_manifest`
- **Internal service starter** — source `team_managed_starter`, support `community_supported` → `team_managed_template` / `community_supported`, host `Runs against the team-managed workspace; provisions a managed namespace on create`, deep link `starter_registry_entry`
- **Community CLI starter** — source `community_starter`, support `experimental` → `community_template` / `experimental_or_bridge`, host `Runs locally; downloads the community pack over the network on first use`, deep link `starter_registry_entry`
- **Local notebook starter** — source `local_only_starter`, support `bridge_behavior` → `local_template` / `experimental_or_bridge`, host `Runs entirely on this machine; bridge generation, not exact first-party output`, deep link `docs_anchor`
- **Mirrored legacy starter** — source `mirrored_starter`, support `deprecated` → `local_template` / `unsupported_or_deprecated`, host `Runs from an offline mirror; deprecated, so a newer starter is preferred`, deep link `docs_anchor`
- **Unlabeled starter** — source `unknown_source_starter`, support `unsupported` → `source_unknown` / `unsupported_or_deprecated`, host `Host boundary unresolved; do not run until the source is clarified`, deep link `no_deep_link`

## Starter parameter rows

- **Application name** — origin `template_default`, layer `default_value`, timing `deferred_after_create` → `portable_template_value`, deep link `template_manifest`
- **Dev server port** — origin `user_input`, layer `user_provided`, timing `applied_immediately` → `portable_user_value`, deep link `template_manifest`
- **Package registry** — origin `workspace_value`, layer `profile_inherited`, timing `requires_confirmation` → `workspace_scoped_value`, deep link `policy_reference`
- **License** — origin `policy_value`, layer `environment_derived`, timing `optional_skippable` → `policy_managed_value`, deep link `policy_reference`
- **Service token** — origin `secret_reference`, layer `computed_derived`, timing `not_applicable` → `secret_reference_not_persisted`, deep link `policy_reference`
- **Owner email** — origin `user_input`, layer `unset_required`, timing `blocked_invalid` → `portable_user_value`, deep link `docs_anchor`
