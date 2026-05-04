# Worked artifact-class examples

Cross-class fixtures for
[`/docs/governance/artifact_hierarchy_and_packet_classes.md`](../../../docs/governance/artifact_hierarchy_and_packet_classes.md)
and the registry rows in
[`/artifacts/governance/packet_class_registry.yaml`](../../../artifacts/governance/packet_class_registry.yaml).

Each fixture walks one launch-bearing surface end-to-end across the
canonical artifact classes — requirement register, ADR, contract
packet, verification corpus, compatibility report when applicable,
claim manifest, runbook/shiproom packet, and public-proof bundle —
demonstrating that downstream steps cite upstream ids verbatim rather
than minting parallel slugs.

Cases:

- `shell_startup_latency_release_chain.yaml` — PERF-SHELL-001
  shell-startup latency bar walked through the renderer ADR, surface
  contract packet, benchmark publication pack, claim row, shiproom
  packet, and public-proof bundle.
- `stable_settings_envelope_release_chain.yaml` — ARCH-SVC-002
  protected-service versioned contract walked through the RPC
  transport and subscription envelope ADRs, the WIT host-world surface
  contract packet, the compatibility report, the verification packet,
  the claim row, the migration pack, the shiproom packet, and the
  public-proof bundle.

Fixtures are descriptive, not prescriptive: they reuse canonical ids
already present in the seeded registers and packets, and they do not
introduce new requirement ids, ADR ordinals, surface ids, or claim row
ids.
