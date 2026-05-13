# Language Invalidation Alpha Fixtures

These fixtures protect the launch-language incremental parse path. Each case
opens a wedge file, applies an editor edit, verifies the previous syntax tree
is reused when available, exports a metadata-only symbol snapshot, and records
a benchmark sample for ordinary typing or a large changed region.
