# Build-support handoff packet, AI confidence callout, and source-confidence chain contract

This packet freezes one shared support-handoff packet, one shared
source-confidence chain, and one shared AI confidence callout for
build issues before run / test / build / debug picker, output pane,
problems pane, AI action sheet, CI overlay, support-bundle exporter,
review pack, and support-handoff intake fragment into incompatible
"send to support" buttons, parallel confidence languages, and AI
narrations that quietly upgrade a parser-fallback row into the same
voice used for a native-adapter row.

It names:

- the typed handoff packet a user opens from a build issue card so
  adapter confidence, raw-event inspector lineage, target identity,
  parser-fallback reasons, and the support / export / escalate
  action set all share one packet shape;
- the typed source-confidence chain that links a raw event back
  through parsed issue, AI summary, diagnostics badge, and
  support-packet claim to the originating run and attempt with one
  shared chain id and one typed `floor_reason_class`;
- the minimum AI confidence callout fields a model surface MUST emit
  alongside any narration of a build result so the model cannot
  describe a parser-fallback row in the same voice it describes a
  native-adapter row.

If this packet, the
[`source_confidence_chain.schema.json`](../../schemas/build/source_confidence_chain.schema.json),
and the fixture corpus under
[`/fixtures/build/support_handoff_cases/`](../../fixtures/build/support_handoff_cases/)
disagree, the machine-readable schema plus the frozen build-adapter
confidence ladder, parser-fallback reason card, raw-event inspector,
target-graph, target-descriptor, run / attempt, task-event-envelope,
AI evidence-packet, and problem / output / evidence chain
vocabularies in
[`/schemas/build/adapter_confidence_state.schema.json`](../../schemas/build/adapter_confidence_state.schema.json),
[`/schemas/build/raw_event_inspector.schema.json`](../../schemas/build/raw_event_inspector.schema.json),
[`/schemas/tooling/target_graph_snapshot.schema.json`](../../schemas/tooling/target_graph_snapshot.schema.json),
[`/schemas/tooling/target_descriptor.schema.json`](../../schemas/tooling/target_descriptor.schema.json),
[`/schemas/tooling/task_event_envelope.schema.json`](../../schemas/tooling/task_event_envelope.schema.json),
[`/schemas/execution/run.schema.json`](../../schemas/execution/run.schema.json),
[`/schemas/execution/attempt.schema.json`](../../schemas/execution/attempt.schema.json),
[`/schemas/ai/evidence_packet.schema.json`](../../schemas/ai/evidence_packet.schema.json),
and
[`/schemas/diagnostics/problem_evidence_chain.schema.json`](../../schemas/diagnostics/problem_evidence_chain.schema.json)
win and this packet plus the companion artifacts update in the same
change.

Companion artifacts:

- [`/schemas/build/source_confidence_chain.schema.json`](../../schemas/build/source_confidence_chain.schema.json)
  — boundary schema for `build_support_handoff_packet_record`,
  `source_confidence_chain_record`, and
  `ai_build_confidence_callout_record`. Re-exports
  `build_adapter_confidence_state`, `confidence_class`,
  `freshness_class`, `import_source_class`, `evidence_basis_class`,
  and `target_identity_authority_class` from the build-adapter and
  raw-event inspector schemas so the packet, the chain, and the
  callout never invent parallel vocabularies.
- [`/fixtures/build/support_handoff_cases/`](../../fixtures/build/support_handoff_cases/)
  — fixture corpus covering local build failure, remote build with
  imported CI evidence, parser-fallback warning, and a support
  packet opened from a degraded (stale) result.

Normative sources projected here:

- `.t2/docs/Aureline_PRD.md`
  — run / test / build / debug picker, output pane, problems pane,
  AI action sheet, CI overlay, support-bundle export, and
  supportability posture; "build results must explain why they are
  trusted, partial, or heuristic" treated as an in-product contract.
- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — supportability as core architecture (safe mode, Project Doctor,
  diagnostic inspectors, redacted support bundles); shared truth
  chains across UI, CLI, support bundles, and doctor flows.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  — adapter confidence ladder, parser-fallback handling, raw-event
  inspector retention, and AI evidence packet boundaries this packet
  projects.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — pickers, output pane headers, problems pane chips, AI action
  sheets, CI overlays, support-bundle exporters, and review packs
  that MUST consume the typed packet, chain, and callout rather than
  mint per-surface "send to support" / "ask AI" / "explain this"
  buttons.

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Why freeze this now

