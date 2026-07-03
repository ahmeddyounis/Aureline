# M5 Security-Advisory Card / Row Primitive: Severity, Affected Surface, Exposure, and Primary-Action Parity

- Packet: `m5-advisory-card-row-primitive:stable:0001`
- Label: `M5 security-advisory card / row primitive: severity, affected surface, exposure state, fixed version or mitigation, signer / source truth, and primary-action parity across channels`
- Affected-surface lanes: 6 (6 stable)
- Anatomy parts: advisory_id, severity, affected_surface, current_exposure, fixed_version_or_mitigation, signer_source_state, primary_action
- Severity classes: informational, low, moderate, high, critical, operational_emergency
- Channels: update_center, marketplace, help_about, support_bundle
- Export fields: advisory_id, severity, action_state, affected_surface, mitigation_state, delivery_profile, freshness_state, continuity_note, disclosure_visibility, history_state
- Proof freshness SLO: 720 hours (last refresh: 2026-06-30T00:00:00Z)

## Affected-surface lanes

- **Desktop App**: `stable`
  - Owner: Desktop app security owner
  - Scope: The desktop-app lane renders the shared advisory row so a critical, installed-and-exposed runtime vulnerability shows severity, affected object, `exposed` state, the fixed version, the signer state, and `update_to_fixed_version` inline — no detail drawer, no generic update banner
  - Shell zone: `activity_rail`
  - Worked advisories: 1
    - `AURELINE-ADV-2026-0101` — critical (exposed), installed-but-affected, row stays visible
- **Extension**: `stable`
  - Owner: Marketplace / extension trust owner
  - Scope: The extension lane renders the shared advisory row so a high-severity, installed-but-blocked extension keeps its row, reads `contained_by_block`, discloses the unsigned distribution, and offers `disable_or_remove` — the installed-but-affected item never disappears
  - Shell zone: `activity_rail`
  - Worked advisories: 1
    - `AURELINE-ADV-2026-0102` — high (contained_by_block), installed-but-affected, row stays visible
- **Remote Helper**: `stable`
  - Owner: Remote-connector trust owner
  - Scope: The remote-helper lane renders the shared advisory row so a moderate, installed-and-awaiting-rollback helper keeps its row, reads `awaiting_rollback`, discloses the mirror lag, and offers `rollback_or_repin` while local continuity is pending the fix
  - Shell zone: `activity_rail`
  - Worked advisories: 1
    - `AURELINE-ADV-2026-0103` — moderate (awaiting_rollback), installed-but-affected, row stays visible
- **Managed Service**: `stable`
  - Owner: Managed-service governance owner
  - Scope: The managed-service lane renders the shared advisory row so an operational-emergency, installed-but-disabled service keeps its row, reads `contained_by_disable`, states `no_safe_local_continuity`, and routes to `contact_admin` instead of a generic update prompt
  - Shell zone: `activity_rail`
  - Worked advisories: 1
    - `AURELINE-ADV-2026-0104` — operational_emergency (contained_by_disable), installed-but-affected, row stays visible
- **Docs Artifact**: `stable`
  - Owner: Docs / knowledge integrity owner
  - Scope: The docs-artifact lane renders the shared advisory row so a low-severity, superseded advisory reads `resolved` with `mitigation_complete`, keeps a signed-snapshot signer state, and states local use is unaffected — resolved advisories stay visible as history
  - Shell zone: `activity_rail`
  - Worked advisories: 1
    - `AURELINE-ADV-2026-0105` — low (resolved)
- **Signing / Update Path**: `stable`
  - Owner: Signing / update path owner
  - Scope: The signing-update-path lane renders the shared advisory row so an informational, not-installed advisory reads `not_affected` with a review action, and a moderate, mitigated-in-place advisory reads `mitigated_in_place` while disclosing offline-mirror lag and offering a support-packet export
  - Shell zone: `activity_rail`
  - Worked advisories: 2
    - `AURELINE-ADV-2026-0106` — informational (not_affected)
    - `AURELINE-ADV-2026-0107` — moderate (mitigated_in_place)
