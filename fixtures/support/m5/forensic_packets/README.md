# M5 Forensic Packet Fixtures

This fixture set snapshots the canonical support-side M5 forensic packet used by
the host-failure drill lane.

Files:

- `packet.json` — canonical packet emitted by `seeded_m5_forensic_packet()`

Key assertions:

- rows distinguish `local_only`, `imported`, `mirrored`, and `uploaded`
  artifact states;
- each row exposes a local preview before any egress path;
- uploaded and mirrored actions stay behind explicit user or policy review;
- no artifact silently widens retention or export scope.
