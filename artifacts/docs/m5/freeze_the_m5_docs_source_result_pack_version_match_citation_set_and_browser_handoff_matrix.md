# M5 Docs Source, Result, Pack, Version-Match, Citation-Set, and Browser-Handoff Matrix

- Packet: `m5-docs-contracts-matrix:stable:0001`
- Schema: `schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json`
- Support export: `artifacts/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/support_export.json`
- Contract doc: `docs/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix.md`
- Fixtures: `fixtures/docs/m5/freeze_the_m5_docs_source_result_pack_version_match_citation_set_and_browser_handoff_matrix/`

## Coverage

- The docs source descriptor is qualified Stable: it carries source class, trust class, locale match, and mirror/offline posture so project docs never masquerade as vendor docs and live external docs always require an explicit handoff.
- The docs result object is qualified Stable: it carries source class, trust class, version-match state, and freshness so a captured or stale result never claims live authority.
- The docs-pack manifest is qualified Stable: it carries source class, version-match state, mirror/offline posture, and locale match with signature state for every installed or mirrored documentation pack.
- The derived-explanation citation set is qualified Stable: a generated explanation is bound to its citations, is never primary authority, and expires with its citation set.
- The version-match state is qualified Stable: version-match and freshness truth between a source and the active build stays visible and never silently upgrades to an exact build match.
- The stale-example finding is qualified Stable: documented examples that drifted from current behavior are surfaced rather than hidden and name the drift reason.
- The browser-handoff object is qualified Beta: it names the handoff reason and the privacy consequence and never silently shares context or impersonates a governed docs surface.
- Every object carries required fields, declared state vocabularies, evidence requirements, proof packet refs, downgrade triggers, rollback posture, and consumer-surface parity.
- The self-describing vocabulary set freezes the canonical tokens for source class, version-match state, freshness, trust class, locale match, mirror/offline posture, browser-handoff reason, and browser-handoff privacy consequence.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.
- The release posture binds the supporting release packet and mirror/offline packet and requires support/export and mirror/offline parity for every object.

## Trust guardrails

The matrix proves that documentation truth stays typed and inspectable: source class, locale, version match, freshness, mirror/offline state, trust class, and citation basis stay visible; project docs never masquerade as vendor docs; derived explanations never outlive their citation sets; version-match and freshness never silently upgrade; mirror/offline state stays disclosed; browser handoff never silently shares context or impersonates a governed docs surface; and stale examples are surfaced rather than hidden. No speculative knowledge-platform or hosted-search product is in scope, and stale or underqualified objects automatically narrow before publication rather than hiding the object.
