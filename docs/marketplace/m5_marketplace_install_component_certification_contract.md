# M5 Marketplace / Install-Review Component Surface Certification (M05-1107)

This is the **closing capstone** of the B131 marketplace / install-review component
family. Where the freeze matrix
(`freeze_the_m5_marketplace_result_row_..._and_diagnostics_component_matrix`) defines the
eight reusable components, the M05-1101..1105 implement lanes narrow each one, and the
M05-1106 accessibility lane proves keyboard / screen-reader / high-zoom / reduced-motion /
CLI-export parity and per-family auto-narrowing, this capstone **certifies** that the shared
component truth holds on every claimed M5 registry / install operating **profile** — and
auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-shell/src/certify_marketplace_and_install_review_component_truth_on_every_claimed_m5_public_mirrored_and_enterprise_registry_profile`
- **Schema:** [`schemas/ui/m5-marketplace-install-component-certification.schema.json`](../../schemas/ui/m5-marketplace-install-component-certification.schema.json)
- **Canonical proof bundle:** `artifacts/release/m5-marketplace-install-proof/support_export.json`
  (the frozen component matrix proof — every profile cites this one bundle)
- **Certification proof:** `artifacts/release/m5-marketplace-install-component-certification-proof/`
- **Fixtures:** `fixtures/ui/m5-marketplace-install-component-certification/`

## What it certifies

The packet is keyed on the **registry / install profile** a user, operator, or support
engineer reads marketplace and install-review truth through, not on the component family it
renders. Eight profiles are certified:

| Profile | Claim ceiling | Verdict |
| --- | --- | --- |
| `public_verified_registry` | `install_ready_result` | green |
| `mirrored_registry` | `reviewable_listing_result` | green |
| `enterprise_registry` | `reviewable_listing_result` | green |
| `side_load_reviewed_registry` | `reviewable_listing_result` | green |
| `stale_compatibility_registry` | narrows to `compatibility_unverified_projection` | yellow |
| `over_budget_throttled_registry` | narrows to `activation_budget_projection` | yellow |
| `rollback_unverifiable_registry` | narrows to `rollback_unverified_projection` | yellow |
| `transferred_publisher_registry` | narrows to `publisher_continuity_projection` | yellow |

Each row scores six truth axes: `visual`, `keyboard`, `screen_reader`, `cli_export`,
`degraded_state`, and `registry_install_truth`.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps an
   `install_ready_result` / `reviewable_listing_result` claim while an axis is not current is
   over-claiming and blocks (red). Disclosing the reduction — narrowing the claim with a
   bound reason and a frozen downgrade trigger — is honestly yellow.
2. **Only a public first-party verified registry may certify `install_ready_result`.** A
   mirrored, enterprise, side-load, or degraded profile that keeps an install-ready claim is
   over-reaching and blocks.
3. **CLI/export parity is always-on.** Every row must keep text / JSON / Markdown
   reconstruction so support and automation can rebuild the source class, compatibility,
   host model, permission posture, transitive widening, activation-budget band, disable
   scope, rollback compatibility, publisher continuity, and quarantine history from the same
   artifact identity the user saw.
4. **The four B131 guardrails hold on every row** (each must be `false`):
   - `hides_permission_widening_or_activation_cost`
   - `hides_publisher_transfer_disable_scope_or_rollback_incompatibility`
   - `collapses_registry_source_class_across_public_mirrored_enterprise`
   - `presents_incompatible_or_over_budget_as_ready`
5. **One canonical bundle.** Every profile cites the single frozen marketplace-install proof
   bundle rather than cloning per-profile evidence.
6. **Metadata-only export.** Raw manifest bodies, permission tokens, and activation-budget
   payloads never cross the boundary.

`derived_status` is never authored — it is recomputed from the axis outcomes, guardrails,
and claim narrowing, and validation rejects any stored status that disagrees with a fresh
derivation.

## Regenerating the artifacts

```
GEN_MARKETPLACE_INSTALL_CERT_ARTIFACTS=1 cargo test -p aureline-shell \
  certify_marketplace_and_install_review_component_truth_on_every_claimed_m5_public_mirrored_and_enterprise_registry_profile::tests::generate_artifacts
```

The checked-in support export is the `include_str!` canonical shared by the tests; a drift
between the seeded builder and the on-disk artifact fails
`checked_in_export_matches_seeded_builder`.
