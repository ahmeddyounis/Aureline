# SDK Versioning And Deprecation Policy

This page is the author-facing policy for beta extension SDK and
public-interface surfaces. The canonical machine-readable packet is
[`artifacts/extensions/m3/lifecycle_metadata_packet.json`](../../../artifacts/extensions/m3/lifecycle_metadata_packet.json);
the boundary schema is
[`schemas/extensions/lifecycle_metadata.schema.json`](../../../schemas/extensions/lifecycle_metadata.schema.json);
and the deprecation packet template is
[`artifacts/extensions/m3/deprecation_packet_template.md`](../../../artifacts/extensions/m3/deprecation_packet_template.md).

SDK docs, compatibility reports, publication tooling, registry metadata,
support exports, and validator reports must cite lifecycle row ids from
the packet. They must not restate support windows or deprecation cutoff
dates locally.

## Stability Labels

| Label | Compatibility promise | Removal rule |
|---|---|---|
| `internal` | No public compatibility promise. | May change at any time. |
| `experimental` | Best-effort preview behavior only. | May change with preview-note visibility. |
| `beta` | Backward compatibility is expected within the release family unless a documented migration exists. | At least one minor release or 90 days of overlap before removal. |
| `stable` | SemVer and documented migration obligations apply. | At least two minor releases and 12 months of overlap before breaking removal. |
| `lts_surface` | Slow-moving contractual surface. | Breaking change only at major or LTS boundary with an explicit migration plan. |
| `deprecated` | Still readable or usable under a published support window. | Replacement or no-direct-replacement posture, migration guide, and removal target are mandatory. |
| `retired` | Not available for new publication or activation. | Existing metadata may remain visible for review, export, or rollback. |

## Versioning Rules

| Surface family | Versioning rule | Compatibility control |
|---|---|---|
| SDK typed API surfaces | SemVer after stabilization; beta rows pin `aureline.sdk.beta` and a support window. | SDK docs and sample packs cite `lifecycle_row:sdk_api_surface.*`. |
| Manifest schemas and fields | JSON schema epoch with additive evolution inside an epoch. | Unknown additive fields are preserved where safe; unknown privileged behavior fails closed. |
| Permission vocabulary | Permission vocabulary epoch. | Removing or changing a permission class requires deprecation metadata. |
| WIT worlds | WIT package versioning. | Host negotiation chooses the declared intersection and fails closed out of range. |
| Publication packets | Publication schema epoch. | Publication tooling refuses packets that omit lifecycle and deprecation refs. |
| Bridge profiles | Profile epoch. | Bridge and shimmed rows must never claim exact native parity. |

## Required Lifecycle Metadata

Every beta, stable, LTS, deprecated, or retired public row must publish:

- `row_id`, `surface_ref`, `surface_kind`, and `stability_label`;
- `versioning_scheme`, `current_version`, `min_supported_version`, and
  `max_tested_version`;
- support-window metadata with introduced version, minimum overlap, and
  removal-not-before target when applicable;
- docs, schema, compatibility report, conformance report where available,
  owner, and consumer refs;
- deprecation posture and downgrade behavior, even when the row is not
  currently deprecated.

## Deprecation Rules

A deprecated API, manifest field, WIT world, permission class, bridge
profile, or publication shape must carry all of the following in the
lifecycle packet and any deprecation packet derived from it:

- affected surface and owner;
- deprecated-since version;
- replacement surface ref or explicit no-direct-replacement reason;
- migration guide ref;
- removal target version or removal target date;
- alias, reader/writer, downgrade, or rollback behavior;
- author, admin, and support impact.

Publication is blocked when a deprecated row has no replacement path or
no-direct-replacement reason, no expiry guidance, or no migration guide.

## Manifest Lifecycle State Guidance

`manifest_field:extension_manifest.lifecycle.state.resolved` is
deprecated. Readers preserve it for the beta overlap window, but new
writers should emit:

| Use | Write |
|---|---|
| Manifest has passed static validation and is ready for runtime admission. | `verified` |
| Extension has been admitted and activated by the runtime. | `activated` |
| Extension is intentionally narrowed after runtime or policy evaluation. | `degraded` |
| Extension is blocked, quarantined, revoked, or removed. | `disabled` or `removed` |

The replacement row is
`manifest_field:extension_manifest.lifecycle.state.verified`; removal is
not before `1.0.0-beta.3` or `2026-08-14`.

## Validation

Validate the lifecycle metadata packet:

```text
python3 tools/extensions/m3/validator_cli/aureline_extension_validator.py \
  --repo-root . \
  validate-lifecycle-packet \
  --packet artifacts/extensions/m3/lifecycle_metadata_packet.json \
  --report artifacts/compat/m3/extension_lifecycle_metadata_report.json \
  --check
```

Build publication packets with lifecycle enforcement:

```text
python3 tools/extensions/m3/publish_extension.py \
  --repo-root . \
  build-packet \
  --lifecycle-packet artifacts/extensions/m3/lifecycle_metadata_packet.json \
  --deprecation-packet-template-ref artifacts/extensions/m3/deprecation_packet_template.md \
  ...
```
