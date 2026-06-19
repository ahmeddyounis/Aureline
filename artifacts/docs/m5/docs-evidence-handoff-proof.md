# Docs Evidence Handoff (prose changes traced to code/schema/run/release)

- Packet: `packet:m5:docs_evidence_handoff:retry_backoff_release`
- Handoff: docs evidence handoff: the retry/backoff release docs sweep
- Promotion: `stable` (0 findings)
- Entries: 4 | Degradations: 1

## Entries

- [readme_edit] `entry:readme:config_example_fix` (update the retry_with_backoff configuration example) — change `README → Configuration → max_elapsed example#configuration`
  - Scope: `export_safe_shared` | export-safe true | reopenable true
  - evidence [source_file] `binding:readme:source_file` — crates/aureline-net/src/retry.rs | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `authoritative_live` | version `exact_build_match` | mirror `online_live` | cited true
  - evidence [symbol] `binding:readme:symbol` — retry::with_backoff | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `authoritative_live` | version `exact_build_match` | mirror `online_live` | cited true
  - evidence [failing_example] `binding:readme:failing_example` — Docs validation → README → with_jitter example (stale) | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `warm_cached` | version `exact_build_match` | mirror `online_live` | cited true
- [changelog_entry] `entry:changelog:retry_backoff_release` (add the retry/backoff changelog entry) — change `Changelog → next channel → retry/backoff#retry-backoff`
  - Scope: `export_safe_shared` | export-safe true | reopenable true
  - evidence [release_object] `binding:changelog:release_object` — Release center → next channel → retry/backoff | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `warm_cached` | version `compatible_minor_drift` | mirror `online_live` | cited true
  - evidence [test_run] `binding:changelog:test_run` — CI → retry/backoff suite → next channel | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `warm_cached` | version `compatible_minor_drift` | mirror `online_live` | cited true
- [api_reference_edit] `entry:api_reference:retry_policy` (update the RetryPolicy API reference) — change `API reference → RetryPolicy#retry-policy`
  - Scope: `export_safe_shared` | export-safe true | reopenable true
  - evidence [api_contract] `binding:api_reference:contract` — schemas/net/retry_policy.schema.json | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `authoritative_live` | version `exact_build_match` | mirror `online_live` | cited true
  - evidence [symbol] `binding:api_reference:symbol` — retry::RetryPolicy | scope `export_safe_shared` | redaction `metadata_safe` | provenance `first_party_verified` | freshness `authoritative_live` | version `exact_build_match` | mirror `online_live` | cited true
- [help_edit] `entry:help:offline_runbook_note` (annotate the operations-runbook help note) — change `Help → Retry and backoff → Operations runbook#operations-runbook`
  - Scope: `local_only` | export-safe false | reopenable true
  - evidence [source_file] `binding:help:imported_runbook_source` — imported ops pack → runbooks → retry_backoff_runbook.md | scope `local_only` | redaction `local_only_redaction_required` | provenance `imported` | freshness `warm_cached` | version `compatible_minor_drift` | mirror `mirror_served` | cited true
  - evidence [human_note] `binding:help:maintainer_note` — Maintainer note → ops owner | scope `local_only` | redaction `local_only_redaction_required` | provenance `local_only_unverified` | freshness `unverified` | version `unknown_target_build` | mirror `offline_cached_usable` | cited true

## Degradations

- [mirror_offline_snapshot/advisory]: one imported ops-pack binding is served from the mirror snapshot; it is held to warm-cached freshness rather than claiming live authority
