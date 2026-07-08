# M5 Support-Class, Evidence-Freshness, Lifecycle, Channel, Deployment-Scope, Compatibility-State, and Explanation-Drawer Badge Matrix

- Packet: `m5-badge-family:stable:0001`
- Label: `M5 support-class, evidence-freshness, lifecycle, channel, deployment-scope, compatibility-state, and explanation-drawer badge matrix`
- Badge families: 6 (6 stable)
- Axis-separation rules: support_class_does_not_imply_freshness, deployment_scope_does_not_imply_lifecycle, lifecycle_does_not_imply_channel, channel_does_not_imply_support_class, compatibility_does_not_imply_support_class, freshness_does_not_imply_compatibility
- Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
- Proof freshness SLO: 720 hours (last refresh: 2026-07-08T00:00:00Z)

## Badge families

- **support_class**: `stable`
  - Owner: Support-class badge owner
  - Scope: One support-class badge naming how supported a thing is — certified, fully supported, community supported, best effort, deprecated, or unsupported — so a support posture is always explicit and never implies anything about evidence freshness
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
- **evidence_freshness**: `stable`
  - Owner: Evidence-freshness badge owner
  - Scope: One evidence-freshness badge naming how fresh the proof behind a claim is — fresh, recent, aging, stale, expired, or unverified — so stale or unverified evidence is never presented as fresh and freshness stays independent of support class
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
- **lifecycle**: `stable`
  - Owner: Lifecycle badge owner
  - Scope: One lifecycle badge naming the lifecycle stage of a thing — stable, beta, preview, experimental, maintenance, or end-of-life — so the stage is always explicit and never stands in for a channel or a support class
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
- **channel**: `stable`
  - Owner: Channel badge owner
  - Scope: One channel badge naming which release channel a thing rides — stable, beta, nightly, edge, LTS, or custom — so the channel is always explicit and never implies a support class
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
- **deployment_scope**: `stable`
  - Owner: Deployment-scope badge owner
  - Scope: One deployment-scope badge naming where a thing runs — desktop-only, local OSS, self-hosted, managed, air-gapped, or mirror-offline — so the scope is always explicit and never implies an experimental or lower lifecycle stage
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
- **compatibility_state**: `stable`
  - Owner: Compatibility-state badge owner
  - Scope: One compatibility-state badge naming how compatible a thing is with the host — compatible, minor skew, major skew, incompatible, migration required, or unknown — so skew and required migrations are never hidden
  - Required labels: identity, value_state, axis_name, explanation_drawer, evidence_source, filter_key
  - Explanation fields: what_it_means, why_shown, what_changes_it, evidence_source, how_to_improve, last_evaluated
  - Accessibility routes: keyboard_focusable, screen_reader_announced, non_hover_reachable, non_color_encoded, high_contrast_safe, support_exportable
