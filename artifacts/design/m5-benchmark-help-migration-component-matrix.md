# M5 Benchmark / Help / Migration Component Matrix

Record kind: `m5_benchmark_help_migration_component_matrix`
Schema version: `1`
Status: frozen for M5 first consumers

This matrix freezes the reusable component contracts used when Aureline explains
measured benchmark claims, build/provenance/service state, support packages,
migration import outcomes, and public/community handoff destinations. It is the
shared field and vocabulary source for the schemas in `schemas/ui/`, the fixtures
in `fixtures/ui/m5-benchmark-help-migration-components/`, and the release proof
packet in `artifacts/release/m5-benchmark-help-migration-proof/`.

The matrix is deliberately metadata-only. Components carry opaque refs,
controlled labels, freshness and downgrade states, and copy/export-safe evidence.
They do not carry raw benchmark traces, raw URLs, raw local paths, raw support
payloads, raw issue bodies, credentials, or private tenant/user identifiers.

## Source Bindings

| Component family | Canonical sources consumed by reference | First consumers |
| --- | --- | --- |
| Benchmark evidence card | `artifacts/benchmarks/m5-benchmark-governance.json`, `artifacts/benchmarks/m5-benchmark-proof-packet.json`, `fixtures/benchmarks/m5-benchmark-certification/manifest.yaml`, `schemas/benchmarks/m5-benchmark-proof-packet.schema.json` | Benchmark report, release notes, docs/help benchmark copy, support export, shiproom proof |
| About/service-health card | `schemas/about/about_card.schema.json`, `schemas/help/service-health-destination.schema.json`, `docs/help/m4/finalize-service-health-destination-truth.md`, `crates/aureline-service-health::finalize_service_health_destination_truth` | Help/About, service-health banner, service-health detail card, diagnostics, support export, release notes |
| Support package card | `schemas/support/m5-support-bundle-consent.schema.json`, `docs/help/support/m5-support-bundle-consent.md`, `artifacts/support/m5/m5-support-bundle-consent.json` | Support Center, CLI/headless export review, local save flow, formal support handoff, support export |
| Importer diff row | `schemas/migration/importer_outcome.schema.json`, `schemas/migration/import_diff_preview.schema.json`, `schemas/migration/import_rollback_checkpoint.schema.json`, `docs/migration/first_run_import_diff_and_rollback_contract.md` | First-run import preview, migration center, CLI/headless import report, docs/help migration guidance, support export |
| Community handoff tile | `schemas/help/m5-handoff-target.schema.json`, `artifacts/help/m5-community-handoff-targets.csv`, `artifacts/help/m5-community-handoff-proof/target_set.json`, `docs/help/m5_community_handoff_targets_contract.md` | Help/About handoff chooser, issue/report template, docs feedback, security disclosure, support handoff, local draft fallback |

## Controlled Labels

