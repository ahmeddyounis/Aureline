# Browser preview runtime + HMR + source mapping (reference scenario)

## Covers acceptance rows

- `ts_js_acceptance_row:browser_preview_and_hmr`

## Binding

- Launch bundle: `launch_bundle:typescript_web_app.seed`
- Archetype row: `archetype_row:ts_web_app_or_service`
- Framework packs (in-scope for this scenario):
  - `framework_pack:typescript_web.vite`
  - `framework_pack:typescript_web.next_js`

## Scenario goal

Prove that preview/dev-server workflows are:

- attributable (origin/target/route truth is explicit);
- safe (no silent widening from local-only to shareable routes);
- honest (system-browser handoff is explicit when required); and
- debuggable (source maps map runtime stack frames back to TS source
  where the workflow claims it).

## Required truth and disclosures

- Remote attach and forwarded endpoints share one route-truth contract;
  preview routes cite forwarded endpoints by opaque ref:
  - `docs/remote/attach_tunnel_port_forward_contract.md`
  - `docs/runtime/browser_runtime_contract.md`
  - `schemas/runtime/preview_route.schema.json`
- When a workflow requires opening a system browser, it must do so through
  a reviewable handoff envelope (no silent external-open):
  - `schemas/integration/browser_handoff_packet.schema.json`

## Benchmark/workflow reservations (must be materialised before certification)

- `workflow.ts_js_browser_preview_and_hmr`

## Evidence hooks

- Attach/tunnel/forward fixtures and downgrade cases:
  `fixtures/remote/attach_cases/`

## Known-limit expectations

- Any narrowing (for example “remote shareable preview not supported for
  air-gapped profiles”) must be captured as a known-limit note, not buried
  in docs prose.

