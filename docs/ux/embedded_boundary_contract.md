# Embedded docs/help boundary contract

This page is the reviewer-friendly entry point for the **embedded docs/help** boundary chrome and the governed **Open in browser** handoff path.

The boundary chrome exists to keep embedded web-like content honest about **owner**, **origin**, **state**, and **authority**. Embedded panes must not blur into product-owned native approval surfaces.

## Canonical contracts

- Render-side boundary card contract: `docs/ux/embedded_surface_boundary_cards.md`
- Boundary card schema: `schemas/ux/embedded_boundary_card.schema.json`
- Browser handoff packet schema: `schemas/integration/browser_handoff_packet.schema.json`
- Owner/origin review rules: `artifacts/ux/owner_origin_chrome_review.yaml`
- Shell baseline fixture (docs/help): `fixtures/ux/embedded_surfaces/docs_help_boundary_contract_card.json`
- Boundary-case corpus (docs/help): `fixtures/ux/embedded_boundary_cases/docs_help_live_verified_card.yaml`

## Live shell walkthrough

1. Run the desktop shell:

   `cargo run -p aureline-shell --bin aureline_shell`

2. Use `Tab` until focus reaches `right_inspector`.
3. Inspect the **Embedded docs/help** panel:
   - owner and publisher/service labels
   - origin label and host/domain disclosure
   - data boundary + state labels
   - source/build disclosure
   - Open in browser action + quoted handoff packet ref
4. Press `Enter` while focused on `right_inspector` to invoke `cmd:docs.open_in_browser`.
5. Confirm the system browser opens this page (this file).

## Failure drill (browser-only exit)

The embedded docs/help boundary chrome MUST make it clear that:

- high-risk approval equivalents (trust elevation, rollback/restore confirmation, AI apply review, high-risk approval sheets) remain **host-owned**, and
- leaving the embedded boundary routes through a typed `browser_handoff_packet_ref` and a product-owned handoff lane.

## Implementation notes

For baseline development and review:

- The shell seeds the docs/help boundary card by deserializing
  `fixtures/ux/embedded_surfaces/docs_help_boundary_contract_card.json` and
  stamping the running build identity + minted timestamp.
- The shell resolves the packet ref `id:browser-handoff:docs-help:project-docs` to a local
  `file://…` URL targeting this document.
Future integrations should resolve packet refs via the browser-handoff registry and audit stream, rather than mapping directly to URLs in UI code.
