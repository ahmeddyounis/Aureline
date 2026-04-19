# ADR 0013 — Docs, Help / About, and service-health truth-source vocabulary and badge contract

- **Decision id:** D-0019 (see `artifacts/governance/decision_index.yaml#D-0019`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-10-01
- **Owner:** `@ahmedyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** product_scope_review
- **Related requirement ids:** none

## Context

Docs panes, the docs browser, the Help / About surfaces, the
service-health view, and the support-summary export all answer the
same family of questions about the same product instance: which
build is this; which docs pack does that build expect; is the
content I am reading authoritative for the build I am running; is
the service backing this row reachable; if I have to leave to a
browser, why; and which client class am I currently looking at. If
each of those surfaces invents its own answer — its own freshness
label, its own "this might be stale" tooltip, its own version
chip, its own service-state vocabulary, its own browser-handoff
reason copy — then the user, the support engineer, the AI
explainer, and the release council each see a different cut of the
same truth and a parity audit between Help, About, the docs pane,
and the service-health view degenerates into a hand-mapping
exercise.

The source documents
(`.t2/docs/Aureline_Technical_Architecture_Document.md`,
`.t2/docs/Aureline_Technical_Design_Document.md`,
`.t2/docs/Aureline_PRD.md`,
`.t2/docs/Aureline_UI_UX_Spec_Document.md`) treat docs source
class, docs-pack governance, About / Help disclosure (version,
channel, install mode, provenance, open-vs-managed boundary),
service-health contract states (`ready`, `degraded`, `local_only`,
`stale`, `contract_mismatch`, `policy_blocked`, `unavailable`),
the "no shadow docs" rule for the schema registry, and the
separation of support class, freshness, and client scope on
badges as first-order product contracts. The ADR-0010 connected-
provider freeze already provides the typed `browser_handoff_packet`
that any docs / help / service-health surface MUST use when it
hands off to the system browser; the ADR-0011 capability-lifecycle
freeze already provides the `freshness_class` and `client_scope`
vocabularies these surfaces inherit. The ADR-0008 settings
resolver already forbids copy-only shadow surfaces on the schema
registry. What is still missing is the shared truth-source
vocabulary that pins which class of source produced a docs row,
how that row's version relates to the running build, why a
freshness downgrade fired, which client surface the row is
rendered on, and — when the action takes the user out of the
product — which `browser_handoff_packet` reason class the
disclosure quotes.

The freeze matters now, ahead of the docs-browser, AI-explanation,
onboarding, support-export, and service-health lanes landing: if
those lanes proliferate before a shared truth-source and badge
vocabulary is frozen, each will mint its own ("docs may be out of
date" on one surface; "build does not match" on another; "stale
mirror" on a third; an "external" link with no reason on a
fourth), and the parity audit between Help, About, docs panes,
service health, and support exports will be impossible to run
mechanically. This ADR closes `D-0019` (docs / Help / About /
service-health truth-source vocabulary and badge contract) so the
docs-browser, Help / About, service-health, AI-explanation,
onboarding, support-export, and citation lanes can instrument
against one contract.

This ADR rides alongside the ADR-0001 identity modes (the
`managed_admin_surface` client scope and the `policy_blocked`
service contract state ride that envelope), the ADR-0004 RPC
transport (truth-source records and badge records cross as typed
payloads; raw docs bytes never do), the ADR-0005 subscription
envelope (every docs / help / service-health row carries an
authority class and a freshness hint), the ADR-0007 secret
broker (no raw secret material reaches docs panes or service-
health rows), the ADR-0008 settings resolver (admin policy may
narrow which docs source classes are exposed and which badges
render), the ADR-0009 execution-context model (every emitted
badge record names an `execution_context_id`), the ADR-0010
connected-provider vocabulary (the `browser_handoff_packet`
record and its `destination_class` / `reason_code` sets are the
ones quoted from docs / help / service-health surfaces), and the
ADR-0011 capability-lifecycle vocabulary (the `freshness_class`
and `client_scope` axes are reused without minting parallel
vocabulary). This ADR does not redefine those contracts; it
defines the truth-source-specific and badge-specific fields they
refer to.

A full Help / About UI and a complete docs-browser implementation
are explicitly out of scope at this milestone; this freeze
establishes the truth-source vocabulary and the badge contract
those later surfaces will honour.

## Decision

Aureline freezes one **truth-source vocabulary** for docs packs,
generated reference, mirrored official docs, curated knowledge
packs, derived explanations, vendor / provider docs, support
runbooks, and external incident / status feeds; one
**version-match-state** axis pinning every rendered row to the
running build's identity (ADR-0011 freshness re-used as the
freshness axis); one **service-contract-state** vocabulary for
the service-health view; one **degraded-state-cause** vocabulary
that names why a row downgraded; one **citation-anchor** record
shape for citing into docs / generated reference / runbooks /
release notes / service-health events; one **browser-handoff
reason subset** that docs / help / service-health surfaces are
allowed to quote from the ADR-0010 packet; and one
**`help_status_badge_record`** that every docs pane, Help / About
surface, service-health row, and support summary projects so a
later parity audit can compare them field-for-field.

Every row rendered on a docs pane, the docs browser, a Help /
About surface, the service-health view, or a support-summary
export resolves to exactly one `help_status_badge_record` and
emits at most one `browser_handoff_packet` reference per
out-of-product link. Docs source class, version-match state,
freshness class, client scope, degraded-state cause, and
browser-handoff reason are **separately addressable fields** even
when a UI chip collapses them; chip collapse is a UI freedom, not
a record-shape freedom.

All rules below are stated in terms of contract, vocabulary, and
event names rather than specific crates so surface changes are
hygiene, not re-litigation.

### Source-of-truth ownership (frozen)

Each surface family has exactly one canonical owner. A surface
that re-renders a row owned by another family MUST quote the
owner's `help_status_badge_record` rather than mint its own; a
surface that mints its own copy of a row another family owns is
non-conforming.

| Surface family                         | Canonical owner                                                                                              | Authority class on the subscription envelope (ADR-0005) |
|----------------------------------------|--------------------------------------------------------------------------------------------------------------|---------------------------------------------------------|
| Docs panes / docs browser              | `docs_pack_registry` (host process) — the docs-pack manifest registry that resolves a docs id + version pin. | `derived_knowledge`                                     |
| Generated reference (CLI / API)        | `generated_reference_index` (host process) — derived from the same build identity as the binary.             | `derived_knowledge`                                     |
| Mirrored official docs                 | `mirrored_docs_index` (host process) — pinned mirror of an upstream pack with a signed manifest.             | `derived_knowledge`                                     |
| Curated knowledge packs                | `curated_knowledge_pack_registry` (host process) — packs vetted by the publishing pipeline.                  | `derived_knowledge`                                     |
| Derived explanations (AI)              | `derived_explanation_session` (caller process, broker-projected) — never authoritative on its own.           | `derived_knowledge` (must cite an authoritative anchor) |
| Vendor / provider docs                 | The connected provider record (ADR-0010); rendered as `inspect_only` imports.                                | `provider_overlay`                                      |
| Support runbooks                       | `support_runbook_registry` (host process) — versioned with the support-export schema.                        | `derived_knowledge`                                     |
| Help / About row                       | `about_packet_publisher` (host process) — single publisher of build identity, channel, install mode, provenance, open-vs-managed boundary. | `policy_entitlement`                                    |
| Service-health row                     | `service_health_aggregator` (host process) — single aggregator that feeds desktop, CLI, support export, and AI surfaces. | `derived_knowledge`                                     |
| Support summary export row             | `support_export_pipeline` (host process) — quotes the four owners above; never re-mints their badges.        | `derived_knowledge`                                     |

Rules (frozen):

1. The `docs_pack_registry`, the `generated_reference_index`, the
   `mirrored_docs_index`, the `curated_knowledge_pack_registry`,
   the `support_runbook_registry`, the `about_packet_publisher`,
   and the `service_health_aggregator` are the **only** authorities
   for their respective rows. Surfaces that render those rows
   subscribe to them; copy-only shadow surfaces are forbidden, in
   the same spirit as the ADR-0008 "one registry, no shadows"
   rule for settings.
2. A derived explanation (AI) MUST cite at least one authoritative
   anchor (a docs anchor, a generated-reference anchor, a runbook
   anchor, or a release-note anchor); a derived explanation that
   cannot cite an authoritative anchor renders as
   `derived_explanation_uncited` and routes to a repair hook.
3. Project docs (the docs pack the running build was published
   against) **outrank** vendor / provider docs when the two
   conflict on a topic that is in the project pack's scope. A
   docs pane that displays vendor docs in place of in-scope
   project docs without a typed `vendor_docs_overrides_project`
   disclosure is non-conforming.
4. The `about_packet_publisher` is the single publisher of build
   identity, channel, install mode, provenance state, docs source,
   open-vs-managed boundary, copy-build-info payload, and
   official / local destinations. Help / About MUST NOT compute
   any of those fields locally; CLI `--version`, the desktop About
   view, the CLI status row, the docs browser footer, and the
   support summary all read the publisher's record.
5. The `service_health_aggregator` is the single source for
   service-contract states; per-feature surfaces (extension panel,
   provider panel, AI panel) MUST NOT mint their own service
   states.

### Source-class vocabulary (frozen)

Every docs / help / service-health row names exactly one source
class. The set is closed; adding a class is additive-minor and
bumps `help_status_badge_schema_version`; repurposing a class is
breaking and requires a new decision row.

| Source class                  | What it represents                                                                                                                       | Default outranks vendor docs? |
|-------------------------------|------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------|
| `project_docs`                | The docs pack the running build was published against. Authoritative for project topics; outranks vendor docs on in-scope topics.        | Yes                           |
| `generated_reference`         | Reference material derived from the build itself (CLI help, command graph, schema registry, settings registry).                          | Yes (within its scope)        |
| `mirrored_official_docs`      | A signed mirror of an upstream pack (e.g. a language standard library reference) that the project chose to mirror locally.               | No (mirrored authority)       |
| `curated_knowledge_pack`      | A vetted, signed knowledge pack published by the project or by a permitted publisher; carries an explicit publisher and revision.        | Conditional (named per pack)  |
| `derived_explanation`         | AI / explainer output. Never authoritative on its own; MUST cite at least one authoritative anchor.                                      | No (must cite)                |
| `vendor_provider_docs`        | Docs fetched from a connected provider (ADR-0010 `inspect_only`); always carries provider freshness and actor class.                     | No (yields to project docs)   |
| `support_runbook`             | A runbook the support-export pipeline pins to a build; rendered on diagnostics, doctor probes, and support summaries.                    | Within support scope          |
| `external_status_feed`        | An external incident / status feed (provider status page, release-publisher status). Always carries `external_status_or_incident_feed`.  | No (always external)          |

Rules (frozen):

1. A docs pane MUST render the source class on every result row
   it shows; a result row without a source class is non-conforming.
2. The "outranks vendor docs" column is the default behaviour;
   admin policy MAY narrow it (force vendor docs to take
   precedence on a named topic) but MAY NOT silently widen it.
3. `derived_explanation` rows MUST render their citation chain
   inline; a derived explanation that hides its citations on the
   primary surface and only exposes them in a tooltip is
   non-conforming.
4. `external_status_feed` rows MUST always be rendered as
   external, MUST always carry a `browser_handoff_packet` ref
   for the user to follow, and MUST NOT be cached past the
   feed's declared freshness window.

### Version-match-state vocabulary (frozen)

Every row also names exactly one version-match state, computed
against the running build's exact-build identity (ADR-related
work; see `D-0011`). The state is what a parity audit reads when
asking "is this row authoritative for the binary I am running?"

| State                               | Meaning                                                                                                                            | Default badge treatment                                            |
|-------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------|
| `exact_build_match`                 | Row's source pins to the running build's exact-build identity. No drift.                                                           | No drift chip required.                                            |
| `compatible_minor_drift`            | Row's source pins to a build at the same major / minor as the running build but a different patch within the declared compat window. | Render a "minor drift" chip with the source pin.                   |
| `incompatible_drift_detected`       | Row's source pins to a build outside the declared compat window. Row MUST NOT render as authoritative.                             | Render a "version mismatch" chip with the running-build pin.       |
| `pre_release_unverified`            | Row's source is a pre-release / nightly pack whose verification has not completed.                                                 | Render a "pre-release" chip; deny mutating actions originating here.|
| `unknown_target_build`              | Row's source could not name a target build (mirror manifest unavailable / signature unverified).                                   | Render a typed degraded-state cause; route to repair hook.         |

Rules (frozen):

1. The version-match state is computed on render against the
   running build's exact-build identity; it is not cached past
   the row's freshness window without a re-verify.
2. `incompatible_drift_detected` and `unknown_target_build` rows
   MUST render with a typed `degraded_state_cause` and a
   `repair_hook_ref` (refresh-pack, switch-channel,
   reinstall-pack, contact-support). Silent fallback to "row
   available" is forbidden.
3. About / Help MUST always render `exact_build_match` against
   the running build's identity; the publisher denies render
   when it cannot resolve the running build (unauthenticated
   doctor probe, broken provenance) and routes to a repair hook.

### Freshness and client-scope axes (re-exported)

The `freshness_class` axis is the one frozen in ADR-0011 and
re-exported here without modification:
`authoritative_live`, `warm_cached`, `degraded_cached`, `stale`,
`unverified`. The `client_scope` axis is the one frozen in
ADR-0011 and re-exported here without modification:
`desktop_product`, `cli`, `companion_surface`, `remote_agent`,
`sdk_or_api`, `managed_admin_surface`.

Rules (frozen):

1. A row that names `freshness_class = authoritative_live` MUST
   have been refreshed against its canonical owner inside the
   surface's refresh window; a surface that cannot prove that
   refresh MUST render `warm_cached`, `degraded_cached`, `stale`,
   or `unverified` as appropriate.
2. A row whose `client_scope` excludes the rendering surface
   MUST NOT render as available there; at most it renders a
   tombstone with a typed `degraded_state_cause` of
   `client_scope_excludes_surface` and routes to the client class
   that does carry it.
3. The browser companion surface — the surface ADR-0001 names
   under `account_free_local`, `self_hosted`, and `managed_cloud`
   identity modes — MUST visibly mark client_scope on every docs
   / help / service-health row and MUST NOT silently surface a
   row whose client scope excludes it.

### Service-contract-state vocabulary (frozen)

Every service-health row names exactly one service-contract state
from the closed set below. The set is the shared vocabulary the
desktop view, the CLI status row, the support summary, and the
diagnostics export quote.

| State                | Meaning                                                                                                                                |
|----------------------|----------------------------------------------------------------------------------------------------------------------------------------|
| `ready`              | Service is reachable, in contract, and within its freshness window.                                                                     |
| `degraded`           | Service is reachable but slow, partial, or rate-limited; a typed `degraded_state_cause` is required.                                    |
| `local_only`         | Service is unavailable but the local-core continuity path applies; affected feature surfaces continue with a visible `local_only` chip. |
| `stale`              | Service was reachable; cached state is older than the row's freshness window; row MUST render `stale` rather than `ready`.              |
| `contract_mismatch`  | Service responded with a contract version Aureline does not understand; row MUST deny mutating actions and route to repair.             |
| `policy_blocked`     | Admin policy or workspace trust posture is denying access to the service from this surface; surface routes to repair / approval.        |
| `unavailable`        | Service is unreachable and no local-core fallback applies; surface routes to support / runbook.                                         |

Rules (frozen):

1. Unaffected services MUST remain `ready`; one outage MUST NOT
   paint the whole product as broken. The aggregator computes
   per-service rows; surfaces that collapse them visually MUST
   keep them separately addressable in the underlying record.
2. `local_only` is a first-class state, not a fallback for
   `unavailable`; a surface that renders `local_only` MUST also
   render the local-core continuity path the user can follow.
3. `policy_blocked` carries the policy epoch and a typed repair
   hook (`request_admin_policy_change`, `request_managed_access`);
   silent removal of the row is forbidden.
4. `contract_mismatch` denies mutating actions and routes to
   repair; it does NOT silently downgrade to `degraded`.

### Degraded-state-cause vocabulary (frozen)

Every row whose freshness, version-match, or service-contract
state is below `authoritative_live` / `exact_build_match` /
`ready` MUST name exactly one degraded-state cause. The set is
closed; adding a cause is additive-minor and bumps the schema
version; repurposing a cause is breaking.

- `pack_unreachable` — docs pack / mirror / runbook is currently
  unreachable.
- `version_mismatch` — row's source is outside the declared
  compat window for the running build.
- `signature_unverified` — pack manifest exists but signature
  verification has not completed.
- `mirror_continuity_broken` — mirror manifest is missing a
  predecessor digest or has a broken signing chain (mirrors the
  ADR-0012 `mirror_continuity_broken` denial reason for
  extensions).
- `pack_quarantined` — admin policy or the publishing pipeline
  has quarantined the pack.
- `local_only_fallback` — the row's canonical owner is
  unreachable and the local-core continuity path applies.
- `policy_blocked` — admin policy is denying the source class
  on this surface.
- `freshness_floor_unmet` — surface's required freshness floor
  was not met in the current refresh window.
- `client_scope_excludes_surface` — the row's client scope does
  not include the rendering surface.
- `derived_explanation_uncited` — a derived explanation could
  not cite an authoritative anchor.
- `vendor_docs_overrides_project` — vendor docs are being
  rendered in place of in-scope project docs (typed disclosure).
- `external_status_feed_unreachable` — external status feed is
  unreachable.
- `unresolved_axis` — a required axis (source class,
  version-match state, freshness class, client scope) could not
  be resolved.

Rules (frozen):

1. Every degraded-state cause MUST come with a `repair_hook_ref`
   (typed hook from the lifecycle ADR-0011 vocabulary:
   `enable_feature_flag`, `request_admin_policy_change`,
   `refresh_freshness`, `migrate_to_replacement`,
   `request_managed_access`, `contact_support`). Markers without
   a repair hook are forbidden.
2. The cause MUST render on the primary surface alongside the
   chip; a surface that hides the cause behind a tooltip while
   the chip reads "stale" is non-conforming.
3. Adding a degraded-state cause is additive-minor and requires
   a `help_status_badge_schema_version` bump.

### Citation-anchor record (frozen)

A `citation_anchor_record` is the typed answer to "where exactly
did this row come from?". Every docs / generated-reference /
runbook / derived-explanation row that backs a user-visible claim
emits at least one anchor. Reserved for full citation / symbol-
reference work to consume mechanically.

A `citation_anchor_record` carries:

- `anchor_id` — opaque, stable id, safe to log.
- `anchor_kind` — one of
  `docs_page_anchor`,
  `generated_reference_anchor`,
  `symbol_reference_anchor`,
  `code_span_anchor`,
  `runbook_step_anchor`,
  `release_note_anchor`,
  `service_health_event_anchor`,
  `mirrored_docs_anchor`,
  `curated_pack_anchor`,
  `vendor_provider_anchor`,
  `external_status_feed_anchor`.
- `source_class` — the source-class vocabulary frozen above.
- `source_revision_ref` — opaque pin to the docs-pack /
  generated-reference / runbook revision the anchor was minted
  against. Required for reconstructing the exact view a user
  saw.
- `target_path_or_symbol_ref` — the relative path inside the
  pack, the symbol id inside the graph, the runbook step id, or
  the service-health event id.
- `target_build_identity_ref` — the running-build id the anchor
  was minted under (so a parity audit can recompute the
  version-match state if the running build changed).
- `freshness_class_at_mint` — the freshness class the anchor was
  minted under.
- `policy_context` — `policy_epoch`, `trust_state`,
  `execution_context_id` (ADR-0008 / ADR-0009 / ADR-0001).
- `redaction_class` — declared redaction class for the anchor on
  logs, traces, support bundles, AI evidence packets, and
  mutation-journal entries.
- `minted_at` — monotonic timestamp.

Rules (frozen):

1. Every derived explanation MUST cite at least one authoritative
   anchor (`docs_page_anchor`, `generated_reference_anchor`,
   `symbol_reference_anchor`, `runbook_step_anchor`, or
   `release_note_anchor`); citing only an
   `external_status_feed_anchor` is not sufficient.
2. Anchors are stable across a docs-pack revision; a docs-pack
   re-publish that breaks an anchor MUST list the anchor in its
   migration notes.
3. Anchors MUST NOT include raw page bodies, raw symbol
   definitions, or raw runbook bodies; they hold ids only and
   the body is fetched from the canonical owner on demand.

### Browser-handoff reason subset (frozen)

Docs / Help / About / service-health surfaces that hand off to
the system browser MUST do so through an ADR-0010
`browser_handoff_packet`. They MUST NOT mint a parallel reason
vocabulary; they quote the existing `reason_code` set, narrowed
to the subset admissible from these surfaces:

- `external_docs_or_runbook` — content lives outside the local
  product (mirrored upstream docs portal, vendor portal, external
  runbook). Most common reason for docs / help.
- `provider_consent_flow` — user must complete a provider consent
  flow before continuing.
- `provider_admin_delegation` — admin / operator must delegate a
  permission on the provider before continuing.
- `license_or_portal_acceptance` — license or portal acceptance
  must occur on the destination.
- `admin_only_surface` — destination is admin-only on the
  provider; surface routes to the admin path.
- `step_up_required` — destination requires a step-up
  authenticator.
- `mutation_not_supported_in_product` — the action exists only on
  the provider's web surface.

Rules (frozen):

1. A browser handoff originating from a docs pane, the docs
   browser, a Help / About surface, the service-health view, or
   a support summary MUST quote one of the reasons above on its
   `browser_handoff_packet.reason_code`. A handoff that quotes a
   reason outside this subset (for example, `publish_now` /
   `release_publish` reasons) from a docs / help surface is
   non-conforming.
2. Every handoff MUST render its
   `disclosure_summary` (ADR-0010 requirement) inline; a docs
   row that hands off without a visible disclosure is
   non-conforming.
3. The user-facing label "Open in browser" is reserved for
   handoffs that quote one of these reasons; surfaces MUST NOT
   reuse the label for in-product navigation.
4. Service-health rows that link to an external incident /
   status feed quote `external_docs_or_runbook` and additionally
   set `external_status_feed = true` on the badge record so
   parity audits can distinguish a docs deep-link from a status
   feed.

### `help_status_badge_record` shape (frozen)

Every docs pane row, docs-browser result row, Help / About row,
service-health row, and support-summary row projects a single
`help_status_badge_record`. The record is the parity-audit
contract: any later audit between Help, About, docs panes, the
service-health view, and the support summary compares this record
field for field. Chip collapsing is a UI freedom; record
addressability is mandatory.

A `help_status_badge_record` carries:

- `record_kind` — discriminator (`help_status_badge_record`).
- `help_status_badge_schema_version` — integer, pinned.
- `badge_id` — opaque, stable id, safe to log.
- `surface_class` — one of
  `docs_pane`, `docs_browser`, `help_about`, `service_health`,
  `support_summary`, `onboarding`, `ai_explanation_overlay`.
- `source_class` — the source-class vocabulary frozen above.
- `version_match_state` — the version-match-state vocabulary
  frozen above.
- `running_build_identity_ref` — opaque pin to the running
  build's exact-build identity (consumed mechanically by parity
  audits and support exports).
- `source_revision_ref` — opaque pin to the source pack /
  reference / runbook revision the row was minted against.
  Null only for `external_status_feed` rows.
- `freshness_class` — re-exported from ADR-0011.
- `client_scopes` — array of client-scope tokens from ADR-0011.
- `service_contract_state` — required when
  `surface_class = service_health`; null otherwise.
- `degraded_state_cause` — required when freshness class is
  below `authoritative_live`, version-match is below
  `exact_build_match`, or service-contract state is below
  `ready`. Null otherwise.
- `repair_hook_ref` — required when `degraded_state_cause` is
  non-null. Reuses the ADR-0011 `repair_hook_ref` shape and
  vocabulary.
- `browser_handoff_packet_ref` — required when the row exposes
  an out-of-product action; null otherwise. Reuses the ADR-0010
  `browser_handoff_packet` envelope; the badge record carries
  only the packet id and the reason_code, never raw URLs.
- `browser_handoff_reason` — copy of the packet's `reason_code`
  (from the subset above), exposed inline on the badge so a
  parity audit can read the reason without joining packets.
- `external_status_feed` — boolean; true when
  `source_class = external_status_feed`.
- `vendor_overrides_project` — boolean; true when this row is
  vendor docs rendered in place of in-scope project docs.
- `citation_anchor_refs` — array of `anchor_id`s the row backs
  itself with. Required (non-empty) for `derived_explanation`
  rows; optional otherwise.
- `lifecycle_label` — optional renderer hint (e.g. `preview`,
  `deprecated`) when the row also carries an ADR-0011 lifecycle
  posture; surface treats it as a separate chip from the
  freshness / version-match chips.
- `policy_context` — `policy_epoch`, `trust_state`,
  `execution_context_id`.
- `redaction_class` — declared redaction class for the badge on
  logs, traces, support bundles, evidence packets, and
  mutation-journal entries. Reuses the ADR-0011 redaction
  vocabulary (`metadata_safe_default`, `operator_only_restricted`,
  `internal_support_restricted`, `signing_evidence_only`).
- `minted_at` — monotonic timestamp.
- `last_refreshed_at` — monotonic timestamp of the last refresh
  against the canonical owner.

Required parity fields (frozen):

The fields below MUST be present on every emitted record so a
later parity audit between Help, About, docs panes, the
service-health view, and the support summary can compare them
mechanically without inventing badge semantics:

- `surface_class`,
- `source_class`,
- `version_match_state`,
- `running_build_identity_ref`,
- `freshness_class`,
- `client_scopes`,
- `degraded_state_cause` (or explicit null),
- `browser_handoff_reason` (or explicit null),
- `external_status_feed` (boolean),
- `vendor_overrides_project` (boolean),
- `policy_context`,
- `redaction_class`.

A surface that emits a badge record without one of these fields
is non-conforming.

### Per-surface projection requirements (frozen)

Each surface MUST project the truth-source vocabulary into its
record using the fields below. A surface that renders a row
without the required projection is non-conforming.

| Surface                              | Required projected fields                                                                                                                                                                                            | Required degraded / handoff treatment                                                                                                              |
|--------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| Docs pane / docs browser result      | `source_class`, `version_match_state`, `freshness_class`, `client_scopes`, `running_build_identity_ref`, `source_revision_ref`, `browser_handoff_reason` (when applicable), `vendor_overrides_project`               | Every degraded cause renders inline with a `repair_hook_ref`; vendor-overrides-project rows render the typed disclosure on the primary surface.    |
| Help / About row                     | All of the above plus the `about_packet_publisher` payload (build identity, channel, install mode, provenance state, docs source, open-vs-managed boundary, copy-build-info payload). About MUST always render `version_match_state = exact_build_match` against the running build. | Help / About denies render and routes to a repair hook when the publisher cannot resolve the running build; silent placeholder is forbidden.       |
| Service-health row                   | All required parity fields plus `service_contract_state` and the per-service refresh window. The aggregator emits one row per service.                                                                               | Per-service `degraded_state_cause` renders inline with a `repair_hook_ref`; surfaces MUST NOT collapse multiple services into one chip silently.   |
| Support summary export row           | All required parity fields plus the consumed `about_packet_publisher` payload (quoted, never re-minted) and the per-row source / version / freshness / handoff record under the support-export redaction envelope. | Support summary refuses to emit a row whose required parity fields are missing; the export pipeline names the missing fields in a typed denial.    |
| Onboarding step                      | All required parity fields plus the onboarding step id; onboarding MAY only quote rows whose `freshness_class` is `authoritative_live` or `warm_cached`.                                                             | Onboarding denies a step that cannot resolve a row at the required freshness floor and routes to a repair hook; silent skip is forbidden.          |
| AI explanation overlay               | All required parity fields plus a non-empty `citation_anchor_refs`. AI overlays MUST render the citation chain inline; tooltip-only citation is forbidden on protected surfaces.                                     | AI overlay denies render with `derived_explanation_uncited` when no authoritative anchor can be cited; silent fallback to a generic answer is forbidden. |

Rules (frozen):

1. Chip collapsing is a UI freedom, not a record-shape freedom.
   A surface that folds source / version / freshness / handoff /
   client-scope into one chip MUST keep the underlying fields
   separately addressable in its record so parity audits, support
   bundles, and AI explanations can read each axis independently.
2. The `about_packet_publisher` payload is single-source. Help,
   About, the docs-browser footer, the CLI status row, and the
   support summary all subscribe to it; a surface that
   re-derives any of its fields locally is non-conforming.
3. The `service_health_aggregator` is single-source. Per-feature
   panels (extension panel, provider panel, AI panel, doctor
   probe) MUST quote it; per-feature service-state vocabulary
   inventions are forbidden.
4. The browser-handoff packet is single-source (ADR-0010). Docs /
   Help / Service-health surfaces MUST NOT mint their own packets
   or call the system browser directly; raw URL launches from
   these surfaces are forbidden.
5. The badge schema version is pinned per emission so a parity
   audit can detect a surface that lags behind the registry.

### Audit events (frozen)

Every emission, every degraded-state transition, every browser
handoff originating from a docs / help / service-health surface,
and every parity refusal emits a structured event on the
`docs_help_service_health` audit stream. Events carry the badge
id, the surface class, the source class, the version-match
state, the freshness class, the service-contract state (when
applicable), the degraded-state cause (when applicable), the
browser-handoff packet id (when applicable), and the policy
context. Events MUST NOT carry raw docs bytes, raw runbook
bodies, raw symbol definitions, or raw URLs.

| Event id                                                    | Fires when                                                                                       |
|-------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| `help_status_badge_emitted`                                 | A `help_status_badge_record` was emitted on a surface.                                           |
| `help_status_badge_refused`                                 | Emission refused (required parity field missing, source-class unresolved, etc.).                 |
| `docs_pack_version_match_changed`                           | Version-match state of a docs row changed (`exact_build_match` -> `compatible_minor_drift`, etc.). |
| `docs_pack_freshness_downgraded`                            | Freshness class moved below `authoritative_live`.                                                |
| `docs_pack_freshness_recovered`                             | Freshness class returned to `authoritative_live`.                                                |
| `vendor_docs_overrode_project`                              | Vendor docs rendered in place of in-scope project docs (typed disclosure recorded).              |
| `derived_explanation_cited`                                 | A derived explanation cited at least one authoritative anchor.                                   |
| `derived_explanation_uncited_refused`                       | A derived explanation could not cite an authoritative anchor; render denied.                     |
| `service_contract_state_changed`                            | Service-contract state transitioned (`ready` -> `degraded`, etc.).                               |
| `service_local_only_engaged`                                | Service entered `local_only`; aggregator recorded the local-core continuity path.                |
| `service_policy_blocked`                                    | Service rendered as `policy_blocked`; surface routed to repair hook.                             |
| `about_packet_publisher_unresolved`                         | About / Help could not resolve the running build; render denied.                                 |
| `browser_handoff_from_docs_help_service_health_issued`      | A browser handoff was issued from a docs / help / service-health surface (names packet id).      |
| `parity_audit_field_missing`                                | A parity audit detected a required parity field missing on a peer surface.                       |
| `help_status_badge_schema_version_bumped`                   | `help_status_badge_schema_version` was bumped.                                                    |

### Denial posture (frozen)

When a docs / help / service-health surface cannot render a row
safely it denies. Denial is typed, visible, auditable, and
repairable. Silent downgrade to a generic "unavailable" chip is
forbidden.

The denial-reason set:

- `source_class_unresolved`
- `version_match_state_unresolved`
- `incompatible_drift_detected`
- `signature_unverified`
- `mirror_continuity_broken`
- `pack_quarantined`
- `freshness_floor_unmet`
- `client_scope_excludes_surface`
- `service_contract_mismatch`
- `policy_blocked`
- `derived_explanation_uncited`
- `vendor_docs_overrode_project_without_disclosure`
- `about_packet_publisher_unresolved`
- `browser_handoff_reason_outside_subset`
- `parity_field_missing`
- `help_status_badge_schema_version_lagging`

Denials fail closed. They MUST NOT silently retry, MUST NOT
substitute a different source class, and MUST emit the
corresponding audit event.

### Process-boundary constraints (frozen)

1. `help_status_badge_record` and `citation_anchor_record`
   instances cross the RPC boundary as typed payloads (ADR-0004).
   Raw docs bytes, raw runbook bodies, raw symbol definitions,
   and raw URLs never cross.
2. The `about_packet_publisher`, `service_health_aggregator`,
   `docs_pack_registry`, `generated_reference_index`,
   `mirrored_docs_index`, `curated_knowledge_pack_registry`,
   and `support_runbook_registry` are authoritative in the host
   process. Extensions, AI tool calls, recipes, and remote
   helpers read these surfaces only through the shared
   subscription envelope (ADR-0005) with authority class
   `derived_knowledge` and a declared freshness hint.
3. Remote-agent attach surfaces a remote-scoped docs / help /
   service-health view whose `client_scope` is `remote_agent`;
   the host surface renders a typed degraded-state cause for
   any row whose client scope excludes the remote agent.
4. Crash dumps and core files MUST NOT inherit unresolved badge
   projections; a crash that lands mid-render discards the
   projection rather than persisting a partial axis set.
5. Mutation-journal entries, save manifests, support bundles,
   and evidence packets name `badge_id`, `source_revision_ref`,
   `running_build_identity_ref`, and `browser_handoff_packet_ref`
   only; they MUST NOT embed raw docs bytes, raw runbook
   bodies, or raw URLs.
6. AI tool calls MUST NOT cache derived explanations past the
   anchor's freshness window without re-citing; a cached
   explanation that outlives its anchors is denied with
   `derived_explanation_uncited`.

### Redaction defaults (frozen)

Every surface that emits observable state declares a redaction
class; the broker-owned redaction pass (ADR-0007) runs before
bytes reach any persistent or exportable sink.

| Surface                              | Default inclusion (badge / anchor fields)                                                                                                                                                                                  |
|--------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `logs_local`                         | `badge_id`, `surface_class`, `source_class`, `version_match_state`, `freshness_class`, `service_contract_state`, `degraded_state_cause`, audit-event ids. Raw URLs / raw bodies excluded.                                  |
| `traces_local`                       | Same as `logs_local`; span names MUST NOT include raw URLs or raw runbook bodies.                                                                                                                                          |
| `support_bundle`                     | Full per-axis values, full degraded-state-cause enumeration, full citation-anchor enumeration with `anchor_kind`, `source_class`, `target_path_or_symbol_ref`. Raw bodies excluded.                                        |
| `evidence_packet`                    | Release-relevant fields: `running_build_identity_ref`, `source_revision_ref`, `version_match_state`, `freshness_class`, full citation-anchor list. Raw bodies never included.                                              |
| `ai_context_capture`                 | `badge_id`, `source_class`, `version_match_state`, `freshness_class`, `citation_anchor_refs`, `disclosure_summary` per handoff packet. Raw bodies and raw URLs never captured.                                             |
| `recipe_manifest`                    | `badge_id`, `source_revision_ref`, `running_build_identity_ref`, `browser_handoff_packet_ref` only. Raw bodies and raw URLs forbidden.                                                                                     |
| `profile_export` / `sync`            | Same as `recipe_manifest`.                                                                                                                                                                                                  |
| `crash_dump`                         | Opt-in only; redaction scan precedes packaging; denied by default for badges whose `policy_context` references a managed policy bundle.                                                                                    |
| `mutation_journal_entry`             | `badge_id`, `surface_class`, `source_class`, `version_match_state`, `freshness_class`, `service_contract_state`, audit-event id. No raw bodies or raw URLs.                                                                |
| `save_manifest` (ADR-0006)           | Same as `mutation_journal_entry`.                                                                                                                                                                                           |
| `claim_manifest`                     | Full per-axis values, full citation-anchor enumeration. Raw bodies never included.                                                                                                                                          |
| `terminal_transcript`                | `badge_id` and `source_class` only; raw URLs require boundary-labelled confirmation before capture.                                                                                                                        |

Overrides are narrowing only; admin policy MAY reduce inclusion
further, but MAY NOT widen beyond the frozen exclusion rules.

### Linkage to neighbouring contracts (frozen)

- **Exact-build identity (`D-0011`).** The
  `running_build_identity_ref` field on every badge record names
  the build identity frozen by `D-0011`. The version-match-state
  axis is computed against that identity. About / Help refuses
  render when the identity cannot be resolved.
- **Docs-pack manifest work.** The `source_revision_ref`,
  `mirrored_docs_index`, `curated_knowledge_pack_registry`,
  `support_runbook_registry`, and `signature_unverified` /
  `mirror_continuity_broken` denial reasons reserve the contract
  surface the docs-pack manifest lane will instrument against;
  this ADR does not implement the manifest itself.
- **Citation / symbol-reference packets.** The
  `citation_anchor_record` shape reserves the contract that
  later citation and symbol-reference packets will quote;
  symbol-reference packets bind anchor ids to graph objects and
  to originating docs-pack revisions so support bundles, AI
  evidence, and exported explanations can reconstruct what was
  cited. This ADR does not implement the symbol-reference
  packet itself.
- **Browser-handoff packet (ADR-0010).** Docs / help /
  service-health surfaces MUST quote the existing packet
  envelope and the named subset of `reason_code` values; they
  MUST NOT mint a parallel packet vocabulary.
- **Capability-lifecycle vocabulary (ADR-0011).** The
  `freshness_class`, `client_scope`, `repair_hook_ref`, and
  `redaction_class` enumerations are re-exported without
  modification; this ADR does not redefine them.
- **Settings resolver (ADR-0008).** Admin policy may narrow
  which source classes render, may pin a docs source class
  per surface, may raise the freshness floor, and may force a
  step-up authenticator on a `browser_handoff` from a docs /
  help surface. Policy MAY NOT silently widen.

### Schema-of-record posture (frozen)

Rust types in the eventual docs-help-service-health crate are the
source of truth. The JSON Schema export at
`schemas/docs/help_status_badge.schema.json` is the cross-tool
boundary every non-owning surface reads. Adding a new
source class, version-match state, service-contract state,
degraded-state cause, anchor kind, audit-event id, or denial
reason is additive-minor and bumps
`help_status_badge_schema_version`; repurposing a value is
breaking and requires a new decision row.

There is no external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004, ADR 0005, ADR 0006, ADR 0007,
ADR 0008, ADR 0009, ADR 0010, ADR 0011, and ADR 0012.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- A full Help / About UI implementation. The vocabulary here
  reserves the parity contract; the UI lands later.
- A complete docs-browser implementation. The vocabulary here
  pins source class, version-match state, freshness, client
  scope, and handoff reasons; the search / index / pack-resolver
  lane lands later.
- The docs-pack manifest itself (manifest schema, signing
  pipeline, mirror manifest format, pack-publishing workflow).
  This ADR reserves `source_revision_ref` and the related denial
  reasons; the manifest lane fills in the bodies.
- The citation / symbol-reference packet body. This ADR reserves
  the `citation_anchor_record` shape; the packet lane wires it
  to graph objects and exports.
- AI explanation infrastructure (model selection, prompt
  templates, provider routing). This ADR pins the disclosure
  contract derived explanations must honour; the explanation
  pipeline rides this contract.
- Service-health probe implementations. This ADR pins the
  vocabulary the aggregator emits; per-service probes land with
  each service.

These lines move only by opening a new decision row, not by
editing this ADR.

### Tradeoff summary

| Axis                          | Chosen stack                                                                                                                                                                            | Best rejected alternative                                                                            | Why chosen wins                                                                                                                                  |
|-------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------|
| **Source-of-truth ownership** | Eight named owners (docs-pack registry, generated-reference index, mirrored docs index, curated knowledge pack registry, derived explanation session, support runbook registry, About publisher, service-health aggregator) with no shadow surfaces | Each surface family owns its own copy and reconciles offline                                          | Per-surface copies recreate the docs / help / health drift this ADR is closing; one publisher per family makes parity audits mechanical.         |
| **Source-class vocabulary**   | Eight closed source classes pinned per row, with a typed "outranks vendor docs" rule for project docs and generated reference                                                            | One generic "docs source" enum                                                                       | A generic enum hides whether a row is project-authoritative, mirrored, vendor, or AI-derived; users and parity audits cannot tell them apart.    |
| **Version-match axis**        | Five-state axis (`exact_build_match`, `compatible_minor_drift`, `incompatible_drift_detected`, `pre_release_unverified`, `unknown_target_build`) computed against the running build identity | Free-form "may be out of date" tooltip                                                               | A tooltip is not machine-readable; parity audits and support exports cannot consume it.                                                          |
| **Freshness / client scope**  | Re-exported from ADR-0011 without modification; one vocabulary across capability rows and docs / help / service-health rows                                                              | Mint a docs-specific freshness vocabulary                                                            | Two freshness vocabularies guarantee drift; reuse keeps lifecycle and docs surfaces parsing the same tokens.                                     |
| **Service-contract states**   | Closed seven-state set (`ready`, `degraded`, `local_only`, `stale`, `contract_mismatch`, `policy_blocked`, `unavailable`) emitted by one aggregator                                       | Per-feature service vocabularies                                                                     | Per-feature vocabularies guarantee drift across panels; one aggregator keeps desktop / CLI / support / AI parity in scope.                       |
| **Degraded-state cause**      | Closed thirteen-cause set with required `repair_hook_ref` (typed hooks reused from ADR-0011)                                                                                              | Free-form "this might be stale" string                                                               | Free-form strings are not actionable; typed causes plus repair hooks let users and admins fix the underlying problem.                            |
| **Citation anchors**          | Typed `citation_anchor_record` with `anchor_kind`, `source_class`, `source_revision_ref`, and `target_build_identity_ref`                                                                | Plain text citation in the explanation body                                                          | Plain text breaks on docs-pack re-publish; typed anchors are stable, auditable, and reconstructible.                                             |
| **Browser-handoff reasons**   | Quote a named subset of the ADR-0010 packet's `reason_code` set; do not mint a parallel vocabulary                                                                                        | Free-form "open in browser" with a copy field                                                        | Free-form copy hides the reason class from parity audits and support exports; quoting ADR-0010 keeps one vocabulary across handoff surfaces.     |
| **Badge record shape**        | One `help_status_badge_record` shared by docs panes, Help / About, service health, support summary, onboarding, and AI overlay; required parity fields enumerated                          | Per-surface badge records                                                                            | Per-surface records make parity audits a hand-mapping exercise; one record makes audits mechanical.                                              |
| **Schema of record**          | Rust types in the eventual docs-help-service-health crate; JSON Schema export at `schemas/docs/help_status_badge.schema.json`                                                              | External IDL + codegen at this milestone                                                              | No second-language consumer yet; the JSON Schema export reserves a clean integration point.                                                      |

Each row carries reopen triggers. A support-bundle finding that
a docs row rendered as `exact_build_match` while a live mirror
manifest reported drift, a parity-audit finding that Help and
About disagree on `version_match_state`, an AI-overlay finding
that a derived explanation rendered without a citation anchor,
a service-health finding that two surfaces showed different
service-contract states for the same service, or a docs-browser
finding that a vendor docs row overrode in-scope project docs
without the typed disclosure reopens the relevant row.

## Consequences

- **Frozen:** the source-of-truth ownership table — eight named
  owners (docs-pack registry, generated-reference index,
  mirrored docs index, curated knowledge pack registry, derived
  explanation session, support runbook registry, About publisher,
  service-health aggregator) — and the rule forbidding copy-only
  shadow surfaces.
- **Frozen:** the source-class vocabulary (`project_docs`,
  `generated_reference`, `mirrored_official_docs`,
  `curated_knowledge_pack`, `derived_explanation`,
  `vendor_provider_docs`, `support_runbook`,
  `external_status_feed`) and the project-docs-outrank-vendor-
  docs default.
- **Frozen:** the version-match-state axis
  (`exact_build_match`, `compatible_minor_drift`,
  `incompatible_drift_detected`, `pre_release_unverified`,
  `unknown_target_build`) and the rule that it is computed
  against the running build's exact-build identity on render.
- **Frozen:** the service-contract-state vocabulary (`ready`,
  `degraded`, `local_only`, `stale`, `contract_mismatch`,
  `policy_blocked`, `unavailable`) and the single-aggregator
  rule.
- **Frozen:** the degraded-state-cause vocabulary, the typed
  repair-hook requirement, and the rule that the cause renders
  on the primary surface alongside the chip.
- **Frozen:** the `citation_anchor_record` shape and the
  anchor-kind vocabulary; reserved for the citation / symbol-
  reference packet lane.
- **Frozen:** the browser-handoff reason subset docs / help /
  service-health surfaces are allowed to quote from ADR-0010;
  surfaces MUST NOT mint a parallel reason vocabulary.
- **Frozen:** the `help_status_badge_record` shape with the
  required parity fields enumerated, and the rule that chip
  collapsing is a UI freedom but record addressability is
  mandatory.
- **Frozen:** the per-surface projection requirements for docs
  panes, Help / About, service-health rows, support-summary
  rows, onboarding steps, and AI overlays.
- **Frozen:** the audit-event ids on the
  `docs_help_service_health` audit stream and the denial-reason
  set. Silent downgrade to a generic "unavailable" chip is
  forbidden; denials fail closed.
- **Frozen:** process-boundary constraints. Raw docs bytes, raw
  runbook bodies, raw symbol definitions, and raw URLs never
  cross RPC; badge and anchor records cross as typed payloads.
- **Frozen:** the schema of record is Rust types in the eventual
  docs-help-service-health crate; the boundary schema lives at
  `schemas/docs/help_status_badge.schema.json`; no external IDL
  or codegen toolchain at this milestone.
- **Permitted:** adding a new source class, version-match state,
  service-contract state, degraded-state cause, anchor kind,
  audit-event id, or denial reason is additive-minor with a
  schema bump. Repurposing any existing value is breaking and
  requires a new decision row.
- **Permitted:** admin policy MAY narrow which source classes
  render on a surface, pin a docs source class per surface,
  raise the freshness floor, force a step-up authenticator on a
  browser handoff from docs / help, or quarantine a pack.
  Policy MAY NOT silently widen beyond the frozen rules.
- **Permitted:** surfaces MAY collapse multiple axes into a
  single chip for dense rendering, provided the underlying
  record retains each axis as a separately addressable field.
- **Follow-up:** the docs-browser, Help / About, service-health,
  AI-explanation, onboarding, support-export, and citation
  lanes instrument against this vocabulary before claiming
  parity guarantees.
- **Follow-up:** the docs-pack manifest lane fills in the
  `source_revision_ref` body and the mirror-continuity contract.
- **Follow-up:** the citation / symbol-reference packet lane
  consumes the `citation_anchor_record` shape and binds anchor
  ids to graph objects.
- **Ratifies:** the ADR-0010 `browser_handoff_packet` envelope
  is the only handoff path docs / help / service-health
  surfaces use. The ADR-0011 `freshness_class`, `client_scope`,
  `repair_hook_ref`, and `redaction_class` vocabularies are
  re-exported without modification. The ADR-0008 "one registry,
  no shadows" rule for settings is mirrored here for docs /
  help / service-health surfaces.

## Alternatives considered

- **Per-surface badge records.** Rejected: per-surface records
  recreate the docs / help / service-health drift this ADR is
  closing; a parity audit becomes a hand-mapping exercise rather
  than a mechanical comparison.
- **Free-form "this might be out of date" tooltips.** Rejected:
  tooltips are not machine-readable; parity audits, support
  exports, and AI overlays cannot consume them. The typed
  degraded-state-cause vocabulary makes each downgrade
  enumerable and repairable.
- **One generic `docs_source` enum.** Rejected: collapses
  project / mirrored / vendor / AI-derived sources, hides which
  rows can be authoritative, and forces per-surface rules ("if
  the source is X then …") to be re-derived on every surface.
- **Mint a docs-specific freshness vocabulary.** Rejected:
  guarantees drift between the lifecycle ADR and the docs /
  help / service-health vocabulary. Re-exporting the
  ADR-0011 axes keeps one vocabulary across the protected
  surfaces.
- **Mint a parallel browser-handoff packet for docs / help.**
  Rejected: ADR-0010 already freezes the typed handoff envelope;
  minting a parallel one would force every callback validator,
  every redaction pass, and every audit consumer to learn two
  shapes. Quoting a named subset of `reason_code` is sufficient.
- **Allow derived explanations without citation.** Rejected: an
  uncited explanation is indistinguishable from a free-form
  guess on every export surface; the typed
  `derived_explanation_uncited` denial keeps the contract
  honest.
- **Render `service_contract_state` as one chip per surface,
  per service.** Rejected: silent collapsing of multiple
  services into one "everything looks fine" chip is exactly the
  drift the single-aggregator rule prevents.
- **External IDL + generator for badge / anchor records.**
  Rejected: same argument ADR 0004 through ADR 0012 make — an
  IDL without a second-language consumer costs more than it
  buys; the JSON Schema export reserves the integration point.
- **Defer to a later milestone.** Rejected: the
  default-if-unresolved narrowing on `D-0019` (no shared
  truth-source vocabulary; per-surface badges; free-form
  freshness copy; raw URL launches from docs / help; per-feature
  service-state vocabularies) would force the docs-browser,
  Help / About, service-health, AI-explanation, onboarding,
  support-export, and citation lanes to land with incompatible
  assumptions that downstream tooling could not reconcile.

The `D-0019` `narrow` default-if-unresolved posture would have
locked the product to per-surface badges with free-form
freshness copy, no version-match axis, no shared service-
contract vocabulary, no typed degraded-state-cause set, no
typed citation anchors, no required repair hooks, no shared
browser-handoff reason subset, and no parity-audit contract.
Accepting this ADR replaces that narrowing with the truth-
source vocabulary, the source-class set, the version-match
axis, the service-contract-state set, the degraded-state-cause
set, the citation-anchor record, the browser-handoff reason
subset, the badge record shape, the per-surface projection
requirements, the audit-event list, and the denial posture
above; the narrowing default does not apply.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md` —
  knowledge source classes (project / generated / mirrored
  official / curated knowledge / derived explanations); docs-pack
  governance (publish / mirror / revoke / version through the
  artifact-governance system); registry / docs-pack delivery
  posture; degradation rules (every degraded state must expose
  reason, affected surfaces, and self-healing or manual-exit
  conditions); browser companion capability posture (visibly
  reduced capability, clear indicators when action requires
  remote authority).
- `.t2/docs/Aureline_Technical_Design_Document.md` — About
  packet (version, channel, install mode, provenance state, docs
  source, open / managed boundary, copy-build-info payload);
  service-health event / card (service family, boundary class,
  contract state, affected workflows, last-checked time,
  local-only continuity note, diagnostics actions); help
  destination descriptor (title, purpose, destination trust
  class, auth expectation, issue-template support, data-exit
  boundary); service-health vocabulary (`ready`, `degraded`,
  `local_only`, `stale`, `contract_mismatch`, `policy_blocked`,
  `unavailable`); single aggregator rule (one shared health
  aggregator feeds desktop, CLI, diagnostics, support export);
  About / Help no-sign-in rule; one publication pipeline for
  About / help / health packets; derived-explanation citation
  rule (must cite files, symbols, docs, or graph evidence);
  no-shadow-settings rule reused as a no-shadow-docs rule.
- `.t2/docs/Aureline_PRD.md` — normative MUST / SHOULD language
  on docs / help / service-health public truth and the
  open-vs-managed boundary disclosure on About / Help.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` — badge family
  taxonomy (status, mode, environment, provider, scope,
  confidence, permission, support class, lifecycle, freshness);
  rule that support class, freshness, and client scope must be
  separate cues rather than one overloaded badge; About / Help
  surface fields (install mode, channel, updater owner, binary
  root class, primary durable-state roots).
- `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md` —
  the typed `browser_handoff_packet` envelope and the
  `reason_code` set this ADR quotes a subset of.
- `docs/adr/0011-capability-lifecycle-and-dependency-markers.md` —
  the `freshness_class`, `client_scope`, `repair_hook_ref`,
  and `redaction_class` vocabularies this ADR re-exports.
- `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md` —
  the "one registry, no shadows" rule mirrored here for the
  docs / help / service-health surfaces.

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0019`
- RFC: none.
- Boundary schema:
  `schemas/docs/help_status_badge.schema.json`
- Badge vocabulary worked examples:
  `artifacts/docs/help_badge_vocabulary.yaml`
- Browser-handoff packet envelope this ADR quotes:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
  and `schemas/integration/browser_handoff_packet.schema.json`.
- Lifecycle / freshness / client-scope vocabulary this ADR
  re-exports:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
  and `schemas/governance/capability_lifecycle.schema.json`.
- Settings resolver whose admin-policy narrowing applies to
  docs / help / service-health surfaces:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`.
- Identity-mode envelope under which `managed_admin_surface`
  client scope and `policy_blocked` service-contract state ride:
  `docs/adr/0001-identity-modes.md`.
- Affected lanes: `governance_lane:product_scope_review`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:design_system_seeds`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance. No supersession.
