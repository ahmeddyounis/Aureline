# Upgrade and downgrade case fixtures

These seed fixtures exercise the update-manifest and helper-version
negotiation contracts in `docs/release/update_and_rollback_contract.md`.
They are structural examples only; they do not claim an updater,
signing service, fleet controller, or helper runtime has been
implemented.

Cases:

- `safe_upgrade.yaml` - verified staged upgrade with rollback and
  helper negotiation evidence.
- `blocked_downgrade_requires_manual_review.yaml` - downgrade blocks
  because state is newer than the target and the migration journal is
  missing.
- `helper_skew_review_only.yaml` - helper skew narrows to review-only
  with explicit dropped capabilities.
- `mirror_fed_update.yaml` - mirror-fed update preserves origin digest,
  signature, revocation, freshness, and manual-import receipt refs.
- `exact_build_reconstruction.yaml` - support reconstructs an exact
  build from manifest, signatures, helper negotiation, and support
  packet refs.
