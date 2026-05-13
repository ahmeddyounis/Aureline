# Remap Packets Alpha Fixtures

This fixture family protects remap and continuity behavior for drifted search
deep links, bookmarks, and navigation-history entries.

Each case carries:

- a `deep_link_remap_packet_record` with old target, new target when known,
  active scope/workset identity, confidence, evidence, and recovery actions;
- a `navigation_continuity_record` that consumes the remap packet from the
  workspace history lane; and
- expected tokens asserted by the Rust fixture tests.

The cases cover moved files, renamed symbols, cross-root recovery outside an
active workset, and an explicit missing-target failure.
