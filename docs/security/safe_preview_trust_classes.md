# Safe-preview trust classes and suspicious-content vocabulary

This document freezes the shared safety vocabulary Aureline uses when a
surface renders raw text, sanitized rich content, trusted local active
content, or isolated remote / embedded content. It exists so editor,
docs, review, install, preview, approval, delete-review, and support /
export surfaces project one contract instead of inventing parallel
rendering, warning, or copy/export labels.

Companion artifacts:

- [`/schemas/security/trust_class.schema.json`](../../schemas/security/trust_class.schema.json)
  — boundary schema for surface trust-resolution records and suspicious-
  content case records.
- [`/schemas/security/text_representation_policy.schema.json`](../../schemas/security/text_representation_policy.schema.json)
  — boundary schema for representation-labeled copy / export decisions.
- [`/schemas/security/text_representation_action.schema.json`](../../schemas/security/text_representation_action.schema.json)
  — action-row schema for availability, defaulting, parity labels, and
  disclosure requirements for each representation-labeled copy / export
  action.
- [`/docs/accessibility/locale_fallback_and_copy_representation_contract.md`](../accessibility/locale_fallback_and_copy_representation_contract.md)
  — accessibility-facing row contract that keeps locale fallback,
  mixed-direction inspection, and representation labels reachable in
  dense, high-zoom, keyboard, and assistive-technology flows.
- [`/fixtures/security/suspicious_content_cases/`](../../fixtures/security/suspicious_content_cases/)
  — worked examples spanning editor content, docs / help pages, rich
  previews, install / attach / approval review, and metadata-only
  fallback.

