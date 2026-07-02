# M5 Settings-Row, Capability-Sheet, Evidence-Chronology, and Chronology-Export Component Matrix

- Packet: `m5-trust-chronology-components:stable:0001`
- Label: `M5 settings-row, capability-sheet, evidence-chronology, and chronology-export component matrix`
- Component families: 6 (6 stable)
- Chronology verbs: created, updated, ran, approved, rejected, failed, recovered, exported
- Chronology export fields: event_verb, provenance, timestamp, object_ref, actor_role, outcome_code, redaction_class
- Proof freshness SLO: 720 hours (last refresh: 2026-06-30T00:00:00Z)

## Component families

- **settings_row**: `stable`
  - Owner: Settings/config component owner
  - Scope: One settings-row model carrying effective-versus-configured truth: it shows the effective value, names the source pill that produced it, explains lock and pending-reload states, holds an invalid value without applying it, and redacts credential-managed values — never conflating effective with configured
  - Shell zone: `main_workspace`
  - Required labels: identity, state, keyboard_route, effective_value, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **capability_sheet**: `stable`
  - Owner: Policy/capability component owner
  - Scope: One capability-sheet model grouping permission requests by consequence class rather than by a flat permission list; it shows transitive downstream scope, supports reduced-scope grants, requires re-consent when scope changes, and keeps revocations in history
  - Shell zone: `transient_overlay`
  - Required labels: identity, state, keyboard_route, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **event_history_row**: `stable`
  - Owner: Activity/evidence component owner
  - Scope: One event/history row model using the stable, closed verb vocabulary and attributing a provenance badge on every event so a human, AI, automation, or remote action is never conflated; its detail stays reopenable and its truth stays in the support export
  - Shell zone: `bottom_panel`
  - Required labels: identity, state, keyboard_route, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **timeline_group**: `stable`
  - Owner: Activity/evidence component owner
  - Scope: One timeline-group model that collapses related events under one heading using the same stable verbs and provenance badges; it groups by object or by time, discloses any filter, and keeps every grouped detail reopenable from durable history
  - Shell zone: `bottom_panel`
  - Required labels: identity, state, keyboard_route, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **narrative_summary_card**: `stable`
  - Owner: Activity/evidence component owner
  - Scope: One narrative-summary-card model that summarizes a chronology span in prose without inventing new verbs: it reuses the stable verb vocabulary and provenance badges, discloses its grouping, and always keeps a reopen path back into the underlying events
  - Shell zone: `right_inspector`
  - Required labels: identity, state, keyboard_route, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
- **chronology_export_preview**: `stable`
  - Owner: Support/export component owner
  - Scope: One chronology-export-preview model that shows exactly which fields will leave the trust boundary — the stable verb, provenance, timestamp, object ref, actor role, outcome code, and redaction class — so an export never silently drops a truth-bearing column and every previewed row is reconstructable from the support export
  - Shell zone: `transient_overlay`
  - Required labels: identity, state, keyboard_route, provenance, audit_reopen_path
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, pointer_optional, high_contrast_safe, support_exportable
