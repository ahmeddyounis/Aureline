# Publish Claim Manifest Tool

`publish_claim_manifest.py` validates the stable claim-publication manifest and
optionally emits the redaction-safe projection consumed by docs, Help/About,
service-health, CLI inspection, support export, release notes, and evaluation
packet builders.

```sh
python3 tools/release/publish-claim-manifest/publish_claim_manifest.py \
  artifacts/release/stable/claim-publication-manifest/manifest.json \
  --projection
```

The tool fails when a surface renders wider than the manifest's effective
claim, a Certified row lacks a current signed reference-workspace report, a
private evaluation filter widens a public claim, or summary/publication fields
do not match the recomputed state.
