# M5 Opacity / Scrim and Overlay-Depth Registries

- Packet: `m5-opacity-scrim-and-overlay-depth-registries:stable:0001`
- Label: `M5 opacity / scrim and overlay-depth registries with canonical lightweight versus blocking depth classes, reduced-motion / power-saver / thermal clamp coverage, orientation-and-text-contrast preservation for blocking modal / sheet / confirm / wizard / credential surfaces, and one shared z-order model no private overlay bypasses across shell, dialog, panel, embedded, notification, and support surfaces`
- Consumer surfaces: 6
- Overlay depth classes: blocking_modal_dialog, blocking_sheet, blocking_confirm_scrim, blocking_wizard_step, blocking_credential_prompt, lightweight_tooltip, lightweight_popover, transient_toast, hover_preview, inline_drawer, side_panel, context_menu, status_hud, depth_class_unclassified
- Runtime clamps: reduced_motion, power_saver, thermal
- Proof freshness SLO: 720 hours (last refresh: 2026-07-13T00:00:00Z)

## Consumer surfaces

- **shell_ui**: `stable`
  - Owner: Shell surface owner
  - Scope: The shell resolves the confirm scrim through the canonical blocking grammar and keeps the workspace orientable; a scrim that erases orientation and a private overlay that bypasses the shared z-order degrade honestly instead of reading as a clean pass
  - Scrim entries: 2 / overlay-depth entries: 2
- **editor_ui**: `stable`
  - Owner: Editor surface owner
  - Scope: The editor renders blocking sheets and lightweight popovers with a solid panel behind text and full clamp coverage; a clamp-incomplete scrim and a detached overlay that does not stack under the shared model both degrade honestly
  - Scrim entries: 3 / overlay-depth entries: 2
- **onboarding_ui**: `stable`
  - Owner: Onboarding surface owner
  - Scope: The onboarding wizard blurs its backdrop with a contrast floor while keeping orientation and tracing each token to the canonical scrim system; a missing contrast treatment and a clamp-incomplete overlay depth degrade honestly
  - Scrim entries: 2 / overlay-depth entries: 2
- **marketplace_ui**: `stable`
  - Owner: Marketplace / embedded surface owner
  - Scope: The embedded dialog surface consumes the canonical blocking modal scrim and traces every token to the scrim system; a raw-opacity scrim and an unclassified overlay depth degrade honestly
  - Scrim entries: 2 / overlay-depth entries: 2
- **settings_ui**: `stable`
  - Owner: Settings surface owner
  - Scope: The settings and notification surfaces render the credential prompt and transient toast under the shared z-order model with a high-contrast border; an unclassified scrim depth and an unstated overlay token degrade honestly instead of stacking outside the grammar
  - Scrim entries: 2 / overlay-depth entries: 3
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved scrim and overlay-depth truth, so an orientation erasure or a private z-order bypass is visible in evidence rather than hidden behind an opacity value
  - Scrim entries: 2 / overlay-depth entries: 2