Build intelligence becomes a credibility risk the moment a "send this
to support" button strips the adapter confidence chip, the moment an
AI summary describes a parser-fallback row in the same voice it
describes a native-adapter row, the moment a downgraded or imported
build result is rendered as a generic "low confidence" without a
typed reason, or the moment a user who opens a support handoff
cannot move from the summary to the raw event inspector without
re-resolving identity by hand.

The build-adapter confidence ladder, the parser-fallback reason card,
and the raw-event inspector are already frozen by
[`/docs/build/adapter_confidence_and_event_inspection_contract.md`](./adapter_confidence_and_event_inspection_contract.md).
What is missing is a typed handoff packet that picks up that
disclosure, a typed chain that joins the packet back through parsed
issue, AI summary, and diagnostics badge to the originating run, and
a typed AI callout that prevents the model from minting a confidence
voice that exceeds the underlying state's ceiling. Without this
packet, surfaces will mint per-feature buttons — the picker writes
"file an issue", the output pane writes "send to support", the AI
action sheet writes "explain", the support exporter writes "share
bundle", and each strips a different combination of disclosure —
and a reviewer will not be able to tell, three weeks later, whether
two handoffs describe the same originating run.

## Three records, one chain id

Every build-support surface reads exactly three record families:

- **`build_support_handoff_packet_record`** is the packet a user
  opens when handing a build issue to support, exporting it to a
  review pack, escalating it to an admin, or filing an issue against
  the workspace. It carries the `build_handoff_intent_class`, the
  underlying `build_adapter_confidence_state`, the
  `confidence_ceiling`, the `freshness_class`, the
  `import_source_class`, the `target_identity_block`, the
  `lineage_ref_block`, the typed `handoff_actions` set, and the
  required `disclosure_copy` sentence.
- **`source_confidence_chain_record`** is the chain that joins the
  packet back through parsed issue, AI summary, diagnostics badge,
  and support-packet claim to the originating run / attempt. It
  carries one ordered list of typed `chain_links` (each link
  citing one schema family by `subject_kind_class`), the chain's
  `top_confidence_ceiling` (which MUST equal the lowest
  `declared_confidence_class` among the cited links), and the
  typed `floor_reason_class` explaining why the ceiling is what it
  is.
- **`ai_build_confidence_callout_record`** is the callout an AI
  surface MUST emit alongside any narration of a build result. It
  carries the `ai_evidence_packet_ref`, the
  `narration_basis_class`, the `inherited_confidence_class`
  (capped at the underlying state's ceiling by an `allOf`
  invariant), the required `required_disclosure_text` sentence,
  the non-empty `forbidden_claim_set`, and the
  `preserves_callout_in_export` flag (always `true`).

All three records share one `source_confidence_chain_record_id`. The
packet cites the chain id; the callout cites the same chain id; the
chain cites the same `originating_run_ref`,
`build_adapter_confidence_state_record_ref`, and
`raw_event_inspector_record_ref` as the packet and the callout. This
is the spec's "support and AI surfaces can cite the same build
evidence chain instead of creating parallel confidence languages"
rule encoded in the schema.

## Confidence inheritance rules

The packet's `confidence_ceiling` and the callout's
`inherited_confidence_class` / `inherited_confidence_ceiling_class`
are pinned to the underlying `build_adapter_confidence_state` by the
`allOf` invariants on
[`/schemas/build/source_confidence_chain.schema.json`](../../schemas/build/source_confidence_chain.schema.json):

| State                | Packet `confidence_ceiling` | Callout `inherited_confidence_class` | `narration_basis_class`                            |
| -------------------- | --------------------------- | ------------------------------------ | -------------------------------------------------- |
| `native_adapter`     | `authoritative_from_source` | `authoritative_from_source`          | `narrating_native_adapter_authoritative_facts`     |
| `declared_adapter`   | `structured_parse_match`    | `structured_parse_match`             | `narrating_declared_adapter_structured_facts`      |
| `inferred_adapter`   | `heuristic_best_effort`     | `heuristic_best_effort`              | `narrating_inferred_adapter_heuristics`            |
| `parser_fallback`    | `heuristic_best_effort`     | `heuristic_best_effort`              | `narrating_parser_fallback_heuristics`             |
| `imported_result`    | ≤ `structured_parse_match`  | `structured_parse_match`             | `narrating_imported_result_read_only_facts`        |
| `stale_result`       | ≤ `degraded_partial`        | `degraded_partial`                   | `narrating_stale_result_degraded_facts`            |
| `unavailable_result` | `unknown`                   | `unknown`                            | `narrating_unavailable_result_no_evidence`         |

