# Local-model pack install, provenance, hardware-fit, and offline/mirror support

This contract materializes the local-model pack catalogue into one export-safe
truth packet whose unit of truth is a pack row. Shell, docs, support export, and
release tooling consume the packet directly instead of re-describing install or
provenance state by hand.

- Packet type: `aureline_ai::LocalModelPackInstallPacket`
- Schema: [`schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json`](../../../schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json)
- Support export: [`artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/support_export.json`](../../../artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/support_export.json)
- Fixtures: [`fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/`](../../../fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/)

## The pack row

Each `LocalModelPackRow` binds, for one local model pack:

| Field | Meaning |
| --- | --- |
| `publisher_id`, `model_id`, `pack_version` | Identity tokens (never a raw URL). |
| `claimed_qualification` | Stable, Beta, Preview, Experimental, Held, or Unavailable. |
| `install_state` | Not installed, acquiring, installed, verified, quarantined, install-failed, or removed. |
| `provenance` | Signature/provenance posture for the pack's bytes. |
| `source_channel` | Direct download, configured mirror, offline bundle, air-gapped sideload, or local cache. |
| `hardware_fit` | Result of checking the pack against the device. |
| `footprint_tier` | Disclosed on-device footprint tier (tiny … extra-large). |
| `accelerator` | Accelerator the pack requires or can use. |
| `provenance_label` | The review-safe label shown alongside the install state. |
| `downgrade_rules` | Closed set of triggers that narrow the claim. |
| `evidence_packet_refs` | Evidence backing a claimed pack. |

## Invariants enforced by validation

- **Provenance before install.** An active-install pack (`installed` or
  `verified`) must carry verified provenance — `signed_publisher_verified` or
  `signed_key_pinned`. An unsigned or unverified pack can never reach an active
  install state.
- **Failed provenance is held aside.** A pack whose provenance is
  `signature_mismatch` or `policy_blocked` must be in a held-aside state
  (`not_installed`, `quarantined`, `install_failed`, or `removed`); it may never
  be shown as installed.
- **Hardware fit before install.** An active-install pack must actually fit the
  device (`fits_comfortably` or `fits_constrained`); a pack that does not fit may
  not be presented as installed.
- **No hidden hardware fit.** A claimed pack may not leave its hardware fit
  `unknown_unverified`.
- **Offline is not a signing waiver.** A claimed pack on a mirror, offline
  bundle, air-gapped sideload, or local-cache channel must still carry verified
  provenance — the same bar as a direct vendor download.
- **Claimed packs carry evidence.** Stable, Beta, and Preview packs must list at
  least one evidence packet ref.
- **Narrow, never hide.** Every pack carries a `proof_stale` downgrade rule and
  every downgrade rule must narrow strictly below the claimed qualification.

## Provenance and freshness

`source_contract_refs` must include this schema, this doc, the provider/model
registry schema, and the frozen M5 AI workflow matrix schema whose qualification
and downgrade vocabularies the packet reuses. The `proof_freshness` block records
the freshness SLO and asserts that stale proof automatically narrows claimed
packs. Reading the checked export through
`current_local_model_pack_install_export` re-validates every invariant, so a
stale or malformed artifact fails the consuming surface rather than shipping an
optimistic claim.

## Boundary

The packet carries no download URLs, mirror endpoints, credential bodies, raw
signature blobs, raw checksums, or exact byte sizes. Validation rejects obvious
credential material and raw URLs in the serialized export.
