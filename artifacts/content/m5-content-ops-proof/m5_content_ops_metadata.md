# Content-Ops Metadata: Snippets, Headings, Captions, and Translator Notes

- Catalog: `m5-content-ops-metadata-catalog:stable:0001`
- Label: `Content-Ops Metadata for Docs/Help Snippets, Export/Report Headings, Captions, and Translator Notes`
- Reference locale: `en`
- Entries: 9
- Proof freshness SLO: 168 hours (last refresh: 2026-06-26T00:00:00Z)

## docs_help_snippet

- `entry.docs.project_doctor_findings` — Project Doctor checked your workspace and found {count} findings in {scope}. [docs_help_snippet] — source: glossary.term.project_doctor; command: command.project.doctor; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001
  - Consumers: docs_help, release_notes, support_export, cli_help
  - placeholder `{count}` (count / locale_formatted_value): The number of findings; pluralize the localized noun, not the token.
  - placeholder `{scope}` (glossary_term_token / controlled_vocabulary_translation): The controlled scope term the count applies to; resolve through the controlled glossary, not a translator-local synonym.
- `entry.docs.open_source_before_apply` — Open the source to review the proposed change before you apply it. [docs_help_snippet] — source: glossary.term.open_source; command: command.review.open_source; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001
  - Consumers: docs_help, cli_help, support_export

## export_report_heading

- `entry.heading.findings_by_severity` — Findings by severity [export_report_heading] — source: glossary.term.severity; command: command.report.generate; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; field: report.column.findings_by_severity
  - Consumers: support_export, release_notes, docs_help
- `entry.heading.findings_exported_count` — {count} findings exported [export_report_heading] — source: glossary.term.export; command: command.report.export; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; field: report.heading.findings_exported_count
  - Consumers: support_export, release_notes
  - placeholder `{count}` (count / locale_formatted_value): The number of exported findings; pluralize the localized noun, not the token.

## screenshot_demo_caption

- `entry.caption.activity_center_live` — Aureline shell showing the activity center. [screenshot_demo_caption] — source: string.shell.activity_center_title; command: command.window.activity_center; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; posture: live; sync: in_sync
  - Consumers: release_notes, docs_help, screenshot_demo_pipeline
- `entry.caption.demo_workspace_mocked` — Demo workspace populated with sample data. [screenshot_demo_caption] — source: string.demo.workspace_label; command: command.window.workspace; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; posture: mocked; sync: in_sync
  - Consumers: release_notes, screenshot_demo_pipeline
- `entry.caption.patch_review_synthetic` — Synthetic preview of the patch review surface. [screenshot_demo_caption] — source: string.review.patch_review_title; command: command.review.patch; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; posture: synthetic; sync: pending_review
  - Consumers: release_notes, screenshot_demo_pipeline, support_export

## translator_note

- `entry.note.project_doctor_findings` — Keep {count} adjacent to the localized noun and pluralize the noun, not the token; {scope} is a controlled glossary term and must use its glossary ref. [translator_note] — source: docs.copy.translation_safe_content_ops_contract; command: command.project.doctor; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; target: entry.docs.project_doctor_findings
  - Consumers: docs_help, support_export
  - placeholder `{count}` (count / locale_formatted_value): The number of findings; pluralize the localized noun, not the token.
  - placeholder `{scope}` (glossary_term_token / controlled_vocabulary_translation): The controlled scope term the count applies to; resolve through the controlled glossary, not a translator-local synonym.
- `entry.note.caption_build_governance` — Caption must name the capture build {build} and stay in sync with the source surface; it must not imply live truth without it. [translator_note] — source: docs.copy.translation_safe_content_ops_contract; command: command.window.activity_center; version: version.channel.stable.2026.06; build: build.m5.content_ops.0001; target: entry.caption.activity_center_live
  - Consumers: screenshot_demo_pipeline, release_notes, support_export
  - placeholder `{build}` (version_or_build_token / literal_unchanged): The capture build identity; preserve it verbatim so the caption ties back to a build.

## Cross-consumer entry reuse

- `entry.caption.activity_center_live`: docs_help, release_notes, screenshot_demo_pipeline
- `entry.caption.demo_workspace_mocked`: release_notes, screenshot_demo_pipeline
- `entry.caption.patch_review_synthetic`: release_notes, screenshot_demo_pipeline, support_export
- `entry.docs.open_source_before_apply`: cli_help, docs_help, support_export
- `entry.docs.project_doctor_findings`: cli_help, docs_help, release_notes, support_export
- `entry.heading.findings_by_severity`: docs_help, release_notes, support_export
- `entry.heading.findings_exported_count`: release_notes, support_export
- `entry.note.caption_build_governance`: release_notes, screenshot_demo_pipeline, support_export
- `entry.note.project_doctor_findings`: docs_help, support_export
