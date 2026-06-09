# M5 Rollback, Downgrade, Claim-Narrowing, and Staged-Promotion Rules Artifact Companion

This file is the artifact-level companion document for the checked-in M5 rollback/downgrade register.

- **Canonical JSON**: `artifacts/release/m5/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.json`
- **Schema**: `schemas/governance/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules.schema.json`
- **Typed consumer**: `crates/aureline-release/src/freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules/mod.rs`

The register is the single source of truth for M5 lane rollback posture, downgrade rules, claim-narrowing automation, and staged-promotion stage tracking. All downstream surfaces ingest it directly.