The packet additionally pins `freshness_class` and
`import_source_class` per state so a managed-CI imported build never
renders as a local build, a stale row never renders as a fresh row,
and an unavailable result never renders as if an adapter ran.

## Required handoff actions

`handoff_actions` is a non-empty set of typed
`handoff_action_entry` rows. Per the `allOf` invariant on the
schema, every non-`unavailable_result` packet MUST include:

- `open_raw_event_inspector_pane` — resolves through
  `lineage_refs.raw_event_inspector_record_ref` so a user can open
  the same inspector pane from any surface.
- `open_originating_run_pane` — resolves through
  `lineage_refs.originating_run_ref` so the run's queue, host, and
  policy decisions are reachable without re-resolving identity.

Packets for `parser_fallback` and `stale_result` additionally MUST
cite `open_parser_fallback_reason_card` so the recognized / guessed
/ missing axes are reachable from the packet.

Other admissible actions:
`open_originating_attempt_pane`, `open_target_graph_view`,
`open_target_descriptor_view`, `open_support_bundle_export`,
`open_review_pack_export`, `open_imported_scan_record`,
`copy_source_confidence_chain_refs_to_review_pack`,
`copy_source_confidence_chain_refs_to_support_bundle`,
`file_workspace_issue_with_chain_attached`,
`escalate_to_workspace_admin_with_chain_attached`,
`escalate_to_managed_provider_support_with_chain_attached`,
`replay_in_doctor_with_imported_evidence`. The single legitimate
collapse is `no_action_admissible_review_only`, reserved for
`unavailable_result` paths whose only honest disclosure is
documenting the absence of evidence.

This is the spec's "users can move from summary to raw evidence
using stable references without lossy translation" rule encoded
directly in the schema.

## Source-confidence chain shape

```
                run_record (schemas/execution/run.schema.json)
                         |
                         | originating_run_ref (one id, all three records)
                         |
             +-----------+-----------+-----------+
             |                       |           |
             v                       v           v
 build_support_handoff_packet      source_confidence_chain   ai_build_confidence_callout
   _record (this schema)           _record (this schema)     _record (this schema)
             |                          |                              |
             | source_confidence        | chain_links[*].subject_ref   | source_confidence
             |   _chain_record_id_ref   |   resolves through typed     |   _chain_record_id_ref
             |                          |   subject_kind_class         |
             +------------+-------------+------------------------------+
                          |
                          v
                   one chain id, one floor_reason_class
                          |
                          v
             +-----+-----+-----+-----+-----+
             |     |     |     |     |     |
             v     v     v     v     v     v
       raw_event  parsed  ai_   diag   support  evidence
         _link    _issue  sum   _badge _packet  _origin_back
                  _link   _link _link  _claim   _reference
                                       _link
```

The chain MUST contain exactly one `raw_event_link` and exactly one
`evidence_origin_back_reference` (encoded as `minContains: 1` /
`maxContains: 1` `allOf` invariants); other link classes are
conditional on whether the surface produced a parsed issue, AI
summary, diagnostics badge, or support-packet claim.

Each `source_chain_link_entry` carries:

- `source_chain_link_class` — which step in the chain this link
  represents.
- `subject_kind_class` — closed vocabulary naming which schema the
  link's `subject_ref` resolves under (`raw_event_inspector_record`,
  `task_event_envelope_record`, `problem_evidence_chain_record`,
  `diagnostic_cluster_record`, `language_diagnostic_chip_record`,
  `ai_evidence_packet_record`, `ai_build_confidence_callout_record`,
  `support_bundle_packet`, `support_pack_item`,
  `build_adapter_confidence_state_record`,
  `parser_fallback_reason_card_record`, `run_record`,
  `attempt_record`).
