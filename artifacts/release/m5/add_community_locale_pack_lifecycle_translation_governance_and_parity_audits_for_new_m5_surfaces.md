# Community Locale-Pack Governance Register Artifact Companion

This file is the artifact-level companion document for the checked-in community locale-pack governance register.

- **Canonical JSON**: `artifacts/release/m5/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.json`
- **Schema**: `schemas/governance/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces.schema.json`
- **Typed consumer**: `crates/aureline-release/src/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces/mod.rs`
- **Validation capture**: `artifacts/release/captures/add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces_validation_capture.json`
- **Generator**: `gen_community_locale_pack_governance.py`

The register is the single source of truth for community locale-pack lanes (core locale, community pack, partner pack, machine-assisted pack), the per-lane translation-governance scorecard, the disclosed maintainer trust tier and fallback disclosure of each, owner manifests, base-locale rollback/downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_community_locale_pack_governance.py` from the repository root after changing the locale-pack lanes.
