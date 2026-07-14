# Shared Repository-Bootstrap Consumers: One Registry Across Surfaces

- Packet: `m5-repository-bootstrap-shared-consumers:stable:0001`
- Surface: `M5 repository-bootstrap shared consumers (one registry across surfaces)`
- Consumer bindings: 15 (6 narrowed)
- Proof freshness SLO: 720 hours (last refresh: 2026-07-14T00:00:00Z)

## Consumer bindings

- **Open local (existing checkout detected, never recloned)** [`rbsc-open-local-acquisition`]: family `open_local` on `acquisition_engine`, representation `desktop_full`, role `source_locator`
- **Open local (existing checkout detected, never recloned)** [`rbsc-open-local-shell`]: family `open_local` on `shell_ui`, representation `desktop_full`, role `source_locator`
- **Open local (existing checkout detected, never recloned)** [`rbsc-open-local-cli`]: family `open_local` on `cli_export`, representation `exported_redacted`, role `source_locator`
- **Clone remote (checkout plan and credential posture shown before the fetch)** [`rbsc-clone-remote-git`]: family `clone_remote` on `git_service`, representation `desktop_full`, role `credential_posture`
- **Clone remote (checkout plan and credential posture shown before the fetch)** [`rbsc-clone-remote-shell`]: family `clone_remote` on `shell_ui`, representation `desktop_full`, role `credential_posture`
- **Clone remote (checkout plan and credential posture shown before the fetch)** [`rbsc-clone-remote-support`]: family `clone_remote` on `support_export`, representation `exported_redacted`, role `credential_posture`
- **Open archive (digest and extraction evidence verified before disk mutation)** [`rbsc-open-archive-diagnostics`]: family `open_archive` on `diagnostics`, representation `desktop_full`, role `evidence_packet`
- **Open archive (digest and extraction evidence verified before disk mutation)** [`rbsc-open-archive-acquisition`]: family `open_archive` on `acquisition_engine`, representation `desktop_full`, role `evidence_packet`
- **Open archive (digest and extraction evidence verified before disk mutation)** [`rbsc-open-archive-workspace`]: family `open_archive` on `workspace_service`, representation `remote_projected`, role `evidence_packet`
- **Import bundle (signer / mirror provenance preserved, trust staged before repo actions)** [`rbsc-import-bundle-trust`]: family `import_bundle` on `trust_service`, representation `desktop_full`, role `staged_trust`
- **Import bundle (signer / mirror provenance preserved, trust staged before repo actions)** [`rbsc-import-bundle-diagnostics`]: family `import_bundle` on `diagnostics`, representation `desktop_full`, role `staged_trust`
- **Import bundle (signer / mirror provenance preserved, trust staged before repo actions)** [`rbsc-import-bundle-docs`]: family `import_bundle` on `docs_help`, representation `remote_projected`, role `staged_trust`
- **Resume snapshot (partial acquisition resumable / discardable, post-open queue never auto-runs)** [`rbsc-resume-snapshot-docs`]: family `resume_snapshot` on `docs_help`, representation `desktop_full`, role `post_open_queue`
- **Resume snapshot (partial acquisition resumable / discardable, post-open queue never auto-runs)** [`rbsc-resume-snapshot-workspace`]: family `resume_snapshot` on `workspace_service`, representation `compact_narrowed`, role `post_open_queue`
- **Resume snapshot (partial acquisition resumable / discardable, post-open queue never auto-runs)** [`rbsc-resume-snapshot-support`]: family `resume_snapshot` on `support_export`, representation `exported_redacted`, role `post_open_queue`
