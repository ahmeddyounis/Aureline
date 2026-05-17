# Suspicious-content shared-detector packet and safe-preview affordance baseline

This packet projects the safe-preview trust-class contract frozen in
[`/docs/security/safe_preview_trust_classes.md`](./safe_preview_trust_classes.md)
into a shared detector contract that every preview-capable surface can
map to — editor content, docs/help pages, rich previews, embedded
docs/webview content, package/install metadata review, approval and
delete review, support/export, and any surface that must hand active
execution off to the system browser.

It exists so a single piece of suspicious content produces one detector
outcome record that editor, docs, preview, install, approval, delete,
support-export, and browser-handoff surfaces all read — instead of each
surface inventing its own trust language, warning posture, copy/export
label, or "open in browser anyway" affordance.

Companion artifacts:

- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  — boundary schema for `surface_trust_resolution_record` and
  `suspicious_content_case_record`. This packet reuses those record
  kinds verbatim; it does not mint a parallel detector schema.
- [`/schemas/security/text_representation_policy.schema.json`](../../schemas/security/text_representation_policy.schema.json)
  — boundary schema for `representation_transfer_record`. Every
  detector outcome resolves to one or more representation-labeled
  transfer records using this schema.
- [`/schemas/security/text_representation_action.schema.json`](../../schemas/security/text_representation_action.schema.json)
  — boundary schema for the per-surface action rows that state which
  representation-labeled transfer is primary, secondary, fallback, or
  unavailable.
- [`/docs/accessibility/locale_fallback_and_copy_representation_contract.md`](../accessibility/locale_fallback_and_copy_representation_contract.md)
  — row-level accessibility contract for locale fallback,
  mixed-direction / invisible-text inspection, and copy/export
  representation parity.
- [`/fixtures/security/suspicious_content_cases/`](../../fixtures/security/suspicious_content_cases/)
  — worked cases spanning raw text, sanitized rich, rich preview with
  downgrade, embedded isolated-remote webview, package/install review,
  remote-attach approval, delete review, support export, and the
  browser-handoff / blocked-execution path.
- [`/docs/security/m3/content_integrity_beta.md`](./m3/content_integrity_beta.md)
  and
  [`/fixtures/content_safety/m3/shared_detector/`](../../fixtures/content_safety/m3/shared_detector/)
  — beta gate packet that proves the shared detector, warning classes, and
  raw/rendered/sanitized/redacted transfer labels stay green across declared
  surfaces.
- [`/artifacts/security/safe_preview_examples/`](../../artifacts/security/safe_preview_examples/)
  — baseline reviewer-facing affordance examples showing how the
  product labels raw versus rendered content, active-content risk, and
  why certain actions are disabled or routed to the system browser.

Normative sources this packet projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — Section 11 content-integrity rules, Appendix AY content-integrity
  and safe-preview matrix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Appendix EN suspicious-content, safe-preview, and copy/export
  templates.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — Section 14.9 suspicious-content and source-integrity pattern.
- [`/docs/security/safe_preview_trust_classes.md`](./safe_preview_trust_classes.md)
  — the vocabulary this packet freezes the detector contract on top
  of. Trust-class spellings (`RawText`, `SanitizedRich`,
  `TrustedLocalActive`, `IsolatedRemoteActive`) and the closed
  suspicious-content class set are used here verbatim.

## Why freeze this now

The trust-class contract answers "what is the surface allowed to do
with this content". It does not yet answer, with one shared vocabulary,
"what is the detector's outcome when it looks at this content".
Without that second answer:

- an editor surface can warn about a bidi identifier, a docs surface
  can keep the same identifier visible with no warning, and a support
  export can strip the warning entirely because each surface resolved
  the finding on its own terms;
- an install review can mark a confusable publisher name "suspicious",
  while the approval surface that carries the same decision a minute
  later uses the generic word "warning" and reopens the attack
  surface;
- an embedded webview can keep its "active" chrome after origin drift
  because no one said, in one place, that the detector's outcome is
  `isolate` only while the live-origin contract holds;
- a confusable link in a docs page can compress to a generic "Open"
  action that silently launches an in-product execution path instead
  of routing to the system browser after raw-target inspection.

Freezing the shared-detector contract now lets every later
implementation inherit:

- one closed detector-outcome vocabulary (`allow`, `sanitize`,
  `isolate`, `block`, `route_to_system_browser`);
