# Deployment profile continuity truth fixtures

This directory stores canonical JSON fixtures emitted by
`cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures`.

## Files

- `page.json` — seeded stable continuity packet
- `summary.json` — seeded packet summary
- `support_export.json` — support-export wrapper for the seeded packet
- `drill_hidden_self_hosted_dependency_withdrawn.json` — self-hosted claim hides
  a vendor dependency and withdraws
- `drill_mirror_freshness_gap_beta.json` — air-gapped mirror-freshness
  disclosure is missing and narrows to beta
- `drill_surface_reuse_gap_beta.json` — one fact family loses Help parity and
  narrows to beta
- `drill_missing_local_safe_fallback_preview.json` — one claimed profile loses
  its fallback card and narrows to preview

## Regeneration

```sh
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- page > fixtures/policy/deployment_profile_continuity_truth/page.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- summary > fixtures/policy/deployment_profile_continuity_truth/summary.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- support-export > fixtures/policy/deployment_profile_continuity_truth/support_export.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-hidden-self-hosted-dependency-withdrawn > fixtures/policy/deployment_profile_continuity_truth/drill_hidden_self_hosted_dependency_withdrawn.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-mirror-freshness-gap-beta > fixtures/policy/deployment_profile_continuity_truth/drill_mirror_freshness_gap_beta.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-surface-reuse-gap-beta > fixtures/policy/deployment_profile_continuity_truth/drill_surface_reuse_gap_beta.json
cargo run -q -p aureline-policy --example dump_deployment_profile_continuity_truth_fixtures -- drill-missing-local-safe-fallback-preview > fixtures/policy/deployment_profile_continuity_truth/drill_missing_local_safe_fallback_preview.json
```
