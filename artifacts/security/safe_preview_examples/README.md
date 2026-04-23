# Safe-preview affordance examples

Reviewer-facing baseline examples of how the product should render
safe-preview affordances before any final UI chrome lands. Paired
with:

- [`/docs/security/suspicious_content_packet.md`](../../../docs/security/suspicious_content_packet.md)
  — the shared-detector contract these examples project.
- [`/docs/security/safe_preview_trust_classes.md`](../../../docs/security/safe_preview_trust_classes.md)
  — the trust-class and suspicious-content vocabulary used verbatim.
- [`/fixtures/security/suspicious_content_cases/`](../../../fixtures/security/suspicious_content_cases/)
  — the underlying worked fixtures. Every example joins to exactly
  one fixture by `related_case_id`.

These files do not redefine `surface_trust_resolution_record` or
`representation_transfer_record`. They freeze the user-facing
affordance baseline — primary, secondary, and disabled actions, the
reason actions are disabled, the representation label shown with
each copy/export, and the chrome fields that must stay visible —
for each detector outcome and surface family.

## Coverage

| Example | Surface family | Detector outcome | Fixture it mirrors |
|---|---|---|---|
| [`raw_text_editor_affordance.yaml`](./raw_text_editor_affordance.yaml) | `editor_content` | `sanitize` (annotation only; content stays `RawText`) | `editor_bidi_identifier.json` |
| [`sanitized_rich_docs_affordance.yaml`](./sanitized_rich_docs_affordance.yaml) | `docs_help_page` | `sanitize` | `docs_help_markdown_representation.json` |
| [`rich_preview_downgrade_affordance.yaml`](./rich_preview_downgrade_affordance.yaml) | `rich_preview` | `isolate` narrowed to `sanitize` | `rich_preview_notebook_downgrade.json` |
| [`embedded_webview_isolated_affordance.yaml`](./embedded_webview_isolated_affordance.yaml) | `rich_preview` | `isolate` | `embedded_webview_isolated_remote.json` |
| [`install_review_strict_affordance.yaml`](./install_review_strict_affordance.yaml) | `install_review` | `block` (in-product active); `RawText` strict | `install_review_confusable_publisher.json` |
| [`browser_handoff_affordance.yaml`](./browser_handoff_affordance.yaml) | `docs_help_page` | `route_to_system_browser` | `browser_handoff_blocked_execution.json` |
| [`support_export_sanitized_affordance.yaml`](./support_export_sanitized_affordance.yaml) | `support_export_surface` | `sanitize` | `support_export_sanitized_snapshot.json` |

## Rules

1. Every example carries `schema_version: 1` and a stable
   `example_id`.
2. `related_case_id` matches the `case_id` of the companion
   fixture's resolution record. A reviewer can hop from the
   affordance example to the fixture and back.
3. `detector_outcome` uses the closed vocabulary frozen in the
   packet (`allow`, `sanitize`, `isolate`, `block`,
   `route_to_system_browser`).
4. Each `transfer_action` entry names a `representation_label`
   drawn from the `representation_action_id` vocabulary and
   `body_posture` drawn from the text-representation policy
   schema. A generic `Copy` or `Export` label is non-conforming.
5. `disabled_actions` is never empty where the outcome is
   `block` or `route_to_system_browser`. The reason for disabling
   must be stated; it is never silently omitted.
6. `required_chrome` names owner/origin fields the surface MUST
   keep visible for the detector outcome to remain valid.
7. Isolated or sanitized content never carries chrome copy that
   equates it with trusted local content.
8. Raw bodies, raw paths, raw URLs, raw secrets, and raw publisher
   / host bytes never appear; opaque handles, class labels, and
   representation labels do.

## Coverage contract

This baseline set MUST keep at least one example per
`detector_outcome` value (`sanitize`, `isolate`, `block`,
`route_to_system_browser`) and at least one example per
safe-preview surface category called out in the packet:
raw text preview, sanitized rich content, rich preview with
downgrade, embedded docs/webview content, package/install
metadata, browser handoff / blocked execution, and support
export. Adding an affordance example for a new surface family or
a new detector outcome is welcome; removing a category already
covered here is a breaking change.