- `subject_ref` — the opaque ref.
- `evidence_basis_class` — re-export of
  `evidence_source_class` from
  [`/schemas/build/adapter_confidence_state.schema.json`](../../schemas/build/adapter_confidence_state.schema.json).
- `declared_confidence_class` — the confidence the underlying
  record declared. The chain's `top_confidence_ceiling` MUST equal
  the lowest `declared_confidence_class` across all links; this is
  what `floor_reason_class` names.

`floor_reason_class` is the chain's typed disclosure of why its
ceiling is what it is. The closed vocabulary covers
`no_floor_applied`,
`parser_fallback_caps_at_heuristic_best_effort`,
`inferred_adapter_caps_at_heuristic_best_effort`,
`declared_adapter_caps_at_structured_parse_match`,
`imported_result_caps_at_structured_parse_match`,
`stale_result_caps_at_degraded_partial`,
`unavailable_result_caps_at_unknown`,
`ai_inference_caps_at_heuristic_best_effort`,
`multi_step_chain_pulls_to_lowest_link`,
`missing_evidence_pulls_to_unknown`, and
`imported_unverified_caps_at_structured_parse_match`. A consumer
that reads the chain MUST cap its own rendered confidence at
`top_confidence_ceiling` and SHOULD render the `floor_reason_class`
as the typed reason chip alongside the confidence chip.

## AI build-confidence callout

Whenever an AI surface narrates a build result it MUST emit one
`ai_build_confidence_callout_record` and the surface MUST render
the callout's `required_disclosure_text` alongside the narration.
The callout pins:

- `ai_evidence_packet_ref` — the ref to the
  [`ai_evidence_packet_record`](../../schemas/ai/evidence_packet.schema.json)
  covering the AI turn. The callout cannot be emitted without an
  evidence packet so AI build narrations cannot escape the
  assembly / route / spend audit trail.
- `build_adapter_confidence_state` — the underlying state.
- `narration_basis_class` — the typed basis the AI is narrating
  under (one of the seven entries in the table above).
- `inherited_confidence_class` /
  `inherited_confidence_ceiling_class` — both pinned by the
  basis-to-state-and-ceiling `allOf` invariants so an AI surface
  cannot raise the displayed confidence above the underlying
  state's ceiling.
- `required_disclosure_text` — a reviewer-facing sentence (≤ 320
  chars) disclosing the basis. MUST be rendered in every surface
  that shows the narration: AI action sheet, output pane header,
  problems pane hover, review pack, CLI JSON.
- `forbidden_claim_set` — non-empty list of reviewer-facing
  sentences naming claims the AI MUST NOT make under this basis.
  Even `narrating_native_adapter_authoritative_facts` callouts
  carry forbidden claims (e.g., AI MUST NOT claim a runtime-role
  attribution the record did not admit).
- `preserves_callout_in_export` — MUST be `true`. A downstream
  export that strips the callout is detectable as schema drift
  rather than a UI choice.

This is the spec's "AI assistance must disclose when it is
narrating native facts versus parser or imported heuristics" rule
encoded directly in the schema.

## Surface contract

Every consumer surface MUST render packet, chain, and callout data
through these three records:

- **Run / test / build / debug picker** renders the underlying
  confidence chip plus a "send to support" affordance that opens
  the `build_support_handoff_packet_record`. The picker MUST NOT
  mint a per-surface "support" button that bypasses the packet.
- **Output pane header** renders the confidence chip, the freshness
  chip, the import-source chip (when imported), and (when AI
  narrated the header) the AI callout's
  `required_disclosure_text`.
- **Problems pane chip** renders the confidence chip per build
  target row and (when AI hover narrated the row) the callout's
  `required_disclosure_text` in the hover.
- **AI action sheet** disables structured-build tool calls when
  the underlying state is `parser_fallback`, `stale_result`, or
  `unavailable_result`. AI narrations MUST emit a callout and
  render `required_disclosure_text` alongside the narration.
- **CI overlay** renders the same chip set as the picker including
  `imported_read_only` / `support_bundle_replay_only` freshness.
- **Support-bundle exporter** preserves all three records and the
  chain id mirrors across exporter and review pack.
