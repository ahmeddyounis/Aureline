# M5 Post-Install Notice/Provenance/SBOM Disclosure Panels

- Packet: `m5-post-install-disclosure-panels:stable:0001`
- Label: `M5 Post-Install Notice/Provenance/SBOM Disclosure Panels for installed and generated artifact families`
- Panels: 4 (4 families covered)

## Panels

- **desktop_build_installer**: `post_install_disclosure:desktop.official.signed_stable`
  - Subject: product_build (Installed product build disclosure)
  - Source: official / signature signed_verified / sbom sbom_attached_verified
  - Notice inventory: inventory_available / revocation revocation_current
  - Visible missing/partial-data rows: 0
- **extension_framework_pack**: `post_install_disclosure:extension.side_loaded.local_archive`
  - Subject: extension_package (Side-loaded extension disclosure)
  - Source: side_loaded / signature signed_unverified / sbom sbom_missing
  - Notice inventory: inventory_missing / revocation revocation_unknown
  - Visible missing/partial-data rows: 5
- **mirrored_offline_artifact**: `post_install_disclosure:mirror.offline_bundle.stale_revocation`
  - Subject: mirrored_transport_artifact (Mirrored offline update bundle disclosure)
  - Source: mirrored / signature signed_verified / sbom sbom_attached_verified
  - Notice inventory: inventory_available / revocation revocation_snapshot_stale
  - Visible missing/partial-data rows: 1
- **generated_export_artifact**: `post_install_disclosure:generated_export.support_report.redistribution_review`
  - Subject: generated_user_artifact (Generated export disclosure)
  - Source: official / signature not_applicable / sbom not_applicable
  - Notice inventory: inventory_partial / revocation revocation_current
  - Visible missing/partial-data rows: 2
