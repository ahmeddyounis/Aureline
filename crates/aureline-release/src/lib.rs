//! Release engineering contracts shared by UI, headless, support, and audit flows.
//!
//! This crate owns release-object types that must stay independent of any
//! single renderer, CI script, or support export. The first module is the
//! release-center object model: release candidates, version-bump proposals,
//! publish targets, artifact bundles, promotion steps, and scoped
//! rollback/revocation records. The correction-train module formalizes the
//! shared correction-train, hotfix, and backport packet form on top of the
//! same rollback and release-candidate refs. The stable-claim-matrix module
//! freezes the stable claim matrix, launch cutline, qualification rows, and
//! shiproom stop rules that decide which surfaces may publish as Stable. The
//! support-class-ledger module is the publication layer on top of that matrix:
//! it publishes the v1.0 support-class assignments, the certified-archetype
//! manifest, and the downgrade automation that narrows a published support
//! class when its backing thins out. The stable-qualification-matrix module
//! finalizes the per-lane qualification rows (desktop, remote/helper,
//! ecosystem, state/schema, provider, accessibility) that ground those claims
//! and, for every cross-binary or cross-service boundary, publishes the
//! mixed-version section — negotiated fields, supported skew window, upgrade and
//! rollback order, and unsupported-state behavior — that decides whether the
//! boundary may inherit a Stable mixed-version claim or is coordinated-upgrade-only.
//! The stable-claim-manifest module is the publication layer that binds all
//! three of those records together: it assigns each published subject one
//! canonical lifecycle label, names the backing claim row, qualification rows, and
//! support-class entry that label depends on, and attaches a packet-freshness SLO
//! so a subject whose proof packet has breached its SLO narrows below the launch
//! cutline automatically before publication. The stable-boundary-manifest module
//! is the deployment-boundary layer on top of that manifest: for every published
//! subject it records, across the local-OSS, self-hosted, managed, and air-gapped
//! value lines, the lifecycle label each line can carry — never wider than the
//! subject's canonical manifest label — so a value line that lacks a capability,
//! whose line evidence is incomplete, or whose proof packet aged out narrows below
//! the cutline before publication while still ingesting the one canonical label.
//! The stable-proof-index module is the requirement-facing layer that closes the
//! loop: for every launch-blocking requirement it records one row binding the
//! requirement to the proof packet that proves it, the waiver (if any) holding it
//! provisionally, and the public claim (a stable-claim-manifest entry) whose
//! lifecycle label that proof backs — never wider than the claim's canonical label
//! — so a requirement whose proof packet aged out or is missing, whose waiver
//! expired, whose requirement evidence is incomplete, or whose backing public claim
//! is itself below the cutline narrows below the launch cutline and holds
//! publication, while the launch-blocking requirement set stays fully covered.
//! The stable-version-windows module is the interface-freeze layer alongside that
//! index: for every public interface surface — a CLI command surface, a wire/state
//! schema, an API, or a manifest format — it freezes the stable version window
//! (floor, current, ceiling, compatibility posture) and the deprecation packet that
//! governs how older versions leave the window, backs each surface against a public
//! claim whose canonical label is a hard ceiling, and narrows below the cutline any
//! surface whose freeze packet aged out or is missing, whose deprecation packet is
//! incomplete or carries an overdue removal, whose waiver expired, whose surface
//! evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the CLI/schema/API/manifest surface kinds and the release-line surface
//! set both stay fully covered.
//! The maintenance-control-packet module is the post-release maintenance layer that
//! sits alongside those freezes: for every maintenance lane — an emergency hotfix lane,
//! a supported-line backport lane, a planned correction-train lane, or a support-window
//! commitment — it records one row binding the lane to the control packet that proves
//! it is staffed, the support window it commits to, and the shared correction-train
//! packet form it rides, backs each lane against a public claim whose canonical label
//! is a hard ceiling, and narrows below the cutline any lane whose control packet aged
//! out or is missing, whose support window is incomplete or has passed its
//! end-of-support date, whose waiver expired, whose lane evidence is incomplete, or
//! whose backing public claim is itself below the cutline — while the
//! hotfix/backport/correction-train/support-window lane kinds and the release-line lane
//! set both stay fully covered.
//! The shiproom-dashboard module is the consuming dashboard layer over all of the above:
//! for every shiproom panel — a claim-truth, qualification, public-proof, or maintenance
//! panel — it records one row binding the panel to the upstream source it ingests, the
//! qualification rows it watches, the freshness packet that proves it is current, and the
//! measurable fitness functions it must clear, backs each panel against a public claim
//! whose canonical label is a hard ceiling, and narrows below the cutline any panel whose
//! freshness packet aged out or is missing, whose fitness function failed or is
//! unmeasured, whose watched qualification row regressed, whose waiver expired, whose
//! panel evidence is incomplete, or whose backing public claim is itself below the cutline
//! — while the claim-truth/qualification/public-proof/maintenance panel kinds and the
//! release-line panel set both stay fully covered, so shiproom and release tooling can
//! fail promotion directly from the dashboard.
//! The optional-surface-qualification module is the claim-narrowing automation alongside all
//! of the above: where the manifest, qualification matrix, and proof index speak for surfaces
//! meant to ship at the cutline, this register governs the *optional* surfaces — opt-in
//! capabilities, optional integrations, secondary platforms, and shipped-but-experimental
//! previews — whose default is *narrowed*. For every optional surface it records one row
//! binding the surface to the public claim it backs and to its qualification packet as an
//! optional value, so a surface that lacks a stable qualification packet entirely, whose
//! packet breached its freshness SLO, whose surface evidence or capability is incomplete,
//! whose waiver expired, or whose backing public claim is itself below the cutline narrows
//! below the launch cutline and never inherits an adjacent qualified surface — while the
//! opt-in/integration/platform/preview surface kinds and the release-relevant surface set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The finalize-qualification-packets-for-optional-surfaces module is the M4-stable-line
//! finalization layer on top of that register: it enumerates every optional surface required
//! for M4 stable promotion — notebook/data-rich, voice/dictation, browser/mobile companion,
//! preview/designer/publish, AI-adjacent, browser-runtime inspectors, package/dependency
//! mutation, infrastructure/cluster live-state, pipeline/run-control overlays, collaboration
//! session admission, observer/follow modes, shared terminal/debug control, consent/retention
//! envelopes, and session export/delete — and records per-deployment-target access modes
//! (desktop local, remote/helper, managed, self-hosted, air-gapped) so a missing packet
//! forces automatic downgrade on every target rather than inheriting an adjacent green row.
//! The benchmark-lab-governance module is the performance-evidence layer beside those
//! gates: where the hot-path-performance-budgets register protects the published p50/p95
//! numbers for each individual hot path, this register governs the benchmark-lab
//! automation lanes, corpus governance assets, and public benchmark publication packs that
//! *produce* those numbers. For every such asset it records one row binding the asset to
//! the public claim it backs and to the proof packet that grounds it (a CI lane health
//! record, a corpus manifest, a protected-metrics revision, or a publication pack), protects
//! each benchmark publication's published p50/p95 budget against the measured numbers (with
//! corpus metadata, lab trace, and a waiver hook for intentionally tightened thresholds),
//! and narrows below the cutline any asset whose proof packet aged out or is missing, whose
//! corpus metadata or benchmark-lab trace is missing, whose waiver expired, whose evidence
//! is incomplete, or whose backing public claim is itself below the cutline — while the
//! nightly-ci/self-capture/corpus/metrics/hardware/image/ledger/publication-pack asset kinds
//! and the release-blocking asset set both stay fully covered, so shiproom and release
//! tooling can fail qualification directly from the register.
//! The cohort-scoreboards module is the signoff-loop layer beside those gates: it
//! finalizes the design-partner, certified-archetype, and stable-cohort
//! scoreboards as one canonical packet, binds every scoreboard row to a public
//! claim ceiling and proof packet, and narrows any row whose packet is stale,
//! metric fails, waiver expires, or required signoff loop is incomplete before the
//! row can widen release, docs, Help/About, or support-export language.
//! The certified-reference-workspaces module is the certification-evidence layer
//! that hardens every marketed Certified archetype: it publishes one current
//! reference-workspace report per archetype, binds each report to the archetype
//! pass-matrix row that carries it, and automates the downgrade that narrows a
//! Certified claim when its report goes stale, missing, or manually edited.
//! The stable-publication-pack module is the outward-facing publication layer over all of
//! the above: where the manifest, proof index, version windows, and maintenance packet
//! govern what the release line *is*, this pack governs what the release line *says about
//! itself* — its known-limits publications, its public benchmark publications, its
//! compatibility publications, and its migration publications. For every such publication
//! it records one row binding the publication to the public claim it backs and to the
//! proof packet that grounds it (a known-limits register, a benchmark-lab trace, a
//! compatibility report, or a migration guide), protects each benchmark publication's
//! published p50/p95 budget against the measured numbers (with corpus metadata, lab
//! trace, and a waiver hook for intentionally tightened thresholds), and narrows below
//! the cutline any publication whose proof packet aged out or is missing, whose measured
//! numbers regressed beyond the published budget, whose corpus metadata or trace is
//! missing, whose waiver expired, whose evidence is incomplete, or whose backing public
//! claim is itself below the cutline — while the known-limit/benchmark/compatibility/
//! migration publication kinds and the release-line publication set both stay fully
//! covered, so shiproom and release tooling can fail publication directly from the pack.
//! The claim-publication-manifest module is the joined publication source consumed by docs,
//! Help/About, service-health, CLI inspection, release notes, public proof, support export,
//! and enterprise evaluation surfaces: it links every rendered claim to current
//! reference-workspace, compatibility, and evaluation report refs, then narrows every
//! destination automatically when backing evidence is stale, missing, dropped, or unsigned.
//! The open-paid-boundary-audit module is the governance-fact layer beside those gates:
//! where the manifest, proof index, and version windows speak for product capabilities and
//! interface surfaces, this audit governs the governance facts the stable launch rests on —
//! where the open-source core ends and the paid/managed tier begins, the licensing posture,
//! the build provenance, and the contribution policy. For every audited subject it records
//! one row binding the subject to the public claim it backs and to its attestation packet,
//! its required audit controls, and an owner sign-off, so a subject whose attestation packet
//! aged out or is missing, whose required audit control is unsatisfied, whose evidence is
//! incomplete, whose owner sign-off is missing, whose waiver expired, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent attested row — while the open-paid-boundary/licensing/provenance/contribution-
//! policy domains and the release-line audit set both stay fully covered, so shiproom and
//! release tooling can fail promotion directly from the audit.
//! The go-no-go-rehearsal module is the launch-rehearsal layer that closes the loop over all
//! of the above: where the manifest, proof index, version windows, and audit govern what the
//! release line *is*, this rehearsal governs whether the release train was actually
//! *exercised* before the go/no-go — the explicit launch cutline signed off, the promotion
//! publish step dry-run, each rollback checkpoint verified to a restore point, and every open
//! exception packet reviewed. For every rehearsal stage it records one row binding the stage
//! to the public claim it backs and to its rehearsal packet, its required rollback
//! checkpoints, an exception packet (if any) holding it provisionally, and an owner sign-off,
//! so a stage whose rehearsal packet aged out or is missing, whose rollback checkpoint is
//! unverified, whose evidence is incomplete, whose owner sign-off is missing, whose exception
//! packet expired, or whose backing public claim is itself below the cutline narrows to a
//! No-Go below the launch cutline and never inherits an adjacent rehearsed stage — while the
//! cutline-review/promotion-step/rollback-checkpoint/exception-review stage kinds and the
//! release-line rehearsal set both stay fully covered, so shiproom and release tooling can
//! fail the go/no-go directly from the rehearsal.
//! The hot-path-performance-budgets module is the performance-layer register beside those
//! gates: for every hot path — startup, restore, quick open, typing, scrolling, search, and
//! Git status — it records one row binding the path to the stable claim manifest entry whose
//! lifecycle label it backs, the benchmark budget that protects the published p50/p95 numbers,
//! the proof packet that grounds them, and the waiver (if any) holding a tightened threshold
//! provisionally, so a path whose measured numbers regressed beyond the published budget,
//! whose proof packet aged out or is missing, whose corpus metadata or benchmark-lab trace is
//! absent, whose waiver expired, whose owner sign-off is missing, or whose backing public
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent backed budget — while the seven hot path kinds and the release-blocking path set
//! both stay fully covered, so shiproom and release tooling can fail promotion directly from
//! the register.
//! The accessibility-surface-signoffs module is the accessibility-layer register beside those
//! gates: for every touched surface — shell, tree, palette, diff, terminal, debugger, settings,
//! auth, and recovery — it records one row binding the surface to the stable claim manifest
//! entry whose lifecycle label it backs, the per-dimension checks that validate keyboard,
//! screen-reader, IME/grapheme/bidi, zoom, high-contrast, and reduced-motion behavior, the
//! proof packet that grounds them, and the waiver (if any) holding a provisional signoff, so
//! a surface whose dimension checks are blocked or pending, whose proof packet aged out or is
//! missing, whose owner sign-off is absent, or whose backing public claim is itself below the
//! cutline narrows below the launch cutline and never inherits an adjacent qualified surface —
//! while the nine surface kinds and the release-blocking surface set both stay fully covered,
//! so shiproom and release tooling can fail promotion directly from the register.
//! The clean-room-rebuild proof module is the exact-build supportability lane beside those
//! gates: for every marketed package-channel row, exact-build symbolication row, and release
//! truth parity surface, it records whether a fresh packet, verified rebuild evidence, and
//! exact-build symbol linkage still support the published claim or have already narrowed below
//! it — while mirror/offline publication coherence stays explicitly governed instead of being
//! inferred from the primary package rows alone.
//! The notebook-and-data-rich-surface-qualification module is the family-specific release
//! guard for notebook and data-heavy promoted surfaces. It keeps document trust,
//! kernel/runtime trust, and output trust as separate packet truths; binds notebook headers,
//! kernel bars, cells, output panes, variable explorers, data tables, result grids, chart
//! summaries, and experiment handoff cards to replay/export, snapshot/golden review,
//! accessibility, support-export, and downgrade-label evidence; and prevents notebook,
//! database/result-grid, or profiler-style language from widening unless that family row has
//! its own current proof.
//! The voice-and-dictation-surface-qualification module is the family-specific release
//! guard for speech input. It requires explicit command-vs-dictation mode truth,
//! push-to-talk or explicit activation defaults, provider/privacy disclosure, bounded
//! transcript handling, unavailable-state fallbacks, accessibility evidence, and command
//! graph parity before any voice or dictation row can render as Stable.
//! The publish-feature-scorecard-and-compatibility-packet-templates module is the
//! template-governance layer for every M5 feature family: it publishes the canonical
//! scorecard-template and compatibility-packet-template definitions that downstream
//! scorecards and compatibility reports must follow, binds each family to its templates,
//! tracks template-section publication state, and narrows any family whose templates are
//! incomplete, stale, missing required sections, or lack owner sign-off — while the
//! notebook/data-rich/ai-adjacent/framework/review/companion/managed-depth family kinds
//! and the release-blocking family set both stay fully covered, so shiproom and release
//! tooling can fail promotion directly from the register.
//! The freeze-the-m5-depth-claim-manifest module is the depth-claim freeze that closes the
//! M5 qualification loop: for every M5 feature family it records one feature-family packet
//! binding the family to the stable depth claim it backs and to a qualification matrix of one
//! cell per dimension — scorecard, compatibility, proof freshness, generated-artifact lineage,
//! locale parity, support-packet currency, accessibility, and downgrade automation — so a
//! family whose proof packet aged out or is missing, whose lineage is absent, whose locale
//! parity drifted, whose support packet lags shipped behavior, whose accessibility is unsigned,
//! whose downgrade automation is undefined, whose waiver expired, whose owner sign-off is
//! missing, or whose backing depth claim is itself below the cutline narrows below the launch
//! cutline and never inherits an adjacent qualified family — while the seven family kinds, the
//! eight qualification dimensions, and the release-blocking family set all stay fully covered,
//! so shiproom and release tooling can fail promotion directly from the manifest.
//! The implement-per-feature-scorecards module is the per-train qualification layer that
//! sits beside the depth-claim manifest: where the manifest speaks for the depth claim each
//! M5 feature *family* publishes, this register speaks for the per-feature *scorecard*, the
//! *owner manifest*, and the explicit *rollback/downgrade automation* every M5 feature train
//! carries. For every M5 train it records one scorecard binding the train to the stable claim
//! it backs, a scorecard of one cell per axis (functionality, performance, accessibility,
//! compatibility, localization, support readiness), an owner-manifest sign-off, and a
//! rollback/downgrade automation record bound to a verified rollback plan and the trigger and
//! floor it narrows to, so a train whose scorecard axis failed or is missing, whose proof
//! packet aged out or is missing, whose owner manifest is unsigned, whose rollback plan is
//! unverified, whose downgrade automation is undefined, whose waiver expired, or whose backing
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent qualified train — while the seven train kinds, the six scorecard axes, and the
//! release-blocking train set all stay fully covered, so shiproom and release tooling can fail
//! promotion directly from the register.
//! The ship-generated-artifact-lineage module is the lineage-truth layer for generated
//! outputs: where the train scorecard register speaks for each feature train, this register
//! speaks for the *lineage surface* every generated-artifact family exposes — scaffolded,
//! AI-generated, notebook-derived, and preview-derived outputs. For every family it records one
//! surface binding the family to the stable claim it backs, a lineage scorecard of one cell per
//! dimension (provenance, inputs, generator identity, transform, reproducibility, disclosure),
//! the disclosed artifact provenance and trust tier, an owner-manifest sign-off, and a
//! rollback/downgrade automation record bound to a verified rollback plan, so a surface whose
//! lineage dimension failed or is missing, whose artifact is not labeled as generated, whose
//! proof packet aged out or is missing, whose owner manifest is unsigned, whose rollback plan is
//! unverified, whose downgrade automation is undefined, whose waiver expired, or whose backing
//! claim is itself below the cutline narrows below the launch cutline and never inherits an
//! adjacent traced surface — while the four generator kinds, the six lineage dimensions, and the
//! release-blocking surface set all stay fully covered, so shiproom and release tooling can fail
//! promotion directly from the register.
//! The add-backport-and-hotfix-rules module is the post-release maintenance-truth layer beside
//! those gates: where the train scorecard register speaks for each feature train, this register
//! speaks for the *maintenance-truth lane* every lane kind exposes — the supported-line backport
//! rule, the emergency hotfix rule, the proof-freshness/evidence-expiry automation, and the
//! Help/About truth surface those lanes publish. For every lane it records one entry binding the
//! lane to the stable claim it backs, a maintenance scorecard of one cell per dimension (backport
//! policy, hotfix policy, proof freshness, evidence expiry, Help/About truth, and docs truth),
//! the disclosed support posture and maintainer trust tier, an owner-manifest sign-off, and a
//! rollback/downgrade automation record bound to a verified frozen-fallback rollback plan, so a
//! lane whose maintenance dimension failed or is missing, whose Help/About truth is undisclosed,
//! whose proof packet aged out or is missing, whose owner manifest is unsigned, whose rollback
//! plan is unverified, whose downgrade automation is undefined, whose waiver expired, or whose
//! backing claim is itself below the cutline narrows below the launch cutline and never inherits
//! an adjacent certified lane — while the four lane kinds, the six maintenance dimensions, and the
//! release-blocking lane set all stay fully covered, so shiproom and release tooling can fail
//! promotion directly from the register.
//! The freeze-the-m5-qualification-row module is the compatibility-governance layer beside those
//! gates: where the depth-claim manifest speaks for the depth claim each feature family publishes,
//! this matrix speaks for the *qualification row* every M5 stable-facing family must hold before it
//! may claim support, parity, or certification. For every family it records one row binding the
//! family to the stable claim it backs, a qualification row of one cell per dimension (platform,
//! deployment profile, archetype/workflow bundle, toolchain envelope, client scope), a declared
//! skew window with its supported class, version floor/ceiling, negotiated fields, and the
//! fail-closed/reconnect-required/reinstall-required/coordinated-upgrade-only behavior a peer
//! outside the window triggers, a support window, a deprecation packet (status, successor, removal
//! date, migration), a proof packet with freshness SLO, and an owner sign-off, so a family whose
//! qualification dimension is incomplete, stale, or retest-pending, whose peer is outside the skew
//! window, whose support window ended, whose deprecation staged a removal, whose waiver expired,
//! whose owner sign-off is missing, or whose backing claim publication is absent narrows below the
//! launch cutline and never inherits an adjacent qualified family — while the seven family kinds,
//! the five qualification dimensions, and the release-blocking family set all stay fully covered,
//! so docs, release notes, CLI inspect, in-product badges, support exports, certification reports,
//! and shiproom dashboards reuse one source of truth and can fail promotion directly from the
//! matrix.
//! The freeze-the-m5-public-contract module is the contract-publication layer beside those gates:
//! where the qualification-row matrix speaks for the compatibility boundary each family exposes,
//! this matrix speaks for the *contract publication* of every M5 artifact family the source docs
//! treat as a published contract. For each family it records one row binding the family to its
//! contract form, stability lane, reader/writer posture, and packaging need, one publication
//! requirement per contract form (JSON Schema, WIT, OpenAPI, Markdown summary, example payloads,
//! migration notes) plus the validator suite and release-packet linkage, and the lifecycle label
//! it is put forward at, so a family missing any required publication evidence raises the matching
//! gap reason and narrows below the launch cutline rather than inheriting an adjacent published
//! family — while the contract-form, gap-reason, and release-blocking family sets stay fully
//! covered, so claim manifests, Help/About, SDK/docs, support exports, and shiproom dashboards reuse
//! one published-contract inventory and can fail promotion directly from the matrix.
//! The implement-canonical-json-schema-packages module is the package-publication layer beneath that
//! matrix: where the matrix records *whether* each family has published its contract forms, this
//! catalog publishes the *JSON Schema package itself* for every M5 family the matrix puts forward as
//! a JSON-Schema-backed contract — a checked-in schema under `schemas/public/m5-json/` with an
//! explicit in-band version field, a lifecycle/stability label that equals the matrix's effective
//! published label, a field-level compatibility contract (additive-field rule, required-field policy,
//! unknown-field preservation, downgrade behavior, and migration-note hooks), an example payload, and
//! a round-trip fixture — so export/import, support export, and docs/help resolve one schema
//! identifier and one lifecycle label per family and durable artifacts round-trip without stripping
//! unknown fields.
//! The qualification-row badge-binding register is the publication layer over that matrix: for
//! every M5 family it binds the machine-readable qualification row to the marketable artifacts that
//! advertise it — a support-class badge that carries the published label, support class, live
//! evidence freshness, and known caveats; an evaluation pack; a compatibility report; and a
//! release-center card — rendered across a closed surface set that always covers release-center,
//! Help/About, service-health, and support-export, so freshness and caveats appear wherever a
//! support-class badge does. A badge may never publish wider than the qualification row it binds,
//! which in turn may never exceed the canonical claim, and the badge auto-narrows to inherit the
//! row when its evaluation pack, compatibility report, or evidence goes stale or missing or when
//! marketable wording would exceed the row; an inherited row narrowing narrows the badge while a
//! binding-layer failure holds promotion directly from the register.
//! The evaluation-pilot-pack register is the private partner-facing layer over the claim-publication
//! manifest: where the manifest is the single public source of truth, this register packages
//! enterprise and ecosystem evaluation/pilot materials on top of it. For every enterprise/ecosystem
//! lane it binds one pack to a named bundle id and its mirror refs, the support contacts, the
//! known-issues deltas beyond the public known-limits, the deployment caveats, and the public claim
//! entry it reuses — whose published label, support class, and exact wording are hard ceilings. A
//! pack may never publish a greener label or broader support class than its public claim, a published
//! pack reuses the public wording verbatim, every known-issues delta is disclosed, and every
//! partner-facing destination (evaluation pack, pilot packet, admin export, support export) renders
//! from the one pack so a narrowed pack downgrades every partner surface at once; "pilot-only" wording
//! can never bypass a support-class limit or stale evidence. An inherited public-claim narrowing
//! downgrades the surfaces but is gated by the claim manifest, while a pack-layer failure (a stale,
//! missing, dropped, or unsigned bundle mirror; stale or missing proof evidence; an expired window or
//! waiver; an over-claiming label or support class; or a missing owner sign-off) on a pack whose public
//! claim is still at or above the cutline holds promotion directly from the register.
//! The claim-scope export-packet register is the support/shiproom/docs/partner-review export layer over
//! the qualification matrix and the claim-publication manifest: for every claimed M5 family it binds one
//! export row that answers exactly which row is being claimed, what freshness and expiry state it carries,
//! what skew window applies, and what stale or retest-needed states are live, without tribal memory. Each
//! row joins the reopen refs (the qualification row, its deprecation packet, and the public claim entry) a
//! shiproom dashboard follows back to the authoritative record, the row-level truth that never collapses
//! into one flag (row state, skew-window class, support class, deprecation status, freshness, validity
//! window, evidence refs, and active stale/retest reasons), and the copy-safe scope wording every audience
//! renders — never greener than the public claim's published label or support class, both hard ceilings. A
//! row may never publish wider than the public claim, a row that holds the public label reuses the public
//! wording verbatim, and every audience (support, shiproom, docs, partner review) must disclose the row
//! freshness, the active stale/retest reasons, and the caveats, so a narrowed row downgrades every audience
//! at once and no exported packet loses the row-level reason. An inherited row downgrade is gated by the
//! matrix and claim manifest, while an export-layer failure (stale or missing export evidence, an expired
//! window or waiver, a missing sign-off, or over-claiming copy) on a row whose public claim is still at or
//! above the cutline holds promotion directly from the register.
//! The assurance-consumer-parity module is the convergence layer over the assurance center, the
//! assurance-claim reducer, the governance/fitness dashboard, the capability-boundary inspector, and
//! the event-provenance inspector: it ingests those five truth packets, normalizes every claim,
//! control-proof, governance, ownership, decision-right, boundary, route, approval, and event item
//! into one fact grammar (a gate, an effective qualification, an owner, a freshness reading, and the
//! evidence refs behind it), and projects each fact onto the About/help, procurement-export,
//! evaluation-packet, support-export, and shiproom/public-truth surfaces so they all read the same
//! fact at the same gate. Each fact's per-consumer projection reads the fact's own gate and every
//! consumer view reads every fact, so a fact narrowed or blocked in one surface can never read
//! stronger in another and any source narrowing carries through to every consumer at once. The packet
//! is metadata-only — it binds to each source by id and registry ref rather than embedding raw bodies
//! and reduces every fact to repo-relative evidence refs — so a refs-only export preserves
//! owner/freshness/route lineage without leaking raw material, and shiproom and release tooling can
//! fail promotion directly from the converged model.