- **Review pack** renders a typed "explain this build result"
  packet citing the chain id, the inspector ref, the parser-fallback
  reason card (when present), and the AI callout (when present).
- **Support-handoff intake** consumes the
  `build_support_handoff_packet_record` directly; a handoff intake
  that strips the chain id, the inspector ref, or the disclosure
  copy is non-conforming.

## Acceptance criteria mapping

Each spec acceptance criterion is encoded in the schema:

- *"Support and AI surfaces can cite the same build evidence chain
  instead of creating parallel confidence languages."* — the
  `source_confidence_chain_record_id_ref` on
  `build_support_handoff_packet_record` and on
  `ai_build_confidence_callout_record` MUST cite one
  `source_confidence_chain_record`. The callout's
  `inherited_confidence_class` is pinned to the underlying state's
  ceiling by `allOf` so a callout cannot raise the displayed
  confidence above what the chain admits.
- *"A downgraded or imported build result remains visibly downgraded
  in issue cards, summaries, and handoff packets."* — the packet's
  `confidence_ceiling`, `freshness_class`, and
  `import_source_class` are pinned to the underlying state by
  `allOf`. `parser_fallback` and `stale_result` packets MUST cite
  `open_parser_fallback_reason_card` so the recognized / guessed /
  missing axes remain reachable from the packet.
- *"Users can move from summary to raw evidence using stable
  references without lossy translation."* — every
  non-`unavailable_result` packet MUST cite
  `open_raw_event_inspector_pane` and `open_originating_run_pane`,
  enforced by the `allOf` invariant on `handoff_actions`.

## Fixture corpus

[`/fixtures/build/support_handoff_cases/`](../../fixtures/build/support_handoff_cases/)
covers the four scenarios named in the spec:

- `local_build_failure_handoff.yaml` — first-party native cargo
  runner emits a compile error for the `aureline-buffer` crate; a
  user opens the handoff packet to file an issue. Packet ceiling
  `authoritative_from_source`; callout basis
  `narrating_native_adapter_authoritative_facts`; chain `floor_reason_class`
  `no_floor_applied`; `handoff_actions` cite
  `open_raw_event_inspector_pane`,
  `open_originating_run_pane`,
  `file_workspace_issue_with_chain_attached`, and
  `copy_source_confidence_chain_refs_to_review_pack`.
- `imported_ci_build_handoff.yaml` — a managed-CI artifact log for
  the `aureline-text` crate is imported and a user opens the
  packet to escalate to managed-provider support. Packet ceiling
  `structured_parse_match`; freshness `imported_read_only`;
  import source `managed_ci_artifact`; callout basis
  `narrating_imported_result_read_only_facts`; chain
  `floor_reason_class`
  `imported_result_caps_at_structured_parse_match`.
- `parser_fallback_warning_handoff.yaml` — the moonbeam JVM build
  fell back to a free-form line parser; a user opens the packet
  from a parser-fallback warning shown on the picker chip. Packet
  ceiling `heuristic_best_effort`; lineage cites the
  `parser_fallback_reason_card_record`; `handoff_actions` cite
  `open_parser_fallback_reason_card` plus the standard
  inspector / run open actions; callout basis
  `narrating_parser_fallback_heuristics` carries a forbidden-claim
  set that includes "MUST NOT claim test outcomes parsed from
  free-form output as authoritative".
- `degraded_stale_result_support_packet.yaml` — a previously
  trusted native-cargo build of `aureline-buffer` is now
  `stale_result` because the toolchain revision changed; a user
  opens the support packet from the degraded chip. Packet ceiling
  `degraded_partial`; freshness `stale_pending_refresh`; lineage
  cites the `parser_fallback_reason_card_record` (stale-row
  variant); chain `floor_reason_class`
  `stale_result_caps_at_degraded_partial`; callout basis
  `narrating_stale_result_degraded_facts`; `handoff_actions`
  cite `open_parser_fallback_reason_card`,
  `open_support_bundle_export`, and
  `replay_in_doctor_with_imported_evidence`.

Every fixture is a multi-document YAML file and validates under
JSON Schema Draft 2020-12 against the boundary schema above plus the
underlying confidence-state, reason-card, and inspector schemas
seeded under
[`/fixtures/build/adapter_confidence_cases/`](../../fixtures/build/adapter_confidence_cases/).
