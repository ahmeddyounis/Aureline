# Local extension development, sideload review, runtime inspector, and publish preview contract seed

This document freezes the contract surfaces that make local extension
authoring, unpacked installs, sideload review, runtime inspection, hot
reload / relaunch disclosure, and publish preview first-class governed
experiences rather than ad hoc command-line side paths.

The machine-readable boundaries are:

- `/schemas/extensions/sideload_review.schema.json`
  — local extension workspace strip, sideload review sheet, runtime
  inspector snapshot, and hot reload / relaunch banner records.
- `/schemas/extensions/publish_preview.schema.json`
  — publish preview sheet record.

These contracts **compose with and do not replace** existing extension
and runtime seeds:

- `/schemas/extensions/effective_permission.schema.json` — permission
  vocabulary and effective permission summary.
- `/schemas/extensions/host_negotiation.schema.json` — runtime family,
  capability-world negotiation, and ABI range disclosures.
- `/schemas/extensions/registry_manifest.schema.json` and
  `/docs/extensions/registry_and_offline_bundle_seed.md` — provenance,
  signature class, trust-badge capping rules, offline/mirror semantics,
  and local-archive posture.
- `/docs/extensions/runtime_budget_packet.md` — runtime budgets,
  activation evidence, and quarantine posture vocabulary.

The schemas are authoritative when the narrative and the schemas
disagree; this document MUST be updated in the same change that lands
any schema bump.

The seed is deliberately narrow. It does **not** ship an extension SDK,
a marketplace backend, a registry service, or a full authoring UI. Its
job is to freeze the vocabulary early enough that local development and
sideload trust flows are not re-invented later.

## What this seed freezes

1. **Required surfaces** and their minimum disclosure fields for local
   authoring, sideload, trust review, runtime inspection, and publish
   preview.
2. **Local/unpacked trust invariants**: local and sideloaded packages do
   not silently inherit high-trust publisher badges.
3. **Review triggers** that force a re-review when a hot reload or local
   update would broaden capability, change runtime class/ABI, introduce
   external executables, rebind identity, or change release channel.
4. **Publish-preview separation of blockers vs warnings**, including the
   check origin class so the UI can explain *why* something is blocked.
5. **Export-friendly attribution**: enough stable IDs and refs exist so
   support, publish, and discovery flows can explain trust changes
   without re-parsing extension internals.

## Surface inventory (minimum required elements)

| Surface | Primary job | Required elements |
|---|---|---|
| **Local extension workspace strip** | show what the author is actively developing | package ID, source locator, runtime class, target host / ABI range, hot-reload state, unsigned / local-only label |
| **Sideload review sheet** | install unpacked or local package safely | source locator or archive identity, signing/unsigned state, runtime class, requested permissions, external executables, update binding (`stay local`, `bind to registry later`) |
| **Runtime inspector** | understand actual behavior after load | activation time, memory, current host identity, granted capabilities, log and failure refs, quarantine/disable action |
| **Hot-reload / relaunch banner** | explain state impact when code or manifest changes | restart/reload scope, preserved-vs-reset state cues, review triggers that pause hot reload, rollback path |
| **Publish preview sheet** | verify what leaves the machine before publication | manifest diff summary, version bump, signing identity, namespace/publisher, docs/templates/conformance status, sample completeness, release channel, outbound destination, release blockers vs warnings |

## Trust, provenance, and redaction rules

- Local, unpacked, or sideloaded packages MUST NOT render `Verified
  publisher`, `Enterprise approved`, or equivalent high-trust badges
  solely because the artifact was produced on the current machine.
  Missing provenance widens warnings; it never narrows disclosure.
- When provenance evidence is missing, consumers MUST label it explicitly
  as `Not provided`, `Partial`, `Unknown`, or `Local only` rather than
  silently omitting the field.
- Raw absolute filesystem paths MUST NOT cross schema boundaries. Local
  surfaces may render full paths in-process, but records and exports
  carry:
  - a `source_path_class` (workspace-relative, home-relative, removable
    media, network mount, stdin stream, not applicable), and
  - a redacted `source_path_hint` suitable for UI display and support
    export.
- Raw log bodies, crash dumps, stack frames, raw wasm bytes, and raw
  external executable binaries MUST NOT cross these boundaries. Records
  carry refs and typed summaries only.

## Review triggers (hot reload cannot silently broaden trust)

A new review step is required when any of the triggers below applies.
When triggered during a dev loop, hot reload MUST pause and the product
MUST route the user through a sideload-style review sheet before
continuing.

Minimum trigger vocabulary:

- Permission widening (requested permissions expand vs the last reviewed
  snapshot).
- Runtime-class change (e.g. Wasm sandbox → external host process).
- Host / ABI rebinding (host ABI range changes, or the package rebinding
  changes what host it targets).
- External executable introduction (a new helper or host executable is
  added, or an executable disclosure changes materially).
- Update binding change (e.g. `stay local` → `bind to registry later`,
  or binding to a registry identity).
- Release channel change (e.g. `preview` → `stable`).

## Hot reload / relaunch state vocabulary

Hot reload state MUST use one explicit vocabulary (no vague “up to date”
labels):

- `applied` — the latest change is reflected.
- `partial` — some changes are reflected; some are pending.
- `restart_required` — runtime restart required before changes apply.
- `rebuild_required` — build step required before reload can proceed.
- `failed` — the last reload failed.
- `unavailable` — hot reload not supported for this runtime/target.

## Runtime inspector (minimum contract)

The runtime inspector snapshot exists so authors and operators can
inspect:

- activation time and activation context refs,
- memory posture (at least one current measurement plus optional
  “budget class” refs),
- current host identity / runtime lane,
- granted capabilities and effective permission snapshot,
- recent failures and crash history refs,
- quarantine/disable actions and their typed reason.

The inspector composes with the activation-evidence packet seed and the
runtime-budget register; it does not mint parallel budget vocabularies.

## Publish preview (blockers vs warnings)

Publish preview MUST separate:

- **Release blockers** — the publish action is denied until resolved.
- **Warnings** — the publish action is allowed but risks are explicitly
  disclosed.

Every blocker or warning MUST carry a `check_origin_class` so the UI can
state whether the finding comes from schema validation, conformance-kit
results, accessibility checks, performance smoke checks, docs/template
completeness checks, signing/identity checks, or registry policy.

Publish preview MUST disclose the exact outbound destination (as a typed
destination descriptor ref or equivalent) and MUST preserve enough
attribution fields that downstream support and publish flows can explain
what trust posture changed and why.

