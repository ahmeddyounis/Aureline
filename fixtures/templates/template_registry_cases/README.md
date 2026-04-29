# Template registry and scaffold-policy worked examples

This directory holds worked examples for the registry, hook-policy, and
generated-project update semantics contract in
[`/docs/templates/template_registry_and_scaffold_contract.md`](../../../docs/templates/template_registry_and_scaffold_contract.md)
and its companion schemas:

- [`/schemas/templates/template_registry_entry.schema.json`](../../../schemas/templates/template_registry_entry.schema.json)
- [`/schemas/templates/scaffold_hook_policy.schema.json`](../../../schemas/templates/scaffold_hook_policy.schema.json)
- [`/schemas/templates/generated_project_update_semantics.schema.json`](../../../schemas/templates/generated_project_update_semantics.schema.json)

Every YAML file carries a `__fixture__` prelude describing the scenario,
the contract sections it exercises, and the acceptance points it backs.
Runtime payloads are one of:

- `template_registry_entry_record`
- `scaffold_hook_policy_record`
- `generated_project_update_semantics_record`

No fixture embeds raw signing keys, raw certificate material, raw
repository URLs, raw absolute paths, raw hook bodies, raw command lines,
raw stdout / stderr, raw secrets, raw manifest bodies, raw diffs, or raw
user-authored file content. Such data is represented by opaque refs,
digests, closed vocabulary, or redaction-aware summaries.

## Cases

- [`official_signed_template.yaml`](./official_signed_template.yaml) —
  official signed registry row with a core signing root, live health
  cadence, validation bundle, no known issues, and open-without-starter
  continuity.
- [`org_mirror_template.yaml`](./org_mirror_template.yaml) —
  organization mirror row that preserves upstream identity while naming
  org trust, support, stale mirror freshness, health cadence, and bypass
  continuity explicitly.
- [`repo_local_generator.yaml`](./repo_local_generator.yaml) —
  repo-local generator row where trust and support remain explicit
  rather than inferred from repository location.
- [`hook_blocked_template.yaml`](./hook_blocked_template.yaml) —
  hook policy denying hidden imperative setup while preserving the
  bypass path.
- [`reapply_with_local_divergence.yaml`](./reapply_with_local_divergence.yaml)
  — update semantics record for a reapply attempt with local divergence,
  preview-required overwrite guard, migration note, validation warning,
  and recovery choices.
