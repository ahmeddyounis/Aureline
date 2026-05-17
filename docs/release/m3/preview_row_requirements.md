# Runtime and ecosystem preview-row packet requirements

Runtime-heavy and ecosystem-heavy rows that Aureline still surfaces during
beta must have a packet under
`artifacts/compat/m3/preview_row_packets/`. A row without a current packet
is not silently marketable; it must be downgraded out of the claimed set.

The packet register is:

- `artifacts/compat/m3/preview_row_packets/preview_row_packets.json`
- `artifacts/compat/m3/preview_row_packets/support_export_projection.json`
- `schemas/compat/m3_preview_row_packet.schema.json`
- `fixtures/ai/m3/preview_row_inputs/`

## Required lanes

Every active row must carry explicit states for:

- notebook trust and structured round-trip risk, even when the row has no
  notebook subject;
- repair posture and rollback/narrowing behavior;
- install-review posture, including native review when a package,
  extension, workflow bundle, toolchain, or imported component can mutate
  the workspace or host;
- compatibility-label truth from the generated compatibility report or
  archetype scorecard; and
- activation-budget truth for runtime, extension, helper, language-host,
  preview-route, or diagnostics work.

For active rows, either compatibility-label truth or activation-budget
truth must be current. If neither is current, the row must be downgraded
out of the claimed beta set before it appears in Help, release notes,
support export, marketplace discovery, review workspace, AI context, or
CLI/headless inspection.

## Notebook handling

Notebook-first data workflow parity is held, not a beta claim. The packet
register includes that held row so support and docs do not infer notebook
trust from the Python service/data-app row. The Python row is only
notebook-adjacent: source review, structured round-trip preview, and
explicit kernel attach are allowed; notebook-first parity remains out of
scope until its own trust, repair, install-review, compatibility, and
activation-budget packets are current.

## Consumer rule

The support-export projection quotes the packet register directly. It
does not export raw notebooks, package payloads, benchmark bodies, private
ranking weights, credentials, or ambient authority. Product, docs/help,
CLI/headless, marketplace, review, and AI surfaces must either quote this
same packet or render the row as unavailable/downgraded.

## Validation

Run:

```bash
python3 ci/check_m3_preview_row_packets.py --repo-root .
```

CI should use:

```bash
python3 ci/check_m3_preview_row_packets.py --repo-root . --check
```

The validator cross-checks the packet rows against
`artifacts/release/m3/claim_manifest.json`, allows held rows only when
they are listed in `artifacts/milestones/m3/claimed_surface_register.json`,
and verifies that the support-export projection has no row-state drift.

