# Tab case fixtures

Seed corpus for multi-view buffer handles and tab dirty-state projection.

These fixtures drive focused tests that ensure:

- opening the same logical document in multiple tabs reuses one buffer authority;
- edits in one view advance the shared revision stream and surface `Modified` in all tabs; and
- saving clears the shared dirty indicator without spawning duplicate buffers.