| Vocabulary | Values |
| --- | --- |
| `freshness_state` | `current`, `live`, `warm_cached`, `cached`, `mirrored`, `offline_pack`, `stale`, `stale_cache`, `policy_limited`, `retest_pending`, `expired`, `quarantined` |
| `downgrade_state` | `none`, `methodology_only`, `narrowed_to_internal`, `retest_pending`, `quarantined`, `cached_service_health`, `local_only_continuity`, `service_degraded`, `policy_limited`, `saved_local_only`, `send_blocked`, `bridge_required_import`, `community_owned_destination`, `unsupported` |
| `benchmark_evidence_source_class` | `lab_reference_run`, `self_capture`, `design_partner_result`, `community_report`, `imported_evidence`, `methodology_only` |
| `service_contract_state` | `ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, `unavailable` |
| `support_package_state` | `review_ready`, `narrowed_review`, `send_blocked`, `saved_local_only`, `submitted`, `stale_schema` |
| `importer_outcome_state` | `imported`, `mapped`, `skipped`, `manual_review`, `bridge_required`, `unsupported` |
| `destination_group` | `help`, `release`, `migration`, `support`, `community`, `extension_vendor`, `local_only` |
| `destination_trust_class` | `official_public`, `official_authenticated`, `community`, `private_security`, `vendor_managed`, `local_only` |
| `copy_format` | `text`, `json`, `markdown` |

## Component Field Sets

### Benchmark Evidence Card

Required fields:

| Field | Contract |
| --- | --- |
| `card_id` | Stable card id quoted by docs/help, release proof, and support export. |
| `benchmark_id` | Stable benchmark/run id preserved in every copy/export and trace/report handoff. |
| `claim_ref` | Opaque ref to the benchmark claim/publication row. |
| `claim_scope` | One of `methodology_only`, `aureline_only_reference`, `head_to_head_comparison`, or `workflow_claim`. |
| `evidence_source_class` | `lab_reference_run`, `self_capture`, `design_partner_result`, `community_report`, `imported_evidence`, or `methodology_only`; self/design-partner/community/imported evidence cannot render as lab/reference proof. |
| `workflow_ref` | Workflow or benchmark suite being claimed. |
| `budget_ref` | Runtime, power, thermal, variance, or rerun budget ref. |
| `measured_value_repr`, `budget_value_repr` | Human-readable measured value versus budget value shown together on the card and preserved in exports. |
| `corpus_ref` | Corpus or corpus-manifest ref. |
| `hardware_or_capture_ref` | Reference hardware profile, lab image, or self-capture source. |
| `cold_warm_state` | Cold, warm, mixed, or not-applicable run state. |
| `sample_size` | Number of samples/runs behind the visible metric. |
| `extension_set_ref` | Extension set used for the run or capture. |
| `power_mode` | Plugged-in, battery, low-power, performance, managed-policy, or unknown power mode. |
| `execution_scope` | Local-only, remote-attached, managed-remote, or mixed execution scope. |
| `as_of_date` | Date the card's benchmark truth was current as `YYYY-MM-DD`. |
| `metric_rows` | Metric rows with value labels and comparison basis; rows may render `not_comparable` without disappearing. |
| `compare_view` | Compare-mode, baseline ref, comparable flag, comparison basis, and caveat refs for compare views. |
| `freshness_state` | Current/stale/retest posture. |
| `downgrade_state` | Explicit downgrade or narrowing reason. |
| `degraded_state` | Explicit degraded state such as `stale_benchmark_evidence` or `self_capture_only`. |
| `downgrade_banner` | Visible downgrade banner state; stale, incomparable, non-lab, unverified, or quarantined evidence must show the banner. |
| `caveat_summary_refs` | Caveat summary refs that must survive copy/export outside the product. |
| `trace_report_export` | Trace/report export refs and booleans proving benchmark id, caveats, workflow/budget truth, and environment truth are included. |
| `copy_export` | Text, JSON, and Markdown copy with export fields sufficient to reconstruct the claim without a screenshot. |

Degraded states:

- `stale_benchmark_evidence`
- `missing_reproduction_pack`
- `incomparable_hardware`
- `narrowed_corpus`
- `self_capture_only`
- `design_partner_limited`
- `community_unverified`
- `imported_evidence_unverified`

### About / Service-Health Card

Required fields:

| Field | Contract |
| --- | --- |
| `card_id` | Stable card id. |
| `card_family` | `about_summary`, `service_health_banner`, or `service_health_status_card`. |
| `build_identity_ref` | Exact build/provenance ref when applicable. |
| `service_family` | Service or local-core family being described. |
| `service_contract_state` | Stable service contract state; local-only and stale are first-class states. |
| `source_trust_class` | `official`, `mirrored_official`, `local_only`, `managed_service`, `community_owned`, `vendor_managed`, or `unknown`. |
| `freshness_state` | `live`, `cached`, `mirrored`, `offline_pack`, `stale_cache`, or `policy_limited`. |
| `local_continuity_state` | `available`, `narrowed`, `unavailable`, or `not_applicable`. |
| `downgrade_state` | Explicit cache, local-only, service-degraded, policy-limited, or unavailable state. |
| `build_summary` | Version, channel, install mode, provenance state, open/local boundary note, and a copy-build-info action that is local, unauthenticated, and does not open a browser. |
| `service_health_summary` | Affected service family, current contract state, cached freshness, local workflows that still work, managed features affected, diagnostics action, and export action. |
| `copy_export` | Copy-safe status and refs for Help/About, diagnostics, and support export. |

Degraded states:

- `cached_service_health`
- `local_only_continuity`
- `service_degraded`
- `policy_limited`
- `unavailable`

### Support Package Card

Required fields:

| Field | Contract |
| --- | --- |
| `package_id` | Stable package/card id. |
| `support_package_ref` | Opaque support package, review sheet, or saved packet ref. |
| `package_state` | `review_ready`, `narrowed_review`, `send_blocked`, `saved_local_only`, `submitted`, or `stale_schema`. |
| `destination_class` | Local, vendor, user-upload, managed-admin, private-security, or official-support destination. |
| `trust_class` | The trust/ownership class of the destination. |
| `local_save_state` | Whether local save is available, already saved local-only, or unavailable. |
| `redaction_state` | Default-safe, policy-narrowed, user-broadened, high-risk blocked, or stale-schema state. |
| `included_counts`, `excluded_counts`, `policy_locked_counts` | Counts by diagnostic data class. |
| `inspect_before_submit_required` | Must be true for any off-machine destination. |
| `package_contents` | Controlled list of included packet sections such as build info, service-health snapshot, diagnostic summary, redaction manifest, import preview, migration diff, policy receipts, local log metadata, or reproduction steps. |
| `local_save_summary` | Whether the packet is saved to the local store, the local packet ref, current submit state, and proof that local inspection does not require auth. |
| `redaction_export_summary` | Redaction state, local/off-machine export state, high-risk exclusion posture, and visible policy-locked exclusions. |
| `submit_later_summary` | Later destination class and guarantees that submission would require explicit user action and inspection, while the current card is not a submission and does not open a browser before local review. |
| `copy_export` | Reviewable text/JSON/Markdown projection; local-only packages copy as saved local evidence, not submitted support. |

Degraded states:

- `saved_local_only`
- `send_blocked`
- `stale_schema`
- `policy_narrowed`
- `blocked_high_risk`

### Importer Diff Row

Required fields:

| Field | Contract |
| --- | --- |
| `row_id` | Stable row id reused by preview, migration center, support, and export. |
| `migration_session_ref` | Durable migration session ref. |
| `source_profile_ref` | Source profile/install ref; never raw local path. |
| `target_ref` | Target profile/workspace/domain ref. |
| `migration_domain` | Governed migration domain. |
| `source_object_ref`, `source_value` | Export-safe source object id and the source value shown in review/export. |
| `target_object_ref`, `target_value` | Export-safe target object id and value when one exists; skipped and unsupported rows may carry null target truth but cannot hide the row. |
| `translated_result` | The target-side imported, mapped, skipped, review, bridge, or unsupported result shown to the reviewer. |
| `outcome_state` | `imported`, `mapped`, `skipped`, `manual_review`, `bridge_required`, or `unsupported`. |
| `compatibility_state` | Compatible, native alternative, bridge-required, manual-review, unsupported, policy-blocked, or insufficient-evidence state. |
| `mapping_basis` | Exact, semantic, capability-based, bridge adapter, heuristic, user override, or not applicable. |
| `reason_class`, `reason_detail_note` | Stable typed reason plus export-safe detail note explaining why the row landed in the outcome group. |
| `lossy_mapping` | True when the translated result narrows or changes source behavior; lossy rows remain visible after apply and in support export. |
| `manual_review_action`, `docs_action` | The row's review action and docs/help action; manual-review rows must carry a required review action and bridge rows must carry a bridge action. |
| `checkpoint_context` | Checkpoint and restore posture; rows that mutate durable truth name checkpoint and restore refs. |
| `degraded_state` | Explicit bridge, checkpoint, restore, compatibility, source-read, or policy degraded state. |
| `export_safe_identifiers` | Stable row, session, source, target, and support/export packet refs that can be copied without raw paths or private payloads. |
| `post_apply_summary_visible`, `support_export_visible` | Booleans proving skipped, lossy, manual-review, bridge-required, and unsupported rows do not disappear once the wizard closes. |
| `copy_export` | Row-level text/JSON/Markdown export; outcome vocabulary must survive support export. |

Grouped review-table fields:

| Field | Contract |
| --- | --- |
| `outcome_group_order` | Stable render order: `imported`, `mapped`, `skipped`, `manual_review`, `bridge_required`, `unsupported`. |
| `groups` | One visible/exported table group per outcome, even when the group is used by a different first consumer. |
| `rows` | Reusable importer diff rows; feature teams consume these instead of inventing ad hoc mapped-versus-unsupported tables. |
| `post_apply_summary` | Visible and exported outcome states plus explicit lossy, skipped, bridge-required, and unsupported row refs. |
| `shortcut_change_digest` | Dedicated post-apply digest for high-frequency shortcut/keybinding changes, kept separate from general settings diffs and included in support export. |
| `bridge_detail_inspectors` | Bridge-required row inspectors with bridge refs, compatibility report refs, and issue-template refs that remain available after apply. |
| `compatibility_report_links` | Post-import compatibility report links for bridge-required and unsupported rows, visible after apply and in support export. |
| `issue_template_export` | Text/JSON/Markdown issue-template export that includes bridge details, compatibility report links, partial apply state, and restore summary. |
| `partial_apply_summary` | Honest post-apply state with completed, unresolved, downgraded, and blocked row refs; partial/downgraded imports cannot render as generic completion. |
| `restore_summary` | Restore availability, checkpoint refs, restore refs, and restore scope note preserved after apply, downgrade, and support export. |

Degraded states:

- `bridge_required_import`
- `checkpoint_missing`
- `restore_unavailable`
- `compatibility_unknown`
- `source_unreadable`
- `policy_blocked`

### Community Handoff Tile

Required fields:

| Field | Contract |
| --- | --- |
| `tile_id` | Stable tile id for Help/About and issue/report consumers. |
| `destination_group` | Grouped lane where the destination appears: help, release, migration, support, community, extension/vendor, or local-only. |
| `destination_type` | Destination type such as public issue template, release notes, migration guidance, support intake, community discussion, extension/vendor support, or local draft. |
| `route` | Governed route such as `public_issue`, `security_disclosure`, `community_support`, or `local_draft`. |
| `ownership_class` | `official`, `community`, `private_security`, `official_authenticated`, `vendor_managed`, or `local_only`. |
| `trust_class` | Destination trust class shown before exit. |
| `version_awareness_state`, `version_awareness_note` | Version current/specific/cached/offline posture and the note shown before opening or copying the destination. |
| `visibility_boundary` | What audience can see the destination or payload. |
| `auth_expectation` | Expected account/auth before submit. |
| `data_exit_boundary` | What leaves the product. |
| `commitment_class` | Official commitment, best-effort community, no-commitment public forum, private security, vendor-owned no-product-commitment, or local draft. |
| `destination_state` | Ready, browser-blocked, offline, policy-blocked, stale cached target, or unsupported profile. |
| `pre_exit_review_required` | Must be true for public/community/private off-product routes. |
| `local_safe_fallback_ref` | Local draft or saved packet fallback that survives failed handoff. |
| `actions` | Open, copy-context, open/copy issue-template, or export-local-draft actions; copy/export actions preserve destination identity and trust class. |
| `copy_export` | Copy-safe handoff summary; public/community ownership must survive copy/export. |

Degraded states:

- `community_owned_destination`
- `browser_blocked`
- `offline`
- `policy_blocked`
- `stale_cached_target`
- `unsupported_profile`

## Copy / Export Invariants

Every component family must offer:

- text copy for a support engineer or release reviewer;
- JSON copy with stable field names and opaque refs;
- Markdown copy for docs, issue templates, and release proof;
- source refs pointing back to the canonical packet, schema, or manifest; and
- `screenshot_only_prohibited = true`.

Copy/export payloads must preserve the same controlled labels as the UI. A card
or tile that renders a green UI state but exports stale, cached, community-owned,
bridge-required, or local-only truth as generic prose is non-conforming.

## Narrowing Rules

- Stale benchmark evidence narrows to `retest_pending` or `methodology_only`.
- Cached service health must show `cached_service_health`; it cannot claim live
  reachability, and it must name affected managed features separately from the
  local-core workflows that still work.
- A support package saved locally remains `saved_local_only` until an explicit
  inspected submit succeeds; its card must show package contents, redaction/export
  state, local packet ref, and submit-later consequence without treating local
  save as support submission.
- Import rows with `bridge_required` must keep bridge and compatibility refs
  visible in preview, migration center, docs/help, and support export.
- Import rows with `skipped`, lossy `mapped`, `manual_review`,
  `bridge_required`, or `unsupported` outcomes must remain visible in
  post-apply summaries and exported migration evidence.
- High-frequency shortcut/keybinding changes render in `shortcut_change_digest`
  after apply; they must not be hidden inside general settings diff counts.
- Post-import bridge and unsupported follow-up paths keep
  `bridge_detail_inspectors`, `compatibility_report_links`, and
  `issue_template_export` available from summaries, not only from the initial
  review table.
- Partial or downgraded imports show `partial_apply_summary` and
  `restore_summary` after apply and in support export; they cannot collapse into
  a single "migration complete" state while unresolved work remains.
- Review tables group rows only by the six stable outcome states; feature teams
  must not replace them with local labels such as approximate, partial, failed,
  best effort, or ignored.
- Community-owned destinations must show `community_owned_destination` and
  cannot inherit an official-support commitment.
- Any first consumer that drops the component's copy/export fields or replaces
  controlled labels with local prose narrows below M5-ready.