#![doc(html_root_url = "https://docs.rs/aureline-release/0.0.0")]

pub mod add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts;
pub mod add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes;
pub mod add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth;
pub mod add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces;
pub mod add_forward_read_back_read_round_trip_and_migration_diff_suites_for_m5_workspace_state_evidence_support_appearance_learning_diagnostic_artifact_families;
pub mod add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families;
pub mod add_shared_assurance_center_release_center_operator_dashboard_support_export_shiproom_about_help_consumers_so_governance_dashboard_components_keep_fitness_ownership_waiver_and_decision_language_aligned;
pub mod add_shared_marketplace_help_settings_onboarding_diagnostics_export_runtime_and_workspace_consumers_so_badge_families_keep_label_explanation_and_downgrade_parity_across_claimed_m5_profiles;
pub mod add_shared_release_center_update_center_about_help_docs_evaluation_and_support_publication_component_consumers;
pub mod bind_qualification_rows_to_marketable_badges_evaluation_packs_and_compatibility_reports_for_every_m5_family;
pub mod browser_mobile_companion_surface_qualification;
pub mod certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows;
pub mod certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family;
pub mod certify_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_truth_on_every_claimed_m5_release_publication_surface;
pub mod certify_schema_publication_wit_openapi_packaging_validator_coverage_and_compatibility_truth_on_every_claimed_m5_public_artifact_family;
pub mod certify_support_class_evidence_freshness_lifecycle_channel_deployment_scope_and_compatibility_badge_truth_on_every_claimed_m5_surface;
pub mod certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index;
pub mod claim_publication_manifest;
pub mod correction_train;
pub mod finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack;
pub mod finalize_compatibility_reports_deprecation_packets_schema_version_windows;
pub mod finalize_design_partner_certified_archetype_and_stable_cohort;
pub mod finalize_experiments_labs_inventory;
pub mod finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance;
pub mod finalize_qualification_packets_for_optional_surfaces_and_enforce;
pub mod finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support;
pub mod finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills;
pub mod freeze_the_cross_surface_hardening_matrix_scorecards_and_evidence_bindings_for_every_m5_depth_surface;
pub mod freeze_the_m5_dependency_intelligence_package_health_and_code_quality_parity_matrix;
pub mod freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix;
pub mod freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph;
pub mod freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix;
pub mod freeze_the_m5_public_contract_schema_publication_wit_openapi_and_interchange_conformance_matrix;
pub mod freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix;
pub mod freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix;
pub mod freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix;
pub mod freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules;
pub mod freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix;
pub mod generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains;
pub mod generate_the_m5_browser_companion_and_embedded_boundary_manifest_with_handoff_eligibility_rows;
pub mod generate_the_m5_channel_profile_provider_rollout_matrix_for_depth_lanes;
pub mod go_no_go_rehearsal;
pub mod harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation;
pub mod harden_docs_help_about_and_service_health_truth;
pub mod harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage;
pub mod harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity;
pub mod implement_artifact_provenance_bundle_cards_and_attestation_or_sbom_status_rows_across_claimed_m5_release_evaluation_support_surfaces;
pub mod implement_canonical_json_schema_packages_explicit_version_fields_and_stability_labels_for_newly_stable_or_beta_m5_durable_artifacts;
pub mod implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages;
pub mod implement_decision_right_cards_and_milestone_dashboard_rows_with_required_forum_reason_satisfied_pending_state_blocker_and_waiver_counts_nearest_gate_and_next_review_continuity;
pub mod implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces;
pub mod implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance;
pub mod implement_fitness_dashboard_tiles_and_governance_report_rows_with_protected_metric_identity_threshold_state_provenance_evidence_freshness_owner_and_report_continuity;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_badge_freshness_lifecycle_deployment_support_or_compatibility_posture_is_stale_limited_imported_or_policy_blocked_across_claimed_m5_surfaces;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_evidence_freshness_waiver_expiry_owner_coverage_support_class_or_decision_right_truth_is_stale_or_partial_across_claimed_m5_governance_dashboard_components;
pub mod implement_keyboard_screen_reader_cli_export_parity_and_publication_component_claim_auto_narrowing;
pub mod implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces;
pub mod implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains;
pub mod implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs;
pub mod implement_promotion_timeline_steps_and_rollback_or_revocation_rows_across_claimed_m5_release_histories;
pub mod implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts;
pub mod implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth;
pub mod implement_release_candidate_cards_and_promotion_blocked_banners_across_claimed_m5_release_center_surfaces;
pub mod implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family;
pub mod implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs;
pub mod implement_service_ownership_cards_and_on_call_strips_with_role_based_owner_escalation_aliases_support_class_freshness_backup_coverage_and_export_safe_continuity;
pub mod implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces;
pub mod implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces;
pub mod implement_waiver_expiry_queue_items_release_gate_banners_and_mitigation_note_cards_with_owner_expiry_milestone_impact_blocked_waived_evidence_stale_vocabulary_and_user_facing_mitigation_truth;
pub mod m5_assurance_center;
pub mod m5_assurance_certification;
pub mod m5_assurance_claim_reducer;
pub mod m5_assurance_consumer_parity;
pub mod m5_assurance_route_governance;
pub mod m5_badge_vocabulary;
pub mod m5_benchmark_help_migration_components;
pub mod m5_boundary_inspector;
pub mod m5_change_impact_card;
pub mod m5_claim_narrowing;
pub mod m5_client_scope_card;
pub mod m5_compatibility_forecast;
pub mod m5_descriptor_badge;
pub mod m5_descriptor_certification;
pub mod m5_descriptor_join;
pub mod m5_descriptor_object;
pub mod m5_event_provenance;
pub mod m5_governance_dashboard;
pub mod m5_omission_guard;
pub mod m5_release_note_evidence;
pub mod m5_service_health_communication;
pub mod m5_support_window_card;
pub mod m5_truth_surface_evidence_ingestion;
pub mod m5_update_lifecycle;
pub mod m5_update_lifecycle_certification;
pub mod m5_update_summary;
pub mod maintenance_control_packet;
pub mod mixed_version_compatibility_and_skew_governance;
pub mod notebook_and_data_rich_surface_qualification;
pub mod open_paid_boundary_audit;
pub mod optional_surface_qualification;
pub mod preview_designer_publish_surface_qualification;
pub mod prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence;
pub mod publish_docs_migration_and_known_limits_packs_for_m5_feature_families;
pub mod publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family;
pub mod publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces;
pub mod publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes;
pub mod publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan;
pub mod publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet;
pub mod publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts;
pub mod publish_the_signed_m4_stable_evidence_pack_plus;
pub mod release_center_model;
pub mod seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails;
pub mod ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards;
pub mod ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains;
pub mod ship_cli_headless_structured_output_schemas_result_code_catalogs_and_schema_reference_links_for_every_new_m5_inspect_export_report_surface;
pub mod ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows;
pub mod ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity;
pub mod ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs;
pub mod ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries;
pub mod ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes;
pub mod ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions;
pub mod ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes;
pub mod ship_version_bump_rows_and_publish_target_review_sheets_across_claimed_m5_publication_lanes;
pub mod shiproom_dashboard;
pub mod stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery;
pub mod stabilize_embedded_surface_boundary_truth;
pub mod stabilize_hot_path_performance_against_published_budgets_for;
pub mod stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication;
pub mod stabilize_the_release_center_promotion_evidence_canary_pilot;
pub mod stable_boundary_manifest;
pub mod stable_claim_manifest;
pub mod stable_claim_matrix;
pub mod stable_proof_index;
pub mod stable_publication_pack;
pub mod stable_qualification_matrix;
pub mod stable_version_windows;
pub mod support_class_ledger;
pub mod voice_and_dictation_surface_qualification;

