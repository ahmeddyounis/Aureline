# Stable Project-Entry Acquisition Truth Packet

Status: stable packet captured

Canonical artifacts:

- Schema: `/schemas/workspace/source-locator-checkout-plan-bootstrap-result.schema.json`
- Rust projection: `crates/aureline-workspace/src/stabilize_source_locator_checkout_plan_bootstrap_result_and_queue/mod.rs`
- Replay gate: `crates/aureline-workspace/tests/stabilize_source_locator_checkout_plan_bootstrap_result_and_queue.rs`
- Fixture: `/fixtures/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue/interrupted_partial_mirror_resume_packet.json`
- Contract doc: `/docs/workspace/m4/stabilize-source-locator-checkout-plan-bootstrap-result-and-queue.md`

Coverage:

- Replays the existing repository acquisition corpus for open local, clone, import, archive, template/prebuild, resume, mirror/offline, policy-guided, and support surfaces.
- Pins an interrupted mirror-backed partial clone that remains `interrupted`, `mirrored`, and `partially_acquired` instead of flattening to generic success.
- Preserves handle-only credential posture and rejects any packet that marks `raw_secret_present`.
- Keeps bootstrap work itemized with evidence refs, reviewability, cancelability, and resumability.

Release proof index:

- `proof:source_locator_checkout_plan_bootstrap_truth`
- Claim ceiling: `manifest_entry:export_and_offboarding_support` (`beta`)
- Evidence refs: this packet, the schema, the fixture, and the replay gate.
