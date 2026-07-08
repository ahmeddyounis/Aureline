# m5-review-request-check-queue-component-certification fixtures

Protected fixtures for the M5 review-component **surface certification** capstone
(M05-955). Each fixture is a full `ReviewComponentCertificationPacket` that validates
against
[`schemas/ui/m5-review-request-check-queue-component-certification.schema.json`](../../../schemas/ui/m5-review-request-check-queue-component-certification.schema.json).

- `provider_freshness_stale_auto_narrowed.json` — the canonical eight-surface
  certification after `apply_downgrade_automation` narrows the desktop review list
  because its provider backing went stale. The surface drops from
  `certified_parity` to `narrowed_parity`, the `provider_local_provenance` axis is
  marked narrowed, and the `provider_freshness_stale` trigger is disclosed —
  component truth stays preserved.
- `detail_pane_and_cli_narrowed.json` — two green surfaces (review detail pane and
  headless CLI) auto-narrowed by stale provider backing, showing certification
  narrowing is per-surface and disclosed rather than silent.

Regenerate with:

```
GEN_REVIEW_COMPONENT_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-review --lib regenerate_review_component_certification_artifacts
```
