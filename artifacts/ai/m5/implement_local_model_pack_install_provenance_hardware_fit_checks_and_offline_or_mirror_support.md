# Local Model Pack Install And Provenance

- Packet: `local-model-pack:stable:0001`
- Schema: `schemas/ai/implement-local-model-pack-install-provenance-hardware-fit-checks-and-offline-or-mirror-support.schema.json`
- Support export: `artifacts/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/support_export.json`
- Fixture: `fixtures/ai/m5/implement_local_model_pack_install_provenance_hardware_fit_checks_and_offline_or_mirror_support/`

## Coverage

The packet materializes the local-model pack catalogue into one row per pack.
Every pack carries an explicit install state plus its provenance class, source
channel, hardware-fit class, footprint tier, and accelerator requirement.

- The directly-downloaded small pack is signed, publisher-verified, and verified
  on a comfortable hardware fit; it runs on CPU only.
- The mirror pack is key-pinned and installed from a configured mirror on a
  constrained fit; offline support does not waive its signature.
- The offline-bundle large pack is signed, publisher-verified, and imported from
  a signed offline bundle; it requires a GPU and fits the device with little
  headroom.
- The sideloaded vision pack failed its signature on an air-gapped sideload, so
  it is held in quarantine, carries no evidence refs, and narrows to
  `unavailable` on stale proof.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The packet refuses to present a pack as ready when provenance or hardware fit
cannot back the claim. An active-install pack must carry verified provenance and
must actually fit the device; a pack whose signature failed must be held aside
rather than shown as installed; a claimed pack may not hide its hardware fit
behind an unverified posture; and a claimed offline or mirror pack must carry the
same verified provenance as a direct download. Every claimed pack narrows rather
than hides on stale proof, reusing the frozen M5 AI workflow matrix
qualification and downgrade vocabularies so no pack row may stay greener than its
evidence. Raw download URLs, mirror endpoints, credential bodies, raw signature
blobs, raw checksums, and exact byte sizes never cross the support boundary.
