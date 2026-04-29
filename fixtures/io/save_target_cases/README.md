# Save-target token fixtures

These fixtures anchor the save-target token and write-guarantee contract:

- [`/docs/io/save_target_token_and_write_guarantee_contract.md`](../../../docs/io/save_target_token_and_write_guarantee_contract.md)
- [`/schemas/io/save_target_token.schema.json`](../../../schemas/io/save_target_token.schema.json)
- [`/schemas/io/save_guarantee_class.schema.json`](../../../schemas/io/save_guarantee_class.schema.json)

They are pre-implementation examples. IDs are opaque and chosen for
readability; they are not planning identifiers.

| Fixture | Guarantee class | Scenario |
|---|---|---|
| [`local_atomic_write.yaml`](./local_atomic_write.yaml) | `atomic_replace_preferred` | Local root has exact generation identity and can atomically replace after compare. |
| [`cloud_backed_in_place_review.yaml`](./cloud_backed_in_place_review.yaml) | `in_place_write_with_review` | Cloud-backed root lacks atomic replace, so matching compare opens review before in-place write. |
| [`remote_revision_token_conflict.yaml`](./remote_revision_token_conflict.yaml) | `remote_conditional_write` | Remote revision token changed; conditional write is blocked and review reopens. |
| [`read_only_mount.yaml`](./read_only_mount.yaml) | `read_only_mount` | Read-only archive view blocks save and records no-write diagnostics. |
| [`policy_blocked_write.yaml`](./policy_blocked_write.yaml) | `policy_blocked` | Policy denies writing to an otherwise writable target. |
| [`missing_capability_fallback.yaml`](./missing_capability_fallback.yaml) | `capability_uncertain` | Root cannot provide a trustworthy identity or generation token, so normal save is blocked. |
