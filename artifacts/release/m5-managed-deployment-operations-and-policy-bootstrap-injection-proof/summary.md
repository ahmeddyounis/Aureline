# M5 Managed-Deployment Operations and Policy-Bootstrap-Injection Registries

- Packet: `m5-managed-deployment-operations-and-policy-bootstrap-injection:stable:0001`
- Label: `M5 managed-deployment operations and policy-bootstrap-injection registries enforcing silent install / uninstall / repair-or-verify / channel-pinning / update-deferral operations with a complete copyable receipt inventory of install ID / timestamp / failure summary / repair-verify receipt, a never-user-controlled managed installer, explicit admin-versus-user ownership, published policy-bootstrap injection, and documented channel-pin / update-deferral continuity across the installer, update, diagnostics, admin, docs, and support surfaces`
- Consumer surfaces: 6
- Managed operations: silent_install, silent_uninstall, repair_or_verify, channel_pin, update_defer, operation_unclassified
- Presentation forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer surfaces

- **installer**: `stable`
  - Owner: Installer/silent-flow owner
  - Scope: The installer resolves the silent-install operation to one inspectable object — operation, operation-target / receipt / failure-diagnostics roots, and the copyable install-ID / timestamp / failure-summary / repair-verify receipt — from the shared registry and reads the managed-policy injection; a receipt that omits the repair/verify confirmation and an injection that hides the deferral window and admin ownership degrade honestly instead of reading as a clean pass
  - Operation entries: 2 / injection entries: 2
- **updater_service**: `stable`
  - Owner: Updater/update-deferral owner
  - Scope: The updater resolves the update-deferral operation and the managed-channel injection; a managed installer presented as user-controlled and an undocumented channel-pin / update-deferral continuity note are caught before a managed update can hide admin ownership or drift its pinning semantics
  - Operation entries: 2 / injection entries: 2
- **diagnostics**: `stable`
  - Owner: Diagnostics surface owner
  - Scope: Diagnostics reports the repair-or-verify operation and its published injection without manual reconstruction; an operation whose ownership is ambiguous — so a failure would strand the user without knowing who owns it — is caught instead of reading as a clean pass
  - Operation entries: 2 / injection entries: 1
- **admin**: `stable`
  - Owner: Admin surface owner
  - Scope: Admin resolves the channel-pin operation while preserving one registry-bound source; a hand-copied per-profile assumption and an injection record on an unclassified surface degrade honestly
  - Operation entries: 2 / injection entries: 2
- **docs_help**: `stable`
  - Owner: Docs/help surface owner
  - Scope: Docs and help render the same resolved managed-operation and published-injection truth the resolvers produced across the canonical, accessible, and audit presentation forms rather than a hand-copied receipt table
  - Operation entries: 2 / injection entries: 2
- **support_export**: `stable`
  - Owner: Support/export owner
  - Scope: The support export carries the same resolved managed-operation and injection truth, so a hand-copied constant, an unstated registry token, an ambiguous ownership, or a managed installer presented as user-controlled is visible in evidence rather than hidden behind a screenshot
  - Operation entries: 2 / injection entries: 1
