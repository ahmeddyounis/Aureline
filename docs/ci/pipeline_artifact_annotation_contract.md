# Pipeline artifact card, annotation row, and structured-viewer / safe-preview / download-only contract

This document freezes how Aureline's CI / pipeline artifact-card and
annotation-row surfaces project provider-side artifact and finding
state into honest local objects so a structured report opens in a
structured viewer, an HTML / binary / bundle artifact never auto-opens
as trusted local UI, an annotation anchor degrades honestly when its
mapping or freshness is approximate, and active content stays inside
a typed safe-preview / download-only path.

The contract is normative. Where this document disagrees with a frozen
upstream contract it cites, the upstream wins and this document MUST
be updated in the same change. Where this document disagrees with a
downstream surface's private artifact / annotation wording, this
document wins and the surface is non-conforming.

The companion artifacts are:

- [`/schemas/ci/pipeline_artifact_card.schema.json`](../../schemas/ci/pipeline_artifact_card.schema.json)
  - boundary schema for the pipeline artifact card
  (`pipeline_artifact_card_record`).
- [`/schemas/ci/pipeline_annotation_row.schema.json`](../../schemas/ci/pipeline_annotation_row.schema.json)
  - boundary schema for the pipeline annotation row
  (`pipeline_annotation_row_record`).
- [`/fixtures/ci/pipeline_artifact_annotation_cases/`](../../fixtures/ci/pipeline_artifact_annotation_cases/)
  - worked fixtures covering a structured report opened in a
  structured viewer, an HTML artifact forced to safe-preview /
  download-only, a stale annotation whose anchor drifted against the
  current workspace, and a binary bundle whose checksum and retention
  window are disclosed before download.

## Composition, not redefinition

This contract rides alongside - it does not re-mint - the
vocabularies already frozen in:

- [`/docs/ci/pipeline_run_and_control_contract.md`](pipeline_run_and_control_contract.md)
  and the run-row, log-pane, and run-control review schemas.
  Every artifact card and every annotation row MUST cite the
  producing `pipeline_run_row_record` by reference; an artifact card
  produced by a step MUST cite the same step identity vocabulary used
  by the log pane. Origin / freshness / `provider_event_ref` /
  `import_session_ref` / `replay_record_ref` semantics are reused
  verbatim and MUST NOT be re-minted in this contract.
