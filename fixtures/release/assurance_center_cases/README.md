# Assurance-center cases

Worked fixtures for the assurance-center, regulated-claim packet, and
compliance-evidence handoff contract:

- `active_regulated_claim.yaml` — a regulated-environment card under an
  active `supported` posture for the air-gapped self-hosted profile,
  with runtime-observed proof inside its freshness window.
- `stale_evidence_downgrade.yaml` — a card whose runtime-observed proof
  has aged past `stale_after`, automatically narrowed to
  `evidence_stale` until the next archetype run.
- `hosted_dependency_blocker.yaml` — a card narrowed to `limited`
  because a hosted-control-plane dependency is active for the
  workflow, conflicting with the no-vendor-hosted-AI claim.
- `customer_managed_keys_claim.yaml` — a card rendering the
  customer-managed-keys posture only after the key-ownership block
  resolves to `customer_managed` and customer-provided plus runtime-
  observed proof rows back the claim.
- `evaluation_export_with_redaction.yaml` — a frozen
  `regulated_review` evaluation packet exporting the active regulated
  card with `metadata_safe_default` redaction applied.

Each case embeds an `assurance_claim_card_record` and, where
applicable, the linked `evaluation_packet_record`. Cases conform to:

- `schemas/release/assurance_claim_card.schema.json`
- `schemas/release/evaluation_packet.schema.json`

Case ids are stable and quoted by the contract document at
`docs/release/assurance_center_and_regulated_claim_contract.md`.
