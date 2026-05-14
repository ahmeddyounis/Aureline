# Diagnostic artifact matrix

This document freezes the item-level diagnostic artifact contract used
by support-pack assembly, bundle preview, release review, and privacy
review. It makes each captured or omitted artifact explainable by stable
item id, privacy class, redaction class, consent posture, storage mode,
size limit, and retention rule.

Companion artifacts:

- [`/artifacts/support/support_evidence_pack_matrix.yaml`](../../artifacts/support/support_evidence_pack_matrix.yaml)
  - machine-readable item matrix and high-risk inclusion rules.
- [`/schemas/support/support_pack_item.schema.json`](../../schemas/support/support_pack_item.schema.json)
  - boundary schema for matrix rows and inclusion fixture cases.
- [`/fixtures/support/pack_inclusion_cases/`](../../fixtures/support/pack_inclusion_cases/)
  - seeded assembly cases for extension, toolchain, renderer, and
  network failures.
- [`/docs/support/support_bundle_preview_contract.md`](./support_bundle_preview_contract.md)
  and
  [`/fixtures/support/support_bundle_preview_cases/`](../../fixtures/support/support_bundle_preview_cases/)
  - preview/export manifest cases that show how matrix item ids,
  deselection posture, redaction states, and actionability warnings
  survive local review and post-export intake.
- [`/docs/support/supportability_slo_and_pack_contract.md`](./supportability_slo_and_pack_contract.md)
  - pack-class, preview-manifest, reopen-manifest, SLO, drill, and
  waiver contract that consumes these item rows.
- [`/docs/support/support_bundle_contract.md`](./support_bundle_contract.md)
  and
  [`/schemas/support/support_bundle.schema.json`](../../schemas/support/support_bundle.schema.json)
  - support-bundle manifest contract that owns `data_class`,
  `support_export_posture`, `storage_mode`, and `embedding_state`.
- [`/artifacts/security/secret_classes.yaml`](../../artifacts/security/secret_classes.yaml)
  and
  [`/artifacts/security/redaction_posture_matrix.yaml`](../../artifacts/security/redaction_posture_matrix.yaml)
  - secret-class and per-surface redaction vocabulary this matrix
  reuses.
- [`/docs/governance/telemetry_and_support_schema_registry.md`](../governance/telemetry_and_support_schema_registry.md)
  - consent, endpoint, retention, and support-transfer separation
  rules.
- [`/docs/observability/signal_class_matrix.md`](../observability/signal_class_matrix.md)
  and
  [`/artifacts/observability/signal_classes.yaml`](../../artifacts/observability/signal_classes.yaml)
  - signal-class defaults referenced by `signal_class_binding`.

Normative source anchors:

