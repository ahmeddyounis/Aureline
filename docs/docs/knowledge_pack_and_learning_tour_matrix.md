# Knowledge-pack and learning-tour matrix

This document publishes the single, shared matrix that documentation and guided-learning surfaces use to stay trustworthy across local, mirrored, and offline environments. It freezes:

- knowledge **source classes** (identity, freshness keys, mirror posture)
- surface **artifact rules** (mutable state, required truth labels, allowed mutations)
- **citation governance** (what must be cited per source class and per derived claim)
- **offline capability** declaration (fully offline, mirror-only, or online-optional)

Companion artifacts:

- `/artifacts/docs/knowledge_pack_rows.yaml` — machine-readable matrix rows
- `/schemas/docs/derived_explanation_descriptor.schema.json` — boundary schema for derived explanation descriptors
- `/fixtures/docs/learning_tour_cases/` — worked examples (offline, mirror-only, stale derived explanations, command-backed tour steps)

This matrix is normative for these surfaces (whether AI-backed or deterministic):

- docs browser results
- glossary cards
- guided-tour steps
- architecture maps
- codebase explanations

## 1) Knowledge source classes (normative)

| `source_class` | Typical content | Identity / freshness key (minimum) | Offline / mirror posture | Citation minimum (what the surface MUST retain) |
|---|---|---|---|---|
| `project_docs` | README, ADR, runbook, design notes, onboarding pages | workspace revision + doc path + optional anchor/span | always local-first | path + revision + anchor/span when available |
| `generated_reference` | OpenAPI docs, rustdoc, schema docs, generated SDK references | build identity + generator identity/version + source revision | cacheable and mirrorable as build artifacts | generator/source contract + build identity + page/anchor |
| `mirrored_official_docs` | signed, mirrored vendor/framework/language docs | pack id + pack revision + locale + upstream digest | signed docs pack; mirrorable; offline-capable | pack id/revision + page/anchor (no raw URL) |
| `curated_knowledge_pack` | glossary cards, tutorials, org onboarding cards, quickstarts | pack id + pack revision + locale + signer | signed pack; mirrorable; offline-capable | pack id/revision + underlying evidence anchors (or explicit inference labels) |
| `derived_explanation` | codebase explainer paragraph, architecture-map narrative, guided-step paraphrase | query hash + graph epoch + docs-pack set + build identity | cached locally; never canonical truth | per claim: supporting anchor(s) **or** inference label + confidence; derived outputs retain upstream anchors |

## 2) Surface artifacts (normative)

All surfaces below MUST be able to answer, for the content currently shown:

- `source_class` / `freshness` basis
- citation set (which anchors support the content; which claims are inference)
- offline capability (`fully_offline_capable`, `mirror_only_capable`, or `online_optional`)

| Surface artifact | Inputs (typical) | Mutable state | Required truth labels (minimum) | Mutation permission | Offline capability declaration | Citation governance |
|---|---|---|---|---|---|---|
| Docs-browser result | docs index + search planner + graph scope | view state only | source class, freshness, locale, trust class, offline capability | view-only | must declare one offline-capability class | must retain the anchor(s) for the selected result; source-class citation minimum applies |
| Glossary card | docs/knowledge pack + graph links + symbol metadata | dismiss / pin | pack revision, freshness, scope, locale fallback | may open related docs/code only through stable commands | must declare one offline-capability class | must cite pack id/revision plus any backing symbol/file/doc anchors for authoritative claims; inference must be labeled |
| Guided-tour step | graph query + doc refs + command refs | progress / completion / dismissal | citation set, freshness, missing-scope indicator, locale fallback | all actions route through stable commands | must declare one offline-capability class | steps that paraphrase sources retain upstream anchors; any claim without anchors is inference-labeled with confidence |
| Architecture map | graph neighborhood + docs links + ownership metadata | zoom / layout state | graph epoch, completeness label, source classes, freshness | view-only unless an explicit follow-up command is invoked | must declare one offline-capability class | derived narrative cites files/symbols/docs or labels inference; map nodes expose evidence refs |
| Codebase explanation | graph query + docs query + explainer engine | cached session state | citations, confidence, inference labels, freshness | no direct mutation; follow-up actions are explicit stable commands | must declare one offline-capability class | every non-trivial statement cites supporting anchors or is marked as inference with confidence; derived explanations never present as canonical truth |

## 3) Citation governance (normative)

Surfaces apply citation rules by `source_class`:

- `project_docs` — cite doc path + revision; include anchor/span when available.
- `generated_reference` — cite generator/source contract plus build identity; include page/anchor.
- `mirrored_official_docs` — cite pack id + pack revision + anchor; never flatten to raw URL authority.
- `curated_knowledge_pack` — cite pack id + pack revision + locale; for authoritative claims, retain the pack’s underlying evidence anchors (or label inference).
- `derived_explanation` — derived content never ships as primary authority: each non-trivial claim carries supporting anchors **or** an inference label + confidence; derived outputs retain upstream anchors so offline/export review can reconstruct sources.

## 4) Offline capability classes (normative)

Every docs/learning surface declares exactly one offline capability class for the content it is currently rendering:

- `fully_offline_capable` — all required bytes and evidence resolve from local files and installed/mirrored packs; no network required.
- `mirror_only_capable` — content/evidence must resolve from a configured mirror or imported bundle; upstream online fetch is not required or assumed.
- `online_optional` — the surface can operate fully offline for local/mirrored sources, but may (explicitly) widen to online fetchers when policy and connectivity allow; any online widening is disclosed as a different posture.

## 5) Worked examples

Fixtures under `/fixtures/docs/learning_tour_cases/` cover:

- offline guided tour step using a mirrored pack (no online dependency)
- mirror-only guided tour step (offline/mirror posture explicitly declared by the deployment profile)
- stale derived explanation descriptor (stale freshness + inference labeling)
- command-backed guided tour step (actions route through stable command ids)

## 6) Sources of truth

This matrix is derived from, and must remain consistent with:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` (Documentation graph rules; Appendix “Documentation, Knowledge-Packs, and Learning-Tour Matrix”)
- `.t2/docs/Aureline_Technical_Design_Document.md` (Docs-integrity and guided-learning invariants; docs browser matrix appendices)
- `/docs/docs/docs_pack_manifest_contract.md` and `/schemas/docs/docs_pack_manifest.schema.json`
- `/docs/docs_integrity/citation_and_reference_contract.md` and `/schemas/docs/citation_anchor.schema.json`
- `/docs/ux/learnability_contract.md`, `/schemas/ux/guided_surface_state.schema.json`, and `/schemas/ux/guided_tour.schema.json`

