# M5 Support Center matrix

The **Support Center matrix** is the one authoritative contract for the Support Center as a product
surface. It replaces scattered, page-local wording about which inspectors a module reuses, which
support data classes it touches, how it redacts on export, and which export modes it offers, with a
single machine-readable matrix — one row per Support Center module, behind a fail-closed readiness
gate.

- Typed model + gate: `aureline-support` crate, `m5_support_center_matrix`
- Packet: `artifacts/support/m5/m5-support-center-matrix.json`
- Reviewer artifact: `artifacts/support/m5/m5-support-center-matrix.md`
- Schema: `schemas/support/m5-support-center-matrix.schema.json`
- Fixtures: `fixtures/support/m5/m5-support-center-matrix/`
- Shiproom review packet: `artifacts/shiproom/m5-support-center-review-packet/support_center_review_packet.md`

## Why this packet exists

M5 keeps adding recovery surfaces a blocked user must reach: Project Doctor, Safe mode, extension
bisect, and the performance / language / index / AI-usage / crash / network / artifacts inspectors,
plus issue-report / crash-intake routing and the support-bundle export preview. Each surface had
grown its own row-local notes for *which descriptors it reuses* and *what is safe to export*. This
packet makes the Support Center information architecture explicit:

- It enumerates the Support Center **modules** as a closed vocabulary.
- It binds environment status, precedence inspection, crash-intake, install/advisory state,
  credential state, and export consent into **one canonical inspector vocabulary** every module
  reuses.
- It defines the support **data classes**, **redaction defaults**, and the
  **local-save / team-share / formal-support** export modes.
- It requires **evidence freshness** and a **downgrade rule** for every claimed module, and
  auto-narrows desktop, CLI/headless, Help/About, shiproom, and formal-support handoff claims when a
  module goes stale, loses an inspector, or lacks consent.

## The modules

`doctor`, `safe_mode`, `bisect`, `performance`, `language`, `index`, `ai_usage`, `crash`, `network`,
`artifacts`, `issue_report_crash_intake`, and `support_bundle_export_preview`. Every module carries
exactly one row; no module inherits a posture from an adjacent one.

## The one canonical inspector vocabulary

Every module reuses a subset of these six descriptors rather than minting its own:

| Inspector | Binds |
| --- | --- |
| `environment_status` | the execution context and why it won |
| `precedence_inspector` | which config/policy layer won and what it shadowed |
| `crash_intake` | crash envelope, exact-build, and symbolication routing |
| `install_advisory_state` | install mode, channel, and active advisories |
| `credential_state` | credential posture without secret bodies |
| `export_consent` | redaction manifest and data-class consent |

## Data classes, redaction, and export modes

Support data classes reuse the frozen export-risk vocabulary: `metadata_only`,
`environment_adjacent`, `code_adjacent`, `high_risk`. Each module declares a redaction default —
`embedded_metadata_only`, `embedded_by_reference`, `retained_local_only`, `excluded_by_default`, or
`excluded_always` — and the export modes it offers: `local_save`, `team_share`, `formal_support`.
Local-save is always a first-class peer of the share/upload modes.

Two redaction-safety invariants are non-negotiable:

- A module that touches `high_risk` material must default to `excluded_always`. No consent can include
  secret-bearing material in a Support Center export.
- A module that offers a sharing mode (`team_share` or `formal_support`) must reuse the
  `export_consent` descriptor, so the consent surface is always bound where it matters.

## The fail-closed readiness gate

Each module declares the readiness it claims (`declared_readiness`) and records three independent
inputs:

1. **Evidence freshness** (`evidence_freshness`) — `current` → `operational`, `aging` → `degraded`,
   `expired` → `inspect_only`, `missing` → `unavailable`.
2. **Inspector availability** — the weakest bound inspector: `available` → `operational`, `degraded`
   → `degraded`, `unavailable` → `unavailable`.
3. **Export consent** — the weakest offered mode: `granted` → `operational`, `required_not_granted`
   → `degraded`, `blocked` → `inspect_only`.

The **published readiness** is the weakest of the declared readiness and those three ceilings. The
**publication** decision follows from it:

- `published` — the module is offered at its declared readiness; nothing narrowed it.
- `narrowed` — the gate lowered the readiness below the declared maximum; the claim is auto-narrowed,
  with a downgrade reason, a recovery path, and the stale/missing fields named.
- `withheld` — the readiness fell to `unavailable`; the module is not offered and lists no actions.

### Downgrade reasons and recovery paths

| Reason | Raised when | Recovery path |
| --- | --- | --- |
| `evidence_stale` | evidence is `aging`, `expired`, or `missing` | `refresh_evidence` |
| `inspector_degraded` | a bound inspector is degraded | `restore_inspector` |
| `inspector_unavailable` | a bound inspector is unavailable | `restore_inspector` |
| `consent_unsatisfied` | an offered mode is ungranted or blocked | `resolve_consent` |

A module whose effective readiness is `unavailable` takes the `withhold_module` recovery path; a
clean module takes `none`. The recorded reasons, path, and publication decision must equal the
recomputed gate, so a downgrade can never be asserted or hidden by hand. A reason is raised only when
its input narrows the module below its **declared** readiness, so a module designed as inspect-only
is not reported as "downgraded" for being inspect-only.

## Guardrails

- **No catch-all.** The matrix is the closed module vocabulary; it is not a dumping ground for
  unrelated admin or debug pages. Adding a module is a deliberate, reviewed change.
- **No silent claim.** A narrowed or withheld module names its recovery path, its caveats, and the
  stale or missing fields driving the narrowing.
- **One redaction/consent vocabulary.** UI, CLI/headless, and formal-support export paths share the
  same data classes, redaction defaults, and consent states; none invents its own.
- **One source of truth.** Desktop shell, CLI/headless, Help/About, shiproom, and formal-support
  handoff each bind to this packet through a consumer binding that must ingest it, preserve its
  published readiness and recovery paths, and narrow with it.

## Consuming the packet

Downstream surfaces render `M5SupportCenterMatrix::export_projection()` rather than restating each
module's readiness by hand, and support/evidence bundles carry
`M5SupportCenterMatrix::support_export(..)`, which preserves the exact matrix and excludes raw private
material. The packet is metadata-only: every field is a typed state, a count, or an opaque ref, and
it carries no credential bodies, raw provider payloads, live authority handles, or workspace
contents.