pub use freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix::{
    current_m5_depth_claim_manifest, DepthClaimExportProjection, DepthClaimExportRow,
    DepthClaimManifest, DepthClaimManifestSummary, DepthClaimManifestViolation, DepthStopAction,
    DepthStopRule, FamilyKind, FamilyPacket, NarrowingReason as DepthClaimNarrowingReason,
    PacketState, QualificationCell, QualificationDimension,
    QualificationState as DepthClaimQualificationState, FREEZE_M5_DEPTH_CLAIM_MANIFEST_JSON,
    FREEZE_M5_DEPTH_CLAIM_MANIFEST_PATH, FREEZE_M5_DEPTH_CLAIM_MANIFEST_RECORD_KIND,
    FREEZE_M5_DEPTH_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains::{
    current_m5_train_scorecard_register, AutomationState as TrainAutomationState,
    DowngradeAutomation, DowngradeTrigger, NarrowingReason as TrainScorecardNarrowingReason,
    ScoreGrade, ScorecardAxis, ScorecardCell, StopAction as TrainScorecardStopAction, TrainKind,
    TrainScorecard, TrainScorecardExportProjection,
    TrainScorecardExportRow, TrainScorecardRegister, TrainScorecardRegisterSummary,
    TrainScorecardRegisterViolation, TrainState, TrainStopRule,
    IMPLEMENT_M5_TRAIN_SCORECARDS_JSON, IMPLEMENT_M5_TRAIN_SCORECARDS_PATH,
    IMPLEMENT_M5_TRAIN_SCORECARDS_RECORD_KIND, IMPLEMENT_M5_TRAIN_SCORECARDS_SCHEMA_VERSION,
};

pub use implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family::{
    build_m5_family_release_graph, current_m5_family_release_graph, BlockerClass, BlockerRow,
    BundleMemberCard, BundleMemberKind, EvidenceFreshnessRow, FamilyGapReason,
    FamilyRemediationAction, FamilyStopRule, M5FamilyReleaseCandidate, M5FamilyReleaseExportProjection,
    M5FamilyReleaseExportRow, M5FamilyReleaseGraph, M5FamilyReleaseGraphSummary,
    M5FamilyReleaseGraphViolation, MemberPresence, ScopedArtifactBundleCard,
    IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_JSON, IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_PATH,
    IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND, IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION,
};

pub use implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs::{
    build_m5_artifact_graph_promotion_ledger, current_m5_artifact_graph_promotion_ledger,
    ArtifactGraphNode, FamilyPromotionLedger, HistoryPointerClass, HistoryReconstructionParity,
    LedgerState as PromotionLedgerState, M5ArtifactGraphPromotionExportProjection,
    M5ArtifactGraphPromotionExportRow,
    M5ArtifactGraphPromotionRegister, M5ArtifactGraphPromotionStopRule,
    M5ArtifactGraphPromotionSummary, M5ArtifactGraphPromotionViolation,
    NarrowingReason as PromotionLedgerNarrowingReason, ParityState as PromotionHistoryParityState,
    PromotionReplayEntry, StopAction as PromotionLedgerStopAction,
    M5_ARTIFACT_GRAPH_PROMOTION_JSON, M5_ARTIFACT_GRAPH_PROMOTION_PATH,
    M5_ARTIFACT_GRAPH_PROMOTION_RECORD_KIND, M5_ARTIFACT_GRAPH_PROMOTION_SCHEMA_VERSION,
};

pub use implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts::{
    current_m5_public_interface_diff_reports, ChangeClass,
    CompatibilityPosture as DiffReportCompatibilityPosture, CompatibilityWindow, ContractAlias,
    ContractDiffReport, ContractDiffReportExportProjection, ContractDiffReportExportRow,
    ContractDiffReportRegister, ContractDiffReportRegisterSummary, ContractDiffReportViolation,
    ContractKind, DeprecationPacket as DiffReportDeprecationPacket,
    DeprecationStatus as DiffReportDeprecationStatus, DiffReportStopRule, InterfaceDiff,
    NarrowingReason as DiffReportNarrowingReason, ReportState as DiffReportState, ReviewPosture,
    StopAction as DiffReportStopAction, SupportClass as DiffReportSupportClass, SupportClassCaveat,
    WindowSupportState, M5_PUBLIC_INTERFACE_DIFF_REPORTS_JSON, M5_PUBLIC_INTERFACE_DIFF_REPORTS_PATH,
    M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND, M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION,
};

pub use implement_contract_ci_gates_release_artifact_graph_linkage_and_shiproom_blockers_for_stale_missing_or_incompatible_m5_schema_spec_packages::{
    current_m5_contract_health_register, Blocker, BlockerDecision, Blockers, BuildIdentity,
    ContractHealthRow, FreshnessState, Gate, GateDescriptor, GateKind, GateOutcome, GraphLinkage,
    HealthState, LaunchCutline as ContractHealthLaunchCutline,
    LifecycleLabel as ContractHealthLifecycleLabel, M5ContractHealthExportProjection,
    M5ContractHealthExportRow, M5ContractHealthRegister, M5ContractHealthSummary,
    M5ContractHealthViolation, MirrorParityState, PackageIdentity, PackageKind,
    M5_CONTRACT_HEALTH_JSON, M5_CONTRACT_HEALTH_PATH, M5_CONTRACT_HEALTH_RECORD_KIND,
    M5_CONTRACT_HEALTH_REGISTER_ID, M5_CONTRACT_HEALTH_SCHEMA_VERSION,
};

pub use add_import_export_validators_and_cross_surface_conformance_runners_for_m5_interchange_families::{
    current_m5_interchange_conformance_register, Blockers as InterchangeBlockers,
    ConformanceClass, ConformanceRow, ConformanceState as InterchangeConformanceState,
    ConsumerAgreement, ConsumerSurface,
    DecisionState as InterchangeDecisionState, DegradedState, Dimension as InterchangeDimension,
    DimensionKind as InterchangeDimensionKind, DimensionOutcome as InterchangeDimensionOutcome,
    InterchangeDirection, LifecycleLabel as InterchangeLifecycleLabel,
    M5InterchangeConformanceExportProjection, M5InterchangeConformanceExportRow,
    M5InterchangeConformanceRegister, M5InterchangeConformanceSummary,
    M5InterchangeConformanceViolation, ReasonCode, Runner as InterchangeRunner,
    Validator as InterchangeValidator, M5_INTERCHANGE_CONFORMANCE_JSON,
    M5_INTERCHANGE_CONFORMANCE_PATH, M5_INTERCHANGE_CONFORMANCE_RECORD_KIND,
    M5_INTERCHANGE_CONFORMANCE_REGISTER_ID, M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION,
};

pub use add_about_help_service_health_provenance_cards_with_signature_attestation_sbom_symbol_rollback_state_and_exact_build_identity_convergence_for_m5_artifacts::{
    build_m5_provenance_cards, current_m5_provenance_cards, CardAction as ProvenanceCardAction,
    CardGapReason as ProvenanceCardGapReason, CardState as ProvenanceCardState,
    M5ProvenanceCardExportProjection, M5ProvenanceCardExportRow, M5ProvenanceCardRegister,
    M5ProvenanceCardSummary, M5ProvenanceCardViolation, ProvenanceBadge,
    ProvenanceBadgeKind, ProvenanceBadgeState, ProvenanceCard, ProvenanceCardBadgeExport,
    ProvenanceCardStopRule, ProvenanceCardSurfaceExport, ProvenanceSurfaceKind, SurfaceBinding,
    M5_PROVENANCE_CARDS_JSON, M5_PROVENANCE_CARDS_PATH, M5_PROVENANCE_CARDS_RECORD_KIND,
    M5_PROVENANCE_CARDS_SCHEMA_VERSION,
};

pub use implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance::{
    current_feature_train_compatibility_register, ChangeFreezeGuidance, CompatibilityCell,
    CompatibilityDimension, DimensionGrade as FeatureTrainDimensionGrade, FeatureTrainCompatibilityRegister,
    FeatureTrainCompatibilitySummary, FeatureTrainExportProjection, FeatureTrainExportRow,
    FeatureTrainLane, FeatureTrainRegisterViolation, FeatureTrainStopRule,
    FreezeState, FreezeTrigger, NarrowingReason as FeatureTrainNarrowingReason, ProviderSupportWindow,
    StopAction as FeatureTrainStopAction, TrainChannel, TrainState as FeatureTrainState,
    TrustTier as FeatureTrainTrustTier,
    FEATURE_TRAIN_COMPATIBILITY_JSON, FEATURE_TRAIN_COMPATIBILITY_PATH,
    FEATURE_TRAIN_COMPATIBILITY_RECORD_KIND, FEATURE_TRAIN_COMPATIBILITY_SCHEMA_VERSION,
};

pub use implement_fitness_dashboard_tiles_and_governance_report_rows_with_protected_metric_identity_threshold_state_provenance_evidence_freshness_owner_and_report_continuity::{
    current_stable_m5_fitness_governance_controls_export, resolve_fitness_tile,
    resolve_governance_report, resolve_provenance_disclosure,
    seeded_m5_fitness_governance_controls_assurance_dashboard_beta_narrowed,
    seeded_m5_fitness_governance_controls_packet,
    seeded_m5_fitness_governance_controls_shiproom_packet_preview_narrowed,
    M5EvidenceFreshness as M5FitnessGovernanceEvidenceFreshness, M5FitnessDeclaredState,
    M5FitnessDegradeReason, M5FitnessGovernanceAnatomyPart, M5FitnessGovernanceConsumerProjection,
    M5FitnessGovernanceConsumerSurface, M5FitnessGovernanceControlsArtifactError,
    M5FitnessGovernanceControlsPacket, M5FitnessGovernanceControlsPacketInput,
    M5FitnessGovernanceControlsViolation, M5FitnessGovernanceExportField,
    M5FitnessGovernanceProofFreshness, M5FitnessGovernanceReleasePosture, M5FitnessGovernanceReview,
    M5FitnessGovernanceRow, M5FitnessGovernanceVocabularySet, M5FitnessTileCase,
    M5FitnessTileResolutionError, M5FitnessTileResolutionInput, M5GovernanceNextAction,
    M5GovernanceReportCase, M5GovernanceReportResolutionError, M5GovernanceReportResolutionInput,
    M5GovernanceReportType, M5ProfileMatchState, M5ProvenanceDisclosure, M5ReportAction,
    M5ReportDegradeReason, M5ReportOutcome, M5ResolvedFitnessTile, M5ResolvedGovernanceReport,
    M5ThresholdState, M5_FITNESS_GOVERNANCE_CONTROLS_PACKET_ID,
    M5_FITNESS_GOVERNANCE_CONTROLS_RECORD_KIND, M5_FITNESS_GOVERNANCE_CONTROLS_SCHEMA_VERSION,
};

pub use implement_waiver_expiry_queue_items_release_gate_banners_and_mitigation_note_cards_with_owner_expiry_milestone_impact_blocked_waived_evidence_stale_vocabulary_and_user_facing_mitigation_truth::{
    current_stable_m5_waiver_gate_controls_export, resolve_mitigation_clarity, resolve_release_gate,
    resolve_waiver_expiry_item, seeded_m5_waiver_gate_controls_operator_board_preview_narrowed,
    seeded_m5_waiver_gate_controls_packet,
    seeded_m5_waiver_gate_controls_shiproom_packet_beta_narrowed, M5AffectedTargetKind,
    M5EvidenceFreshness as M5WaiverGateEvidenceFreshness, M5GateAction, M5GateDegradeReason,
    M5MitigationClarity, M5ReleaseGateCase, M5ReleaseGateResolutionError,
    M5ReleaseGateResolutionInput, M5ResolvedReleaseGate, M5ResolvedWaiverExpiryItem,
    M5WaiverDegradeReason, M5WaiverExpiryItemCase, M5WaiverExpiryItemResolutionError,
    M5WaiverExpiryItemResolutionInput, M5WaiverGateAnatomyPart, M5WaiverGateConsumerProjection,
    M5WaiverGateConsumerSurface, M5WaiverGateControlsArtifactError, M5WaiverGateControlsPacket,
    M5WaiverGateControlsPacketInput, M5WaiverGateControlsViolation, M5WaiverGateExportField,
    M5WaiverGateNextAction, M5WaiverGateProofFreshness, M5WaiverGateReleasePosture,
    M5WaiverGateReview, M5WaiverGateRow, M5WaiverGateVocabularySet, M5WaiverItemAction,
    M5_WAIVER_GATE_CONTROLS_PACKET_ID, M5_WAIVER_GATE_CONTROLS_RECORD_KIND,
    M5_WAIVER_GATE_CONTROLS_SCHEMA_VERSION,
};

pub use implement_service_ownership_cards_and_on_call_strips_with_role_based_owner_escalation_aliases_support_class_freshness_backup_coverage_and_export_safe_continuity::{
    current_stable_m5_service_ownership_on_call_controls_export, resolve_on_call_strip,
    resolve_service_ownership_card,
    seeded_m5_service_ownership_on_call_controls_operator_board_preview_narrowed,
    seeded_m5_service_ownership_on_call_controls_packet,
    seeded_m5_service_ownership_on_call_controls_service_health_beta_narrowed,
    M5OnCallAvailabilityState, M5OnCallDegradeReason, M5OnCallRoleTier, M5OnCallStripAction,
    M5OnCallStripCase, M5OnCallStripResolutionError, M5OnCallStripResolutionInput,
    M5OwnerFreshness, M5OwnerSource, M5OwnershipAnatomyPart, M5OwnershipCardAction,
    M5OwnershipConsumerProjection, M5OwnershipConsumerSurface, M5OwnershipDegradeReason,
    M5OwnershipExportField, M5OwnershipNextAction, M5OwnershipProofFreshness,
    M5OwnershipReleasePosture, M5OwnershipReview, M5OwnershipRow, M5OwnershipVocabularySet,
    M5ResolvedOnCallStrip, M5ResolvedServiceOwnershipCard,
    M5ServiceOwnershipCardCase, M5ServiceOwnershipOnCallControlsArtifactError,
    M5ServiceOwnershipOnCallControlsPacket, M5ServiceOwnershipOnCallControlsPacketInput,
    M5ServiceOwnershipOnCallControlsViolation, M5ServiceOwnershipResolutionError,
    M5ServiceOwnershipResolutionInput, M5ServiceSupportClass,
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_PACKET_ID,
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_RECORD_KIND,
    M5_SERVICE_OWNERSHIP_ON_CALL_CONTROLS_SCHEMA_VERSION,
};

pub use implement_decision_right_cards_and_milestone_dashboard_rows_with_required_forum_reason_satisfied_pending_state_blocker_and_waiver_counts_nearest_gate_and_next_review_continuity::{
    current_stable_m5_decision_right_milestone_controls_export, resolve_decision_right_card,
    resolve_milestone_dashboard_row,
    seeded_m5_decision_right_milestone_controls_operator_board_preview_narrowed,
    seeded_m5_decision_right_milestone_controls_packet,
    seeded_m5_decision_right_milestone_controls_shiproom_board_beta_narrowed,
    M5DecisionCardAction, M5DecisionMilestoneAnatomyPart, M5DecisionMilestoneConsumerProjection,
    M5DecisionMilestoneConsumerSurface, M5DecisionMilestoneExportField,
    M5DecisionMilestoneNextAction, M5DecisionMilestoneProofFreshness,
    M5DecisionMilestoneReleasePosture, M5DecisionMilestoneReview, M5DecisionMilestoneRow,
    M5DecisionMilestoneVocabularySet, M5DecisionRightCardCase, M5DecisionRightDegradeReason,
    M5DecisionRightMilestoneControlsArtifactError, M5DecisionRightMilestoneControlsPacket,
    M5DecisionRightMilestoneControlsPacketInput, M5DecisionRightMilestoneControlsViolation,
    M5DecisionRightResolutionError, M5DecisionRightResolutionInput, M5EvidenceFreshness,
    M5MilestoneDegradeReason, M5MilestoneRowAction, M5MilestoneRowCase,
    M5MilestoneRowResolutionError, M5MilestoneRowResolutionInput, M5ResolvedDecisionRightCard,
    M5ResolvedMilestoneRow, M5ReviewSatisfactionState,
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_PACKET_ID,
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_RECORD_KIND,
    M5_DECISION_RIGHT_MILESTONE_CONTROLS_SCHEMA_VERSION,
};

pub use add_community_locale_pack_lifecycle_translation_governance_and_parity_audits_for_new_m5_surfaces::{
    current_locale_pack_governance_register, AutomationState as LocalePackAutomationState,
    DimensionGrade as LocalePackDimensionGrade, DowngradeAutomation as LocalePackDowngradeAutomation,
    DowngradeTrigger as LocalePackDowngradeTrigger, GovernanceCell, GovernanceDimension,
    LocalePackExportProjection, LocalePackExportRow, LocalePackGovernanceRegister,
    LocalePackGovernanceSummary, LocalePackLane, LocalePackRegisterViolation, LocalePackStopRule,
    NarrowingReason as LocalePackNarrowingReason, PackChannel, PackState,
    StopAction as LocalePackStopAction, TranslationGovernance, TrustTier as LocalePackTrustTier,
    LOCALE_PACK_GOVERNANCE_JSON, LOCALE_PACK_GOVERNANCE_PATH, LOCALE_PACK_GOVERNANCE_RECORD_KIND,
    LOCALE_PACK_GOVERNANCE_SCHEMA_VERSION,
};

pub use add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes::{
    current_maintenance_truth_register, AutomationState as MaintenanceTruthAutomationState,
    AutomationTrigger as MaintenanceTruthAutomationTrigger,
    DimensionGrade as MaintenanceTruthDimensionGrade,
    DowngradeAutomation as MaintenanceTruthDowngradeAutomation, LaneKind as MaintenanceLaneKind,
    LaneState as MaintenanceLaneState, MaintenanceCell, MaintenanceDimension,
    MaintenanceTruthExportProjection, MaintenanceTruthExportRow, MaintenanceTruthLane,
    MaintenanceTruthRegister, MaintenanceTruthStopRule, MaintenanceTruthSummary,
    MaintenanceTruthViolation, NarrowingReason as MaintenanceTruthNarrowingReason,
    StopAction as MaintenanceTruthStopAction, SupportDisclosure,
    TrustTier as MaintenanceTruthTrustTier, MAINTENANCE_TRUTH_JSON, MAINTENANCE_TRUTH_PATH,
    MAINTENANCE_TRUTH_RECORD_KIND, MAINTENANCE_TRUTH_SCHEMA_VERSION,
};

pub use implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces::{
    current_field_readiness_register, AutomationState as FieldReadinessAutomationState,
    AutomationTrigger as FieldReadinessAutomationTrigger,
    DimensionGrade as FieldReadinessDimensionGrade,
    DowngradeAutomation as FieldReadinessDowngradeAutomation, FieldReadinessExportProjection,
    FieldReadinessExportRow, FieldReadinessRegister, FieldReadinessStopRule, FieldReadinessSummary,
    FieldReadinessSurface, FieldReadinessViolation, NarrowingReason as FieldReadinessNarrowingReason,
    ReadinessCell, ReadinessDimension, StopAction as FieldReadinessStopAction,
    SupportDisclosure as FieldReadinessSupportDisclosure, SurfaceKind as FieldReadinessSurfaceKind,
    SurfaceState as FieldReadinessSurfaceState, TrustTier as FieldReadinessTrustTier,
    FIELD_READINESS_JSON, FIELD_READINESS_PATH, FIELD_READINESS_RECORD_KIND,
    FIELD_READINESS_SCHEMA_VERSION,
};

pub use implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces::{
    current_stable_m5_badge_claim_primitive_export, resolve_badge_claim, M5BadgeClaimAnatomyPart,
    M5BadgeClaimConsumerProjection, M5BadgeClaimConsumerSurface, M5BadgeClaimError,
    M5BadgeClaimExportField, M5BadgeClaimGovernanceReview, M5BadgeClaimInput,
    M5BadgeClaimPrimitiveArtifactError, M5BadgeClaimPrimitivePacket,
    M5BadgeClaimPrimitivePacketInput, M5BadgeClaimPrimitiveViolation, M5BadgeClaimProofFreshness,
    M5BadgeClaimReleasePosture, M5BadgeClaimResolutionCase, M5BadgeClaimRow,
    M5BadgeClaimVocabularySet, M5BadgeNextAction, M5ClaimNarrowingNote, M5EffectiveClaimPosture,
    M5EvidenceFreshnessValue, M5FreshnessReducesClaimReason, M5ResolvedBadgeClaim,
    M5SupportClassBadgeValue, M5_BADGE_CLAIM_ARTIFACT_REF, M5_BADGE_CLAIM_CSV_REF,
    M5_BADGE_CLAIM_DOC_REF, M5_BADGE_CLAIM_FIXTURE_DIR, M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID,
    M5_BADGE_CLAIM_PRIMITIVE_RECORD_KIND, M5_BADGE_CLAIM_REPORT_REF, M5_BADGE_CLAIM_SCHEMA_REF,
};

pub use implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces::{
    current_stable_m5_maturity_badge_primitive_export, resolve_lifecycle_channel_badge,
    M5ChannelBadgeValue, M5EffectiveMaturityPosture, M5LifecycleBadgeValue,
    M5LifecycleChannelBadgeError, M5LifecycleChannelBadgeInput, M5LifecycleChannelResolutionCase,
    M5LifecycleSunsetReason, M5MaturityBadgeAnatomyPart, M5MaturityBadgeConsumerProjection,
    M5MaturityBadgeConsumerSurface, M5MaturityBadgeExportField, M5MaturityBadgeGovernanceReview,
    M5MaturityBadgeNextAction, M5MaturityBadgePrimitiveArtifactError, M5MaturityBadgePrimitivePacket,
    M5MaturityBadgePrimitivePacketInput, M5MaturityBadgePrimitiveViolation,
    M5MaturityBadgeProofFreshness, M5MaturityBadgeReleasePosture, M5MaturityBadgeRow,
    M5MaturityBadgeVocabularySet, M5MigrationNote, M5ResolvedLifecycleChannelBadge,
    M5_MATURITY_BADGE_ARTIFACT_REF, M5_MATURITY_BADGE_CSV_REF, M5_MATURITY_BADGE_DOC_REF,
    M5_MATURITY_BADGE_FIXTURE_DIR, M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID,
    M5_MATURITY_BADGE_PRIMITIVE_RECORD_KIND, M5_MATURITY_BADGE_REPORT_REF,
    M5_MATURITY_BADGE_SCHEMA_REF,
};

pub use ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs::{
    current_generated_artifact_lineage_register, AutomationState as LineageAutomationState,
    DimensionGrade, DowngradeAutomation as LineageDowngradeAutomation,
    DowngradeTrigger as LineageDowngradeTrigger, GeneratedArtifactLineageRegister,
    GeneratedArtifactLineageSummary, GeneratorKind, LineageCell, LineageDimension,
    LineageExportProjection, LineageExportRow, LineageProvenance, LineageRegisterViolation,
    LineageState, LineageStopRule, LineageSurface, NarrowingReason as LineageNarrowingReason,
    StopAction as LineageStopAction, TrustTier, GENERATED_ARTIFACT_LINEAGE_JSON,
    GENERATED_ARTIFACT_LINEAGE_PATH, GENERATED_ARTIFACT_LINEAGE_RECORD_KIND,
    GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
};

pub use ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries::{
    current_m5_boundary_skew_inspectors, ActionRisk, BoundaryInspector, BoundaryKind,
    BoundarySkewInspectorExportProjection, BoundarySkewInspectorExportRow,
    BoundarySkewInspectorRegister, BoundarySkewInspectorRegisterSummary,
    BoundarySkewInspectorViolation, DowngradeSubject, GatePosture, GatedAction, InspectorState,
    InspectorStopRule, InspectorVerdict, NarrowingReason as BoundarySkewNarrowingReason,
    SkewWindow as BoundarySkewWindow, SkewWindowClass as BoundarySkewWindowClass,
    StopAction as BoundarySkewStopAction, UpgradeLeadSide, UpgradeOrderGuide, UpgradeStep,
    SHIP_M5_BOUNDARY_SKEW_INSPECTORS_JSON, SHIP_M5_BOUNDARY_SKEW_INSPECTORS_PATH,
    SHIP_M5_BOUNDARY_SKEW_INSPECTORS_RECORD_KIND, SHIP_M5_BOUNDARY_SKEW_INSPECTORS_SCHEMA_VERSION,
};

pub use ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes::{
    current_m5_evaluation_pilot_packs, EvalPack, EvalPackDestination,
    EvalPackDestinationRendering, EvalPackExportProjection, EvalPackExportRow, EvalPackIssueSeverity,
    EvalPackKnownIssue, EvalPackLaneKind, EvalPackMirrorKind, EvalPackMirrorRef,
    EvalPackNarrowingReason, EvalPackRegister, EvalPackState, EvalPackStopAction, EvalPackStopRule,
    EvalPackSummary, EvalPackSupportContact, EvalPackValidityWindow, EvalPackViolation,
    M5_EVALUATION_PILOT_PACKS_JSON, M5_EVALUATION_PILOT_PACKS_PATH,
    M5_EVALUATION_PILOT_PACKS_RECORD_KIND, M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION,
};

pub use implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth::{
    current_m5_claim_scope_export_packets, ClaimScopeAudience, ClaimScopeAudienceRendering,
    ClaimScopeExportProjection, ClaimScopeExportRegister, ClaimScopeExportRow, ClaimScopeReason,
    ClaimScopeRow, ClaimScopeRowState, ClaimScopeStopAction, ClaimScopeStopRule, ClaimScopeSummary,
    ClaimScopeValidityWindow, ClaimScopeViolation, ScopeEvidenceKind, ScopeEvidenceRef,
    M5_CLAIM_SCOPE_EXPORT_PACKETS_JSON, M5_CLAIM_SCOPE_EXPORT_PACKETS_PATH,
    M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND, M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION,
};

pub use ship_benchmark_corpora_reference_workspace_expansions_and_m5_specific_protected_fitness_dashboards::{
    current_fitness_surface_register, AutomationState as FitnessAutomationState,
    AutomationTrigger as FitnessAutomationTrigger, CorpusProvenance,
    DimensionGrade as FitnessDimensionGrade, DowngradeAutomation as FitnessDowngradeAutomation,
    FitnessCell, FitnessDimension, FitnessSurfaceExportProjection, FitnessSurfaceExportRow,
    FitnessSurfaceLane, FitnessSurfaceRegister, FitnessSurfaceStopRule, FitnessSurfaceSummary,
    FitnessSurfaceViolation, NarrowingReason as FitnessSurfaceNarrowingReason,
    StopAction as FitnessSurfaceStopAction, SurfaceKind as FitnessSurfaceKind,
    SurfaceState as FitnessSurfaceState,
    TrustTier as FitnessSurfaceTrustTier, FITNESS_SURFACE_JSON, FITNESS_SURFACE_PATH,
    FITNESS_SURFACE_RECORD_KIND, FITNESS_SURFACE_SCHEMA_VERSION,
};

pub use m5_benchmark_help_migration_components::{
    current_about_service_health_card, current_benchmark_evidence_card,
    current_benchmark_evidence_cards, current_community_handoff_tile,
    current_community_handoff_tiles, current_importer_diff_row, current_importer_review_table,
    current_m5_benchmark_help_migration_component_certification, current_support_package_card,
    validate_benchmark_evidence_cards, validate_community_handoff_tiles, AboutDowngradeState,
    AboutServiceHealthCard, AboutServiceHealthCardFamily, AboutServiceHealthCardViolation,
    AboutSourceTrustClass, BenchmarkClaimScope, BenchmarkCompareMode, BenchmarkCompareView,
    BenchmarkComparisonBasis, BenchmarkCopyExport, BenchmarkDegradedState,
    BenchmarkDowngradeBanner, BenchmarkDowngradeBannerLabel, BenchmarkDowngradeState,
    BenchmarkEvidenceCard, BenchmarkEvidenceCardViolation, BenchmarkEvidenceSourceClass,
    BenchmarkFreshnessState, BenchmarkHelpMigrationComponentFamily, BenchmarkMetricRow,
    BenchmarkTraceReportExport, BuildProvenanceState, BuildSummary, ColdWarmState,
    CommunityHandoffRoute, CommunityHandoffTile, CommunityHandoffTileViolation,
    ComponentCertificationReason, ComponentCertificationRow, ComponentCertificationState,
    ComponentCertificationSummary, ComponentCertificationViolation, ComponentCopyExport,
    DataClassCounts, ExecutionScope, ExportState, HandoffAction, HandoffActionKind,
    HandoffAuthExpectation, HandoffCommitmentClass, HandoffDataExitBoundary,
    HandoffDestinationGroup, HandoffDestinationState, HandoffDestinationType,
    HandoffOwnershipClass, HandoffTrustClass, HandoffVisibilityBoundary, ImporterCheckpointContext,
    ImporterCompatibilityState, ImporterDegradedState, ImporterDiffRow, ImporterDiffRowViolation,
    ImporterExportSafeIdentifiers, ImporterMappingBasis, ImporterMigrationDomain,
    ImporterOutcomeGroup, ImporterOutcomeReasonClass, ImporterOutcomeState,
    ImporterPostApplySummary, ImporterReviewAction, ImporterReviewActionKind, ImporterReviewTable,
    InstallMode, LocalAction, LocalActionKind, LocalContinuityState, LocalSaveState,
    LocalSaveSummary, M5BenchmarkHelpMigrationComponentCertification, PackageContentKind,
    PowerMode, RedactionExportSummary, RedactionState, ReleaseChannel,
    ServiceContractState as AboutServiceContractState, ServiceFreshnessState, ServiceHealthSummary,
    SubmitLaterSummary, SubmitState, SupportDestinationClass, SupportPackageCard,
    SupportPackageCardViolation, SupportPackageState, SupportTrustClass, VersionAwarenessState,
    M5_ABOUT_SERVICE_HEALTH_CARD_FIXTURE_REF, M5_ABOUT_SERVICE_HEALTH_CARD_JSON,
    M5_ABOUT_SERVICE_HEALTH_CARD_RECORD_KIND, M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_REF,
    M5_ABOUT_SERVICE_HEALTH_CARD_SCHEMA_VERSION, M5_BENCHMARK_EVIDENCE_CARD_FIXTURE_REF,
    M5_BENCHMARK_EVIDENCE_CARD_JSON, M5_BENCHMARK_EVIDENCE_CARD_RECORD_KIND,
    M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_REF, M5_BENCHMARK_EVIDENCE_CARD_SCHEMA_VERSION,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_CERTIFICATION_RECORD_KIND,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_CERTIFICATION_SCHEMA_VERSION,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_MATRIX_REF,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_PROOF_PACKET_JSON,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_PROOF_PACKET_REF,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_PROOF_RECORD_KIND,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_SUPPORT_EXPORT_JSON,
    M5_BENCHMARK_HELP_MIGRATION_COMPONENT_SUPPORT_EXPORT_REF,
    M5_COMMUNITY_HANDOFF_TILE_FIXTURE_REF, M5_COMMUNITY_HANDOFF_TILE_JSON,
    M5_COMMUNITY_HANDOFF_TILE_RECORD_KIND, M5_COMMUNITY_HANDOFF_TILE_SCHEMA_REF,
    M5_COMMUNITY_HANDOFF_TILE_SCHEMA_VERSION, M5_IMPORTER_DIFF_ROW_FIXTURE_REF,
    M5_IMPORTER_DIFF_ROW_JSON, M5_IMPORTER_DIFF_ROW_RECORD_KIND, M5_IMPORTER_DIFF_ROW_SCHEMA_REF,
    M5_IMPORTER_DIFF_ROW_SCHEMA_VERSION, M5_IMPORTER_REVIEW_TABLE_FIXTURE_REF,
    M5_IMPORTER_REVIEW_TABLE_RECORD_KIND, M5_SUPPORT_PACKAGE_CARD_FIXTURE_REF,
    M5_SUPPORT_PACKAGE_CARD_JSON, M5_SUPPORT_PACKAGE_CARD_RECORD_KIND,
    M5_SUPPORT_PACKAGE_CARD_SCHEMA_REF, M5_SUPPORT_PACKAGE_CARD_SCHEMA_VERSION,
};

pub use ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions::{
    current_release_visibility_register, AutomationState as ReleaseVisibilityAutomationState,
    AutomationTrigger as ReleaseVisibilityAutomationTrigger,
    DimensionGrade as ReleaseVisibilityDimensionGrade,
    DowngradeAutomation as ReleaseVisibilityDowngradeAutomation,
    NarrowingReason as ReleaseVisibilityNarrowingReason, ReadinessCell as ReleaseVisibilityReadinessCell,
    ReadinessDimension as ReleaseVisibilityReadinessDimension, ReleaseVisibilityExportProjection,
    ReleaseVisibilityExportRow, ReleaseVisibilityRegister, ReleaseVisibilityStopRule,
    ReleaseVisibilitySummary, ReleaseVisibilitySurface, ReleaseVisibilityViolation,
    StopAction as ReleaseVisibilityStopAction,
    SupportDisclosure as ReleaseVisibilitySupportDisclosure,
    SurfaceKind as ReleaseVisibilitySurfaceKind, SurfaceState as ReleaseVisibilitySurfaceState,
    TrustTier as ReleaseVisibilityTrustTier, RELEASE_VISIBILITY_JSON, RELEASE_VISIBILITY_PATH,
    RELEASE_VISIBILITY_RECORD_KIND, RELEASE_VISIBILITY_SCHEMA_VERSION,
};

pub use ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes::{
    build_publication_review_register, current_publication_review_register, AuthDisclosure,
    AuthDisclosureState, MigrationFlag, NarrowingReason as PublicationReviewNarrowingReason,
    ParityState, PublicSurfaceImpact, PublicationReviewExportProjection,
    PublicationReviewExportRow, PublicationReviewRegister, PublicationReviewSheet,
    PublicationReviewStopRule, PublicationReviewSummary, PublicationReviewViolation,
    PublishTargetReview, ReviewParity, ReviewSheetState, StopAction as PublicationReviewStopAction,
    VersionBumpReview, PUBLICATION_REVIEW_JSON, PUBLICATION_REVIEW_PATH,
    PUBLICATION_REVIEW_RECORD_KIND, PUBLICATION_REVIEW_SCHEMA_VERSION,
};

pub use publish_docs_migration_and_known_limits_packs_for_m5_feature_families::{
    current_publication_pack_register, AutomationState as FamilyPackAutomationState,
    AutomationTrigger as FamilyPackAutomationTrigger, DimensionGrade as FamilyPackDimensionGrade,
    DowngradeAutomation as FamilyPackDowngradeAutomation,
    NarrowingReason as FamilyPackNarrowingReason,
    PublicationPackExportProjection as FamilyPackExportProjection,
    PublicationPackExportRow as FamilyPackExportRow, PublicationPackRegister as FamilyPackRegister,
    PublicationPackStopRule as FamilyPackStopRule, PublicationPackSummary as FamilyPackSummary,
    PublicationPackSurface as FamilyPackSurface, PublicationPackViolation as FamilyPackViolation,
    ReadinessCell as FamilyPackReadinessCell, ReadinessDimension as FamilyPackReadinessDimension,
    StopAction as FamilyPackStopAction, SupportDisclosure as FamilyPackSupportDisclosure,
    SurfaceKind as FamilyPackSurfaceKind, SurfaceState as FamilyPackSurfaceState,
    TrustTier as FamilyPackTrustTier, PUBLICATION_PACK_JSON, PUBLICATION_PACK_PATH,
    PUBLICATION_PACK_RECORD_KIND, PUBLICATION_PACK_SCHEMA_VERSION,
};

pub use certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows::{
    current_m5_publication_cert_register, AutomationState as PublicationCertAutomationState,
    AutomationTrigger as PublicationCertAutomationTrigger, CertState,
    DimensionCell as PublicationCertDimensionCell,
    DimensionGrade as PublicationCertDimensionGrade,
    DowngradeAutomation as PublicationCertDowngradeAutomation,
    M5PublicationCertExportProjection, M5PublicationCertExportRow, M5PublicationCertRegister,
    M5PublicationCertRow, M5PublicationCertStopRule, M5PublicationCertSummary,
    M5PublicationCertViolation, MirrorOfflineParity,
    NarrowingReason as PublicationCertNarrowingReason, PublicationDimension, PublishTargetPosture,
    StopAction as PublicationCertStopAction, SupportDisclosure as PublicationCertSupportDisclosure,
    TrustTier as PublicationCertTrustTier, M5_PUBLICATION_CERT_JSON, M5_PUBLICATION_CERT_PATH,
    M5_PUBLICATION_CERT_RECORD_KIND, M5_PUBLICATION_CERT_SCHEMA_VERSION,
};

pub use certify_qualification_matrix_truth_mixed_version_deprecation_governance_and_claim_publication_automation_on_every_claimed_m5_family::{
    current_m5_family_certification, CertificationPillar, CertificationPillarKind,
    CertificationReason, CertificationState, CertificationStopAction, CertificationStopRule,
    CertificationSummary, CertificationValidityWindow, CertificationViolation,
    FamilyCertificationExportProjection, FamilyCertificationExportRow, FamilyCertificationPacket,
    M5FamilyCertificationRegister, M5_FAMILY_CERTIFICATION_JSON, M5_FAMILY_CERTIFICATION_PATH,
    M5_FAMILY_CERTIFICATION_RECORD_KIND, M5_FAMILY_CERTIFICATION_SCHEMA_VERSION,
};

pub use certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index::{
    current_m5_cert_train_register, AutomationState as CertTrainAutomationState,
    AutomationTrigger as CertTrainAutomationTrigger, DimensionGrade as CertTrainDimensionGrade,
    DowngradeAutomation as CertTrainDowngradeAutomation,
    M5CertTrainExportProjection as CertTrainExportProjection,
    M5CertTrainExportRow as CertTrainExportRow, M5CertTrainRegister as CertTrainRegister,
    M5CertTrainStopRule as CertTrainStopRule, M5CertTrainSummary as CertTrainSummary,
    M5CertTrainSurface as CertTrainSurface, M5CertTrainViolation as CertTrainViolation,
    NarrowingReason as CertTrainNarrowingReason, ReadinessCell as CertTrainReadinessCell,
    ReadinessDimension as CertTrainReadinessDimension, StopAction as CertTrainStopAction,
    SupportDisclosure as CertTrainSupportDisclosure, SurfaceKind as CertTrainSurfaceKind,
    SurfaceState as CertTrainSurfaceState, TrustTier as CertTrainTrustTier, M5_CERT_TRAIN_JSON,
    M5_CERT_TRAIN_PATH, M5_CERT_TRAIN_RECORD_KIND, M5_CERT_TRAIN_SCHEMA_VERSION,
};

pub use claim_publication_manifest::{
    current_claim_publication_manifest, ClaimDowngradeRule, ClaimNarrowingReason,
    ClaimPublicationDecision, ClaimPublicationEntry, ClaimPublicationManifest,
    ClaimPublicationRecord, ClaimPublicationSummary, ClaimPublicationSurfaceEntry,
    ClaimPublicationSurfaceExport, ClaimPublicationViolation, ClaimReportRef, ClaimSurface,
    ClaimValidityWindow, EffectiveClaim, EvaluationFilter, EvidenceState,
    PublicationAction as ClaimPublicationAction, ReportFamily, SupportClass as ClaimSupportClass,
    SurfaceProjection, CLAIM_PUBLICATION_MANIFEST_JSON, CLAIM_PUBLICATION_MANIFEST_PATH,
    CLAIM_PUBLICATION_MANIFEST_RECORD_KIND, CLAIM_PUBLICATION_MANIFEST_SCHEMA_VERSION,
};

pub use add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::{
    current_m5_claim_publication_manifests, M5ClaimDestination, M5ClaimDestinationRendering,
    M5ClaimManifestState, M5ClaimNarrowingReason, M5ClaimPublication,
    M5ClaimPublicationExportProjection, M5ClaimPublicationExportRow, M5ClaimPublicationRegister,
    M5ClaimPublicationStopRule, M5ClaimPublicationSummary, M5ClaimPublicationViolation,
    M5ClaimReportKind, M5ClaimReportRef, M5ClaimReportState, M5ClaimStopAction,
    M5ClaimValidityWindow, M5PublishedClaim, M5_CLAIM_PUBLICATION_MANIFESTS_JSON,
    M5_CLAIM_PUBLICATION_MANIFESTS_PATH, M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND,
    M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION,
};
pub use correction_train::{
    BackportDecision, BackportMatrixRow, CorrectionEvidence, CorrectionItem, CorrectionRisk,
    CorrectionScope, CorrectionTrainPacket, CorrectionTrainViolation, CorrectionTriage,
    PacketTemplates, ReleaseNotesRefs, SupportProjection, TargetChannelUpdate, TriageLane,
    CORRECTION_TRAIN_PACKET_RECORD_KIND, CORRECTION_TRAIN_PACKET_SCHEMA_VERSION,
    SECURITY_OR_TRUST_ISSUE_CLASSES, SHARED_PACKET_FORM_TERMS, SUPPORTED_LINE_CLASSES,
};

pub use finalize_benchmark_lab_automation_corpus_governance_and_public_benchmark_publication_pack::{
    current_benchmark_lab_governance, AssetAction, AssetState, BenchmarkLabGovernance,
    BenchmarkLabGovernanceExportProjection, BenchmarkLabGovernanceExportRow,
    BenchmarkLabGovernanceSummary, BenchmarkLabGovernanceViolation, GovernanceAssetKind,
    GovernanceAssetRow, GovernanceRule, GapReason as BenchmarkLabGapReason, QualificationRecord,
    BENCHMARK_LAB_GOVERNANCE_JSON, BENCHMARK_LAB_GOVERNANCE_PATH,
    BENCHMARK_LAB_GOVERNANCE_RECORD_KIND, BENCHMARK_LAB_GOVERNANCE_SCHEMA_VERSION,
};

pub use finalize_design_partner_certified_archetype_and_stable_cohort::{
    current_cohort_scoreboards, CohortScoreboardRow, CohortScoreboards,
    CohortScoreboardsExportProjection, CohortScoreboardsExportRow, CohortScoreboardsSummary,
    CohortScoreboardsViolation, RequiredSignoff, ScoreboardAction, ScoreboardGapReason,
    ScoreboardLane, ScoreboardMetric, ScoreboardPublicationRecord, ScoreboardRule, ScoreboardState,
    SignoffLoop, COHORT_SCOREBOARDS_JSON, COHORT_SCOREBOARDS_PATH, COHORT_SCOREBOARDS_RECORD_KIND,
    COHORT_SCOREBOARDS_SCHEMA_VERSION,
};
pub use finalize_experiments_labs_inventory::{
    audit_finalize_experiments_labs_inventory_page, build_page_from_inventory,
    seeded_finalize_experiments_labs_inventory_page,
    validate_finalize_experiments_labs_inventory_page,
    FinalizeExperimentsLabsInventoryCliProjection, FinalizeExperimentsLabsInventoryCliRow,
    FinalizeExperimentsLabsInventoryDefect, FinalizeExperimentsLabsInventoryError,
    FinalizeExperimentsLabsInventoryPage, FinalizeExperimentsLabsInventoryRow,
    FinalizeExperimentsLabsInventorySummary, FinalizeExperimentsLabsInventorySupportExport,
    InventoryDependencyMarker, InventoryNarrowReasonClass, InventoryQualificationClass,
    InventorySurfaceClass, KillSwitchVisibilityRow,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_ARTIFACT_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_DEFECT_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_DOC_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_PAGE_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_ROW_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SCHEMA_VERSION,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SHARED_CONTRACT_REF,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUMMARY_RECORD_KIND,
    FINALIZE_EXPERIMENTS_LABS_INVENTORY_SUPPORT_EXPORT_RECORD_KIND,
};

pub use finalize_ime_grapheme_bidi_unicode_high_contrast_zoom_density_pseudoloc_rtl_locale_pack_and_desktop_platform_conformance::{
    current_desktop_platform_conformance, CheckKind, CheckState, ConformanceAction, ConformanceDomain,
    ConformanceState, DesktopPlatformConformance, DesktopPlatformConformanceRule,
    DesktopPlatformConformanceRow, DesktopPlatformConformanceSummary,
    DesktopPlatformConformanceViolation, GapReason as ConformanceGapReason,
    DESKTOP_PLATFORM_CONFORMANCE_JSON, DESKTOP_PLATFORM_CONFORMANCE_PATH,
    DESKTOP_PLATFORM_CONFORMANCE_RECORD_KIND, DESKTOP_PLATFORM_CONFORMANCE_SCHEMA_VERSION,
};

pub use finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills::{
    current_security_response_packet, EmergencyControl, GapReason as ResponseGapReason,
    MirrorDrillCheckpoint, ResponseAction, ResponseExportProjection, ResponseExportRow,
    ResponseKind, ResponsePublicationRecord, ResponseRule, ResponseRow, ResponseState,
    SecurityResponsePacket, SecurityResponsePacketSummary, SecurityResponsePacketViolation,
    SECURITY_RESPONSE_PACKET_JSON, SECURITY_RESPONSE_PACKET_PATH,
    SECURITY_RESPONSE_PACKET_RECORD_KIND, SECURITY_RESPONSE_PACKET_SCHEMA_VERSION,
};

pub use finalize_compatibility_reports_deprecation_packets_schema_version_windows::{
    current_finalize_compatibility_reports_deprecation_packets_schema_version_windows,
    CompatibilityOutcome, CompatibilityReportPacket, DeprecationDetail, FinalizeAction,
    FinalizeCompatibilityReportsDeprecationPacketsSchemaVersionWindows, FinalizeExportProjection,
    FinalizeExportRow, FinalizeKind, FinalizePublicationRecord, FinalizeRow, FinalizeRule,
    FinalizeState, FinalizeSummary, FinalizeViolation, GapReason as FinalizeGapReason,
    MigrationDetail, Scorecard, ValidityWindow as FinalizeValidityWindow,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_JSON,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_PATH,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_RECORD_KIND,
    FINALIZE_COMPATIBILITY_REPORTS_DEPRECATION_PACKETS_SCHEMA_VERSION_WINDOWS_SCHEMA_VERSION,
};

pub use finalize_qualification_packets_for_optional_surfaces_and_enforce::{
    current_finalize_qualification_packets_for_optional_surfaces_and_enforce, DeploymentAccessMode,
    DeploymentQualification, DeploymentTarget, FinalizeNarrowAction as OptionalSurfaceNarrowAction,
    FinalizeNarrowReason as OptionalSurfaceNarrowReason, FinalizeOptionalSurface,
    FinalizeOptionalSurfaceKind, FinalizeQualificationPacketsForOptionalSurfacesAndEnforce,
    FinalizeQualificationSummary, FinalizeQualificationViolation, FinalizeSurfaceExportProjection,
    FinalizeSurfaceExportRow, FinalizeSurfacePublicationRecord, FinalizeSurfaceState,
    FinalizeSurfaceStopRule, FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_JSON,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_PATH,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_RECORD_KIND,
    FINALIZE_QUALIFICATION_PACKETS_FOR_OPTIONAL_SURFACES_AND_ENFORCE_SCHEMA_VERSION,
};

pub use finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support::{
    current_finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support,
    ConsumingSurface, DowngradePropagationStatus, FreshnessObjectAction, FreshnessObjectExportProjection,
    FreshnessObjectExportRow, FreshnessObjectGapReason, FreshnessObjectKind, FreshnessObjectPublicationRecord,
    FreshnessObjectRule, FreshnessObjectRow, FreshnessObjectState, FreshnessObjectSummary,
    FreshnessObjectViolation,
    FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport,
    ValidityWindow as FreshnessValidityWindow,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_JSON,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_PATH,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND,
    FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION,
};

pub use go_no_go_rehearsal::{
    current_go_no_go_rehearsal, GoNoGoRehearsal, GoNoGoRehearsalSummary, GoNoGoRehearsalViolation,
    RehearsalAction, RehearsalExportProjection, RehearsalExportRow, RehearsalGapReason,
    RehearsalPublicationRecord, RehearsalRule, RehearsalStageRow, RehearsalState,
    RollbackCheckpoint, StageKind, GO_NO_GO_REHEARSAL_JSON, GO_NO_GO_REHEARSAL_PATH,
    GO_NO_GO_REHEARSAL_RECORD_KIND, GO_NO_GO_REHEARSAL_SCHEMA_VERSION,
};

pub use harden_docs_help_about_and_service_health_truth::{
    current_docs_help_about_service_health_truth, AboutProvenanceCard, DestinationTrustClass,
    DocsHelpAboutServiceHealthTruth, DocsHelpAboutServiceHealthTruthViolation, HelpDestination,
    PackageSafetyDisclosure, ServiceContractState, TruthAction, TruthExportProjection,
    TruthExportRow, TruthPublicationRecord, TruthRow, TruthRule, TruthState, TruthSummary,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_JSON, DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_PATH,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_RECORD_KIND,
    DOCS_HELP_ABOUT_SERVICE_HEALTH_TRUTH_SCHEMA_VERSION,
};

pub use stabilize_embedded_surface_boundary_truth::{
    current_embedded_surface_boundary_truth, AuthHandoffSnapshot,
    BoundaryState as EmbeddedBoundaryState, BrowserFallbackSnapshot, EmbeddedSurfaceBoundaryTruth,
    EmbeddedSurfaceBoundaryTruthViolation, GapReason as EmbeddedSurfaceGapReason,
    NativeApprovalSnapshot, SourceTruthSnapshot, SurfaceKind as EmbeddedSurfaceKind,
    TruthAction as EmbeddedSurfaceTruthAction,
    TruthExportProjection as EmbeddedSurfaceTruthExportProjection,
    TruthExportRow as EmbeddedSurfaceTruthExportRow,
    TruthPublicationRecord as EmbeddedSurfaceTruthPublicationRecord,
    TruthRow as EmbeddedSurfaceTruthRow, TruthRule as EmbeddedSurfaceTruthRule,
    TruthState as EmbeddedSurfaceTruthState, TruthSummary as EmbeddedSurfaceTruthSummary,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_JSON, EMBEDDED_SURFACE_BOUNDARY_TRUTH_PATH,
    EMBEDDED_SURFACE_BOUNDARY_TRUTH_RECORD_KIND, EMBEDDED_SURFACE_BOUNDARY_TRUTH_SCHEMA_VERSION,
};

pub use harden_certified_reference_workspaces_archetype_pass_matrices_and_downgrade_automation::{
    current_certified_reference_workspaces, ArchetypePassMatrixExportRow, ArchetypePassMatrixRow,
    CertifiedReferenceWorkspaces, CertifiedReferenceWorkspacesExportProjection,
    CertifiedReferenceWorkspacesSummary, CertifiedReferenceWorkspacesViolation,
    DowngradeReason as ReferenceWorkspaceDowngradeReason,
    DowngradeRule as ReferenceWorkspaceDowngradeRule, MatrixAction, MatrixRowState,
    PublicationDecision as ReferenceWorkspacePublicationDecision,
    PublicationDecisionRecord as ReferenceWorkspacePublicationDecisionRecord,
    ReferenceWorkspaceExportRow, ReferenceWorkspaceReport, ReportState, ValidityWindow,
    CERTIFIED_REFERENCE_WORKSPACES_JSON, CERTIFIED_REFERENCE_WORKSPACES_PATH,
    CERTIFIED_REFERENCE_WORKSPACES_RECORD_KIND, CERTIFIED_REFERENCE_WORKSPACES_SCHEMA_VERSION,
};

pub use harden_the_release_artifact_graph_with_one_build_identity_provenance_sbom_notices_attestation_and_mirror_parity::{
    current_harden_release_artifact_graph, ArtifactFamilyAction, ArtifactFamilyExportRow,
    ArtifactFamilyGapReason, ArtifactFamilyKind, ArtifactFamilyRow, ArtifactFamilyRule,
    ArtifactFamilyState, HardenReleaseArtifactGraph, HardenReleaseArtifactGraphExportProjection,
    HardenReleaseArtifactGraphSummary, HardenReleaseArtifactGraphViolation,
    PublicationDecision as ArtifactGraphPublicationDecision,
    PublicationDecisionRecord as ArtifactGraphPublicationDecisionRecord,
    HARDEN_RELEASE_ARTIFACT_GRAPH_JSON, HARDEN_RELEASE_ARTIFACT_GRAPH_PATH,
    HARDEN_RELEASE_ARTIFACT_GRAPH_RECORD_KIND, HARDEN_RELEASE_ARTIFACT_GRAPH_SCHEMA_VERSION,
};

pub use harden_the_critical_dependency_register_fork_replace_log_third_party_import_manifest_and_reuse_spdx_notice_coverage::{
    current_harden_critical_dependency_register, HardenCriticalDependencyRegister,
    HardenCriticalDependencyRegisterExportProjection, HardenCriticalDependencyRegisterSummary,
    HardenCriticalDependencyRegisterViolation, LaneAction, LaneExportRow, LaneGapReason,
    LaneKind as DependencyLaneKind, LaneRow, LaneRule, LaneState, PublicationDecision as DependencyRegisterPublicationDecision,
    PublicationDecisionRecord as DependencyRegisterPublicationDecisionRecord,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_JSON, HARDEN_CRITICAL_DEPENDENCY_REGISTER_PATH,
    HARDEN_CRITICAL_DEPENDENCY_REGISTER_RECORD_KIND, HARDEN_CRITICAL_DEPENDENCY_REGISTER_SCHEMA_VERSION,
};

pub use m5_truth_surface_evidence_ingestion::{
    current_m5_truth_surface_ingestion, ContradictionRule as TruthIngestContradictionRule,
    FamilyKind as TruthIngestFamilyKind, IngestPublicationRecord, IngestReason, IngestState,
    IngestSummary, IngestionRow, IngestionViolation, M5TruthSurfaceIngestion,
    PostureClass as TruthIngestPostureClass, SourceRefs as TruthIngestSourceRefs, TruthSurface,
    M5_TRUTH_SURFACE_INGESTION_JSON, M5_TRUTH_SURFACE_INGESTION_PATH,
    M5_TRUTH_SURFACE_INGESTION_RECORD_KIND, M5_TRUTH_SURFACE_INGESTION_SCHEMA_VERSION,
};

pub use maintenance_control_packet::{
    current_maintenance_control_packet, ControlAction, ControlPublicationRecord, ControlRule,
    ControlState, GapReason as MaintenanceGapReason, LaneKind, MaintenanceControlPacket,
    MaintenanceControlPacketSummary, MaintenanceControlPacketViolation,
    MaintenanceExportProjection, MaintenanceExportRow, MaintenanceRow, SupportPosture,
    SupportWindow, MAINTENANCE_CONTROL_PACKET_JSON, MAINTENANCE_CONTROL_PACKET_PATH,
    MAINTENANCE_CONTROL_PACKET_RECORD_KIND, MAINTENANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use open_paid_boundary_audit::{
    current_open_paid_boundary_audit, AuditAction, AuditControl, AuditDomain,
    AuditExportProjection, AuditExportRow, AuditGapReason, AuditPublicationRecord, AuditRow,
    AuditRule, AuditState, OpenPaidBoundaryAudit, OpenPaidBoundaryAuditSummary,
    OpenPaidBoundaryAuditViolation, OPEN_PAID_BOUNDARY_AUDIT_JSON, OPEN_PAID_BOUNDARY_AUDIT_PATH,
    OPEN_PAID_BOUNDARY_AUDIT_RECORD_KIND, OPEN_PAID_BOUNDARY_AUDIT_SCHEMA_VERSION,
};

pub use publish_the_signed_m4_stable_evidence_pack_plus::{
    current_signed_m4_stable_evidence_pack, BundleAction, BundleExportProjection, BundleExportRow,
    BundleGapReason, BundleRule, BundleState, EvidenceBundleKind, EvidenceBundleRow,
    SignedM4StableEvidencePack, SignedM4StableEvidencePackViolation,
    SIGNED_M4_STABLE_EVIDENCE_PACK_JSON, SIGNED_M4_STABLE_EVIDENCE_PACK_PATH,
    SIGNED_M4_STABLE_EVIDENCE_PACK_RECORD_KIND, SIGNED_M4_STABLE_EVIDENCE_PACK_SCHEMA_VERSION,
};

pub use prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence::{
    current_clean_room_rebuild_proof, ChannelFamilyAction, ChannelFamilyCategory, ChannelFamilyExportRow,
    ChannelFamilyGapReason, ChannelFamilyKind, ChannelFamilyRow, ChannelFamilyRule, ChannelFamilyState,
    CleanRoomRebuildProof, CleanRoomRebuildProofExportProjection, CleanRoomRebuildProofSummary,
    CleanRoomRebuildProofViolation, PublicationDecision as CleanRoomRebuildPublicationDecision,
    PublicationDecisionRecord as CleanRoomRebuildPublicationDecisionRecord, RebuildState,
    SymbolicationState, CLEAN_ROOM_REBUILD_PROOF_JSON, CLEAN_ROOM_REBUILD_PROOF_PATH,
    CLEAN_ROOM_REBUILD_PROOF_RECORD_KIND, CLEAN_ROOM_REBUILD_PROOF_SCHEMA_VERSION,
};

pub use ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains::{
    current_m5_rehearsal_automation_register, M5RehearsalAutomationExportProjection,
    M5RehearsalAutomationExportRow, M5RehearsalAutomationRegister, M5RehearsalAutomationSummary,
    M5RehearsalAutomationViolation, M5RehearsalRow, RebuildProvenance,
    RehearsalAction as M5RehearsalAction, RehearsalAutomationState, RehearsalExpiryEntry,
    RehearsalExpiryFeed, RehearsalGapReason as M5RehearsalGapReason, RehearsalKind,
    RehearsalRecord, RehearsalResult, RehearsalStopRule, SHIP_M5_REHEARSAL_AUTOMATION_JSON,
    SHIP_M5_REHEARSAL_AUTOMATION_PATH, SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND,
    SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION,
};

pub use optional_surface_qualification::{
    current_optional_surface_qualification, NarrowAction, NarrowReason, OptionalSurface,
    OptionalSurfaceKind, OptionalSurfaceQualification, OptionalSurfaceQualificationSummary,
    OptionalSurfaceQualificationViolation, SurfaceExportProjection, SurfaceExportRow,
    SurfacePublicationRecord, SurfaceState, SurfaceStopRule, OPTIONAL_SURFACE_QUALIFICATION_JSON,
    OPTIONAL_SURFACE_QUALIFICATION_PATH, OPTIONAL_SURFACE_QUALIFICATION_RECORD_KIND,
    OPTIONAL_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use voice_and_dictation_surface_qualification::{
    current_voice_and_dictation_surface_qualification, ActivationDefault, CommandParityContract,
    ProcessingClass, TranscriptPrivacyControls, TranscriptRetention,
    VoiceAndDictationSurfaceQualification, VoiceFallbackState, VoiceMode, VoiceProjection,
    VoiceQualificationSummary, VoiceQualificationViolation, VoiceSurfaceKind, VoiceSurfaceRow,
    VoiceUiPrimitives, VOICE_DICTATION_SURFACE_QUALIFICATION_JSON,
    VOICE_DICTATION_SURFACE_QUALIFICATION_PATH, VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND,
    VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use preview_designer_publish_surface_qualification::{
    current_preview_designer_publish_surface_qualification, ActionSafetyLineage,
    BrowserInspectionBoundary, ExportedArtifactTruth, FallbackPaths, GeneratedSourceTruth,
    PreviewDesignerPublishExportProjection, PreviewDesignerPublishExportRow,
    PreviewDesignerPublishQualificationSummary, PreviewDesignerPublishQualificationViolation,
    PreviewDesignerPublishSurfaceKind, PreviewDesignerPublishSurfaceQualification,
    PreviewDesignerPublishSurfaceRow, QualificationProjection as PreviewDesignerPublishProjection,
    SafePreviewPosture, SourceMappingQuality, SourceSyncState,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_JSON,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_PATH,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_RECORD_KIND,
    PREVIEW_DESIGNER_PUBLISH_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use release_center_model::{
    ArtifactBundleCard, ArtifactFamilyClass, ArtifactGraphConsistency, ArtifactPayloadRefs,
    AuthSourceClass, BlastRadiusClass, BreakGlassDisclosure, BreakGlassStateClass,
    CompatibilityImpactClass, CompatibilityNote, ContinuityClass, ContinuityNote,
    DryRunAvailabilityClass, DryRunDisclosure, EvidenceFreshnessClass, EvidenceRef,
    ImmutableDigest, PromotionEventClass, PromotionReadiness, PromotionStage,
    PromotionTimelineStep, PublishTargetClass, PublishTargetDescriptor, ReleaseCandidate,
    ReleaseCenterHeadlessPlan, ReleaseCenterModelValidationReport, ReleaseCenterModelViolation,
    ReleaseCenterObjectIdentityIndex, ReleaseCenterObjectModel, ReleaseCenterSupportAuditExport,
    ReleaseCenterUiState, RollbackOrRevocationKind, RollbackOrRevocationRecord, RolloutRing,
    SemanticChangeClass, SignatureStateClass, TargetMutabilityClass, TargetVisibilityClass,
    VersionBumpProposal, RELEASE_CENTER_OBJECT_MODEL_RECORD_KIND,
    RELEASE_CENTER_OBJECT_MODEL_SCHEMA_VERSION,
};

pub use shiproom_dashboard::{
    current_shiproom_dashboard, Comparator, DashboardExportProjection, DashboardExportRow,
    DashboardPanel, DashboardPublicationRecord, FitnessFunction, FitnessStatus, PanelKind,
    PanelState, QualificationStopRule, ShiproomDashboard, ShiproomDashboardSummary,
    ShiproomDashboardViolation, StopAction as DashboardStopAction, StopReason,
    SHIPROOM_DASHBOARD_JSON, SHIPROOM_DASHBOARD_PATH, SHIPROOM_DASHBOARD_RECORD_KIND,
    SHIPROOM_DASHBOARD_SCHEMA_VERSION,
};

pub use stable_boundary_manifest::{
    current_stable_boundary_manifest, BoundaryAction, BoundaryExportProjection, BoundaryExportRow,
    BoundaryPublicationRecord, BoundaryRow, BoundaryRule, BoundaryState,
    NarrowingReason as BoundaryNarrowingReason, StableBoundaryManifest,
    StableBoundaryManifestSummary, StableBoundaryManifestViolation, ValueLine, ValueLineProfile,
    ValueLineRollup, STABLE_BOUNDARY_MANIFEST_JSON, STABLE_BOUNDARY_MANIFEST_PATH,
    STABLE_BOUNDARY_MANIFEST_RECORD_KIND, STABLE_BOUNDARY_MANIFEST_SCHEMA_VERSION,
};

pub use stable_claim_manifest::{
    current_stable_claim_manifest, FreshnessSlo, FreshnessSloState, ManifestEntry,
    ManifestExportProjection, ManifestExportRow, ManifestPublicationRecord, ManifestState,
    NarrowingReason, ProofPacket, PublicationAction, PublicationRule, StableClaimManifest,
    StableClaimManifestSummary, StableClaimManifestViolation, STABLE_CLAIM_MANIFEST_JSON,
    STABLE_CLAIM_MANIFEST_PATH, STABLE_CLAIM_MANIFEST_RECORD_KIND,
    STABLE_CLAIM_MANIFEST_SCHEMA_VERSION,
};

pub use stable_proof_index::{
    current_stable_proof_index, GapReason, IndexAction, ProofIndexExportProjection,
    ProofIndexExportRow, ProofPublicationRecord, ProofRow, ProofRule, ProofState, StableProofIndex,
    StableProofIndexSummary, StableProofIndexViolation, STABLE_PROOF_INDEX_JSON,
    STABLE_PROOF_INDEX_PATH, STABLE_PROOF_INDEX_RECORD_KIND, STABLE_PROOF_INDEX_SCHEMA_VERSION,
};

pub use stable_publication_pack::{
    current_stable_publication_pack, BenchmarkBudget, GapReason as PublicationGapReason,
    PackPublicationRecord, PublicationAction as PackPublicationAction, PublicationKind,
    PublicationPackExportProjection, PublicationPackExportRow, PublicationRow,
    PublicationRule as PackPublicationRule, PublicationState, StablePublicationPack,
    StablePublicationPackSummary, StablePublicationPackViolation, STABLE_PUBLICATION_PACK_JSON,
    STABLE_PUBLICATION_PACK_PATH, STABLE_PUBLICATION_PACK_RECORD_KIND,
    STABLE_PUBLICATION_PACK_SCHEMA_VERSION,
};

pub use stable_claim_matrix::{
    current_stable_claim_matrix, DowngradeReason, LaunchCutline, OwnerSignoff, PromotionDecision,
    PromotionDecisionRecord, QualificationEvidence, QualificationState, QualificationWaiver,
    ShiproomStopRule, StableClaimExportProjection, StableClaimExportRow, StableClaimLevel,
    StableClaimMatrix, StableClaimMatrixSummary, StableClaimMatrixViolation, StableClaimRow,
    StopAction, STABLE_CLAIM_MATRIX_JSON, STABLE_CLAIM_MATRIX_PATH,
    STABLE_CLAIM_MATRIX_RECORD_KIND, STABLE_CLAIM_MATRIX_SCHEMA_VERSION,
};

pub use stable_qualification_matrix::{
    current_stable_qualification_matrix, BoundaryFamily,
    DowngradeReason as QualificationDowngradeReason, DowngradeRule as QualificationDowngradeRule,
    MixedVersionPosture, MixedVersionSection, OrderRecord, OutOfWindowPosture,
    PromotionDecisionRecord as QualificationPromotionDecisionRecord, QualificationAction,
    QualificationExportProjection, QualificationExportRow, QualificationRow, QualificationRowScope,
    SkewWindow, StableQualificationMatrix, StableQualificationMatrixSummary,
    StableQualificationMatrixViolation, UnsupportedStateBehavior, STABLE_QUALIFICATION_MATRIX_JSON,
    STABLE_QUALIFICATION_MATRIX_PATH, STABLE_QUALIFICATION_MATRIX_RECORD_KIND,
    STABLE_QUALIFICATION_MATRIX_SCHEMA_VERSION,
};

pub use stable_version_windows::{
    current_stable_version_windows, CompatibilityPosture, DeprecationNotice, DeprecationPacket,
    DeprecationStatus, FreezePublicationRecord, FreezeRule, GapReason as VersionWindowGapReason,
    StableVersionWindows, StableVersionWindowsSummary, StableVersionWindowsViolation, SurfaceKind,
    VersionWindow, VersionWindowExportProjection, VersionWindowExportRow, WindowAction, WindowRow,
    WindowState, STABLE_VERSION_WINDOWS_JSON, STABLE_VERSION_WINDOWS_PATH,
    STABLE_VERSION_WINDOWS_RECORD_KIND, STABLE_VERSION_WINDOWS_SCHEMA_VERSION,
};

pub use support_class_ledger::{
    current_support_class_ledger, ArchetypeCertification, CertificationStatus, CertifiedArchetype,
    CertifiedCutline, DowngradeAction, DowngradeReason as LedgerDowngradeReason, DowngradeRule,
    EvidencePathClass, LedgerOwnerSignoff, LedgerState, LedgerWaiver, PublicationDecision,
    PublicationDecisionRecord as SupportPublicationDecisionRecord, SupportClass, SupportClassEntry,
    SupportClassExportProjection, SupportClassExportRow, SupportClassLedger,
    SupportClassLedgerSummary, SupportClassLedgerViolation, SupportEvidence,
    SUPPORT_CLASS_LEDGER_JSON, SUPPORT_CLASS_LEDGER_PATH, SUPPORT_CLASS_LEDGER_RECORD_KIND,
    SUPPORT_CLASS_LEDGER_SCHEMA_VERSION,
};

pub use browser_mobile_companion_surface_qualification::{
    current_browser_mobile_companion_surface_qualification,
    BrowserMobileCompanionSurfaceQualification, CompanionAuthority, CompanionClientKind,
    CompanionFreshness, CompanionProjection, CompanionQualificationSummary,
    CompanionQualificationViolation, CompanionScope, CompanionSurfaceRow, CompanionVisibleLabel,
    DesktopHandoffTruth, BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_JSON,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_PATH,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_RECORD_KIND,
    BROWSER_MOBILE_COMPANION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};

pub use stabilize_accessibility_signoff_across_shell_tree_palette_diff_terminal_debugger_settings_auth_and_recovery::{
    current_accessibility_surface_signoffs, AccessibilitySurfaceSignoffExportProjection,
    AccessibilitySurfaceSignoffExportRow, AccessibilitySurfaceSignoffRule,
    AccessibilitySurfaceSignoffRow, AccessibilitySurfaceSignoffs,
    AccessibilitySurfaceSignoffsSummary, AccessibilitySurfaceSignoffsViolation,
    DimensionCheck, DimensionKind, DimensionState, GapReason as AccessibilityGapReason,
    SignoffAction, SignoffState, SurfaceKind as AccessibilitySurfaceKind,
    ACCESSIBILITY_SURFACE_SIGNOFFS_JSON, ACCESSIBILITY_SURFACE_SIGNOFFS_PATH,
    ACCESSIBILITY_SURFACE_SIGNOFFS_RECORD_KIND, ACCESSIBILITY_SURFACE_SIGNOFFS_SCHEMA_VERSION,
};

pub use stabilize_hot_path_performance_against_published_budgets_for::{
    current_hot_path_performance_budgets, BudgetAction, BudgetState, GapReason as HotPathGapReason,
    HotPathBudget, HotPathBudgetRow, HotPathBudgetRule, HotPathExportProjection, HotPathExportRow,
    HotPathKind, HotPathPerformanceBudgets, HotPathPerformanceBudgetsSummary,
    HotPathPerformanceBudgetsViolation, PromotionRecord, HOT_PATH_PERFORMANCE_BUDGETS_JSON,
    HOT_PATH_PERFORMANCE_BUDGETS_PATH, HOT_PATH_PERFORMANCE_BUDGETS_RECORD_KIND,
    HOT_PATH_PERFORMANCE_BUDGETS_SCHEMA_VERSION,
};

pub use stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication::{
    current_stabilize_the_known_limits_matrix_public_support_windows_and_stable_line_ownership_publication,
    StabilizeAction, StabilizeExportProjection, StabilizeExportRow, StabilizeGapReason,
    StabilizeKind, StabilizePublicationRecord, StabilizeRow, StabilizeRule, StabilizeState,
    StabilizeSummary, StabilizeTheKnownLimitsMatrixPublicSupportWindowsAndStableLineOwnershipPublication,
    StabilizeViolation, STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_JSON,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_PATH,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_RECORD_KIND,
    STABILIZE_THE_KNOWN_LIMITS_MATRIX_PUBLIC_SUPPORT_WINDOWS_AND_STABLE_LINE_OWNERSHIP_PUBLICATION_SCHEMA_VERSION,
};

pub use freeze_the_m5_feature_train_matrix_scorecards_and_dependency_graph::{
    current_m5_feature_train_matrix, M5Action, M5DependencyEdge, M5DependencyKind,
    M5FeatureTrainExportProjection, M5FeatureTrainExportRow, M5FeatureTrainMatrix,
    M5FeatureTrainMatrixSummary, M5FeatureTrainMatrixViolation, M5GapReason, M5LaneKind, M5LaneRow,
    M5Scorecard, M5ScorecardState, M5StopRule,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_JSON,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_PATH,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_RECORD_KIND,
    FREEZE_M5_FEATURE_TRAIN_MATRIX_SCORECARDS_AND_DEPENDENCY_GRAPH_SCHEMA_VERSION,
};
pub use freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::{
    current_m5_publication_matrix, AttestationAvailability, EvidenceCompleteness,
    ExactBuildIdentity, M5ArtifactFamilyKind, M5PublicationAction, M5PublicationExportProjection,
    M5PublicationExportRow, M5PublicationGapReason, M5PublicationMatrix, M5PublicationMatrixRow,
    M5PublicationMatrixSummary, M5PublicationMatrixViolation, M5PublicationStopRule, MirrorFreshness,
    MirrorOfflineExpectation, RollbackRevocationPosture, SbomScope, SymbolSourceMapAvailability,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_JSON,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_PATH,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_RECORD_KIND,
    FREEZE_THE_M5_RELEASE_CANDIDATE_PUBLISH_TARGET_ARTIFACT_BUNDLE_AND_EXACT_BUILD_PUBLICATION_MATRIX_SCHEMA_VERSION,
};
pub use freeze_the_m5_rollback_downgrade_claim_narrowing_and_staged_promotion_rules::{
    current_m5_rollback_downgrade_register, DowngradeKind, M5ClaimNarrowingRule, M5DowngradeRule,
    M5PromotionStage, M5RollbackAction, M5RollbackDowngradeExportProjection,
    M5RollbackDowngradeExportRow, M5RollbackDowngradeRegister, M5RollbackDowngradeRow,
    M5RollbackDowngradeState, M5RollbackDowngradeSummary, M5RollbackDowngradeViolation,
    M5RollbackGapReason, M5RollbackStopRule, PromotionStageKind, RollbackPathState, StageState,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_JSON,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_PATH,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_RECORD_KIND,
    FREEZE_THE_M5_ROLLBACK_DOWNGRADE_CLAIM_NARROWING_AND_STAGED_PROMOTION_RULES_SCHEMA_VERSION,
};
pub use generate_m5_proof_freshness_backport_and_evidence_expiry_automation_for_depth_trains::{
    current_m5_depth_train_automation_register, AutomationAction, AutomationGapReason,
    AutomationState, AutomationStopRule, BackportEligibility, BackportKind, EvidenceExpiryRecord,
    EvidenceKind, M5DepthTrainAutomationExportProjection, M5DepthTrainAutomationExportRow,
    M5DepthTrainAutomationRegister, M5DepthTrainAutomationSummary, M5DepthTrainAutomationViolation,
    M5DepthTrainRow,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_JSON,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_PATH,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_RECORD_KIND,
    GENERATE_M5_PROOF_FRESHNESS_BACKPORT_AND_EVIDENCE_EXPIRY_AUTOMATION_FOR_DEPTH_TRAINS_SCHEMA_VERSION,
};

pub use publish_feature_scorecard_and_compatibility_packet_templates_for_every_m5_family::{
    current_m5_template_register, CompatibilityPacketSectionKind, CompatibilityPacketTemplate,
    CompatibilityPacketTemplateSection, M5FamilyKind, M5FamilyTemplateRow, M5TemplateRegister,
    M5TemplateRegisterExportProjection, M5TemplateRegisterExportRow, M5TemplateRegisterSummary,
    M5TemplateRegisterViolation, ScorecardSectionKind, ScorecardTemplate, ScorecardTemplateSection,
    TemplateAction, TemplateGapReason, TemplateRegisterState, TemplateSectionState,
    TemplateStopRule,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_JSON,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_PATH,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_RECORD_KIND,
    PUBLISH_FEATURE_SCORECARD_AND_COMPATIBILITY_PACKET_TEMPLATES_FOR_EVERY_M5_FAMILY_SCHEMA_VERSION,
};

pub use publish_the_m5_admin_policy_story_register_for_notebook_ai_data_companion_and_sync_lanes::{
    current_m5_admin_policy_story_register, AdminPolicyAction, AdminPolicyGapReason,
    AdminPolicyLaneState, AdminPolicyStory, AdminPolicyStoryItem, AdminPolicyStoryItemKind,
    AdminPolicyStoryItemState, AdminPolicyStopRule, M5AdminPolicyLaneKind, M5AdminPolicyLaneRow,
    M5AdminPolicyRegisterExportProjection, M5AdminPolicyRegisterExportRow,
    M5AdminPolicyRegisterSummary, M5AdminPolicyRegisterViolation, M5AdminPolicyStoryRegister,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_JSON,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_PATH,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_RECORD_KIND,
    PUBLISH_THE_M5_ADMIN_POLICY_STORY_REGISTER_FOR_NOTEBOOK_AI_DATA_COMPANION_AND_SYNC_LANES_SCHEMA_VERSION,
};

pub use publish_the_m5_storage_retention_export_and_offboarding_matrix_for_new_durable_artifacts::{
    current_m5_storage_retention_matrix, ArtifactRetentionAction, ArtifactRetentionGapReason,
    ArtifactRetentionPosture, ArtifactRetentionState, ArtifactRetentionStopRule,
    M5ArtifactRetentionExportProjection, M5ArtifactRetentionExportRow, M5ArtifactRetentionRow,
    M5ArtifactRetentionSummary, M5ArtifactRetentionViolation, M5DurableArtifactKind,
    M5StorageRetentionMatrix, RetentionPostureIndicator, RetentionPostureIndicatorKind,
    RetentionPostureIndicatorState,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_JSON,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_PATH,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_RECORD_KIND,
    PUBLISH_THE_M5_STORAGE_RETENTION_EXPORT_AND_OFFBOARDING_MATRIX_FOR_NEW_DURABLE_ARTIFACTS_SCHEMA_VERSION,
};

pub use publish_the_m5_feature_family_register_owner_map_and_proof_corpus_plan::{
    current_m5_feature_family_register, M5FeatureFamilyAction, M5FeatureFamilyGapReason,
    M5FeatureFamilyKind, M5FeatureFamilyRegister, M5FeatureFamilyRegisterExportProjection,
    M5FeatureFamilyRegisterExportRow, M5FeatureFamilyRegisterSummary,
    M5FeatureFamilyRegisterViolation, M5FeatureFamilyRow, M5FeatureFamilyState,
    M5FeatureFamilyStopRule, ProofCorpusItemKind, ProofCorpusItemState, ProofCorpusPlan,
    ProofCorpusPlanEntry,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_JSON,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_PATH,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_RECORD_KIND,
    PUBLISH_THE_M5_FEATURE_FAMILY_REGISTER_OWNER_MAP_AND_PROOF_CORPUS_PLAN_SCHEMA_VERSION,
};

pub use publish_the_m5_local_model_provider_graduation_and_spend_governance_control_packet::{
    current_m5_control_packet_register, ControlPacketAction, ControlPacketGapReason,
    ControlPacketItem, ControlPacketItemKind, ControlPacketItemState, ControlPacketLaneState,
    ControlPacketStopRule, ControlPacketStory, M5ControlPacketLaneKind, M5ControlPacketLaneRow,
    M5ControlPacketRegister, M5ControlPacketRegisterExportProjection,
    M5ControlPacketRegisterExportRow, M5ControlPacketRegisterSummary,
    M5ControlPacketRegisterViolation,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_JSON,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_PATH,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_RECORD_KIND,
    PUBLISH_THE_M5_LOCAL_MODEL_PROVIDER_GRADUATION_AND_SPEND_GOVERNANCE_CONTROL_PACKET_SCHEMA_VERSION,
};

pub use seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails::{
    current_m5_health_bundle_matrix, CertifiedArchetypeKind, HealthBundle, HealthBundleAction,
    HealthBundleGapReason, HealthBundleKind, HealthBundleRow, HealthBundleRowState,
    HealthIndicator, HealthIndicatorKind, HealthIndicatorState, M5HealthBundleMatrix,
    M5HealthBundleMatrixExportProjection, M5HealthBundleMatrixExportRow,
    M5HealthBundleMatrixSummary, M5HealthBundleMatrixViolation, RegressionGuardrailRule,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_JSON,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_PATH,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_RECORD_KIND,
    SEED_THE_M5_CERTIFIED_ARCHETYPE_HEALTH_BUNDLE_MATRIX_AND_REGRESSION_GUARDRAILS_SCHEMA_VERSION,
};

pub use stabilize_the_release_center_promotion_evidence_canary_pilot::{
    current_ring_promotion_control, Action as PromotionAction, GapReason as PromotionGapReason,
    KillSwitchPosture, PromotionDecision as RingPromotionDecision, PromotionPublicationRecord,
    PromotionRule, PromotionState, PromotionSubjectExportRow, PromotionSubjectKind,
    PromotionSubjectRow, Ring, RingPromotionControl, RingPromotionControlExportProjection,
    RingPromotionControlSummary, RingPromotionControlViolation, RollbackStopTrigger,
    RollbackTriggerKind, SoakWindow, RING_PROMOTION_CONTROL_JSON, RING_PROMOTION_CONTROL_PATH,
    RING_PROMOTION_CONTROL_RECORD_KIND, RING_PROMOTION_CONTROL_SCHEMA_VERSION,
};
