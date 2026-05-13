# Scope counts alpha fixtures

These fixtures prove the alpha search result surface keeps scope counts and
empty states distinct:

- visible rows are not treated as loaded rows;
- loaded rows are not treated as all matching rows;
- rows hidden by the current workset, sparse slice, or policy view stay
  inspectable; and
- empty states distinguish no match, no match in the selected workset,
  missing excluded-root indexes, and trust/policy blocking.

The same count packet is also attached to a graph/AI candidate-shaped row so
cross-repo scope markers keep repo identity, freshness, and outside-scope state.
