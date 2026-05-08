# Reproducibility packet seed

This document defines the minimal “reproducibility packet” that future release
and support workflows can rely on without hand-assembled notes.

The packet is intentionally a seed: it captures the build identity, pinned
inputs, artifact digests, and rebuild instructions required to explain and
repeat a build.

## Canonical sources

- Baseline build identity contract: `docs/build/reproducible_build_baseline.md`
- Exact-build identity vocabulary: `docs/build/exact_build_identity_model.md`
- Clean-room rebuild lane (command + output shape): `docs/build/cleanroom_rebuild_lane.md`

## Generate a packet

Generate a self-contained reproducibility packet directory:

```sh
./ci/cleanroom_rebuild.sh --out-dir target/cleanroom-rebuild
```

Offline variant:

```sh
./ci/cleanroom_rebuild.sh --offline --out-dir target/cleanroom-rebuild
```

## Expected contents

The output directory is the packet. At minimum it should contain:

- `build_identity.json` — baseline build identity record (schema:
  `schemas/build/build_identity.schema.json`)
- `cleanroom_input_manifest.json` — pinned inputs, commands, and trust assumptions
- `artifact_digests.json` — digest manifest for built outputs in the lane
- `provenance_capture.json` — capture summary tying the build identity to artifact families

Consumers (support exports, crash artifacts, provenance stubs) should join back
to this packet using the same exact-build identity ref vocabulary and the
baseline build identity axes instead of carrying surface-local version strings.
