# M5 Post-Install Notice/Provenance, Community-Handoff, Reproduction-Packet, and Device-Permission/Auth-Boundary Matrix

- Packet: `m5-public-handoff-matrix:stable:0001`
- Label: `M5 Post-Install Notice/Provenance, Community-Handoff, Reproduction-Packet, and Device-Permission/Auth-Boundary Matrix`
- Objects: 8 (6 stable)
- Proof freshness SLO: 168 hours (last refresh: 2026-06-30T00:00:00Z)

## Objects

- **post_install_notice**: `stable`
  - Owner: Help/About owner
  - Scope: Post-install notice / provenance disclosure card that stays inspectable after install; it discloses how the build arrived (official, mirrored, side-loaded, or unknown) and its notice freshness, and never softens an unknown source into an implied official one
  - Vocabularies: provenance_class, notice_freshness_state
  - Rollback: notice_stays_inspectable_after_install
- **provenance_disclosure**: `stable`
  - Owner: Help/About owner
  - Scope: Provenance / source-authenticity disclosure that pins one provenance class and its notice freshness; the marketplace and About surfaces read the same provenance truth, and a degraded or unverified provenance narrows rather than implying authority
  - Vocabularies: provenance_class, notice_freshness_state
  - Rollback: provenance_labeled_never_implied
- **community_handoff_route**: `stable`
  - Owner: Ecosystem owner
  - Scope: Official-versus-community outbound route descriptor that declares route trust class, visibility, and support class before launch; a community destination is never presented as an official authenticated one, and a failed or blocked launch retains drafted material and falls back to a local save
  - Vocabularies: route_trust_class, continuity_state
  - Rollback: route_declares_visibility_before_launch
- **reproduction_packet**: `stable`
  - Owner: Supportability owner
  - Scope: Redaction-safe reproduction packet that is previewed and redacted before share; raw paths, hostnames, usernames, tokens, and diagnostics are excluded by default, the share is blocked until the preview is confirmed, and a failed handoff keeps the packet retained for a local save
  - Vocabularies: redaction_state, continuity_state
  - Rollback: redaction_preview_required_before_share
- **offline_capture_continuity**: `stable`
  - Owner: Supportability owner
  - Scope: Offline-capture continuity record proving capture survives a failed or blocked handoff; the captured material is saved locally with its redaction posture intact and an explicit open-later / retry action, so capture is never lost when a route cannot launch
  - Vocabularies: continuity_state, redaction_state
  - Rollback: offline_capture_saved_local
- **device_permission_boundary**: `beta`
  - Owner: Voice/capture owner
  - Scope: Device / microphone capture permission and capability-limit boundary; the surface states its permission state and stays within the granted capability scope, the capture chrome is clearly disclosed rather than impersonating native chrome, and a revoked or denied permission narrows the claim
  - Vocabularies: capture_permission_state, boundary_chrome_honesty
  - Rollback: capture_stays_within_granted_scope
- **embedded_auth_boundary**: `beta`
  - Owner: Browser/auth boundary owner
  - Scope: Embedded webview / auth boundary that never impersonates native trusted product chrome; it labels the embedded or external surface and its route trust class so credentials are never entered into a surface posing as native chrome, and an unattributed impersonation is blocked
  - Vocabularies: boundary_chrome_honesty, route_trust_class
  - Rollback: boundary_never_impersonates_native_chrome
- **service_health_notice**: `stable`
  - Owner: Service-health owner
  - Scope: Release / service-health communication notice that pins the destination route trust class and its notice freshness; the update and service-health surfaces read the same freshness truth, and a stale or unverified notice narrows rather than implying current service authority
  - Vocabularies: route_trust_class, notice_freshness_state
  - Rollback: route_declares_visibility_before_launch
