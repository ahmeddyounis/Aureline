# Corpus Intake Checklist

Use this checklist before adding partner, customer, field-derived, or
otherwise non-synthetic material to any corpus, fixture set, benchmark
packet, release packet, compatibility report, migration suite,
conformance pack, or support scenario.

## Intake Record

Every intake record must name:

- stable intake id
- proposed corpus asset id
- source class: `synthetic`, `public_sanitised`, `partner_customer_derived`, `design_partner_derived`, or `field_derived`
- corpus owner, evidence owner, privacy reviewer, legal reviewer, and release owner
- intended claim rows and public surfaces
- proposed storage path and retention class
- whether raw source bytes, generated metadata, screenshots, logs, traces, or support packets are included

## Required Review

Before CI or public proof admission, the intake record must show:

- redaction decision: approved synthetic summary, approved redacted sample, or rejected
- license decision: repo-owned, compatible public license, written permission, or rejected
- retention decision: permanent seed, milestone-scoped seed, derived regenerable, or rejected
- access decision: public CI, internal CI only, manual-review only, or rejected
- export posture: safe for public proof, support-only metadata, or not exportable
- approval date and reviewer refs

## Admission Gate

The sample can join CI or a release packet only when:

- `approved_for_ci` is true
- `approved_for_public_proof` is true for public-proof use
- `clearance_state` is `cleared_for_ci_and_public_proof`
- every raw private byte is either absent, tokenized, or explicitly excluded by the redaction report
- retention and destruction expectations are recorded
- the intended claim impact is listed in `fixtures/registry/corpus_registry.yaml`

If any item is missing, the sample stays manual-review only and must not
be referenced by benchmark, compatibility, migration, supportability, or
public-proof claim bindings.
