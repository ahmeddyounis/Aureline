# SDK conformance packet (beta lane)

This page is the reviewer-facing entrypoint for the SDK conformance
packet that backs claimed beta ecosystem rows. The packet binds the
extension validator-suite report, the canonical SDK v1 starter-pack
sample outcome, the lifecycle/deprecation metadata packet, a
docs-freshness sweep, and the native-runtime versus
compatibility-bridge scorecard into one checked-in record per SDK /
runtime line.

Stable-facing claims can cite this packet by id rather than pointing
only at the validator source tree. When the underlying contract,
lifecycle, or compatibility state drifts, the packet, the bridge
scorecard, and the SDK docs downgrade together.

## Governed artifacts

- **Packet schema:**
  [`schemas/extensions/sdk_conformance_packet.schema.json`](../../../schemas/extensions/sdk_conformance_packet.schema.json)
- **Scorecard schema:**
  [`schemas/extensions/bridge_compatibility_scorecard.schema.json`](../../../schemas/extensions/bridge_compatibility_scorecard.schema.json)
- **Generator CLI:**
  [`tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py`](../../../tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py)
- **Packet input fixtures:**
  [`fixtures/extensions/m3/sdk_conformance_packet/`](../../../fixtures/extensions/m3/sdk_conformance_packet/)
- **Machine-readable packet:**
  [`artifacts/extensions/m3/sdk_conformance_packet.json`](../../../artifacts/extensions/m3/sdk_conformance_packet.json)
- **Reviewer-facing packet:**
  [`artifacts/extensions/m3/sdk_conformance_packet.md`](../../../artifacts/extensions/m3/sdk_conformance_packet.md)
- **Bridge-compatibility scorecard:**
  [`artifacts/extensions/m3/bridge_compatibility_scorecard.json`](../../../artifacts/extensions/m3/bridge_compatibility_scorecard.json)

## What the packet binds

One `sdk_conformance_packet` record covers exactly one SDK / runtime
line and pins:

1. **Validator suite result.** Aggregates the
   `extension_conformance_suite_report` emitted by the validator CLI.
   The packet quotes case count, matched-expectation count, aggregate
   blocker count, and observed scenario classes; it does not invent its
   own pass/fail vocabulary.
2. **Sample-pack outcome.** Reads the canonical SDK v1 starter-pack
   fixture (Wasm component + supervised external host), summarizes
   surface counts and validation-class counts, and asserts the fixture's
   declared expectations match the underlying record.
3. **Lifecycle/deprecation metadata.** Cites the canonical lifecycle
   packet (`extension_lifecycle_metadata_packet:aureline.sdk.beta`) and
   the lifecycle validation report. A failed lifecycle validation
   refuses the conformance packet closed.
4. **Docs freshness findings.** Sweeps stable-facing SDK pages, sample
   manifests, and extension-author guides for the tokens that pin them
   to the currently claimed runtime contract (semver, lifecycle packet
   path, bridge matrix id, versioning policy ref, conformance kit
   report ref). Missing or drifted tokens emit a typed finding.
5. **Bridge-compatibility scorecard.** Projects the bridge matrix at
   `artifacts/compat/m3/bridge_matrix.yaml` into native_green,
   bridge_amber, shimmed_amber, partial_amber, and unsupported_red
   classes so release notes, marketplace surfaces, and support packets
   cannot collapse a native row and a bridge row behind one
   undifferentiated green chip.

## Decision and reason vocabulary

The packet emits a closed `decision_class`:

- `ready_for_authors` — every claimed SDK/runtime row is available in
  beta, the validator suite passes, lifecycle metadata validates,
  docs are fresh, and the bridge matrix covers native, bridge,
  shimmed, and unsupported.
- `partially_ready_preview_surfaces_only` — at least one claimed
  surface is `preview_in_beta`; the rest pass.
- `refused_inconsistent_input` — at least one non-green reason
  prevents publishing the packet as proof.

The closed `reason_class` vocabulary is:

| Reason class | Meaning |
|---|---|
| `all_claimed_surfaces_available_in_beta` | Every claimed surface is GA in beta. |
| `some_claimed_surfaces_preview_in_beta` | At least one surface is preview-only. |
| `validator_suite_failed` | Validator suite emitted a non-pass result class. |
| `sample_pack_refused` | Starter-pack record refused (e.g. missing wasm sample). |
| `lifecycle_metadata_packet_failed` | Lifecycle metadata validator failed closed. |
| `docs_freshness_drift_detected` | A required token is missing from a stable-facing doc. |
| `bridge_matrix_widens_native_claim` | A native row was scored as anything other than `native_green`. |
| `bridge_matrix_missing_required_state` | One of `native`, `bridge`, `shimmed`, or `unsupported` is absent. |
| `lifecycle_metadata_missing_required_surface` | The lifecycle packet is missing a row the SDK line depends on. |

## Bridge-compatibility scorecard semantics

The scorecard turns the bridge matrix into a per-lane row that
separates the native-runtime answer from the compatibility-bridge
answer:

| Bridge state | Scorecard class | Native check | Bridge check |
|---|---|---|---|
| `native` | `native_green` | `native_supported` | `bridge_not_applicable` |
| `bridge` | `bridge_amber` | `native_not_applicable` | `bridge_translated_with_caveats` |
| `shimmed` | `shimmed_amber` | `native_not_applicable` | `bridge_shimmed_static_only` |
| `partial` | `partial_amber` | `native_not_applicable` | `bridge_partial_subset_documented` |
| `unsupported` | `unsupported_red` | `native_unsupported` | `bridge_unsupported` |

Every non-native lane carries at least one machine-readable
non-green reason so release notes, marketplace badges, and support
packets can downgrade their claims automatically.

The bridge matrix referenced by the canonical scorecard is
`extension_bridge_matrix:m3.beta`, owned at
[`artifacts/compat/m3/bridge_matrix.yaml`](../../../artifacts/compat/m3/bridge_matrix.yaml).

The canonical packet id for the beta SDK line is
`sdk_conformance_packet:aureline.sdk.beta-1.0.0-beta.1`; the scorecard
id is `bridge_compatibility_scorecard:aureline.sdk.beta-1.0.0-beta.1`.

## CLI usage

Generate the canonical beta-line packet, scorecard, and Markdown
summary:

```text
python3 tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py \
  --repo-root . \
  generate \
  --fixture fixtures/extensions/m3/sdk_conformance_packet/ready_for_authors_beta_line.json \
  --packet-json artifacts/extensions/m3/sdk_conformance_packet.json \
  --packet-md   artifacts/extensions/m3/sdk_conformance_packet.md \
  --scorecard-json artifacts/extensions/m3/bridge_compatibility_scorecard.json
```

Verify the committed artifacts match the generator output (the CI
form):

```text
python3 tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py \
  --repo-root . \
  generate \
  --fixture fixtures/extensions/m3/sdk_conformance_packet/ready_for_authors_beta_line.json \
  --packet-json artifacts/extensions/m3/sdk_conformance_packet.json \
  --packet-md   artifacts/extensions/m3/sdk_conformance_packet.md \
  --scorecard-json artifacts/extensions/m3/bridge_compatibility_scorecard.json \
  --check
```

Re-run any fixture to inspect a different outcome:

```text
python3 tools/extensions/m3/sdk_conformance_packet/aureline_sdk_conformance_packet.py \
  --repo-root . \
  generate \
  --fixture fixtures/extensions/m3/sdk_conformance_packet/partially_ready_preview_beta_line.json
```

A `refused_inconsistent_input` outcome prints the typed reason and
the per-lane non-green reasons to stderr without writing the packet
files when `--check` is passed.

## Docs-freshness contract

Each freshness entry pins one token a stable-facing doc must cite.
The closed `check_class` vocabulary is:

- `sdk_line_semver_token`
- `lifecycle_metadata_packet_ref`
- `bridge_matrix_id`
- `bridge_matrix_path`
- `versioning_policy_ref`
- `deprecation_packet_template_ref`
- `conformance_kit_report_ref`
- `sample_pack_starter_pack_ref`
- `consuming_surface_ref`

A missing doc emits `cite_missing`; a present doc that does not carry
the required token emits `drifted`. Both refuse the packet closed
with `docs_freshness_drift_detected`. SDK docs, sample manifests,
extension-author guides, and bridge scorecards therefore drift
together when the contract, lifecycle, or compatibility state moves.

## Consumer contract

- **Release notes and shiproom packets.** Quote the packet decision
  and reason classes verbatim. Never invent a local "SDK is ready"
  string; never widen a partial preview to a green chip.
- **Marketplace and install review.** Read the bridge scorecard's
  per-lane row when rendering native, bridge, shimmed, partial, or
  unsupported chrome. A surface that hides the per-lane scorecard
  class is denied with `review_disclosure_incomplete` per the SDK
  publication contract.
- **Support exports.** Reuse the packet's `consuming_surfaces` and the
  scorecard's `consuming_surface_refs` rather than inventing local
  evidence chips. The packet pins `redaction_class =
  metadata_safe_default`; raw test outputs, raw stack frames, raw
  signing-key material, and raw artifact bytes never appear.
- **Air-gapped mirrors / offline bundles.** Re-emit the same packet
  bytes when the SDK line is republished into a sealed bundle. The
  bridge matrix referenced by the packet is the same matrix mirrors
  consume.

## Guardrails

- No widening claims past the validator suite result class.
- No widening claims past the sample-pack starter-pack decision class.
- No collapsing native and bridge rows under one undifferentiated
  green label; the scorecard's `bridge_state_class` →
  `scorecard_class` mapping is total and the schema rejects widening
  in the typed allOf rules.
- No re-publishing the packet when docs-freshness sweeps drift; the
  generator refuses closed and the CI form fails the build.

## Out of scope

- Expanding the compatibility bridge runtime itself.
- Redesigning the extension SDK beyond what is required to publish
  trustworthy proof packets.
- A marketplace recommendation engine or vanity catalog surface.