- [`/docs/security/safe_preview_trust_classes.md`](../security/safe_preview_trust_classes.md)
  and [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  - trust-class vocabulary (`RawText`, `SanitizedRich`,
  `TrustedLocalActive`, `IsolatedRemoteActive`), connectivity-state,
  active-content policy. An artifact card MUST resolve to one of
  those trust classes; this contract narrows which trust classes are
  admissible for which artifact kinds and which safe-open paths.
- [`/docs/ux/output_log_viewer_contract.md`](../ux/output_log_viewer_contract.md)
  and [`/schemas/ux/output_viewer_object.schema.json`](../../schemas/ux/output_viewer_object.schema.json)
  - viewer-object truth for output, log, result-grid, and
  artifact-preview surfaces. Whenever an artifact card admits
  `open_in_structured_viewer`, `open_in_safe_preview_sanitized`, or
  `open_in_safe_preview_metadata_only`, the surface that opens the
  artifact MUST mint an `output_viewer_object_record` and the card
  MUST cite that viewer object by reference. Size, truncation,
  viewer-mode, freeze / autoscroll, copy / export representation,
  blocked-active-content posture, and textual-fallback semantics
  remain owned by the viewer object.
- [`/schemas/debug/debug_artifact_manifest.schema.json`](../../schemas/debug/debug_artifact_manifest.schema.json)
  - hash-algorithm, content-digest, and artifact-family vocabulary.
  Every artifact card content digest MUST use the same
  `hash_algorithm_class` enumeration; an artifact card that names a
  workspace-attached debug artifact MUST cite the matching
  `debug_artifact_manifest` entry by reference and MUST NOT mint a
  parallel digest model.
- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  - browser-handoff packets. An annotation row whose only admissible
  open-provider action requires browser handoff MUST cite a typed
  handoff packet ref.
- [`/schemas/runtime/execution_context.schema.json`](../../schemas/runtime/execution_context.schema.json)
  - the local execution-context record. An artifact card or
  annotation row that has a corresponding **local task or debug
  result** (a locally-built artifact, a locally-emitted lint
  diagnostic, a locally-recomputed checksum) MUST cite that local
  execution-context ref so the surface can render local truth as the
  primary affordance.
- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  for the redaction class and connectivity vocabulary used here.

If this document disagrees with those sources, those sources win and
this document plus the schemas are updated in the same change.

This document does not ship live artifact registry adapters, live
log streaming infrastructure, structured-report viewer
implementations, or browser-side download handlers. It freezes the
contract those implementations will read and write.

## Why freeze this now

CI / pipeline artifact and annotation surfaces are where active
content most easily slips into the IDE chrome:

1. *A test runner uploads an HTML coverage report; opening it in a
   trusted webview gives every uploaded payload local-script
   privileges.*
2. *A SARIF or JUnit report is rendered as plain text instead of
   inside a typed structured viewer, so reviewers cannot see severity
   buckets or anchors.*
3. *A binary bundle is downloaded silently with no checksum, no size
   bucket, and no retention window, so reviewers cannot tell whether
   the bytes match the producer.*
4. *An annotation row anchored to a file / line that no longer
   matches the local workspace is rendered as if the anchor were
   exact, so the reviewer jumps to the wrong line.*
5. *A scanner uploads a finding with no anchor at all and the row is
   silently anchored to the run, so the user sees a line-anchored
   warning that has no real anchor.*
6. *Provider retention drops the artifact bytes; the card still
   advertises an "Open" action that will silently 404.*

This contract closes that gap with **one artifact card record and
one annotation row record**, both routing through closed
safe-open-path / local-action / open-provider-action vocabulary, both
secondary to local execution truth, both honest about freshness and
anchor mapping.

## Scope

Frozen at this revision:

- the `pipeline_artifact_card_record` projected by every surface that
  lists or renders an artifact produced by a CI / pipeline / build /
  deploy / release run (runs panel artifact list, log-pane inline
  artifact chip, support packet, AI evidence packet, command palette
  result);
- the `pipeline_annotation_row_record` projected by every surface
  that lists CI / pipeline / scanner / linter / compiler / test-runner
  findings against a run (annotations panel, gutter chip, problems
  panel, support packet, AI evidence packet, command palette
  result);
- the **safe-open-path closed vocabulary** that names how the
  artifact card may invite the user to open or download an artifact;
- the **structured-viewer-vs-safe-preview-vs-download-only** rule
  that forces structured reports into a structured viewer, HTML and
  active-content artifacts into safe-preview or download-only, and
  binary / executable / container / archive artifacts into
  download-only;
- the **anchor-honesty rule** that forces an annotation to drop out
  of `exact_anchor_against_current_workspace` whenever the workspace
  has drifted, the anchor refers to an imported snapshot only, the
  anchor failed to map to the current workspace, or the source has no
  anchor at all;
- the **stale-or-unavailable provider-state** rule that forces a card
  whose retention window has expired or whose provider state is
  stale to a typed unavailable / cached posture and out of any
  structured-viewer or safe-preview path that depends on live bytes;
- the **redaction posture** that keeps raw URLs (provider host,
  artifact download URL, run URL, branch URL, commit URL), raw
  absolute paths, raw artifact bodies, raw bytes, raw scanner
  payloads, raw author email addresses, raw bearer / OAuth /
  delegated / approval tokens, and raw upstream payloads off this
  boundary.

## Out of scope

- Live artifact registry / object-store integrations (GitHub Actions
  artifact store, GitLab CI cache, Azure Pipelines artifacts, S3 / GCS
  / Azure Blob, JFrog / Nexus / Artifactory, ECR / GCR / ACR, custom
  org artifact stores). This document freezes the contract those
  integrations will satisfy.
- Live structured-report viewers (SARIF viewer, JUnit / TAP viewer,
  OpenAPI viewer, lcov viewer, JSON / YAML structured viewers).
  Building viewers for every report or artifact type is explicitly
  out of scope at this revision.
- Live browser-side download handlers, retention-window pollers, or
  artifact-rehydrate services.
- The live queue-drain service that flushes deferred-publish
  request-artifact-redownload mutations against providers.

## 1. Pipeline artifact card

Every artifact produced by a CI / pipeline / check / build / deploy /
release run rendered in the product emits one
`pipeline_artifact_card_record`.

### 1.1 Required fields

| Field | Meaning |
|---|---|
| `pipeline_artifact_card_id` | Opaque card id, stable across refreshes within the same import session. |
| `scope_ref` | Typed scope (`workspace`, `window`, `review_workspace`, `remote_session`, `tenant`, `companion_surface`). |
| `pipeline_run_row_ref` | The `pipeline_run_row_record` that produced this artifact. |
| `step_identity` | Producing step identity (re-exported step-identity class plus opaque step ref / label) or `not_applicable_run_level_artifact` for run-level artifacts. |
| `artifact_label` | Reviewer-visible artifact name. Redaction-aware; raw URLs, raw absolute paths, raw author email addresses MUST NOT appear here. |
| `artifact_kind_class` | Closed artifact-kind vocabulary (see 1.2). |
| `media_type_class` | Closed media-type vocabulary (see 1.3). |
| `size_disclosure` | Exact / approximate / unknown byte-count disclosure plus a typed size bucket and a typed truncation-or-cap reason. Silent size truncation is non-conforming. |
| `content_digest_disclosure` | Closed digest-class plus a digest pair (algorithm + hex value) re-using the debug-artifact `hash_algorithm_class`, OR a typed `denied_no_digest` posture. Silent missing digest is non-conforming. |
| `trust_class` | Re-exported trust class (`RawText`, `SanitizedRich`, `TrustedLocalActive`, `IsolatedRemoteActive`). |
| `active_content_policy_class` | Closed active-content policy class re-exported from the output-viewer vocabulary. |
| `safe_open_path` | Closed safe-open-path vocabulary (see 1.4). |
| `viewer_mode_class` | Closed viewer-mode vocabulary re-exported from the output-viewer object (`inline`, `virtualized`, `open_detail`, `blocked_active_content`, `textual_fallback`, `snapshot_review`, `not_applicable_no_in_product_open`). |
| `output_viewer_object_ref` | Reference to the underlying `output_viewer_object_record` when the safe-open path mints one. Required when `safe_open_path` is `open_in_structured_viewer`, `open_in_safe_preview_sanitized`, or `open_in_safe_preview_metadata_only`; null when `safe_open_path` is `download_only_no_in_product_open` or `denied_no_open_path`. |
| `retention_window` | Closed retention-window vocabulary plus an optional `expires_at` timestamp. Silent missing retention is non-conforming. |
| `origin_class` | Closed origin vocabulary re-exported from the run row (`live_provider_overlay`, `cached_provider_overlay`, `replayed_import`, `imported_snapshot`, `support_packet_import`). |
| `freshness_class` | Re-exported freshness class. |
| `provider_event_ref` | Re-exported per the run-row contract; required for `live_provider_overlay` / `cached_provider_overlay`. |
| `import_session_ref` | Re-exported per the run-row contract; required for `replayed_import` / `imported_snapshot` / `support_packet_import`. |
| `replay_record_ref` | Re-exported per the run-row contract; required for `replayed_import`. |
| `local_truth_binding` | The local execution-context this artifact composes with (a locally-built artifact, a locally-recomputed checksum) and the closed authority verdict, mirroring the run-row contract. |
| `policy_context` | Identity-mode, policy-epoch, trust-state, execution-context-id, mirroring the run-row contract. |
| `redaction_class` | Re-exported redaction class. |
| `card_summary_label` | Reviewable typed sentence summarising the card. |
| `minted_at` | Mint timestamp. |

### 1.2 Artifact-kind closed vocabulary

`artifact_kind_class` is a closed vocabulary:

- `structured_report_artifact`
- `html_bundle_artifact`
- `binary_bundle_artifact`
- `executable_or_loadable_artifact`
- `container_image_artifact`
- `archive_compressed_artifact`
- `source_archive_artifact`
- `log_archive_artifact`
- `image_or_media_artifact`
- `document_artifact`
- `source_map_bundle_artifact`
- `generated_source_mapping_artifact`
- `generic_blob_artifact`
- `unknown_artifact_kind_provider_owned`

A surface MAY narrow this set; no surface MAY widen, redefine, or
rename a kind.

### 1.3 Media-type closed vocabulary

`media_type_class` is a closed vocabulary:

- `structured_json`
- `structured_yaml`
- `structured_xml`
- `sarif_report`
- `junit_xml_report`
- `lcov_coverage_report`
- `cobertura_coverage_report`
- `openapi_document`
- `html_document`
- `javascript_bundle`
- `css_bundle`
- `text_log`
- `image_raster`
- `image_vector_svg`
- `document_pdf`
- `archive_zip`
- `archive_tar`
- `archive_tar_gz`
- `container_image_oci`
- `executable_binary`
- `loadable_library`
- `source_map_file`
- `generic_octet_stream`
- `unknown_media_type_provider_owned`

`unknown_media_type_provider_owned` is the correct posture when the
provider returns a media type the contract does not recognise yet; it
MUST NOT be flattened into `generic_octet_stream` to avoid a typed
disclosure.

### 1.4 Safe-open-path closed vocabulary

`safe_open_path` is a closed vocabulary:

- `open_in_structured_viewer` - admissible only for structured /
  report media types (see 1.5).
- `open_in_safe_preview_sanitized` - admissible for HTML, image, PDF,
  and other rich content with active-content-blocked posture.
- `open_in_safe_preview_metadata_only` - admissible when only metadata
  may cross the boundary (no rendered bytes).
- `download_only_no_in_product_open` - admissible for binary,
  executable, container, archive, and any artifact that cannot be
  safely rendered in-product at this revision.
- `denied_no_open_path` - admissible only when the artifact bytes are
  unavailable (retention expired, provider denied, import truncated)
  or when policy forbids any open path.

Rules:

1. `open_in_structured_viewer` MUST resolve to a structured /
   report media type from the `structured_*`, `sarif_report`,
   `junit_xml_report`, `lcov_coverage_report`,
   `cobertura_coverage_report`, or `openapi_document` set, and the
   trust class MUST be `RawText` or `SanitizedRich`. A structured
   viewer MUST NOT advertise `TrustedLocalActive`.
2. `open_in_safe_preview_sanitized` MUST resolve to a rich-content
   media type (`html_document`, `javascript_bundle`, `css_bundle`,
   `image_raster`, `image_vector_svg`, `document_pdf`) and the trust
   class MUST be `SanitizedRich` or `IsolatedRemoteActive`; the
   active-content policy MUST be one of
   `active_content_blocked_trust`, `active_content_blocked_policy`,
   `active_content_blocked_sandbox`, or
   `active_content_blocked_representation`. A surface that opens an
   HTML artifact as `TrustedLocalActive` is non-conforming.
3. `open_in_safe_preview_metadata_only` is admissible for any kind
   when bytes cannot be rendered safely; the trust class MUST be
   `RawText` or `SanitizedRich`; the viewer-mode MUST be
   `textual_fallback` or `snapshot_review`.
4. `download_only_no_in_product_open` is admissible for any kind;
   `viewer_mode_class` MUST be `not_applicable_no_in_product_open`
   and `output_viewer_object_ref` MUST be null. The card MUST surface
   a typed download disclosure (size + checksum + retention) before
   the user is invited to download.
5. `denied_no_open_path` MUST set `viewer_mode_class =
   not_applicable_no_in_product_open`, `output_viewer_object_ref`
   null, and a typed `safe_open_path_denial_label`.

### 1.5 Structured-viewer-vs-safe-preview-vs-download-only matrix
(frozen)

| Artifact kind | Admissible safe-open paths | Forbidden safe-open paths |
|---|---|---|
| `structured_report_artifact` (sarif, junit, lcov, openapi, json/yaml/xml structured) | `open_in_structured_viewer`, `open_in_safe_preview_metadata_only`, `download_only_no_in_product_open`, `denied_no_open_path` | `open_in_safe_preview_sanitized` (a structured report is not rich content) |
| `html_bundle_artifact` | `open_in_safe_preview_sanitized`, `open_in_safe_preview_metadata_only`, `download_only_no_in_product_open`, `denied_no_open_path` | `open_in_structured_viewer` |
| `binary_bundle_artifact`, `executable_or_loadable_artifact`, `container_image_artifact`, `archive_compressed_artifact`, `source_archive_artifact` | `download_only_no_in_product_open`, `denied_no_open_path`, `open_in_safe_preview_metadata_only` | `open_in_structured_viewer`, `open_in_safe_preview_sanitized` |
| `log_archive_artifact` | `download_only_no_in_product_open`, `open_in_safe_preview_metadata_only`, `denied_no_open_path` | `open_in_structured_viewer`, `open_in_safe_preview_sanitized` |
| `image_or_media_artifact` | `open_in_safe_preview_sanitized`, `download_only_no_in_product_open`, `denied_no_open_path` | `open_in_structured_viewer` |
| `document_artifact` | `open_in_safe_preview_sanitized`, `open_in_safe_preview_metadata_only`, `download_only_no_in_product_open`, `denied_no_open_path` | `open_in_structured_viewer` |
| `source_map_bundle_artifact`, `generated_source_mapping_artifact` | `open_in_structured_viewer` (when the surface treats them as structured), `download_only_no_in_product_open`, `open_in_safe_preview_metadata_only`, `denied_no_open_path` | `open_in_safe_preview_sanitized` |
| `generic_blob_artifact`, `unknown_artifact_kind_provider_owned` | `download_only_no_in_product_open`, `open_in_safe_preview_metadata_only`, `denied_no_open_path` | `open_in_structured_viewer`, `open_in_safe_preview_sanitized` |

### 1.6 Active-content-cannot-auto-open invariant (frozen)

A card whose `active_content_policy_class = active_content_allowed`
MUST NOT resolve to `trust_class = TrustedLocalActive` on this
boundary. CI / pipeline artifacts arrive from a provider boundary;
they are never trusted local UI, even when the workspace itself is
trusted.

Rules:

1. `html_bundle_artifact`, `javascript_bundle`, `css_bundle`,
   `image_vector_svg`, and any media type that may carry active
   handlers MUST resolve to `trust_class = SanitizedRich` or
   `IsolatedRemoteActive`.
2. `executable_binary`, `loadable_library`, and `container_image_oci`
   MUST resolve to `safe_open_path =
   download_only_no_in_product_open` or
   `denied_no_open_path`. They never auto-open in-product.
3. `active_content_allowed` is admissible only when the card resolves
   to `IsolatedRemoteActive` plus a sandbox-isolated posture; in that
   case the safe-open path MUST be
   `open_in_safe_preview_sanitized` and the viewer-mode MUST be
   `blocked_active_content` or `textual_fallback`.

### 1.7 Retention-window closed vocabulary

`retention_window_class` is a closed vocabulary:

- `provider_default_retention`
- `provider_extended_retention`
- `provider_short_lived_retention`
- `retention_expired_no_bytes_available`
- `retention_unknown_provider_owned`
- `local_only_no_provider_retention`
- `imported_snapshot_retention_static`

Rules:

1. `retention_expired_no_bytes_available` MUST set `safe_open_path =
   denied_no_open_path` and MUST cite a typed
   `retention_explanation_label`.
2. A card whose `retention_window_class` is
   `provider_short_lived_retention`, `provider_default_retention`, or
   `provider_extended_retention` MUST cite an `expires_at` timestamp
   when the provider supplies one; `null` is admissible only with a
   typed `retention_explanation_label` naming why no timestamp was
   available.
3. `local_only_no_provider_retention` is the correct posture when
   the artifact is a locally-built or locally-captured artifact that
   never crossed a provider boundary; in that case
   `provider_event_ref` MUST be null and the local-truth binding
   MUST resolve to `local_truth_is_authoritative`.

### 1.8 Content-digest disclosure rule (frozen)

`content_digest_disclosure` carries one of:

- `digest_class = provider_attested` plus a non-null
  `content_digest` (algorithm + value).
- `digest_class = locally_recomputed` plus a non-null
  `content_digest` and a non-null
  `local_truth_binding.execution_context_record_ref`.
- `digest_class = signature_verified` plus a non-null
  `content_digest` and a non-null `signature_evidence_ref`.
- `digest_class = unverified_provider_owned` plus a typed
  `digest_explanation_label`.
- `digest_class = denied_no_digest` plus a typed
  `digest_explanation_label`.

Rules:

1. `provider_attested`, `locally_recomputed`, and
   `signature_verified` MUST cite a non-null `content_digest`.
2. `denied_no_digest` MUST set `safe_open_path =
   download_only_no_in_product_open` or `denied_no_open_path`; the
   surface MUST NOT advertise a structured-viewer or safe-preview
   open path when no digest is available.
3. `hash_algorithm_class` re-uses the debug-artifact algorithm
   vocabulary (`sha1_git`, `sha256_git`, `sha256_content`,
   `blake3_content`).

### 1.9 Stale-or-unavailable provider-state rule (frozen)

When the producing run row's freshness has dropped out of
`authoritative_live` AND the artifact bytes are not locally cached:

1. The card MUST set `freshness_class` to the same non-live class
   carried by the run row (`warm_cached`, `degraded_cached`,
   `stale`, `unverified`).
2. The card MUST narrow `safe_open_path` to
   `open_in_safe_preview_metadata_only`,
   `download_only_no_in_product_open`, or `denied_no_open_path`. A
   structured-viewer open path that depends on live bytes is not
   admissible at non-live freshness.
3. The card MUST surface a typed `stale_or_unavailable_label`
   explaining what is no longer guaranteed live (the bytes, the
   digest, the retention window, the provider rendering).

## 2. Pipeline annotation row

Every CI / pipeline / scanner / linter / compiler / test-runner
finding rendered in the product emits one
`pipeline_annotation_row_record`.

### 2.1 Required fields

| Field | Meaning |
|---|---|
| `pipeline_annotation_row_id` | Opaque row id. |
| `scope_ref` | Typed scope (re-exported). |
| `pipeline_run_row_ref` | The producing `pipeline_run_row_record`. |
| `step_identity` | Producing step identity (re-exported) or `not_applicable_run_level_annotation`. |
| `annotation_source_class` | Closed annotation-source vocabulary (see 2.2). |
| `scanner_or_tool_label` | Reviewer-visible scanner / tool label. Redaction-aware. |
| `annotation_message_label` | Reviewer-visible message. Redaction-aware. |
| `severity_class` | Closed severity vocabulary (see 2.3). |
| `confidence_class` | Closed confidence vocabulary (see 2.4). |
| `anchor_kind_class` | Closed anchor-kind vocabulary (see 2.5). |
| `anchor` | Anchor block whose required sub-refs depend on `anchor_kind_class`. Raw absolute paths and raw line content MUST NOT appear here; the anchor carries opaque file ref + line range / symbol ref / manifest path ref / package ref only. |
| `anchor_freshness_class` | Closed anchor-freshness vocabulary (see 2.6). |
| `local_action_class` | Closed local-action vocabulary (see 2.7). |
| `open_provider_action_class` | Closed open-provider-action vocabulary (see 2.8). |
| `browser_handoff_packet_ref` | Required (non-null) when `open_provider_action_class` is `open_in_provider_with_handoff_admissible`. |
| `pipeline_artifact_card_ref` | Optional ref to the artifact this annotation came from (a SARIF report, a JUnit report). Required (non-null) when `annotation_source_class = third_party_uploaded_sarif`. |
| `origin_class` | Re-exported origin vocabulary. |
| `freshness_class` | Re-exported freshness class. |
| `provider_event_ref` | Re-exported per the run-row contract. |
| `import_session_ref` | Re-exported per the run-row contract. |
| `replay_record_ref` | Re-exported per the run-row contract. |
| `policy_context` | Re-exported policy context. |
| `redaction_class` | Re-exported redaction class. |
| `row_summary_label` | Reviewable typed sentence summarising the row. |
| `minted_at` | Mint timestamp. |

### 2.2 Annotation-source closed vocabulary

`annotation_source_class` is a closed vocabulary:

- `provider_native_check_annotation`
- `provider_native_log_annotation`
- `provider_scanner_sast`
- `provider_scanner_dast`
- `provider_scanner_dependency`
- `provider_scanner_secret`
- `provider_scanner_iac`
- `provider_scanner_container_image`
- `provider_linter`
- `provider_compiler_diagnostic`
- `provider_test_runner_failure`
- `third_party_uploaded_sarif`
- `ai_proposed_annotation_pending_review`
- `unknown_annotation_source_provider_owned`

A surface MAY narrow this set; no surface MAY widen, redefine, or
rename a source class.

### 2.3 Severity closed vocabulary

`severity_class` is a closed vocabulary:

- `error`
- `warning`
- `notice`
- `info`
- `blocker`
- `security_critical`
- `security_high`
- `security_medium`
- `security_low`
- `security_informational`
- `unknown_severity_provider_owned`

Rules:

1. `unknown_severity_provider_owned` is the correct posture when the
   provider returns a severity the contract does not recognise yet;
   it MUST NOT be flattened into `info` or `warning`.
2. `security_*` classes are admissible only when
   `annotation_source_class` is one of
   `provider_scanner_sast`, `provider_scanner_dast`,
   `provider_scanner_dependency`, `provider_scanner_secret`,
   `provider_scanner_iac`, `provider_scanner_container_image`, or
   `third_party_uploaded_sarif`. A linter / compiler / test-runner
   row that wants to escalate to a security severity MUST be
   re-emitted as a scanner row.

### 2.4 Confidence closed vocabulary

`confidence_class` is a closed vocabulary:

- `confirmed_by_local_run`
- `high_confidence_provider_attested`
- `medium_confidence_heuristic`
- `low_confidence_heuristic`
- `ai_proposed_pending_review`
- `unknown_confidence_provider_owned`

Rules:

1. `confirmed_by_local_run` MUST cite a non-null
   `local_execution_context_record_ref` whose execution-context-id
   resolves successfully against the same anchor.
2. `ai_proposed_pending_review` MUST pair with
   `annotation_source_class =
   ai_proposed_annotation_pending_review`; it MUST NOT mint typed
   security severities until reviewed.

### 2.5 Anchor-kind closed vocabulary

`anchor_kind_class` is a closed vocabulary:

- `file_line_anchor`
- `file_range_anchor`
- `file_only_anchor`
- `symbol_anchor`
- `manifest_path_anchor`
- `package_or_dependency_anchor`
- `container_layer_anchor`
- `run_or_step_only_no_anchor`
- `unanchored_review_required`
- `ai_proposed_anchor_pending_review`

Rules:

1. `file_line_anchor` MUST cite a non-null `file_ref` plus a non-null
   `line_range`.
2. `file_range_anchor` MUST cite a non-null `file_ref` plus a non-null
   `line_range` whose end is greater than or equal to its start.
3. `file_only_anchor` MUST cite a non-null `file_ref`.
4. `symbol_anchor` MUST cite a non-null `symbol_ref` plus an optional
   `file_ref`.
5. `manifest_path_anchor` MUST cite a non-null `manifest_path_ref`.
6. `package_or_dependency_anchor` MUST cite a non-null `package_ref`.
7. `container_layer_anchor` MUST cite a non-null `container_layer_ref`.
8. `run_or_step_only_no_anchor` and `unanchored_review_required`
   MUST set every anchor sub-ref to null and MUST resolve
   `local_action_class` to `no_local_action_admissible` or
   `denied_no_action_no_anchor`.
9. `ai_proposed_anchor_pending_review` MUST pair with
   `confidence_class = ai_proposed_pending_review`.

### 2.6 Anchor-freshness closed vocabulary

`anchor_freshness_class` is a closed vocabulary:

- `exact_anchor_against_current_workspace`
- `approximate_anchor_drifted_workspace`
- `anchor_against_imported_snapshot_only`
- `stale_anchor_workspace_moved`
- `anchor_unmapped_against_current_workspace`
- `denied_no_anchor_resolution`
- `unknown_anchor_freshness_provider_owned`

Rules:

1. `exact_anchor_against_current_workspace` is admissible only when
   `anchor_kind_class` is `file_line_anchor`, `file_range_anchor`,
   `file_only_anchor`, `symbol_anchor`, `manifest_path_anchor`,
   `package_or_dependency_anchor`, or `container_layer_anchor`, AND
   the anchor resolves cleanly against the current workspace; a
   surface MUST NOT claim exact mapping for an imported-snapshot-only
   anchor.
2. `approximate_anchor_drifted_workspace` MUST surface a typed
   `anchor_drift_explanation_label` and MUST narrow
   `local_action_class` to one of
   `jump_to_local_anchor_with_drift_disclosure_admissible`,
   `open_local_diff_admissible`, or
   `no_local_action_admissible`. A surface that jumps to a drifted
   anchor without disclosing drift is non-conforming.
3. `anchor_against_imported_snapshot_only` MUST set
   `local_action_class` to
   `open_in_safe_preview_admissible`,
   `no_local_action_admissible`, or
   `denied_no_action_no_anchor`; jump-to-local is not admissible
   because there is no local correspondent.
4. `stale_anchor_workspace_moved` and
   `anchor_unmapped_against_current_workspace` MUST set
   `local_action_class` to `no_local_action_admissible` or
   `denied_no_action_no_anchor`.
5. `denied_no_anchor_resolution` MUST set both
   `local_action_class = denied_no_action_no_anchor` and
   `open_provider_action_class = denied_no_provider_action`.

### 2.7 Local-action closed vocabulary

`local_action_class` is a closed vocabulary:

- `jump_to_local_anchor_admissible`
- `jump_to_local_anchor_with_drift_disclosure_admissible`
- `open_in_safe_preview_admissible`
- `open_local_diff_admissible`
- `no_local_action_admissible`
- `denied_no_action_no_anchor`

Rules:

1. `jump_to_local_anchor_admissible` is admissible only when
   `anchor_freshness_class = exact_anchor_against_current_workspace`.
2. `jump_to_local_anchor_with_drift_disclosure_admissible` is
   admissible only when `anchor_freshness_class =
   approximate_anchor_drifted_workspace` and the surface renders the
   drift disclosure inline.
3. `open_in_safe_preview_admissible` is admissible when the
   annotation references an artifact that resolves to a
   safe-previewable card (`pipeline_artifact_card_ref` is non-null
   and the card's `safe_open_path` is
   `open_in_safe_preview_sanitized` or
   `open_in_safe_preview_metadata_only`).

### 2.8 Open-provider-action closed vocabulary

`open_provider_action_class` is a closed vocabulary:

- `open_provider_run_admissible`
- `open_provider_log_at_anchor_admissible`
- `open_provider_artifact_admissible`
- `open_in_provider_with_handoff_admissible`
- `no_provider_action_admissible`
- `denied_no_provider_action`

Rules:

1. `open_in_provider_with_handoff_admissible` MUST cite a non-null
   `browser_handoff_packet_ref`.
2. `open_provider_log_at_anchor_admissible` is admissible only when
   the producing run row has a log pane that supports anchor pinning
   (`pipeline_log_view_record.pin_state` of `pinned_first_failure`,
   `pinned_last_warning`, `pinned_step_boundary`, or
   `pinned_user_anchor`). A surface that pins a provider log line
   that no longer exists is non-conforming.
3. `denied_no_provider_action` MUST surface a typed
   `provider_action_denial_label`.

## 3. Cross-surface review rules

1. **Active or rich content cannot auto-open as trusted local UI.**
   No artifact card may resolve to `trust_class =
   TrustedLocalActive`. HTML / JS / CSS / SVG / PDF / image / archive
   / executable / container artifacts MUST resolve to
   `SanitizedRich`, `IsolatedRemoteActive`, or `RawText`. A surface
   that promotes an uploaded HTML coverage report to a trusted
   webview is non-conforming.
2. **Annotation anchors degrade honestly.** When the workspace has
   drifted, the anchor refers to an imported snapshot, the anchor
   failed to map, or the source has no anchor at all, the
   annotation row MUST set `anchor_freshness_class` and
   `local_action_class` to the matching typed values. A surface that
   jumps to a drifted line without disclosing drift, or that pretends
   an unanchored finding has a line, is non-conforming.
3. **Stale or unavailable provider state never masquerades as live.**
   A card whose retention window has expired or whose freshness has
   dropped out of `authoritative_live` MUST narrow `safe_open_path`
   to a non-live path and MUST surface a typed
   `stale_or_unavailable_label`.
4. **Structured reports stay structured; rich content stays
   sanitized; binary content stays download-only.** The matrix in
   1.5 is normative. A SARIF report opened as plain text is
   non-conforming; an HTML coverage report opened as a trusted
   webview is non-conforming; an executable binary auto-opened
   in-product is non-conforming.
5. **Checksum and retention are first-class.** A card MUST disclose
   `content_digest_disclosure` and `retention_window` before the
   user is invited to download. A surface that offers "Download"
   without size + checksum + retention is non-conforming.
6. **Local truth remains primary.** When a locally-built artifact
   has a corresponding provider artifact, the card MUST resolve
   `local_truth_binding` per the run-row contract; the local truth
   stays the primary affordance. When a locally-emitted lint
   diagnostic matches a provider scanner row, the annotation MUST
   resolve `confidence_class = confirmed_by_local_run`.
7. **Redaction stays metadata-safe.** Raw URLs (provider host,
   artifact download URL, run URL, branch URL, commit URL), raw
   absolute paths, raw artifact bodies, raw bytes, raw scanner
   payloads, raw author email addresses, and raw bearer / OAuth /
   delegated / approval tokens MUST NOT cross this boundary.

## 4. Forbidden collapses

The following collapses are non-conforming:

- Rendering an `html_bundle_artifact` card with `trust_class =
  TrustedLocalActive`.
- Rendering an `executable_or_loadable_artifact` or
  `container_image_artifact` card with `safe_open_path =
  open_in_structured_viewer` or `open_in_safe_preview_sanitized`.
- Rendering a `structured_report_artifact` card with `safe_open_path
  = open_in_safe_preview_sanitized` (a structured report is not rich
  content; it must open in the structured viewer or as
  metadata-only).
- Offering a "Download" affordance without `content_digest_disclosure`
  and `retention_window`.
- Painting a card whose retention window is
  `retention_expired_no_bytes_available` with any safe-open path
  other than `denied_no_open_path`.
- Painting a card whose freshness has dropped out of
  `authoritative_live` with `safe_open_path =
  open_in_structured_viewer` when the structured viewer depends on
  live bytes.
- Rendering an annotation row with `anchor_kind_class =
  file_line_anchor` and a null `file_ref` or null `line_range`.
- Rendering an annotation row with
  `anchor_freshness_class =
  approximate_anchor_drifted_workspace` and
  `local_action_class = jump_to_local_anchor_admissible` (drifted
  anchors require the drift-disclosure variant).
- Rendering an annotation row with
  `anchor_kind_class = run_or_step_only_no_anchor` and any
  `local_action_class` other than `no_local_action_admissible` or
  `denied_no_action_no_anchor`.
- Rendering an `ai_proposed_annotation_pending_review` row with a
  `security_*` severity before review.
- Rendering an `open_in_provider_with_handoff_admissible` action
  without a non-null `browser_handoff_packet_ref`.
- Rendering a `third_party_uploaded_sarif` row without a non-null
  `pipeline_artifact_card_ref`.
- Flattening a typed `unknown_*_provider_owned` value into a generic
  recognised value (`generic_octet_stream`, `info`, `warning`).
- Exposing raw URLs, raw absolute paths, raw author email addresses,
  raw artifact bodies, raw bytes, raw scanner payloads, or raw
  bearer / OAuth / delegated / approval tokens on this boundary.

## 5. Worked fixtures

Worked fixtures live under
[`/fixtures/ci/pipeline_artifact_annotation_cases/`](../../fixtures/ci/pipeline_artifact_annotation_cases/).
They cover, at this revision:

- `structured_report_sarif_card.yaml` -
  a SARIF report uploaded by a SAST scanner; the card resolves
  `artifact_kind_class = structured_report_artifact`,
  `media_type_class = sarif_report`, `safe_open_path =
  open_in_structured_viewer`, `trust_class = SanitizedRich`,
  `viewer_mode_class = open_detail`, and a non-null
  `output_viewer_object_ref`.
- `html_artifact_safe_preview_download_only.yaml` -
  an HTML coverage report; the card resolves
  `artifact_kind_class = html_bundle_artifact`, `media_type_class =
  html_document`, `safe_open_path = open_in_safe_preview_sanitized`,
  `trust_class = SanitizedRich`, `active_content_policy_class =
  active_content_blocked_trust`, `viewer_mode_class =
  blocked_active_content`, and surfaces a typed
  `download_only_alternative_label` so the user can fall back to
  download-only without the surface auto-opening the report as a
  trusted webview.
- `stale_annotation_workspace_drifted.yaml` -
  a scanner-emitted finding whose original `file_line_anchor` has
  drifted against the current workspace; the row resolves
  `anchor_freshness_class =
  approximate_anchor_drifted_workspace`, `local_action_class =
  jump_to_local_anchor_with_drift_disclosure_admissible`, and a
  typed `anchor_drift_explanation_label`.
- `binary_bundle_with_checksum_retention.yaml` -
  a binary release bundle artifact; the card resolves
  `artifact_kind_class = binary_bundle_artifact`, `media_type_class
  = executable_binary`, `safe_open_path =
  download_only_no_in_product_open`, `viewer_mode_class =
  not_applicable_no_in_product_open`, a non-null
  `content_digest_disclosure` (sha256_content) and a typed
  `retention_window_class = provider_default_retention` with an
  `expires_at` timestamp.

## 6. Source anchors

- UI / UX Spec runs panel artifact list, log-pane inline artifact
  chip, problems / annotations panel, support packet artifact /
  annotation rows, AI evidence packet artifact / annotation rows.
- TAD CI / pipeline / checks artifact and annotation integration
  rules.
- Safe-preview trust-class contract (`RawText`, `SanitizedRich`,
  `TrustedLocalActive`, `IsolatedRemoteActive`).
- Output / log viewer contract (`output_viewer_object_record`,
  `viewer_mode`, `active_content_policy_class`).
- Pipeline run / log / run-control contract
  (`pipeline_run_row_record`, step identity, origin / freshness).
- Debug-artifact manifest contract (`hash_algorithm_class`,
  `content_digest`).
- Browser-handoff packet contract.

## 7. Change discipline

- Adding a new `artifact_kind_class`, `media_type_class`,
  `safe_open_path`, `retention_window_class`, `digest_class`,
  `annotation_source_class`, `severity_class`, `confidence_class`,
  `anchor_kind_class`, `anchor_freshness_class`,
  `local_action_class`, or `open_provider_action_class` value is
  additive-minor and requires a schema version bump.
- Repurposing an existing value is breaking and requires a new
  decision row.
- Re-exported vocabularies (`trust_class`, `freshness_class`,
  `origin_class`, `redaction_class`, `viewer_mode`,
  `active_content_policy_class`, `hash_algorithm_class`,
  `step_identity_class`) are owned by their home schemas and MUST
  NOT be narrowed, widened, or renamed in this contract.

Building actual structured-report viewers, live artifact registry
adapters, retention-window pollers, and browser-side download
handlers is explicitly out of scope at this revision. This contract
freezes the artifact card, the annotation row, the safe-open-path
matrix, the anchor-honesty rule, and the composition rules. Surface
implementations land in their owning crates against this contract.
