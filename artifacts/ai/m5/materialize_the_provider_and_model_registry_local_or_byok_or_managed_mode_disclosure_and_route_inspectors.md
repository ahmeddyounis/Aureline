# Provider Route Disclosure And Inspectors

- Packet: `provider-route-disclosure:stable:0001`
- Schema: `schemas/ai/materialize-the-provider-and-model-registry-local-or-byok-or-managed-mode-disclosure-and-route-inspectors.schema.json`
- Support export: `artifacts/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/support_export.json`
- Fixture: `fixtures/ai/m5/materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors/`

## Coverage

The packet materializes the provider/model registry into one inspector row per
claimed AI route. Every route carries an explicit mode badge — local, BYOK, or
managed — plus its locality, region, retention, cost disclosure, tool
side-effect class, and automation authority.

- Local routes (on-device explain and local companion edit) keep their bytes,
  retention, and cost fully on-device.
- BYOK routes (vendor-direct and self-hosted) call the user's endpoint with the
  user's key and disclose region, retention, and metered or flat-rate cost.
- Managed routes (enterprise gateway and first-party hosted) disclose pinned
  regions, bounded retention with operator or export access, and budget-capped
  cost.
- The held local-vision route is not a claimed lane, so it carries no evidence
  refs and narrows to `unavailable` on stale proof.
- Proof freshness SLO is 168 hours with automatic narrowing on stale proof.

## Safety

The disclosure refuses to hide cost, provider, region, retention, or automation
authority behind generic language. The declared mode must agree with the
route's locality, a local route must be fully on-device for region, retention,
and cost, and a claimed managed or BYOK route may not leave region, retention,
or cost unverified. A route whose tools mutate must carry a human apply
authority; there is no autonomous self-apply authority at all, so the disclosure
fails closed. Every claimed route narrows rather than hides on stale proof,
reusing the frozen M5 AI workflow matrix qualification and downgrade
vocabularies so no inspector row may stay greener than its evidence.
