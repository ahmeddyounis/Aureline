# M5 Git-History Surface Certification fixtures

Protected fixtures for the `certify_git_history_component_truth_on_every_claimed_m5_git_history_surface`
lane (workstream B113). Each fixture is a full
`GitHistoryCertificationPacket` that validates against
`schemas/ui/m5-git-history-surface-certification.schema.json`.

- `provider_review_state_stale_auto_narrowed.json` — the canonical eight-surface
  packet after auto-narrowing the `history_sidebar` surface because its
  provider-linked recovery backing went stale: the claim narrows from
  `recoverable_in_product` to `locally_recoverable` and the
  `local_recovery_provenance` axis is marked narrowed with the
  `provider_review_state_stale` trigger.
- `risky_mutation_and_exported_recovery_narrowed.json` — the canonical packet after
  auto-narrowing the `review_workspace` and `cli_headless` surfaces for stale
  provider-linked recovery.

Regenerate with:

```
GEN_GIT_HISTORY_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-git --lib regenerate_git_history_certification_artifacts
```