- `.t2/docs/Aureline_PRD.md` section 10.22.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` sections
  21.7, 21.8, Appendix BT.3, and supportability architecture notes.
- `.t2/docs/Aureline_Milestones_Document.md` section 3.11.1.

If this document disagrees with the machine-readable matrix or schema,
the schema controls shape and the matrix controls row values.

## Reviewer contract

Every diagnostic artifact is classified as exactly one matrix item.
Reviewers should be able to answer these questions from a single row:

| Question | Matrix field |
|---|---|
| What is the stable item identity? | `item_id` |
| Is this metadata, environment-adjacent, code-adjacent, or high risk? | `privacy_class` and `diagnostic_data_class` |
| Does it enter telemetry, support, local-only diagnostics, or no export path? | `signal_class_binding` and `collection_path_class` |
| What redaction vocabulary applies? | `support_bundle_redaction_class` and `secret_redaction_surfaces` |
| Is it embedded, referenced, retained locally, an optional upload, or excluded? | `support_export_posture`, `default_storage_mode`, and `default_embedding_state` |
| What consent is required? | `consent_class` and `high_risk_rule_ref` |
| What size and retention limits apply? | `size_policy` and `retention_class` |
| How do manifests point back to the row? | `manifest_reference_contract` |

Support-pack assembly must not infer a broader capture scope from a
symptom. It selects item ids, applies the row's default posture, and
records any opt-in or omission against the same item id.

## Matrix rows

| Item id | What it covers | Privacy class | Default support behavior |
|---|---|---|---|
| `support.item.build_identity` | exact build, version, channel, platform, install profile, docs match state | `metadata_only` | embedded by default on user/admin export |
| `support.item.extension_inventory` | enabled extensions, versions, trust state, permission classes, quarantine/budget state | `metadata_only` | metadata summary embedded; extension private storage excluded |
| `support.item.docs_inventory` | docs or knowledge-pack ids, revisions, mirror freshness, digests | `metadata_only` | metadata summary embedded; pack bodies excluded |
| `support.item.connection_profiles` | connection classes, driver or engine family, proxy/tunnel posture, auth source class | `environment_adjacent` | metadata summary embedded after preview; raw endpoints, DSNs, headers, and credentials excluded |
| `support.item.secret_broker_state` | trust-store class, unlock state, alias-only counts, denial and projection classes | `metadata_only` | redacted state embedded; raw values and raw handle ids excluded |
| `support.item.policy_trust_state` | policy fingerprint, epoch, source/signer class, workspace trust state, locked-setting counts | `metadata_only` | embedded by default; full policy body and secret fields excluded |
| `support.item.execution_context_summary` | toolchain versions, command/invocation ids, target class, route class, policy source, resolver confidence, activator class, degraded state | `environment_adjacent` | metadata summary embedded; raw environment, full PATH, args, and file contents excluded |
| `support.item.imported_diagnostics` | scanner import sessions, delta packets, suppression/baseline registers, review packet refs, imported/live labels, raw-payload refs | `metadata_only` | metadata summary embedded; raw scanner bodies, source text, raw paths, URLs, provider payloads, and secrets excluded |
| `support.item.runtime_traces` | renderer, IPC, LSP, DAP trace manifests and redacted slices | `high_risk` | local-only retained by default; raw trace export requires explicit trace opt-in |
| `support.item.crash_dump_or_core` | crash envelopes, dump/core manifests, optional upload tickets, redaction scan result | `high_risk` | manifest only by default; raw memory upload requires high-friction consent or policy |
| `support.item.crash_incident_trail` | crash envelope, dump ref, symbolication state, trace IDs, support-bundle manifest ref, safe next actions | `environment_adjacent` | metadata row embedded by default; raw dump bytes and raw stack bodies excluded |
| `support.item.user_selected_code` | user-selected code snippets, notebook cells, and bounded mutation excerpts | `code_adjacent` | excluded by default; explicit item-level opt-in required |
| `support.item.raw_secrets` | raw tokens, passwords, key material, certificates, raw broker handles | `high_risk` | forbidden; only omission and redaction markers may appear |
| `support.item.full_shell_history` | full terminal scrollback or shell history files | `high_risk` | forbidden; narrow terminal metadata or selected excerpts must use a more specific reviewed row |

## High-risk inclusion rules

Crash dumps and cores:

- raw dump/core bytes never embed in an ordinary support bundle;
- the default bundle includes only crash envelope, dump manifest,
  exact-build refs, redaction scan state, and optional upload ticket;
- raw upload requires reviewed `crash_dump_export_opt_in`, destination
  disclosure, size disclosure, and visible memory-risk copy;
- local bytes remain user-controlled, while uploaded bytes follow case
  close, the 30-day diagnostic floor, or stricter policy hold.

Runtime traces:

- renderer, IPC, LSP, and DAP trace bodies stay local-only by default;
- exported bundles carry trace manifest, time window, correlation ids,
  truncation state, and redaction marker counts;
- raw or redacted trace slices require `trace_export_opt_in`;
- unpinned raw trace retention rotates after seven days by default.

Code-adjacent artifacts:

- snippets and notebook cells are excluded until the user chooses exact
  ranges or cell ids in preview;
- each selected artifact is bounded, hashed, and linked to an
  `opt_in_selection_ref`;
- selection cannot widen to surrounding files, notebooks, outputs, or a
  workspace-wide capture.

Raw content and prohibited classes:

- raw secret values have no support-pack consent path;
- full shell history has no ordinary support-pack consent path;
- the manifest may include omission reasons, item ids, redaction
  markers, and class labels so reviewers can prove absence without
  carrying forbidden data.

## Shared item-id references

The matrix defines one reference contract for support bundles, exact
build identity, redaction summaries, and opt-in selections:

| Consumer | Required reference |
|---|---|
| Support-bundle artifact row | `artifact_manifest[].support_pack_item_id` |
| Exact-build join | `artifact_manifest[].exact_build_identity_refs[]` |
| Redaction summary | `redaction_summary.items[].item_id` |
| Opt-in selection | `opt_in_selections[].item_id` |
| Privacy review | `artifact_manifest[].data_class` |

The support-bundle schema keeps `support_pack_item_id` optional for
backward compatibility, but new support-pack rows should populate it.
Omissions must also cite the same item id so "not included" stays as
auditable as "included".

## Signal and secret-class alignment

`signal_class_binding` names the closest governed signal family without
changing the collection path:

- crash envelope metadata aligns with `crash_panic_reports`, but dump
  bytes still require support opt-in rather than telemetry upload;
- version/channel rows align with `install_update_active_version_counts`,
  but support-bundle export remains manual and auditable;
- trace rows align with `performance_metrics`, but raw trace bodies stay
  local-only unless reviewed;
- forbidden rows use `forbidden_secret_bearing_artifact` so reviewers
  can identify a never-export artifact from the matrix alone.

`secret_redaction_surfaces` points to the secret broker redaction
vocabulary. The support exporter applies the named surface rules before
writing local logs, traces, bundles, crash manifests, terminal excerpts,
or clipboard-derived records.

## Fixture expectations

The inclusion cases under
[`/fixtures/support/pack_inclusion_cases/`](../../fixtures/support/pack_inclusion_cases/)
show that a support pack can explain common failures without blanket
capture:

- extension failures include build identity, extension inventory, policy
  and trust state, and crash envelope metadata, while raw dumps and full
  shell history stay omitted;
- toolchain failures include execution-context and toolchain summaries
  without raw environment variables or command transcripts;
- renderer failures include exact build and local trace manifests, while
  trace payloads stay local-only unless explicitly selected;
- network failures include connection classes and secret-broker state
  without raw headers, DSNs, tokens, or hostnames.
