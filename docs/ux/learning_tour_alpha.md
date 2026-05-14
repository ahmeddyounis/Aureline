# Learning Tour Alpha

This alpha lane makes guided learning inspectable without creating a second
command, docs, or progress model. Tour packages, exercise rails, contextual
teaching surfaces, and progress snapshots resolve through:

- `schemas/docs/learning_tour_alpha.schema.json`
- `schemas/user/learning_progress_alpha.schema.json`
- `artifacts/docs/learning_tour_alpha.yaml`
- `fixtures/docs/learning_tour_alpha/`

## Contract

Tour packages are versioned by `package_id`, `package_version`, and
`package_revision_ref`. Each package names its docs pack revision, install
state, degradation class, citation refs, and exact reopen ref. The alpha
manifest must include local-only, cached, mirror-only, and not-installed
states so consuming surfaces can show honest availability instead of silently
hiding unavailable learning content.

Steps never anchor to raw screen coordinates. A step target is a stable
command, docs node, source file, symbol, graph node, or help anchor. Each step
states whether scope widened, carries success criteria, exposes hint/reveal,
skip/reset, sandbox, reversibility, and exact reopen metadata, and separates
explanation from action.

Mutation-capable exercises route through the ordinary command path. The
manifest must name the command id, preview sheet, approval path, and evidence
packet rule before an apply-capable action can run. The contextual teaching
surface reads the same row and exposes separate explanation and action refs.

Learning progress is user-owned profile state. The progress snapshot can tune
tip intensity, jargon level, AI explanation posture, mutation guardrails,
bookmarks, and dismissals, but it cannot change trust boundaries or data
ownership. Repo packs, classroom artifacts, and telemetry-style readers get no
default read access.

## First Consumer

The shell consumes `artifacts/docs/learning_tour_alpha.yaml` through the
learning-tour alpha module. The first projection is a contextual teaching rail
for the import-preview exercise:

- current step: `step:aureline.import.preview-before-apply`
- command: `cmd:workspace.import_profile`
- docs anchor: `docs:anchor:onboarding_alpha:keymap_bridge`
- support export row: `support-row:learning-tour-alpha.import-preview`

This keeps the teaching surface inspectable from runtime, support export, and
user-owned progress state using the same identifiers.
