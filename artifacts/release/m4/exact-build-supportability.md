# Exact-build supportability packet

Reviewer-facing proof packet for the clean-room rebuild, exact-build symbolication,
release-center parity, and mirror/offline publication coherence register.

Canonical machine source:

- Artifact:
  [`/artifacts/release/m4/clean-room-rebuild-proof.json`](./clean-room-rebuild-proof.json)
- Companion doc:
  [`/docs/m4/clean-room-rebuild-and-symbolication.md`](../../../docs/m4/clean-room-rebuild-and-symbolication.md)
- Typed consumer:
  `aureline_release::prove_clean_room_rebuild_exact_build_symbolication_release_center_parity_and_mirror_offline_publication_coherence`

## Packet identity

- `packet_ref`: `artifacts/release/m4/exact-build-supportability.md`
- `proof_index_ref`: `artifacts/release/stable_proof_index.json#proof:clean_room_rebuild`
- `build_identity_ref`: `build-id:aureline:stable:2026-06-02`
- Captured: `2026-06-02`

## Scope

This packet backs all fifteen rows in the checked-in proof artifact:

- package channels: stable desktop, stable CLI, stable remote agent, preview desktop,
  portable, managed-install, mirror/offline
- exact-build symbolication rows: stable crash symbols, stable source maps, preview crash symbols
- parity surfaces: release center, About/Help, rollback metadata, advisory publication,
  mirror/offline publication pack

## Rebuild verification evidence

- toolchain pins keep the rebuild lane on the exact compiler, linker, signing, and
  packaging inputs used for the promoted build identity
- deterministic build flags keep paths, timestamps, and archive ordering stable across
  clean-room rebuild attempts
- binary diff analysis compares the rebuilt desktop, CLI, and remote-agent outputs against
  the promoted release payloads and confirms exact-build identity for the primary stable rows
- the portable and managed-install lanes still run as rehearsal captures; the portable row
  remains narrowed until its rebuild is verified, while managed-install holds on a time-bounded waiver

## Symbolication evidence

- symbol-server upload confirmations exist for the stable crash-symbol and preview crash-symbol rows
- source-map linkage confirms the stable source maps resolve back to the promoted build identity
- release support exports can therefore map crash evidence to the exact build without ad hoc reconstruction

## Packet freshness SLO

- freshness target: 14 days maximum age
- warn window: 3 days remaining
- SLO register: `docs/m4/clean-room-rebuild-and-symbolication.md#packet-freshness-slo`

## Status

- current: stable desktop, stable CLI, stable remote agent, preview desktop, stable crash symbols,
  stable source maps, preview crash symbols, release center parity, About/Help parity,
  rollback metadata parity, advisory publication parity
- current on waiver: managed-install package channel
- rehearsal-backed and narrowed: portable package channel
- stale: mirror/offline package channel and mirror/offline publication pack

## Reviewer conclusion

Primary stable package rows, exact-build symbolication rows, and release-center parity rows are
all current. Publication stays on hold only because the mirror/offline packet is stale and the
portable rebuild remains a rehearsal instead of a verified clean-room rebuild.
