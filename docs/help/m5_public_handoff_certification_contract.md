# M5 public-handoff & capture-boundary certification contract

This lane (`M05-723`) is the capstone certification for the M5 post-install
notice/provenance, official-versus-community handoff, redaction-safe
reproduction-packet, and device-permission/webview/auth-boundary surfaces. It maps
every governed object frozen by the
[public-handoff / capture-boundary matrix](m5_public_handoff_matrix_contract.md) to
**current proof** for disclosure freshness, boundary honesty, and redaction
readiness, and it **auto-narrows** any object whose proof is stale, whose boundary
drifted, or whose redaction is unsafe before that object can keep a Stable public
claim.

The certification is minted in code by
`crate::m5_public_handoff_certification` and emitted by the headless inspector
`aureline_shell_m5_public_handoff_certification`. Release, help, support, and
public-truth automation consume **one** certification result rather than restating
handoff/capture posture by hand.

## Records

- **Certification packet** (`PublicHandoffCertificationPacket`) — one
  `HandoffCertificationRow` per governed object, each carrying its certified
  surface, proof packet refs, disclosure freshness, boundary-honesty and
  redaction-readiness posture, consumer surfaces, any active waiver, the derived
  green/yellow/red status, and the exact stale-proof causes. Aggregated counts,
  active waivers, stale-proof causes, and blocking findings are recomputed from the
  rows so the auto-narrowing is never asserted.
- **Boundary-truth dashboard** (`HandoffTruthDashboard`) — the light projection
  release / public-truth automation reads to auto-narrow claimed surfaces.
- **Support export** (`PublicHandoffCertificationSupportExport`) — the packet and
  dashboard quoted in full with stable case ids.

## Governed objects

The certification covers exactly these eight object kinds, and refuses to ship if
any is missing:

- `post_install_notice`
- `provenance_disclosure`
- `community_handoff_route`
- `reproduction_packet`
- `offline_capture_continuity`
- `device_permission_boundary`
- `embedded_auth_boundary`
- `service_health_notice`

## Derived status (auto-narrowing)

The row status is derived, never asserted:

- **red** (blocked) if the row hides a native-chrome impersonation
  (`undisclosed_impersonation`), would let raw sensitive material leave
  (`unsafe_material`), claims Stable on unverified proof, claims Stable with no
  backing proof, or claims Stable on stale proof with no active waiver.
- **yellow** (disclosed narrowing) if the object is frozen below Stable, runs on a
  disclosed cache/warming/waivered-stale/unverified posture, discloses a boundary
  gap, or carries a partial redaction posture.
- **green** otherwise.

A disclosed boundary gap, or a Stable-qualified object running on stale proof, may
only stay yellow (rather than red) when an active waiver discloses it. A narrowed or
blocked row must disclose a reason.

## Artifacts

| Path | Record |
| ---- | ------ |
| `artifacts/help/m5-public-handoff-certification.md` | Markdown certification report. |
| `artifacts/release/m5-public-handoff-proof/packet.json` | Full certification packet. |
| `artifacts/release/m5-public-handoff-proof/dashboard.json` | Boundary-truth dashboard. |
| `artifacts/release/m5-public-handoff-proof/support_export.json` | Support-export wrapper. |
| `fixtures/help/m5-public-handoff-certification/packet.json` | Protected packet fixture. |
| `fixtures/help/m5-public-handoff-certification/dashboard.json` | Protected dashboard fixture. |
| `fixtures/help/m5-public-handoff-certification/support_export.json` | Protected support-export fixture. |
| `fixtures/help/m5-public-handoff-certification/compact.txt` | Deterministic compact lines. |
| `schemas/help/m5-public-handoff-certification.schema.json` | Boundary schema. |

## Verify

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_certification -- validate
cargo test -p aureline-shell --test m5_public_handoff_certification_fixtures
cargo test -p aureline-shell m5_public_handoff_certification
```

The Rust `validate_public_handoff_certification_packet` is the authoritative gate.
Do not hand-edit the artifacts or fixtures; regenerate them with the subcommands
above (`packet`, `dashboard`, `support-export`, `markdown`, `compact`).
