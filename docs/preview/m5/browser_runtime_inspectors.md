# M5 browser-runtime inspectors

This document is the contract for the M5 DOM/CSS/console/network/storage
browser-runtime inspectors. It binds the **five inspector lanes** onto a single
shared packet so the runtime target, attach depth, source-mapping quality,
session freshness, and data posture of every inspector stop hiding inside
provider-specific extension chrome.

Where the
[source-first preview / browser-runtime inspection matrix](freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix.md)
freezes the *qualification* of each claimed preview/runtime surface, the
[preview-session descriptors](preview_session_descriptors.md) materialize the
*per-session* state each surface presents, and the
[inspect-to-source tree mapping](inspect_to_source_tree_mapping.md) materializes
the *per-node* source-mapping truth, this packet materializes the *per-inspector*
truth behind every claimed browser-runtime inspection surface.

Source remains canonical; the inspector packet is derivative — never a second
writable truth model. Every inspector names its runtime target, attach depth,
mapping quality, freshness, and redaction posture before any value body, jump, or
mutation affordance appears, and a runtime-only or captured-snapshot row never
masquerades as saved source state.

## Source of truth

- Packet type: `BrowserRuntimeInspectorPacket`
  (`crates/aureline-preview/src/browser_runtime_inspectors/`).
- Boundary schema:
  `schemas/preview/browser_runtime_inspectors.schema.json`.
- Checked support export:
  `artifacts/preview/m5/browser_runtime_inspectors/support_export.json`.
- Markdown summary:
  `artifacts/preview/m5/browser_runtime_inspectors.md`.
- Protected fixtures:
  `fixtures/preview/m5/browser_runtime_inspectors/`.
- Conformance dump: `cargo run -p aureline-preview --example dump_m5_browser_runtime_inspectors [support|summary]`.

## Inspector lanes

Each inspector lane carries at least one row: `dom`, `css`, `console`, `network`,
and `storage`. The console, network, and storage lanes surface sensitive value
bodies; the DOM and CSS lanes surface structure and computed-style metadata.

## Target-kind vocabulary

Every claimed browser-runtime surface names its target through one vocabulary, so
the chrome can never blur an embedded preview into a product-native browser or a
captured snapshot into a live runtime:

| Target kind | Meaning |
| --- | --- |
| `embedded_preview` | An embedded preview rendered inside the shell or extension host |
| `external_browser` | An external/system browser process driven over a transport |
| `simulator_or_emulator` | An OS-vendor simulator or emulator runtime |
| `device_browser` | A browser running on a tethered physical device |
| `remote_preview_session` | A preview session on a remote / container / managed runtime |
| `captured_snapshot` | A captured snapshot with no live runtime behind it |

A captured-snapshot target is the one and only target whose freshness is
`captured_snapshot`; every other target is a live runtime.

## Attach depth, mapping quality, and freshness

- **Attach depth** (`no_attach` → `dom_only` → `dom_and_styles` →
  `dom_styles_network` → `dom_styles_network_storage`) must actually reach the
  inspector lane it backs: a storage inspector needs `dom_styles_network_storage`,
  a network inspector needs at least `dom_styles_network`, a CSS inspector needs
  at least `dom_and_styles`, and a DOM or console inspector needs at least a live
  `dom_only` attach. A shallow attach never advertises a deeper inspector by
  silence.
- **Mapping quality** (`exact` / `approximate` / `generated_only` / `runtime_only`)
  names how good the source mapping behind the inspected element is. Only an
  `exact` or `approximate` (source-backed) row may claim saved source state.
- **Freshness** (`live` / `reconnected` / `stale` / `captured_snapshot`) names how
  fresh the session is right now and must be consistent with the continuity.

## Redaction-safe value handling

Console, network, and storage inspectors must carry a redaction-safe posture so
cookies, tokens, storage entries, and request/response bodies never leak into
generic diagnostics or support exports by default:

| Posture | Meaning | Allowed on sensitive lane |
| --- | --- | --- |
| `redacted_by_default` | Sensitive values redacted; only typed metadata crosses | yes |
| `metadata_only` | Only counts / keys / types cross; no values | yes |
| `hashed_reference` | Values replaced by opaque hashes or refs | yes |
| `non_sensitive_passthrough` | Inspector exposes no sensitive value class | no (DOM/CSS only) |

## Session continuity and attributability

Session identity is threaded into reconnect, imported-snapshot, and stale-session
continuity so browser-runtime history stays attributable:

- `fresh_attach` carries no prior session ref; every other continuity carries a
  `prior_session_ref` so history is attributable.
- `reconnected` re-pins a prior session without forcing a downgrade.
- `imported_snapshot` and `stale_session` are degraded states: each records a
  `downgrade_trigger` and a precise, non-generic `degraded_label`, and never
  silently re-upgrades. A row with no trigger carries no degraded label.

## Mutation safety

A mutation-capable browser-runtime action cannot appear without an explicit
side-effect class, a target identity, and a review/confirmation posture:

- The `side_effect_class` (`dom_mutation` / `style_override` / `storage_write` /
  `network_replay` / `console_eval`) must match the inspector lane.
- The row's `target_identity_ref` names what the mutation acts on.
- The `review_posture` (`confirmation_required` / `review_required` /
  `blocked_needs_elevation`) is always required.
- A mutation may never target a `captured_snapshot`.

`BrowserRuntimeInspectorPacket::validate` rejects a packet that:

- omits a required inspector lane, target kind, or mapping-quality class, or
  demonstrates no mutation-capable row or continuity-preserving downgrade row;
- lets an attach depth fall short of its inspector lane;
- lets freshness disagree with continuity, or a target kind disagree with
  freshness on liveness;
- declares a prior-session ref presence inconsistent with the continuity;
- lets a sensitive inspector lane expose values without a redaction-safe posture;
- lets a `runtime_only` row claim saved source state, or any non-source-backed row
  claim saved source state;
- offers a mutation with a mismatched side-effect class, a missing target
  identity, or against a captured snapshot;
- lets a row carry a downgrade trigger without a precise label, or a precise label
  without a trigger;
- carries a row without evidence;
- fails any guardrail or consumer-projection invariant; or
- carries raw boundary material in the export.

## Guardrails

- Source remains canonical; the inspector packet is derivative — never a second
  writable truth model.
- Runtime state never hides source-mapping uncertainty behind an inspector label.
- Inspect-only rows are never auto-upgraded into write-capable designer flows.
- Embedded preview/browser boundaries are not blurred into product authority.
- Sensitive console / network / storage values are redacted by default.
- A mutation-capable action requires a side-effect class, target identity, and a
  review/confirmation posture.
- Session identity stays attributable across reconnect, imported snapshot, and
  stale-session continuity.

## Consumers

Product, docs/help, diagnostics, support export, and release-control surfaces
ingest these inspector rows directly instead of cloning inspector terminology by
hand, and support / diagnostics exports can reconstruct exactly what redaction
posture the user saw for each inspector.