- one mapping from detector outcome to resolved trust class and
  representation-labeled transfer;
- one set of evidence fields carried into review, approval, export,
  delete-review, and support bundles;
- one safe-preview affordance baseline that labels raw versus
  rendered content, names active-content risk, and explains why
  certain actions are disabled or routed elsewhere.

## Scope

In scope for this revision:

- the shared-detector outcome vocabulary and its one-to-one
  projection onto the frozen trust classes;
- the evidence-field expectations for detector outcome records and
  their suspicious-content findings;
- representation-labeled copy/export expectations per detector
  outcome;
- safe-preview affordance baselines for raw text, sanitized rich,
  rich preview, embedded docs/webview, package/install metadata,
  and browser-handoff / blocked-execution cases;
- the rule that isolated or sanitized content is never represented
  as equivalent to trusted local content.

Out of scope until a later lane lands:

- the actual sandboxing engine, process boundary, or CSP/iframe
  policy for `IsolatedRemoteActive` surfaces;
- the full threat-detection implementation (heuristics, ML
  detectors, package reputation scoring);
- final visual styling for warning states, trust badges, or the
  system-browser handoff confirmation;
- the final linker between detector outcome records and the
  observability lane beyond the evidence fields frozen here.

## Shared-detector contract

Every preview-capable surface projects one detector outcome record
per subject. The record is not a new schema — it is a disciplined
use of the existing trust and representation schemas so downstream
surfaces can read one thing instead of five.

### Detector outcome vocabulary (closed)

| Outcome id | Meaning | Projects to trust class | Default transfer posture |
|---|---|---|---|
| `allow` | no suspicious content worth annotating; render per surface default | surface default (`RawText` or `SanitizedRich`) | default per class |
| `sanitize` | render after active / scriptable portion is removed; keep raw and escaped inspection reachable | `SanitizedRich` | `copy_rendered`, `copy_raw`, `copy_escaped`, `export_sanitized_snapshot` |
| `isolate` | keep active inside a declared isolation boundary under verified origin, connectivity, and permissions | `IsolatedRemoteActive` (remote/embedded) or `TrustedLocalActive` (local sandbox) | `copy_rendered` (while verified), `export_sanitized_snapshot`, `export_metadata_only` |
| `block` | refuse to render actively in-product; present `RawText` inspection or static sanitized snapshot only | `RawText` or `SanitizedRich` (snapshot) | `copy_raw`, `copy_escaped`, `export_metadata_only` |
| `route_to_system_browser` | in-product execution of the target is refused; the user inspects the raw target and then hands off to the system browser | `SanitizedRich` on the originating surface; no in-product active class for the target | `copy_raw`, `copy_escaped`, `copy_rendered`, `export_sanitized_snapshot` |

Rules:

- The detector's outcome is a rendering/execution decision, not a
  copy label. Copy and export labels always name the representation
  separately (see §Representation-labeled transfer).
- `isolate` never survives origin change, connectivity loss, sandbox
  loss, policy narrowing, approval-scope drift, or support/export
  boundary. The effective outcome narrows to `sanitize` (static
  snapshot) or `block` (metadata-only).
- `route_to_system_browser` does not pretend the external destination
  is sanitized or isolated. It names the correct place for the active
  behavior to happen (the system browser), and the originating
  surface stays `SanitizedRich` with raw-target inspection reachable
  before the handoff.
- An outcome is only as strong as its chrome. If owner, origin, or
  capability chrome cannot be rendered, the outcome narrows one
  step toward `block`.

### Evidence fields

A detector outcome carries, via the existing record kinds:

- `case_id` — joins the `surface_trust_resolution_record`,
  `suspicious_content_case_record`, and the matching
  `representation_transfer_record` set for one subject.
- `surface_family` — from the closed set in the trust-class schema.
- `content_subject_ref`, `owner_identity_ref`, `origin_identity_ref`,
  `origin_kind` — opaque identity handles. Raw bodies, paths, URLs,
  or secrets never appear.
- `current_trust_class`, `connectivity_state`,
  `effective_allowed_behaviors`, `default_transfer_actions` — the
  resolved render/execution state.
- `upgrade_trigger_requirements`, `downgrade_trigger_observations`,
  `effective_downgrade_posture` — why the outcome can move up or
  must move down.
- `required_owner_origin_chrome` — chrome fields that must stay
  visible for the outcome to remain valid.
