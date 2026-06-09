# M5 Depth-Train Automation Register

## Overview

This artifact is the canonical M5 depth-train automation register. It binds every M5 feature family to proof-freshness SLOs, backport eligibility rules, and evidence expiry tracking so that stale or underqualified lanes narrow automatically before promotion.

## Checked-in artifact

- `artifacts/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.json`

## Schema

- `schemas/governance/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains.schema.json`

## Fixtures

- `fixtures/release/m5/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains/`

## Validation capture

- `artifacts/release/captures/generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains_validation_capture.json`

## Downstream consumers

- `crates/aureline-release` — typed model, validation, and export projection
- `crates/aureline-cli` — headless inspection (planned)
- `crates/aureline-help` — Help/About truth surfacing (planned)
