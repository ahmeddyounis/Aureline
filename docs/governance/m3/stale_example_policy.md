# Stale-Example And Public-Proof Policy

This policy governs beta rows that are published through docs, Help/About,
release notes, support exports, CLI/headless help, or public-proof packets.
Those rows may only promote when docs examples, proof packets, and freshness
badges agree on the same shared vocabulary.

## Scope

The policy applies to:

- protected examples listed in `artifacts/ci/m3_docs_truth_source_map.yaml`;
- marketed beta rows in `artifacts/release/m3/claim_manifest.json`;
- public-proof rows in `artifacts/milestones/m3/public_proof_index.md`;
- Help/About badge vocabulary in `artifacts/docs/help_badge_vocabulary.yaml`;
- generated reports and captures under `artifacts/docs/m3/`.

## Blocking Rules

Promotion is blocked when any of these conditions are true:

- a protected example fails schema validation or pins a token no longer present
  in the claim-manifest vocabulary;
- the stale-example capture, docs-freshness capture, or public-proof index
  capture has a failing status;
- a marketed row has no `freshness.badge_class`;
- a marketed row uses `stale`, `unverified`, or a freshness token outside the
  shared Help/About and claim-manifest vocabulary;
- a marketed row's freshness badge is wider than the public-proof row for the
  same claim family;
- a public-proof row is past its `stale_after` window or the checked-in
  downgrade matrix no longer matches the canonical public-proof packet.

Degraded proof may remain publishable only when the claim manifest narrows the
row to the same or stricter posture. The product, docs, and support surfaces
must never present a fresher badge than the proof row can support.

## Required Outputs

The docs public-proof lane must keep these outputs current:

- `artifacts/docs/m3/docs_truth_report.md`
- `artifacts/docs/m3/public_proof_parity_report.md`
- `artifacts/docs/m3/captures/m3_docs_freshness_validation_capture.json`
- `artifacts/docs/m3/captures/m3_stale_example_validation_capture.json`
- `artifacts/docs/m3/captures/m3_docs_public_proof_parity_capture.json`

The `docs_freshness` row in the public-proof index must list all of these as
current outputs so stale-example and parity failures are release-bearing, not
editorial follow-ups.

## Commands

Run the full headless gate:

```sh
bash ci/check_m3_docs_truth.sh
```

Run only the parity blocker:

```sh
python3 -m tools.ci.m3.docs_public_proof_gate --repo-root .
```

Use `--check` in CI to fail when generated reports or captures drift from the
checked-in sources.
