# SDK v1 manifest authoring guide: binding to a verified publisher identity

This guide is the canonical walkthrough for binding an extension to a
verified publisher identity and keeping publisher continuity across
versions. It is referenced as
`manifest_guide:publisher_identity_walkthrough:1.0.0` in the
[SDK v1 starter pack](./README.md).

## Step 1: bind to a publisher identity row

Every manifest baseline carries a non-empty `publisher_identity_ref`
plus a typed `PublisherTrustTierClass`:

- `verified_publisher` — bound to a verified publisher identity
  through a signed attestation;
- `community_publisher` — community-tier identity with reduced
  install authority;
- `organisational_publisher` — bound to an organisational identity
  under a contract;
- `unverified_publisher` — unverified identity; install surface is
  review-only until verification lands; and
- `quarantined_publisher` — quarantined; install is refused.

A manifest whose `publisher_identity_ref` is empty is denied with
`publisher_identity_required`. A manifest that pretends to be
`anonymous_publisher_class` outside of a denial-drill row is denied
with `publisher_anonymous`.

## Step 2: pin a publisher signing key

Every manifest baseline carries a non-empty `publisher_signing_key_ref`
that resolves to the signing key the registry / mirror reads to verify
publication. The starter pack does not re-emit the signing key; it
inherits the manifest baseline's binding.

## Step 3: keep publisher continuity across versions

The permission-manifest delta evaluator refuses re-consent if:

- the next manifest's `publisher_trust_tier_class` is
  `quarantined_publisher` (reason
  `refused_publisher_quarantined`),
- the next manifest's `publisher_lifecycle_state_class` is
  `retired` (reason `refused_publisher_lifecycle_retired`),
- the next manifest's `extension_lifecycle_state_class` is
  `retired` or `quarantined` (reason
  `refused_extension_lifecycle_retired`), or
- the next manifest's `manifest_origin_source_class` is
  `unknown_source_class` (reason
  `refused_origin_source_unknown`).

## Repair affordance

If the publisher identity check refuses, bind the manifest to a
verified publisher identity, refresh the signing key reference, and
rerun the publisher continuity packet validator:

```text
cargo test -p aureline-extensions manifest_baseline
cargo test -p aureline-extensions review_alpha
```

The starter-pack lane delegates publisher identity truth to the
permission-manifest beta lane; widening on opaque publisher identity is
denied closed.