- `label_examples` — the on-surface labels (badge, primary copy,
  primary export, warning) that make the outcome legible.
- `related_finding_refs` — zero or more
  `suspicious_content_finding_record` ids. Each finding names a
  `content_class`, a `location_kind`, a `visibility_impact`, its
  reveal affordances, and its suppression scope.
- `stricter_annotation_requirements` — additional requirements the
  outcome places on the claiming surface (for trust-decision,
  approval, and delete-review modes).

The detector's evidence bundle is the union of these fields across
the resolution record and any attached finding records. No surface
introduces a parallel evidence vocabulary.

### Mapping surfaces to outcomes

| Surface family | Typical outcome(s) | Notes |
|---|---|---|
| `editor_content` | `allow` or `sanitize` on plain text; `block` equivalent is "annotate and do not normalize" because the editor never "executes" text | raw bytes always visible; suspicious-content labels stay attached to the exact token |
| `review_surface` | same as `editor_content`, with review-hunk annotations that survive export | review hunks inherit the editor's outcome |
| `docs_help_page` | `sanitize` default; `route_to_system_browser` on external links; `block` for any in-product active execution of docs links | raw/rendered divergence labels stay visible together |
| `rich_preview` | `sanitize` default; `isolate` when all `IsolatedRemoteActive` or `TrustedLocalActive` triggers hold; narrows to `sanitize` (static snapshot) or `block` (metadata-only) on guarantee loss | active badge never survives origin, connectivity, or capability loss |
| `install_review`, `publish_review`, `remote_attach_review` | `block` for in-product active rendering of the package body; `RawText` with strict trust-decision mode for identifiers; metadata-only export | stronger annotation mode is mandatory before commit |
| `approval_surface` | outcome inherited from the reviewed item, always stricter; `block` for any active carry-forward after scope or origin drift | approval never silently re-upgrades to an `isolate` outcome |
| `delete_review_surface` | last-visible-evidence `sanitize` or `block` (metadata-only) | warnings survive until deletion completes |
| `support_export_surface` | `sanitize` (static snapshot) or `block` (metadata-only) only | never active; never upgrades |

A surface with no suspicious-content findings still projects a
resolution record with outcome `allow` so downstream readers never
have to distinguish "no finding" from "no record".

### Representation-labeled transfer

Every detector outcome resolves to one or more
`representation_transfer_record` entries under
`/schemas/security/text_representation_policy.schema.json`. Rules:

- Generic `Copy` or `Export` labels are forbidden where rendered,
  active, escaped, or sanitized representations differ materially.
- `copy_rendered` on a surface with `raw_rendered_divergence` MUST
  keep `copy_raw` visible on the same surface.
- `copy_escaped` remains reachable anywhere suspicious content is
  present.
- `export_sanitized_snapshot` preserves owner/origin attribution,
  trust class, and snapshot age when the source is not live
  anymore.
- `export_metadata_only` must state its `fallback_reason` and keep
  `metadata_only_reason` disclosed. It is the fail-closed export.
- Active content never survives a support/export boundary as active.

Per-surface availability and defaulting are captured by
`text_representation_action_record` entries under
`/schemas/security/text_representation_action.schema.json`. The action
rows preserve the same canonical label stems across editor, diff, docs
preview, install review, terminal transcript, support export, and
security review surfaces.

### Isolated or sanitized is never "trusted local"

A recurring failure mode is letting an isolated or sanitized preview
drift into looking like trusted local content in the user's eyes.
The packet forbids this:

- `IsolatedRemoteActive` surfaces MUST show publisher/host and
  connectivity chrome next to the trust-class badge; the badge is
  never the bare string "Active".
- `SanitizedRich` surfaces MUST keep raw / rendered / sanitized
  representation labels distinct on transfer actions; "Copy" alone
  is not conforming.
- An `isolate` outcome that loses any of origin, connectivity,
  sandbox, policy, or owner chrome narrows to `sanitize` (static
  snapshot) or `block` (metadata-only). The `isolate` chrome is not
  allowed to remain after downgrade.
- `route_to_system_browser` never describes the external destination
  as "safe" or "sanitized" inside the product. It names the raw
  target and confirms the handoff to the system browser.

## Safe-preview affordance baseline

