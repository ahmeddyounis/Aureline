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
- Migration parity scoreboard: `artifacts/migration/m2_parity_scoreboard.yaml`
- Import-gap taxonomy: `artifacts/migration/import_gap_taxonomy.yaml`
- Retained import diagnostics packet: `docs/migration/import_diagnostics_packet.md`
- Machine-readable known-limits packet: `artifacts/milestones/m2/known_limits_alpha.yaml`
- Reference-workspace dry run: `artifacts/milestones/m2/reference_workspace_dry_run.md`
- Benchmark/publication rehearsal: `artifacts/benchmarks/m2_publication_rehearsal.md`

## Active Known Limits

| Known-limit id | Class | Severity | Applies to | Partner-facing effect |
|---|---|---|---|---|
| `known_limit:external_alpha.scope.claimed_wedges_only` | `workflow_narrowed` | `major` | `alpha_wedge:typescript_javascript`, `alpha_wedge:python` | Only TypeScript / JavaScript web app or service and Python service or data app workflows are alpha claims. Other language requests route to scope review. |
| `known_limit:external_alpha.deployment.local_or_helper_only` | `platform_narrowed` | `major` | local desktop and helper-backed local rows | Managed-cloud daily-driver parity is not an alpha claim. |
| `known_limit:external_alpha.notebook_handoff_only` | `workflow_narrowed` | `moderate` | Python debug/refactor tasks | Python notebook behavior is limited to handoff disclosure. Full notebook parity is not claimed. |
| `known_limit:external_alpha.browser_mobile_companion_out` | `platform_narrowed` | `major` | browser and mobile companion surfaces | Browser or mobile companion parity is out of scope for alpha task completion. |
| `known_limit:external_alpha.support_export_redaction_required` | `support_export_narrowed` | `blocking` | support exports, traces, screenshots, logs, transcripts | Privacy-sensitive artifacts must pass redaction review before partners are asked to share them. |
| `known_limit:external_alpha.migration_evidence_seeded` | `migration_path_limited` | `moderate` | migration, import, and first-run parity reports | Migration feedback may enter alpha evidence, but it cannot widen switching claims without a current migration packet. |
| `known_limit:external_alpha.migration_bridged_parity_not_native` | `migration_path_limited` | `moderate` | extension/provider continuity, bridge-backed rows, native-alternative recommendations | Bridge-backed or replacement rows must stay labeled as bridge/replacement continuity and must not be described as native parity. |
| `known_limit:external_alpha.migration_lossy_mapping_visible` | `migration_path_limited` | `moderate` | keymaps, launch/debug configs, theme/token mappings, settings aliases | Lossy mappings are allowed only when the original source, mapped target, caveat, and retained diagnostics remain visible. |
| `known_limit:external_alpha.migration_unsupported_runtime_explicit` | `migration_path_limited` | `major` | webview-heavy extensions, arbitrary Lua runtime, arbitrary Elisp runtime, plugin-managed source state | Unsupported runtime or plugin state remains visible as unsupported and cannot be hidden under a broad import-success claim. |
| `known_limit:external_alpha.migration_manual_followup_required` | `migration_path_limited` | `major` | imported tasks, run/debug configs, execution-context mappings, conflict rows | Manual follow-up rows must remain open until the user accepts, edits, rejects, or rolls them back; they cannot count as migrated workflow parity. |
| `known_limit:external_alpha.no_raw_partner_content` | `corpus_narrowed` | `blocking` | partner repositories, support bundles, traces, screenshots, terminal transcripts | Raw partner content is never required for initial filing and must not enter public packets without clearance. |
| `known_limit:external_alpha.launch_bundle_seed_not_certified` | `workflow_narrowed` | `major` | launch bundles, archetype badges, Start Center bundle gallery, mirror/offline install review | Bundle rows are setup seeds only. Badges must open the underlying evidence packet and may not imply certified or replacement-grade status. |
| `known_limit:external_alpha.reference_workspace_dry_run_synthetic_only` | `corpus_narrowed` | `moderate` | reference-workspace dry runs, benchmark packets, public-proof rehearsals | The first dry run uses synthetic described-byte fixtures, not partner repositories or materialized benchmark workspaces. |
| `known_limit:external_alpha.publication_rehearsal_methodology_only` | `competitor_parity_narrowed` | `major` | benchmark/publication rehearsal, public-proof packets, release evidence | The rehearsal is methodology-only and cannot publish benchmark, competitor-comparison, certified, or replacement-grade claims. |
| `known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only` | `corpus_narrowed` | `moderate` | quick open, symbol search, ranking-reason cards, keyboard review | Search alpha validation covers protected synthetic fixtures and partial-index drills. It is not partner-repository ranking proof, all-language parity, or complete graph explainer coverage. |

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
- Any migration issue below native parity must cite the parity scoreboard row,
  import-gap taxonomy row, retained diagnostics refs, and known-limit id rather
  than using screenshots or summary prose as the source of truth.
- Any reference-workspace dry-run report must cite
  `artifacts/milestones/m2/known_limits_alpha.yaml` and keep publication
  wording methodology-only until materialized benchmark and support-export proof
  exist.
- Any search alpha review or support export that cites ranking-reason,
  keyboard, or partial-index evidence must also cite
  `known_limit:external_alpha.search_alpha_synthetic_and_partial_index_only`
  until partner-repository ranking and full graph explainer evidence exists.

## Rollback Posture

When a report proves a task is outside the current claim surface, downstream
packets narrow the claim before widening the task pack. The scoreboard row stays
blocked or conditional until fresh evidence, updated known-limit refs, and a
passing validator capture land together.
