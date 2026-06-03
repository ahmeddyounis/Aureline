# Clean-room rebuild and exact-build symbolication

This document explains how Aureline records clean-room rebuild verification,
exact-build symbolication, release-center parity, and mirror/offline publication
coherence in one canonical proof artifact.

## What clean-room rebuild verification means

A clean-room rebuild verification proves that a promoted package can be rebuilt from
its pinned toolchain and governed inputs without relying on the original build host.
For Aureline, that means the desktop, CLI, and remote-agent package rows must show
that the reproduced outputs still match the promoted build identity.

## How exact-build symbolication is confirmed

Exact-build symbolication is confirmed when the crash-symbol archives and source maps
uploaded for a release still link back to that release's immutable build identity.
The stable crash-symbol and stable source-map rows therefore stay current only when
support tooling can resolve crash evidence without guesswork.

## What release-center parity checks cover

Release-center parity checks cover every public truth surface that repeats release
identity or supportability state: the release center itself, About/Help, rollback
metadata, advisory publication, and the mirror/offline publication pack. These rows
prove that public-facing metadata is not wider or different from the promoted build.

## Mirror/offline publication coherence requirements

Mirror and offline publication coherence requires two things:

1. the mirror/offline package channel must still be backed by a fresh packet proving
   the same build identity and packaging truth as the primary channel rows
2. the mirror/offline publication pack must present parity metadata that matches the
   release center and support surfaces for that same build

If either packet goes stale, the claim narrows automatically and publication holds.

## Coverage summary

| Category | Rows | Current or on waiver | Narrowed |
|---|---|---:|---:|
| Package channels | stable desktop, stable CLI, stable remote agent, preview desktop, portable, managed-install, mirror/offline | 5 | 2 |
| Symbolication | stable crash symbols, stable source maps, preview crash symbols | 3 | 0 |
| Parity surfaces | release center, About/Help, rollback metadata, advisory publication, mirror/offline publication pack | 4 | 1 |

At the checked-in snapshot the artifact covers fifteen rows total: twelve rows hold
their claim, three are narrowed, and one of the holding rows is covered by an active waiver.

## Packet freshness SLO policy

### Packet freshness SLO

- target maximum age: 14 days
- warn window: 3 days
- stale behavior: once the packet breaches the target age, the affected row must narrow
  and any blocking rule tied to that reason may hold publication

## How to refresh a stale packet

1. recapture the mirror/offline evidence against the current promoted build identity
2. update `artifacts/release/m4/exact-build-supportability.md` with the new capture details
3. refresh the affected `proof_packet.captured_at` values in
   `artifacts/release/m4/clean-room-rebuild-proof.json`
4. rerun the Rust validation tests and the CI check script

## Canonical proof artifact

- Artifact: [`/artifacts/release/m4/clean-room-rebuild-proof.json`](../../artifacts/release/m4/clean-room-rebuild-proof.json)
- Reviewer packet: [`/artifacts/release/m4/exact-build-supportability.md`](../../artifacts/release/m4/exact-build-supportability.md)
