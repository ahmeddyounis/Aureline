# Package Restore Cases

These fixtures anchor the package restore, mirror-promotion, and
offline-continuity contract:

- [`/docs/ecosystem/package_restore_and_mirror_continuity_contract.md`](../../../docs/ecosystem/package_restore_and_mirror_continuity_contract.md)
- [`/schemas/ecosystem/package_restore_preview.schema.json`](../../../schemas/ecosystem/package_restore_preview.schema.json)
- [`/schemas/ecosystem/mirror_promotion_row.schema.json`](../../../schemas/ecosystem/mirror_promotion_row.schema.json)
- [`/schemas/ecosystem/offline_continuity_card.schema.json`](../../../schemas/ecosystem/offline_continuity_card.schema.json)

The cases keep package-set reproducibility separate from source-class
truth. Matching package ids, versions, and artifact digests do not erase
whether the package came from a public registry, approved mirror, offline
bundle, local archive, installed copy, or quarantined installed copy.

## Index

| Fixture | Schema | Key coverage |
| --- | --- | --- |
| [`public_registry_restore.yaml`](./public_registry_restore.yaml) | `package_restore_preview` | Live public-registry restore with exact lock match and active revocation/yank status. |
| [`approved_mirror_restore.yaml`](./approved_mirror_restore.yaml) | `package_restore_preview` | Approved-mirror restore that points at a same-artifact mirror promotion row. |
| [`approved_mirror_promotion_same_artifact.yaml`](./approved_mirror_promotion_same_artifact.yaml) | `mirror_promotion_row` | Mirror promotion where origin and mirror artifacts are the same artifact. |
| [`approved_mirror_promotion_repackaged_blocked.yaml`](./approved_mirror_promotion_repackaged_blocked.yaml) | `mirror_promotion_row` | Mirror promotion blocked because the mirror produced a repackaged identity. |
| [`offline_continuity_card.yaml`](./offline_continuity_card.yaml) | `offline_continuity_card` | Offline bundle card with warm cached revocation/advisory data and publisher continuity. |
| [`offline_bundle_restore.yaml`](./offline_bundle_restore.yaml) | `package_restore_preview` | Offline-bundle restore linked to the continuity card. |
| [`local_archive_restore.yaml`](./local_archive_restore.yaml) | `package_restore_preview` | Local archive import that prompts for trust instead of inheriting public trust. |
| [`quarantined_installed_copy_restore.yaml`](./quarantined_installed_copy_restore.yaml) | `package_restore_preview` | Installed copy retained for provenance after quarantine; restore is blocked. |