The affordance baseline captures how the product should label and
route safe-preview content before any final UI chrome lands. It is
rendered in the companion artifacts
[`/artifacts/security/safe_preview_examples/`](../../artifacts/security/safe_preview_examples/)
and maps one-to-one to the fixture corpus under
[`/fixtures/security/suspicious_content_cases/`](../../fixtures/security/suspicious_content_cases/).

The baseline answers three questions for every preview-capable
surface:

1. **What representation is the user looking at?**
   Raw bytes, rendered view, sanitized snapshot, or metadata-only
   envelope. The answer is named out loud on the surface and on
   every transfer action.
2. **What can the user do from here, and what is off the table?**
   Primary actions, secondary actions, and disabled actions (with
   the reason) are all listed on the surface. Disabled actions are
   never silently omitted.
3. **If the active path is not safe here, where does it belong?**
   The surface names the correct destination: system browser for
   external targets, review sheet for install/approval/delete
   decisions, metadata-only export for support bundles.

Baseline coverage (one example per category):

| Example | Fixture it mirrors | Outcome | What it demonstrates |
|---|---|---|---|
| `raw_text_editor_affordance.yaml` | `editor_bidi_identifier.json` | `sanitize` on the annotation layer; content remains `RawText` | Inline suspicious-content marker, raw/escaped copy, no silent normalization |
| `sanitized_rich_docs_affordance.yaml` | `docs_help_markdown_representation.json` | `sanitize` | Raw versus rendered labels kept distinct; both copy paths visible |
| `rich_preview_downgrade_affordance.yaml` | `rich_preview_notebook_downgrade.json` | `isolate` narrowed to `sanitize` | Active badge removed on trust loss; static snapshot with snapshot_age |
| `embedded_webview_isolated_affordance.yaml` | `embedded_webview_isolated_remote.json` | `isolate` | Publisher/host, origin-verified, connectivity chrome required; narrows on drift |
| `install_review_strict_affordance.yaml` | `install_review_confusable_publisher.json` | `block` (in-product active); strict `RawText` | Raw publisher inspection before confirm; metadata-only export |
| `browser_handoff_affordance.yaml` | `browser_handoff_blocked_execution.json` | `route_to_system_browser` | In-product execution disabled; raw-target inspection before handoff |
| `support_export_sanitized_affordance.yaml` | `support_export_sanitized_snapshot.json` | `sanitize` | Export is static snapshot or metadata-only; warning state preserved |

Each example is reviewable without a renderer implementation. They
freeze the affordance contract so later work does not have to
rediscover whether "Open" should launch in-product, whether "Copy"
should carry raw or rendered bytes, or whether "Active" should
remain after connectivity loss.

## Acceptance rules

A surface conforms to this packet if and only if:

- It projects exactly one `surface_trust_resolution_record` per
  subject, with `current_trust_class`, `connectivity_state`,
  `effective_allowed_behaviors`, `default_transfer_actions`, and
  `required_owner_origin_chrome` drawn from the closed
  trust-class vocabulary.
- Suspicious-content findings use `suspicious_content_case_record`
  with the closed class set and closed `location_kind` vocabulary;
  each finding is anchored to a concrete location.
- Every copy or export action resolves to a
  `representation_transfer_record` whose `action_id`,
  `representation_class`, `body_posture`, and
  `required_disclosure_fields` are drawn from the
  text-representation policy schema.
- The surface preserves raw/rendered distinction: rendered
  representations never displace raw representations where the
  detector outcome is `sanitize`, `isolate`, `block`, or
  `route_to_system_browser`.
- Isolated and sanitized content carries chrome that keeps it
  clearly non-equivalent to trusted local content.
- The surface narrows toward `sanitize` (static snapshot) or
  `block` (metadata-only) on any downgrade trigger; it never
  retains an `isolate` outcome after the triggers that justified
  it.

## Out-of-scope clarifications

This packet does not freeze:

- the sandbox, iframe, process, or CSP policy that implements
  `IsolatedRemoteActive`;
- the heuristics that decide whether a class-level finding is
  present (character-table lookups, bidi-parsing depth, link
  similarity scoring);
- the system-browser handoff confirmation dialog copy beyond the
  affordance baseline;
- the observability lane's full event shape for detector
  outcomes; only the evidence fields carried on the records above
  are frozen here.

Extending the detector (new suspicious-content class, new
representation action, new surface family) is an additive-minor
change to the companion schemas and this packet, not a surface-local
invention.
