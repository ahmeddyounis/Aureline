# M5 Certification-Train Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5
certification-train register.

- **Canonical JSON**: `artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`
- **Schema**: `schemas/governance/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.schema.json`
- **Typed consumer**: `crates/aureline-release/src/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index/mod.rs`
- **Validation capture**: `artifacts/release/captures/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index_validation_capture.json`
- **Generator**: `gen_m5_cert_train_register.py`

The register is the single source of truth for certifying the full M5 feature
train and publishing the canonical M5 evidence index (the feature-family
certifications, the qualification packets, the compatibility reports, and the
evidence index that ties them together across the feature families — notebooks,
DB/API, profiler, AI, frameworks, companion, sync, and offboarding), the
per-surface readiness scorecard, the disclosed support posture and trust tier of
each, owner manifests, downgrade automation, and the promotion verdict. All
downstream surfaces ingest it directly. Regenerate it with `python3
gen_m5_cert_train_register.py` from the repository root after changing the
certification surfaces.
