# Settings effective-setting example fixtures

These fixtures are short, reviewable scenarios that anchor the
vocabulary frozen in
[`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../../../docs/adr/0008-settings-definition-and-effective-configuration-resolver.md)
and validated by the schema at
[`/schemas/settings/effective_setting.schema.json`](../../../schemas/settings/effective_setting.schema.json).

Each fixture carries one `effective_setting_record` per the schema,
exercises at least one frozen scope, preview class, write intent,
lock state, denial reason, or restart posture, and names the ADR
section that motivates it. Together they anchor the scope names, the
precedence order, the effective-setting record shape, the lock
states, the denial reasons, the write-intent set, the restart-posture
set, the preview-class set, and the control-stack fields to concrete
inputs and observable outcomes.

**Scope rules**

- Fixtures validate against `schemas/settings/effective_setting.schema.json`
  as `effective_setting_record`; they do not encode wire bytes,
  ADR-0005 subscription envelopes, or ADR-0004 RPC envelopes.
- A fixture MUST exercise at least one frozen `scope_id`,
  `preview_class`, `write_intent`, `lock_state`, `write_denial_reason`,
  or `restart_posture`, and MUST name the ADR section that motivates
  it.
- Raw secret bytes MUST NOT appear; `credential_alias` values are
  opaque aliases only (ADR-0007).
- Setting ids, source labels, monotonic timestamps, mutation-journal
  refs, and explain-why refs are chosen to read well rather than to
  reflect any real deployment.

**Index**

| Fixture                                                                                            | Setting id                          | Winning scope                    | Write intent                                              | Preview class                                         | Key axes exercised                                                                                 | ADR section                                                              |
|----------------------------------------------------------------------------------------------------|-------------------------------------|----------------------------------|-----------------------------------------------------------|-------------------------------------------------------|----------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------|
| [`user_global_wins.yaml`](./user_global_wins.yaml)                                                 | `editor.tab_size`                   | `user_global`                    | `allowed`                                                 | `safe_apply`                                          | Shadow chain with one contributing non-default scope; stable lifecycle; user_profile_workspace auth | Effective-setting record (frozen) + Precedence order (frozen)            |
| [`workspace_over_imported_profile.yaml`](./workspace_over_imported_profile.yaml)                   | `editor.format_on_save`             | `workspace`                      | `allowed`                                                 | `safe_apply`                                          | Four-scope shadow chain; workspace overrides imported profile and user_global without widening trust| Precedence order (frozen)                                                |
| [`machine_specific_excluded_from_sync.yaml`](./machine_specific_excluded_from_sync.yaml)           | `runtime.gpu_adapter_hint`          | `machine_specific`               | `allowed`                                                 | `safe_apply`                                          | Machine-only scope; synced_by_default=false; workspace_capability dependency satisfied             | Allowed scopes (frozen) - machine_specific                               |
| [`admin_policy_narrowing.yaml`](./admin_policy_narrowing.yaml)                                     | `security.ai.egress_policy`         | `admin_policy_narrowing`         | `denied`                                                  | `rollback_checkpoint_and_approval_required`           | Policy constrains allowed set; narrowing_ceiling_active=true; lock_state=policy_constrained        | Lock and constraint state (frozen) + Precedence order (frozen)           |
| [`session_override_promotion.yaml`](./session_override_promotion.yaml)                             | `editor.word_wrap`                  | `session_or_command_override`    | `allowed`                                                 | `safe_apply`                                          | Ephemeral scope; expires_at set to session end; does not auto-promote to durable scope             | Scope-broadening rule (frozen) - invariant 3                             |
| [`preview_and_rollback_high_risk_change.yaml`](./preview_and_rollback_high_risk_change.yaml)       | `security.network.allow_list`       | `workspace`                      | `allowed_with_rollback_checkpoint_and_approval`           | `rollback_checkpoint_and_approval_required`           | Rollback checkpoint ref + approval ticket required; preview_state=presented; restart_extensions     | Preview-class and rollback-checkpoint expectations                       |
