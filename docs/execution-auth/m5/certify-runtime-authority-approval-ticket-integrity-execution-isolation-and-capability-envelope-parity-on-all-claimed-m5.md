# Certify runtime-authority parity on all claimed M5 executing profiles

This document is the canonical contract for the M5 **runtime-authority parity
certification** packet: the export-safe verdict that joins the four per-dimension
authority proofs for every claimed M5 executing surface into one inspectable
result and **auto-narrows any surface that lacks current proof**. The frozen
runtime-authority matrix states *what authority each surface claims*; the
surface-resolution, approval-ticket, capability-envelope, child-derivation,
authority-lifecycle, and launch-inspector contracts each prove *one dimension* of
that claim. This packet certifies that those proofs are present, current, and
parity-consistent before a surface keeps its claim, so execution isolation,
approval issuance, capability envelopes, and secret projection behave like one
product contract instead of per-surface folklore. Desktop, command, policy,
CLI/headless, help/About, diagnostics, support-export, and release surfaces
ingest one certification object instead of cloning per-surface authority status
prose.

- Implementation: `crates/aureline-policy/src/certify_runtime_authority_approval_ticket_integrity_execution_isolation_and_capability_envelope_parity_on_all_claimed_m5/`
- Boundary schema: `schemas/execution-auth/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.schema.json`
- Support export (truth source): `artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/support_export.json`
- Markdown summary: `artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md`
- Fixtures: `fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5/`
- Producer / validator: `cargo run -p aureline-policy --example dump_m5_runtime_authority_certification`

## Track invariant

No ambient privilege. No certified surface confers ambient machine authority, and
no AI tool, recipe, extension, browser route, or remote helper self-issues
authority: the certification reflects only the externally issued lineage proven by
the upstream contracts. Secret references are handle-only; raw secret material,
credential bodies, and live ticket signatures never cross the certification
boundary. When a dimension cannot be proven on a claimed surface, the surface
**narrows or fails closed** — recording the downgrade trigger, the narrowed
fallback, and a concrete recovery action — instead of silently widening. The
effective qualification is never wider than the claimed qualification.

## Parity dimensions

Each claimed surface is certified against four parity dimensions, every one bound
to the upstream contract that proves it:

| Dimension | Token | Proven by |
| --- | --- | --- |
| Execution isolation | `execution_isolation` | execution-surface-resolution sandbox descriptors |
| Approval-ticket integrity | `approval_ticket_integrity` | approval-ticket issuance / replay / expiry ledger |
| Capability-envelope parity | `capability_envelope_parity` | capability-envelope packets |
| Runtime-authority lineage | `runtime_authority_lineage` | frozen runtime-authority matrix |

Each dimension carries a proof status (`current`, `stale`, `missing`, or
`unsupported_backend`), the source artifact and schema refs, the stable packet id
it certifies, the timestamp the proof was observed, and an export-safe note.

## Auto-narrowing gate

The entry verdict is the **worst** proof status across the four dimensions:

| Worst status | Verdict | Effective qualification | Narrowed fallback | Downgrade trigger |
| --- | --- | --- | --- | --- |
| all `current` | `certified` | unchanged (matrix claim) | — | — |
| `stale` | `narrowed_stale_proof` | `preview` | `narrow_to_sanitized_preview` | `upstream_dependency_narrowed` |
| `missing` | `narrowed_missing_proof` | `held` | `require_fresh_ticket` | `upstream_dependency_narrowed` |
| `unsupported_backend` | `failed_closed_unsupported_backend` | `unavailable` | `fail_closed_block` | `enforcement_backend_missing` |

A single non-current dimension is enough to strip a surface of its full claim.
The narrowed qualification is always strictly more restricted than the claim, so a
missing, stale, or unsupported proof can never silently widen a claim. An
unsupported enforcement backend always fails the surface closed rather than
running it unconfined.

## Validation invariants

`M5RuntimeAuthorityCertificationPacket::validate` rejects, among others:

- a surface marked `certified` while any dimension proof is not `current`
  (`certified_surface_carries_unproven_proof`) or a verdict that does not match
  the folded proof statuses (`verdict_inconsistent_with_proofs`) — the
  silent-widening regressions;
- a narrowed entry whose effective qualification is wider than or equal to its
  claim (`narrowed_qualification_widened`) or off the verdict's floor
  (`narrowed_qualification_off_floor`);
- an `unsupported_backend` verdict that does not fail closed
  (`unsupported_backend_not_fail_closed`);
- a narrowed entry missing its trigger, fallback, or recovery action;
- incomplete surface coverage (`surface_coverage_incomplete`), a duplicate
  surface, or an entry that does not cover all four dimensions;
- a proof binding that references the wrong source packet for its dimension
  (`proof_source_mismatch`);
- any raw boundary material in the export (`raw_boundary_material_in_export`).

## Consumers

The certification is the single source of runtime-authority parity truth.
Product, help/About, diagnostics, and release surfaces ingest the same
certification result rather than cloning per-surface authority status text;
release evidence gates promotion on it, so a surface that loses current proof is
visibly narrowed before publication instead of shipping an unproven claim.

## Regenerating

Edit the frozen builders in the module, then regenerate the checked artifacts and
fixtures:

```sh
DIR_A=artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5
DIR_F=fixtures/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification > $DIR_A/support_export.json
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- markdown > artifacts/execution-auth/m5/certify-runtime-authority-approval-ticket-integrity-execution-isolation-and-capability-envelope-parity-on-all-claimed-m5.md
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture all-certified > $DIR_F/all_surfaces_certified.json
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-missing > $DIR_F/with_missing_proof_surface.json
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-stale > $DIR_F/with_stale_proof_surface.json
cargo run -q -p aureline-policy --example dump_m5_runtime_authority_certification -- fixture with-unsupported > $DIR_F/with_unsupported_backend_surface.json
```

A crate test asserts the checked-in support export deserializes back to the
frozen in-code packet unchanged, so drift fails CI.
