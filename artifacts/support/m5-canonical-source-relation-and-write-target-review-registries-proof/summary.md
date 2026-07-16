# M5 Canonical-Source-Relation and Write-Target-Review Registries

- Packet: `m5-canonical-source-relation-and-write-target-review-registries:stable:0001`
- Label: `M5 canonical-source-relation and write-target-review registries emitting one machine-readable retirement manifest per retiring supported line or stable-facing capability — one typed field per manifest section: the last-supported version / channel pinned to an exact build, the retirement trigger, the cutoff date, the successor reference, the disable path, and the export / rollback route — each bound to one object-class identity with its exact-build joins, so a retired class never disappears silently and no new install or new tenant can still select it, with canonical / accessible / audit resolution-form coverage, and a machine-readable manifest-change diff (cutoff-date-change, replacement-path-change, or disable-or-export-route-change) that turns a changed cutoff date or replacement path into a visible, typed diff event rather than a silent mutation across CLI, docs / help, partner-packet, and support-bundle surfaces`
- Consumer surfaces: 6
- Report sections: read_only_path_object, generated_artifact_object, policy_locked_object, managed_mirrored_object, projection_object, captured_snapshot_object, object_class_unclassified
- Resolution forms: canonical_object, accessible_summary, audit_record
- Proof freshness SLO: 720 hours (last audit: 2026-07-15T00:00:00Z)

## Consumer surfaces

- **tab_chrome**: `stable`
  - Owner: Release-center owner
  - Scope: The release center resolves a retiring class to one typed canonical-source-relation object — the object identity, last-supported version / channel pinned to an exact build, retirement trigger, cutoff date, successor reference, disable path, and export / rollback route — from the shared registry and proves the cutoff-date-change diff for that class; a manifest object missing its exact-build joins and a diff that keeps support language ahead of the closed support note degrade honestly instead of leaving a retired class to read as still supported
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **status_bar**: `stable`
  - Owner: Help/docs owner
  - Scope: Help / docs resolves the retirement-trigger field and the disable-or-export-route-change diff while keeping the active diff reason visible; a retiring class widening its claim without a preserved rollback / export route and a resolution-form gap on a diff are caught before a screenshot can reintroduce a still-supported reading
  - Correction-report entries: 2 / claim-history-diff entries: 2
- **support_export_packet**: `stable`
  - Owner: Support owner
  - Scope: Support resolves the successor-reference field while keeping its public-facing successor / exit claim matched to the closed support note and reports the write-target-review outcome; a manifest entry that is a hand-copied per-entry assumption and a diff on an unclassified diff scope degrade honestly
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **diff_review_header**: `stable`
  - Owner: Marketplace/registry owner
  - Scope: The marketplace / registry resolves the cutoff-date field and the replacement-path-change diff bound to the registry so a retired class can no longer be selected in a new install or by a new tenant; an unstated registry token on a manifest entry is caught before it can drift
  - Correction-report entries: 2 / claim-history-diff entries: 1
- **command_palette**: `stable`
  - Owner: Install/update owner
  - Scope: Install / update renders the same resolved canonical-source-relation and write-target-review truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied class table; the export / rollback-route field and the disable-or-export-route-change diff stay inspectable off-renderer so no new install can still select a retired class
  - Correction-report entries: 1 / claim-history-diff entries: 1
- **ai_automation_path**: `stable`
  - Owner: Partner/procurement owner
  - Scope: The partner / procurement feed carries the same resolved canonical-source-relation and write-target-review truth, so a hand-copied constant, an unstated registry token, a manifest widening its claim without a preserved rollback / export route, or support language running ahead of the closed support note is visible in evidence — a cutoff-date change, a replacement-path change, or a disable / export-route change — rather than hidden behind a screenshot
  - Correction-report entries: 1 / claim-history-diff entries: 1
