# External Alpha Known Limits

This packet is the canonical known-limits surface for external alpha design
partners. It narrows the partner guide, intake packet, task pack, feedback
taxonomy, support exports, and public-proof packets to the same scope as the
alpha wedge matrix.

## Canonical Inputs

- Alpha scope matrix: `artifacts/milestones/m2/alpha_wedge_matrix.yaml`
- Alpha go/no-go scoreboard: `artifacts/milestones/m2/exit_gate_scoreboard.yaml`
- Intake packet: `artifacts/milestones/m2/design_partner_intake_packet.md`
- Task pack: `artifacts/milestones/m2/design_partner_task_pack.md`
- Partner guide: `docs/alpha/design_partner_guide.md`
- Feedback taxonomy: `artifacts/feedback/design_partner_feedback_taxonomy.yaml`
- Upstream intake checklist: `artifacts/program/design_partner_intake_checklist.yaml`
- Known-limits contract: `docs/product/known_limits_contract.md`
- Launch bundle manifests: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`, `artifacts/bundles/python_launch_bundle_alpha.yaml`
- Archetype seed rows: `artifacts/certification/m2_archetype_seed_rows.yaml`

## Active Known Limits

| Known-limit id | Class | Severity | Applies to | Partner-facing effect |
|---|---|---|---|---|
| `known_limit:external_alpha.scope.claimed_wedges_only` | `workflow_narrowed` | `major` | `alpha_wedge:typescript_javascript`, `alpha_wedge:python` | Only TypeScript / JavaScript web app or service and Python service or data app workflows are alpha claims. Other language requests route to scope review. |
| `known_limit:external_alpha.deployment.local_or_helper_only` | `platform_narrowed` | `major` | local desktop and helper-backed local rows | Managed-cloud daily-driver parity is not an alpha claim. |
| `known_limit:external_alpha.notebook_handoff_only` | `workflow_narrowed` | `moderate` | Python debug/refactor tasks | Python notebook behavior is limited to handoff disclosure. Full notebook parity is not claimed. |
| `known_limit:external_alpha.browser_mobile_companion_out` | `platform_narrowed` | `major` | browser and mobile companion surfaces | Browser or mobile companion parity is out of scope for alpha task completion. |
| `known_limit:external_alpha.support_export_redaction_required` | `support_export_narrowed` | `blocking` | support exports, traces, screenshots, logs, transcripts | Privacy-sensitive artifacts must pass redaction review before partners are asked to share them. |
| `known_limit:external_alpha.migration_evidence_seeded` | `migration_path_limited` | `moderate` | migration, import, and first-run parity reports | Migration feedback may enter alpha evidence, but it cannot widen switching claims without a current migration packet. |
| `known_limit:external_alpha.no_raw_partner_content` | `corpus_narrowed` | `blocking` | partner repositories, support bundles, traces, screenshots, terminal transcripts | Raw partner content is never required for initial filing and must not enter public packets without clearance. |
| `known_limit:external_alpha.launch_bundle_seed_not_certified` | `workflow_narrowed` | `major` | launch bundles, archetype badges, Start Center bundle gallery, mirror/offline install review | Bundle rows are setup seeds only. Badges must open the underlying evidence packet and may not imply certified or replacement-grade status. |

## Routing Rules

- Feedback category `scope_or_known_limit` must cite at least one known-limit id
  from this packet.
- Feedback category `privacy_redaction` must cite
  `known_limit:external_alpha.support_export_redaction_required` or
  `known_limit:external_alpha.no_raw_partner_content`.
- Feedback category `support_export_or_diagnostics` must cite a support-export
  known limit when the export cannot reconstruct the task safely.
- Any request for managed cloud, browser/mobile companion parity, full notebook
  parity, or a new language wedge opens scope review. It is not treated as a
  failed alpha task.
- Any launch-bundle or archetype badge must open the linked manifest,
  certification seed row, or proof packet. It must not behave as an ungrounded
  marketing label.

## Rollback Posture

When a report proves a task is outside the current claim surface, downstream
packets narrow the claim before widening the task pack. The scoreboard row stays
blocked or conditional until fresh evidence, updated known-limit refs, and a
passing validator capture land together.
