# Prebuild Descriptor Alpha Fixtures

This fixture lane protects the warm-start descriptor seed consumed by Start
Center and launch-bundle review. The manifest fixes the descriptor ids and
acceptance states that must remain present:

- local warm-start metadata with source, freshness, and target class;
- stale prebuild metadata that rejects reuse with an explicit drift reason;
- managed resume metadata that requires reauth before any resume claim.

Run:

```sh
python3 ci/check_prebuild_descriptor_alpha.py --repo-root . --render-warm-start-gallery
```
