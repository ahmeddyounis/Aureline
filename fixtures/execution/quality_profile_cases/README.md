# Quality-profile and on-save fixtures

These fixtures anchor the record families defined by
[`/docs/execution/quality_profile_and_on_save_contract.md`](../../../docs/execution/quality_profile_and_on_save_contract.md)
and the schemas:

- [`/schemas/execution/quality_profile.schema.json`](../../../schemas/execution/quality_profile.schema.json)
- [`/schemas/execution/on_save_plan.schema.json`](../../../schemas/execution/on_save_plan.schema.json)

They are pre-implementation examples. IDs are opaque and chosen for
readability; they are not planning identifiers.

| Fixture | Schema | Record kind | Scenario |
|---|---|---|---|
| [`policy_locked_effective_profile.yaml`](./policy_locked_effective_profile.yaml) | `quality_profile` | `effective_quality_profile_record` | Admin policy pins formatter and scanner choices while preserving the shadowed local/profile layers. |
| [`imported_config_downgrade.yaml`](./imported_config_downgrade.yaml) | `quality_profile` | `quality_profile_source_record` | Imported tool config maps partially and is visible as a downgraded source rather than silently disappearing. |
| [`scanner_import_compatibility_blocked.yaml`](./scanner_import_compatibility_blocked.yaml) | `quality_profile` | `scanner_import_session_record` | SARIF-like scan import is preserved as read-only evidence because profile/tool compatibility blocks local delta comparison. |
| [`on_save_trivia_safe_plan.yaml`](./on_save_trivia_safe_plan.yaml) | `on_save_plan` | `on_save_plan_record` | Format-on-save admits a trivia-safe formatter and a read-only scanner against one staged buffer. |
| [`additional_edit_review_required.yaml`](./additional_edit_review_required.yaml) | `on_save_plan` | `additional_edit_review_record` | A provider-dependent organize-imports action declares multi-file additional edits and is held for review with issue linkage. |