Normative sources this document projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  — Section 11 content-integrity rules, Appendix AY content-integrity
  and safe-preview matrix, and the safe-preview trust-class seed.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md`
  — Appendix EN suspicious-content, safe-preview, and copy / export
  templates.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  — Section 14.9 suspicious-content and source-integrity pattern.
- [`/docs/ux/shell_interaction_safety_contract.md`](../ux/shell_interaction_safety_contract.md)
  — the shell-wide preview and representation contract this document
  narrows for source and preview safety.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — redaction defaults every metadata-only export inherits.

## Why freeze this now

If safe preview remains implicit, the same bytes can look safe in one
surface, suspicious in another, and impossible to reconstruct in
support or review. The failure mode is not only accidental rendering:

- a docs preview can imply that rendered markup is the same thing as
  source bytes;
- an install prompt can compress a confusable publisher name into a
  generic warning with no raw-inspection path;
- a remote or embedded preview can keep an "active" badge after origin,
  connectivity, or sandbox guarantees are gone;
- a support export can carry a generic `Copy` or `Export` label that
  hides whether the payload contains raw bytes, escaped source, a
  sanitized snapshot, or metadata only.

Freezing the vocabulary now lets every later implementation inherit:

- one closed trust-class set;
- one suspicious-content class set;
- one owner / origin chrome contract;
- one downgrade posture when trust or connectivity is lost; and
- one representation-labeled transfer model for bytes leaving the
  product.

## Scope

In scope for this revision:

- the four safe-preview trust classes defined in the architecture,
  including allowed behavior, copy defaults, upgrade / downgrade
  triggers, owner / origin chrome, and label expectations;
- the suspicious-content vocabulary for bidi controls, invisible
  formatting, mixed-script confusables, whole-script confusables, and
  rendered-versus-source divergence;
- stronger annotation rules on install, publish, attach, approval, and
  delete-review surfaces;
- the transfer actions `copy_raw`, `copy_rendered`, `copy_escaped`,
  `export_sanitized_snapshot`, and `export_metadata_only`;
- downgrade-to-static-snapshot and metadata-only behavior when origin,
  connectivity, or trust guarantees are lost.

Out of scope until a later lane lands:

- final renderer implementation details for every media type;
- full notebook, extension-webview, or remote-runtime execution policy;
- final visual styling for warning states or trust badges;
- automated detection heuristics beyond the vocabulary and observable
  response this document freezes.

## Frozen trust classes

The trust-class names are the architecture names verbatim. They are not
renamed, abbreviated, or localized in packets.

### RawText

`RawText` is the exact-byte presentation class.

- Allowed behavior:
  render exact bytes, show targeted warning overlays, reveal suspicious
  code points, provide raw and escaped inspection, and permit
  representation-labeled raw / escaped transfer.
- Forbidden behavior:
  executing script, interpreting remote widgets, silently normalizing
  bytes, or replacing the raw representation with a richer render
  without an explicit user action that moves to a different trust
  class.
- Default transfer posture:
  `copy_raw` is primary; `copy_escaped` is always available on
  suspicious-content surfaces; `export_metadata_only` is the fallback
  when policy or redaction blocks raw bytes.
- Upgrade rules:
  `RawText` never auto-escalates. Moving from `RawText` to a richer
  class requires an explicit user action and the destination class's
  guarantees.
- Downgrade rules:
  if exact bytes are no longer available, the surface falls back to a
  static snapshot or metadata-only envelope and MUST say which occurred.
- Required owner / origin chrome:
  trust-class badge, owner identity, origin identity, representation
  label on transfer actions, and snapshot age when bytes are no longer
  live.
- Label expectations:
  `Raw text`, `Copy raw`, `Copy escaped`, and a targeted suspicious-
  content label such as `Bidi control present`.

### SanitizedRich

`SanitizedRich` is the non-active rendered class.

- Allowed behavior:
  render sanitized markup, images, structure, and formatting after the
  active / scriptable portion has been removed or disabled.
- Forbidden behavior:
  ambient filesystem access, ambient network access, active widget
  execution, hidden script execution, or copy/export labels that imply
  the rendered snapshot is the same thing as source bytes.
- Default transfer posture:
  `copy_rendered` and `export_sanitized_snapshot` are primary;
  `copy_raw` remains available when source bytes exist; `copy_escaped`
  remains available when suspicious content is present.
- Upgrade rules:
  active behavior requires explicit trust elevation plus either local
  trust (`TrustedLocalActive`) or verified remote isolation
  (`IsolatedRemoteActive`).
- Downgrade rules:
  sanitizer failure, source/origin mismatch, missing owner identity, or
  lost connectivity pushes the surface to `RawText`, a static sanitized
  snapshot, or metadata-only, depending on which representation remains
  safe and truthful.
- Required owner / origin chrome:
  trust badge, owner identity, origin identity, snapshot age if the
  render is frozen, and a representation label on every transfer.
- Label expectations:
  `Sanitized preview`, `Copy rendered preview`, `Copy raw source`, and
  `Export sanitized snapshot`.

### TrustedLocalActive

`TrustedLocalActive` is the active-content class for trusted local
content running inside a declared local capability sandbox.

- Allowed behavior:
  active behavior confined to declared local capabilities, visible trust
  state, and explicit owner/origin chrome.
- Forbidden behavior:
  undeclared network or filesystem access, background escalation after
  trust changes, or exports that imply the live active view itself left
  the product unchanged.
- Default transfer posture:
  `copy_rendered` is primary for the current view; `export_sanitized_snapshot`
  is the default export; `copy_raw` and `copy_escaped` remain reachable
  if the source representation exists.
- Upgrade rules:
  trusted workspace, explicit user elevation, policy allow, and a
  declared capability sandbox are all required.
- Downgrade rules:
  trust revocation, policy narrowing, runtime health failure, missing
  owner/origin chrome, or support/export boundaries force downgrade to
  `SanitizedRich`, static snapshot, or metadata-only. Active content
  never survives a support/export boundary as active.
- Required owner / origin chrome:
  trust badge, owner identity, origin identity, capability summary,
  trust state, and representation label on every transfer.
- Label expectations:
  `Trusted local active preview`, `Capabilities: local only`,
  `Copy rendered output`, and `Export sanitized snapshot`.

### IsolatedRemoteActive

`IsolatedRemoteActive` is the active-content class for web-like or
remote / embedded content that stays inside a declared isolated process
or web sandbox with visible origin and permission summary.

- Allowed behavior:
  active rendering inside the declared isolation boundary while origin,
  connectivity, and policy guarantees remain intact.
- Forbidden behavior:
  hidden origin changes, silent reuse of approval after host / owner /
  route drift, or transfer labels that pretend the isolated active view
  is equivalent to raw bytes.
- Default transfer posture:
  `copy_rendered` is only allowed while the origin remains verified;
  `export_sanitized_snapshot` or `export_metadata_only` are the default
  exports; raw / escaped inspection is available only when the source
  contract actually exposes it.
- Upgrade rules:
  explicit user action, verified origin identity, live connectivity,
  declared remote-origin contract, declared permissions, and isolation
  health.
- Downgrade rules:
  origin change, origin loss, connectivity loss, policy narrowing,
  sandbox loss, approval-scope drift, or delete-review evidence
  preservation force downgrade to static snapshot, sanitized snapshot,
  or metadata-only. A disconnected active surface MUST NOT keep the same
  affordance as a live verified origin.
- Required owner / origin chrome:
  trust badge, owner / publisher / host identity, origin identity,
  permission summary, connectivity state, snapshot age after downgrade,
  and representation labels.
- Label expectations:
  `Isolated remote preview`, `Origin verified`, `Permissions`,
  `Copy rendered preview`, and `Export metadata only` when the body is
  withheld.

## Cross-class rules

- Trust class is a rendering and execution contract, not a copy label.
  Copy/export labels always name the transferred representation
  separately.
- No class may silently widen authority after a route, host, owner,
  policy, or trust change.
- Owner/origin chrome is required on the primary surface. Hover-only or
  buried inspector-only disclosure is non-conforming on trust-decision
  surfaces.
- Losing guarantees narrows the surface. The contract fails toward raw
  inspection, static snapshot, or metadata-only; it does not fail toward
  "try rendering it anyway".
- Support/export never carries active content as active. The best-case
  export is a sanitized snapshot; the fail-closed export is metadata-
  only.

## Surface defaults and downgrade posture

| Surface family | Default trust class | Required stricter mode | Default transfer posture | Required downgrade when guarantees are lost |
|---|---|---|---|---|
| `editor_content` / `review_surface` | `RawText` | standard suspicious-content mode | `copy_raw`, `copy_escaped`; metadata-only if policy blocks bytes | raw inspection first; metadata-only if bytes cannot safely leave |
| `docs_help_page` | `SanitizedRich` | standard plus raw/rendered divergence labeling | `copy_rendered`, `copy_raw`, `export_sanitized_snapshot` | sanitized static snapshot, else metadata-only |
| `rich_preview` | `SanitizedRich` by default; may elevate to `TrustedLocalActive` or `IsolatedRemoteActive` | active-content badge and owner/origin chrome | `copy_rendered`, `export_sanitized_snapshot` | static snapshot first, metadata-only if no verified snapshot remains |
| `install_review` / `publish_review` / `remote_attach_review` | `RawText` with strict trust-decision display mode | raw identifier visibility, stronger confusable treatment, owner/origin always visible | `copy_raw`, `copy_escaped`, metadata-only export summary | never render as active; metadata-only if the body itself is withheld |
| `approval_surface` | class inherited from the reviewed item, but stricter display mode is mandatory | approval scope, owner/origin, suspicious-content warnings always visible before commit | representation-labeled transfer only | static snapshot or metadata-only if the approved origin or scope drifts |
| `delete_review_surface` | class inherited from the item under review | last-visible-evidence mode | representation-labeled evidence copy; metadata-only delete summary allowed | preserve raw/escaped inspection or static snapshot until review completes; then metadata-only summary after deletion |
| `support_export_surface` | never active; `SanitizedRich` or metadata-only only | export review mode | `export_sanitized_snapshot` or `export_metadata_only` | metadata-only whenever source, origin, or redaction rules block a truthful snapshot |

## Owner / origin chrome

Every trust-sensitive surface projects the same minimum chrome fields:

- `trust_class_badge`
- `owner_identity`
- `origin_identity`
- `representation_label` on every copy or export action

Active or downgraded surfaces add:

- `capability_summary` for `TrustedLocalActive`
- `publisher_or_host` and `connectivity_state` for
  `IsolatedRemoteActive`
- `snapshot_age` whenever the visible content is a frozen snapshot
- `suspicious_content_badge` whenever a warning is attached to the
  current content
- `approval_scope` on approval surfaces
- `deletion_consequence` on delete-review surfaces

These fields are part of the contract, not decorative chrome. Compact
layouts may rearrange them but may not hide them at the commit point.

## Suspicious-content vocabulary

The suspicious-content detector library and every claiming surface use
the same closed class set.

| Suspicious-content class | Detects | Minimum reveal affordance | Default response | Suppression scope |
|---|---|---|---|---|
| `bidi_control` | reordering controls that can change how text appears relative to byte order | inline marker, code-point inspector, raw/escaped inspection | annotate at exact location; never normalize away | workspace or admin policy only |
| `invisible_formatting` | non-printing format characters that materially affect identifiers, filenames, or labels | inline marker, code-point inspector, raw/escaped inspection | annotate and preserve exact bytes until explicit fix | workspace or admin policy only |
| `mixed_script_confusable` | look-alike text built from mixed scripts | warning badge plus raw/escaped path | stronger warning on trust-decision surfaces | workspace or admin policy only |
| `whole_script_confusable` | look-alike text where a full token can be mistaken for another script | warning badge plus raw/escaped path | stronger warning on trust-decision surfaces | workspace or admin policy only |
| `raw_rendered_divergence` | rendered output that does not faithfully communicate the source representation | raw/rendered toggle and representation labels | keep raw and rendered paths visible together | not suppressible globally |

Rules:

- Warnings stay attached to the exact token, hunk, field, or summary row
  that contains the suspicious content.
- `copy_raw` and `copy_escaped` remain reachable wherever suspicious
  content is present.
- Surfaces MAY offer fixes, but the fix flow must preview the byte-level
  change before commit.
- A surface must preserve suspicious-content labels into exported packet
  summaries and delete-review summaries whenever the warning explains
  why a human would need to inspect the raw or escaped form later.

## Stronger annotation mode on trust-decision surfaces

Install, publish, attach, approval, and delete-review surfaces use a
stricter annotation mode than ordinary editing or docs preview because
the user is deciding whether to trust, approve, or remove a thing.

| Surface family | Stronger requirement |
|---|---|
| `install_review` | publisher, package, and artifact identifiers keep raw/escaped inspection available inline; signed owner context stays visible before confirm |
| `publish_review` | destination namespace, publisher identity, and any suspicious token stay visible in the primary review; generic success copy is forbidden before the review closes |
| `remote_attach_review` | host/origin identity, route, and suspicious labels remain visible across approval renewal and connectivity change |
| `approval_surface` | raw or escaped inspection path remains available until the approval is committed or denied; owner/origin and approval scope stay visible |
| `delete_review_surface` | warnings remain visible until deletion completes so the last reviewable evidence is not destroyed behind a generic delete prompt |

## Representation transfer policy

This document narrows the shell-wide representation vocabulary to the
transfer-safe actions that move bytes or summaries off a surface. The
active on-screen states `sandboxed` and `generated` remain part of the
broader shell contract, but transfers from safe-preview surfaces resolve
to one of the actions below.

| Action id | Representation class | Meaning | Required label shape |
|---|---|---|---|
| `copy_raw` | `raw` | exact source bytes or exact source text representation | `Copy raw ...` |
| `copy_rendered` | `rendered` | the current rendered view, not the underlying source bytes | `Copy rendered ...` |
| `copy_escaped` | `escaped` | source representation with metacharacters made safe to inspect or paste elsewhere | `Copy escaped ...` |
| `export_sanitized_snapshot` | `sanitized` | static snapshot with active/scriptable content removed | `Export sanitized snapshot` |
| `export_metadata_only` | `blocked_metadata_only` | metadata envelope only; raw body withheld for policy or safety | `Export metadata only` |

Rules:

- A generic `Copy` or `Export` label is forbidden where rendered,
  active, escaped, or sanitized representations differ materially.
- `copy_rendered` on a surface with `raw_rendered_divergence` MUST keep
  `copy_raw` visible on the same surface.
- `copy_escaped` is not a decorative extra. It is the safe-inspection
  path for suspicious content that would otherwise remain ambiguous in
  logs, chat, or support notes.
- `export_sanitized_snapshot` must preserve owner/origin attribution,
  trust class, and snapshot age when the source is not live anymore.
- `export_metadata_only` must say why the body is withheld and which
  trust class the source last held.

## Downgrade and loss-of-guarantee rules

The downgrade posture is explicit and typed.

- If a surface loses owner or origin identity, it narrows. Active
  classes do not stay active with unknown ownership.
- If connectivity is lost on a remote or embedded surface, the product
  shows a static snapshot with age and last-verified origin when such a
  snapshot exists; otherwise it shows metadata-only.
- If sanitization fails or can no longer be guaranteed, the product
  falls back to `RawText` or metadata-only. It never keeps a
  "sanitized" badge on unverifiable content.
- If support/export redaction or policy blocks the body, the product
  exports metadata-only even if the on-screen surface still has a richer
  local view.
- If approval scope, host route, or trust policy changes between review
  and commit, the product reopens the review in the narrowed class or
  emits metadata-only review state. Silent carry-forward is forbidden.

## Machine-readable contract

The schemas freeze two boundary layers:

- `trust_class.schema.json`
  exports `surface_trust_resolution_record` and
  `suspicious_content_case_record` so trust class, downgrade posture,
  suspicious-content findings, and stronger-annotation mode can cross
  tool boundaries without carrying raw bodies.
- `text_representation_policy.schema.json`
  exports `representation_transfer_record` so copy/export actions,
  representation labels, and metadata-only fallback remain explicit and
  mechanically reviewable.
- `text_representation_action.schema.json`
  exports `text_representation_action_record` so each surface can state
  whether `Copy raw`, `Copy rendered`, `Copy escaped`,
  `Export sanitized snapshot`, or `Export metadata only` is available,
  primary, secondary, or fallback.

The fixture corpus under
[`/fixtures/security/suspicious_content_cases/`](../../fixtures/security/suspicious_content_cases/)
anchors the vocabulary to concrete surfaces. Every fixture is reviewable
without needing a renderer implementation.
