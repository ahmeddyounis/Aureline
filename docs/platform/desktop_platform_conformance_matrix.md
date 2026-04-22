# Desktop-platform conformance matrix

This document is the pre-implementation matrix for Aureline's claimed
desktop profiles. It turns "desktop-first" into named platform claims
that release, support, accessibility, and compatibility packets can
cite without inventing a second platform truth model.

If this document and
[`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
ever disagree, the YAML wins for tooling and this document must be
updated in the same change.

Companion artifacts:

- [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  — machine-readable claimed-profile roster, deployment-pattern support
  notes, platform-owned primitive rows, owners, validation methods, and
  known narrowings.
- [`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml)
  — seeded scenario and drill ids for multi-window, monitor-topology,
  mixed-DPI, suspend/resume, off-screen recovery, and restart/reopen
  continuity on claimed desktop rows.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  — install-profile card and updater/deployment vocabulary the platform
  rows compose over.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — compatibility rows that later release and claim packets extend by
  reference.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AA,
  desktop-platform rules, and the desktop-integration section.
- `.t2/docs/Aureline_Milestones_Document.md` sections 6.7, 6.20, and
  7.9.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` sections 6.6, 6.7, 23.41,
  and 23.42.
- [`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — credential store, trust-store, and degraded-storage posture.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — browser handoff, provider actor, and approval vocabulary.
- [`/docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`](../adr/0015-embedded-surface-boundary-and-auth-handoff.md)
  — system-browser-first auth and embedded-auth prohibition.

The topology suite plus suspend/resume or display-reconnect drills named
by the conformance rows resolve to the scenario ids and drill ids in the
window/display verification matrix above rather than to ad hoc
checklists.

## Claim-status meanings

| Status | Meaning |
|---|---|
| `claimed` | Release, support, and docs may speak about the row as a defended platform claim. |
| `claimed_narrow` | The row is claim-bearing only on the named deployment path or with an explicit narrower affordance surface. |
| `best_effort` | The path may work, but it does not widen the stable support contract or release wording. |
| `unclaimed` | The path is intentionally outside the current claim and must be named early so support/export lanes do not over-infer. |

## Claimed profile roster

All owner scopes below currently resolve to `@ahmeddyounis` under the
repository's solo-maintainer posture. The owner scope ids are listed
verbatim so later packets can cite them without aliasing.

### macOS

| Profile id | OS / arch | Display stack | Browser callback | Proxy / trust | Secret store / IME | Package / deployment assumptions | Notes |
|---|---|---|---|---|---|---|---|
| `macos_15_plus_universal` | macOS 15+; universal binary | Quartz / WindowServer; Spaces and fullscreen are claim-bearing | Normative: system default browser through Launch Services callback ownership. Named browser brand coverage is best-effort, not separately certified. | Normative: System Settings proxy/PAC plus Keychain trust roots and imported org CA bundles. | Normative: Keychain Services; macOS Text Input Services and system IMEs. | Claimed: PKG and notarized DMG direct/managed rows. Claimed_narrow: signed portable app-bundle ZIP. Best-effort: Homebrew-managed path. Unclaimed: offline-bundle lane. | Portable rows do not widen default-handler ownership, dock registration, or recent-item ownership beyond the direct install row. |

### Windows

| Profile id | OS / arch | Display stack | Browser callback | Proxy / trust | Secret store / IME | Package / deployment assumptions | Notes |
|---|---|---|---|---|---|---|---|
| `windows_11_23h2_plus_x86_64` | Windows 11 23H2+; x86_64 | DWM / Win32 windowing; per-monitor DPI and docking restore are claim-bearing | Normative: system default browser through registered protocol/loopback-safe callback path. | Normative: WinHTTP/WinINET effective proxy plus Windows certificate stores and enterprise CA handling. | Normative: Credential Manager / DPAPI-backed secure storage; TSF/IME, AltGr, and dead-key path. | Claimed: MSI/MSIX direct and managed rows plus signed offline bundle. Claimed_narrow: signed portable ZIP. Unclaimed: `winget` external-package-manager row and Windows ARM64. | Protocol-handler ownership changes must be previewable during install/update; last-writer-wins is not a claim-bearing posture. |

### Linux

| Profile id | OS / arch | Display stack | Browser callback | Proxy / trust | Secret store / IME | Package / deployment assumptions | Notes |
|---|---|---|---|---|---|---|---|
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | Ubuntu 24.04 LTS; x86_64 | GNOME Wayland with `xdg-desktop-portal` | Normative: system default browser through `xdg-open` / portal and desktop-file callback return. Named browser brand coverage is best-effort. | Normative: GNOME proxy settings plus launcher env where present; distro CA bundle and imported enterprise CA via distro trust tooling. | Normative: Secret Service via `libsecret` / GNOME Keyring; IBus on GNOME session. | Claimed: tarball/AppImage direct row. Claimed_narrow: managed, portable, and offline-bundle/customer-mirror rows on the named Ubuntu profile only. Best-effort: distro package-manager lane. | This is the widest Linux claim-bearing row and the anchor for managed/offline Linux support wording. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | Ubuntu 24.04 LTS; x86_64 | GNOME X11 | Normative: desktop-file callback via `xdg-open`; same default-browser rule as Wayland row. | Normative: GNOME proxy settings plus launcher env; Ubuntu trust bundle tooling. | Normative: Secret Service via `libsecret` / GNOME Keyring; IBus. | Claimed: tarball/AppImage direct row. Unclaimed: managed, offline-bundle, and customer-mirror widening on X11. Best-effort: portable and distro package-manager lane. | X11 is explicitly named so Linux claims do not hide display-stack differences behind one vague "Linux" label. |
| `linux_fedora_current_gnome_wayland_x86_64` | Fedora current; x86_64 | GNOME Wayland with `xdg-desktop-portal` | Normative: desktop-file callback via `xdg-open` / portal. | Normative: GNOME proxy settings plus launcher env; Fedora trust bundle and enterprise CA import tooling. | Normative: Secret Service via `libsecret` / GNOME Keyring; IBus. | Claimed: tarball/AppImage direct row. Unclaimed: managed/offline-bundle/customer-mirror widening. Best-effort: distro package-manager lane. | Badge and notification behavior are claim-bearing only on the named GNOME profile, not on every Fedora desktop environment. |
| `linux_debian_stable_gnome_x11_x86_64` | Debian stable; x86_64 | GNOME X11 | Normative: desktop-file callback via `xdg-open`. | Normative: GNOME proxy settings plus launcher env; Debian trust bundle and enterprise CA import tooling. | Normative: Secret Service via `libsecret` / GNOME Keyring; IBus. | Claimed: tarball direct row. Best-effort: AppImage and distro package-manager lane. Unclaimed: managed/offline-bundle/customer-mirror widening. | KWallet and KDE-specific secret-store/badge behavior remain outside the current claimed row set. |

## Deployment-pattern and field-support notes

| Profile(s) | Individual install | Managed install | Portable install | Offline bundle / customer mirror | External package manager | Field-support note |
|---|---|---|---|---|---|---|
| `macos_15_plus_universal` | `claimed` | `claimed` | `claimed_narrow` | `unclaimed` | `best_effort` | Claim-bearing rows are the PKG/notarized-DMG install lanes. Homebrew ownership suppresses in-product update claims and does not widen default-handler ownership. |
| `windows_11_23h2_plus_x86_64` | `claimed` | `claimed` | `claimed_narrow` | `claimed` | `unclaimed` | Silent install/update/rollback, protocol ownership, and support diagnostics are claim-bearing on MSI/MSIX/offline-bundle rows. `winget` is intentionally outside the current claimed row set. |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | `claimed` | `claimed` | `claimed_narrow` | `claimed_narrow` | `best_effort` | The Ubuntu Wayland row is the only Linux row that currently carries managed/offline/customer-mirror widening. Distro-native package-manager paths stay best-effort only. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | `claimed` | `unclaimed` | `best_effort` | `unclaimed` | `best_effort` | X11 is claim-bearing for direct local use only; support/export copy must not imply the Ubuntu Wayland managed/offline posture here. |
| `linux_fedora_current_gnome_wayland_x86_64` | `claimed` | `unclaimed` | `best_effort` | `unclaimed` | `best_effort` | Fedora current is a direct-install claim row. Managed, mirror, and portable widening remain outside the current defended profile. |
| `linux_debian_stable_gnome_x11_x86_64` | `claimed` | `unclaimed` | `unclaimed` | `unclaimed` | `best_effort` | Debian stable is intentionally the narrowest Linux claim row so support/export copy does not imply parity with Ubuntu-managed or Fedora Wayland packaging lanes. |

## Conformance rows

### macOS 15+ universal

| Surface | Platform-owned primitive | Owner | Validation | Release bar | Known narrowings or degraded states |
|---|---|---|---|---|---|
| Keyboard notation and menu accelerators | macOS modifier glyphs plus the app menu and global menu bar | `shell_command_system / @ahmeddyounis` | keyboard-path review plus manual macOS menu smoke | `stable_blocker` | Non-default keymap translation is outside this row; the row only claims host-native notation and routing. |
| Window chrome and menu model | native titlebar traffic lights, fullscreen button, and menu-bar ownership | `aureline-render / @ahmeddyounis` | window chrome smoke plus fullscreen/Spaces drill | `stable_blocker` | Portable lane may narrow dock ownership, but not titlebar or menu semantics. |
| File/path presentation | POSIX path grammar, volume labels, and Finder-style display naming where UI needs a human label | `aureline-vfs / @ahmeddyounis` | path-presentation review plus alias/canonical-path smoke | `stable_blocker` | Canonical-path truth still wins before risky writes; human display labels may differ from literal path bytes. |
| Native open/save dialog routing | AppKit-native open/save dialogs and overwrite/read-only disclosure | `aureline-vfs / @ahmeddyounis` | dialog-routing smoke plus overwrite/recovery review | `stable_blocker` | Remote/generated-path disclosure may narrow actions, but the dialog copy must stay explicit. |
| System open, reveal in shell, and open from terminal | Finder reveal, `open` handoff, and shell/drag-drop entry path | `aureline-vfs / @ahmeddyounis` | integration smoke plus wrong-target drill | `stable_blocker` | Portable rows do not claim machine-global default-open ownership. |
| Default-browser callback and auth return | Launch Services callback ownership for system-browser sign-in return | `shell_command_system / @ahmeddyounis` | origin-validation suite plus manual sign-in drill | `stable_blocker` | Named browser-family certification is best-effort outside the OS default-browser path. |
| Deep-link / protocol-handler interstitial | Launch Services protocol/file-handler validation plus widen-authority interstitial | `shell_command_system / @ahmeddyounis` | deep-link spoof-resistance drill plus review-link reopen smoke | `stable_blocker` | Portable installs may open handoff links but must not silently seize shared protocol ownership. |
| Embedded-auth boundary and device-code fallback | system-browser-first auth with host-owned device-code and fallback state | `security_trust_review / @ahmeddyounis` | embedded-surface boundary audit plus browser/device-code drill | `stable_blocker` | Embedded password collection is unclaimed unless a separately approved exception row exists. |
| Proxy/PAC inheritance and route source | System Settings proxy and PAC resolution | `security_trust_review / @ahmeddyounis` | proxy/PAC lab plus route-inspection smoke | `claimed_enterprise_support_blocker` | Per-profile manual overrides may exist later, but they do not widen the OS-inherited enterprise claim. |
| Enterprise CA and client-certificate handling | Keychain trust roots, config-profile CA import, and keychain-backed client-cert selection | `security_trust_review / @ahmeddyounis` | CA/client-cert lab plus certificate failure drill | `claimed_enterprise_support_blocker` | Raw private-key export is out of scope by rule; only brokered/OS-backed handles are claim-bearing. |
| Credential store lock/deny and session-only degrade | Keychain Services lock, deny, unavailable, and visible session-only fallback | `security_trust_review / @ahmeddyounis` | keychain lock/deny drill plus session-only degradation audit | `claimed_enterprise_support_blocker` | Session-only fallback is a degraded state, not a parity claim with a healthy keychain. |
| IME, bidi, and dead-key input path | macOS Text Input Services, dead keys, emoji path, and bidi composition | `aureline-render / @ahmeddyounis` | IME/bidi corpus plus multilingual manual lab | `stable_blocker` | Multi-cursor composition narrowing must be explicit when a coherent IME apply path does not exist. |
| DPI, fractional scaling, multi-monitor, and Spaces/desktops | Retina scaling, monitor moves, fullscreen, and Spaces restore | `aureline-render / @ahmeddyounis` | topology suite plus sleep/wake display drill | `stable_blocker` | Off-screen recovery must be explicit; silent stranded windows are release-blocking. |
| OS-notification click-through and lock-screen privacy | Notification Center reopen target plus summary-only lock-screen copy by default | `shell_command_system / @ahmeddyounis` | notification privacy script plus reopen drill | `stable_blocker` | Notifications may offer one safe action only; no direct privileged apply path from notification surface. |
| Badge semantics and quiet-hours alignment | Dock badge and durable-attention count-class alignment | `design_system_seeds / @ahmeddyounis` | quiet-hours matrix audit plus badge semantics review | `claimed_profile_blocker` | Badge counts must map to durable state classes; no mixed-class inflation. |
| Theme/contrast and density live apply | macOS appearance/contrast changes applied without restart where the platform allows | `aureline-render / @ahmeddyounis` | theme/contrast live-apply smoke | `stable_blocker` | Density tokens may change later, but live apply is the claim-bearing behavior. |
| Accessibility bridge | AX API bridge for core editor, shell, and review surfaces | `aureline-render / @ahmeddyounis` | accessibility regression suite | `stable_blocker` | Any custom-rendered surface without AX parity remains outside the stable claim until covered explicitly. |
| Wake-from-sleep and display-reconnect revalidation | sleep/wake revalidation of callbacks, remote state, and display topology | `aureline-render / @ahmeddyounis` | suspend/resume plus display-reconnect drill | `stable_blocker` | Wake paths must not silently replay privileged or remote actions. |
| Removable-volume return and missing-root recovery | placeholder/locate/close flow for missing volumes and later return | `aureline-vfs / @ahmeddyounis` | missing-root plus removable-volume return drill | `stable_blocker` | The row claims truthful placeholder recovery, not silent disappearance or auto-rewrite of target roots. |
| Install/update/rollback and deployment hooks | PKG/notarized-DMG install family, managed package hooks, and rollback evidence posture | `release_evidence / @ahmeddyounis` | packaging-lane report plus install/update/rollback drill | `release_candidate_blocker` | Homebrew is best-effort only; offline-bundle lane is unclaimed on macOS. |

### Windows 11 23H2+ x86_64

| Surface | Platform-owned primitive | Owner | Validation | Release bar | Known narrowings or degraded states |
|---|---|---|---|---|---|
| Keyboard notation and menu accelerators | standard Windows accelerator notation and menu mnemonics | `shell_command_system / @ahmeddyounis` | keyboard-path review plus Windows shell smoke | `stable_blocker` | AltGr correctness is governed by the IME row, not by accelerator rendering. |
| Window chrome and menu model | DWM titlebar/system-menu conventions, maximize/snap/fullscreen behavior | `aureline-render / @ahmeddyounis` | window chrome smoke plus snap/docking drill | `stable_blocker` | Preview and portable channels may coexist, but window/system-menu semantics must not diverge by channel. |
| File/path presentation | drive-letter, UNC, and backslash path presentation with literal target preservation | `aureline-vfs / @ahmeddyounis` | path-presentation review plus UNC/share smoke | `stable_blocker` | Canonicalization before risky writes remains mandatory; display formatting must not hide the real target root. |
| Native open/save dialog routing | Windows common item dialogs with overwrite/read-only/trust disclosure | `aureline-vfs / @ahmeddyounis` | dialog-routing smoke plus overwrite/recovery review | `stable_blocker` | Generated or remote targets may narrow actions but must still disclose boundary and recovery path. |
| System open, reveal in shell, and open from terminal | Explorer reveal, shell open, and `start`/registered file-association handoff | `aureline-vfs / @ahmeddyounis` | integration smoke plus wrong-target drill | `stable_blocker` | Portable rows must not claim machine-global recent-item or handler ownership. |
| Default-browser callback and auth return | registered protocol/loopback-safe callback return from the default browser | `shell_command_system / @ahmeddyounis` | origin-validation suite plus manual sign-in drill | `stable_blocker` | Browser-family coverage outside the default-browser path is best-effort; SmartScreen-safe packaging remains part of the claim. |
| Deep-link / protocol-handler interstitial | registered app protocol plus widen-authority interstitial and wrong-target recovery | `shell_command_system / @ahmeddyounis` | deep-link spoof-resistance drill plus review-link reopen smoke | `stable_blocker` | Side-by-side channels must preview ownership changes; last-writer-wins is unclaimed. |
| Embedded-auth boundary and device-code fallback | system-browser-first auth with host-owned fallback sheet | `security_trust_review / @ahmeddyounis` | embedded-surface boundary audit plus browser/device-code drill | `stable_blocker` | Embedded password collection is outside the claim unless an approved exception row exists. |
| Proxy/PAC inheritance and route source | WinHTTP/WinINET effective proxy and PAC resolution | `security_trust_review / @ahmeddyounis` | proxy/PAC lab plus route-inspection smoke | `claimed_enterprise_support_blocker` | Manual per-process override does not widen the system-inherited claim. |
| Enterprise CA and client-certificate handling | Current User/Local Machine certificate stores plus enterprise CA import and client-cert selection | `security_trust_review / @ahmeddyounis` | CA/client-cert lab plus certificate failure drill | `claimed_enterprise_support_blocker` | Raw client-cert private-key export is outside the claim by rule. |
| Credential store lock/deny and session-only degrade | Credential Manager / DPAPI-backed storage with visible deny/unavailable/session-only states | `security_trust_review / @ahmeddyounis` | credential-store lock/deny drill plus session-only degradation audit | `claimed_enterprise_support_blocker` | Session-only fallback remains degraded and must be visibly labeled. |
| IME, bidi, and dead-key input path | TSF/IME, AltGr, dead keys, and bidi composition | `aureline-render / @ahmeddyounis` | IME/bidi corpus plus multilingual manual lab | `stable_blocker` | Composition narrowing must be explicit; no silent commit/cancel on focus churn. |
| DPI, fractional scaling, multi-monitor, and Spaces/desktops | per-monitor DPI, taskbar/fullscreen behavior, and restore across docking changes | `aureline-render / @ahmeddyounis` | topology suite plus docking/undocking drill | `stable_blocker` | Off-screen stranded windows or sheets are release-blocking. |
| OS-notification click-through and lock-screen privacy | toast click-through, Action Center reopen target, and privacy-safe lock-screen copy | `shell_command_system / @ahmeddyounis` | notification privacy script plus reopen drill | `stable_blocker` | One safe primary action only; no notification path may bypass preview/review rules. |
| Badge semantics and quiet-hours alignment | taskbar badge and durable-attention count-class alignment | `design_system_seeds / @ahmeddyounis` | quiet-hours matrix audit plus badge semantics review | `claimed_profile_blocker` | Badge truth must survive quiet-hours and suppression rules without inflating counts. |
| Theme/contrast and density live apply | Windows theme/high-contrast changes applied live where supported | `aureline-render / @ahmeddyounis` | theme/high-contrast live-apply smoke | `stable_blocker` | Restart-only fallback is unclaimed for the stable Windows row. |
| Accessibility bridge | UIA bridge for core editor, shell, and review surfaces | `aureline-render / @ahmeddyounis` | accessibility regression suite | `stable_blocker` | Any custom surface without UIA parity remains outside the stable claim. |
| Wake-from-sleep and display-reconnect revalidation | suspend/resume revalidation for callbacks, remote state, and monitor topology | `aureline-render / @ahmeddyounis` | suspend/resume plus display-reconnect drill | `stable_blocker` | Wake paths may not silently replay privileged or remote actions. |
| Removable-volume return and missing-root recovery | missing-root placeholder with locate/cached-context/close actions | `aureline-vfs / @ahmeddyounis` | missing-root plus removable-volume return drill | `stable_blocker` | Network-share disappearance must not look like local data loss. |
| Install/update/rollback and deployment hooks | MSI/MSIX/offline-bundle family, managed deployment hooks, and rollback evidence posture | `release_evidence / @ahmeddyounis` | packaging-lane report plus install/update/rollback drill | `release_candidate_blocker` | `winget` remains unclaimed until it has a dedicated install-profile card and evidence row. |

### Linux claimed GNOME desktop set

The Linux rows below apply only to the named profiles:
`linux_ubuntu_24_04_gnome_wayland_x86_64`,
`linux_ubuntu_24_04_gnome_x11_x86_64`,
`linux_fedora_current_gnome_wayland_x86_64`, and
`linux_debian_stable_gnome_x11_x86_64`. They do not widen to "Linux"
generally.

| Surface | Platform-owned primitive | Owner | Validation | Release bar | Known narrowings or degraded states |
|---|---|---|---|---|---|
| Keyboard notation and menu accelerators | GNOME session menu/accelerator rendering on the named Wayland/X11 rows | `shell_command_system / @ahmeddyounis` | keyboard-path review plus GNOME Wayland/X11 shell smoke | `stable_blocker` | Desktop-environment-specific mnemonic rendering outside the named GNOME rows is best-effort only. |
| Window chrome and menu model | GNOME Wayland/X11 window controls and claimed client-side decoration behavior | `aureline-render / @ahmeddyounis` | window chrome smoke plus GNOME Wayland/X11 drill | `stable_blocker` | Alternate compositors/window managers are unclaimed. |
| File/path presentation | POSIX path grammar, mount-point identity, and shell-safe copy presentation | `aureline-vfs / @ahmeddyounis` | path-presentation review plus mount/share smoke | `stable_blocker` | Human-readable labels may differ from literal path bytes, but risky writes still resolve through canonical target truth. |
| Native open/save dialog routing | `xdg-desktop-portal` chooser on Wayland and claimed GNOME-native chooser path on X11 | `aureline-vfs / @ahmeddyounis` | dialog-routing smoke plus portal/X11 chooser drill | `stable_blocker` | If portal routing is unavailable on a claimed Wayland row, the row degrades visibly; silent fallback is unclaimed. |
| System open, reveal in shell, and open from terminal | desktop-file/`xdg-open` handoff, file-manager reveal, and terminal launch path | `aureline-vfs / @ahmeddyounis` | integration smoke plus wrong-target drill | `stable_blocker` | Non-GNOME file-manager integration is best-effort only. |
| Default-browser callback and auth return | `xdg-open` / portal default-browser handoff with desktop-file/custom-scheme return | `shell_command_system / @ahmeddyounis` | origin-validation suite plus manual sign-in drill | `stable_blocker` | Named browser-family certification is best-effort; the claim is the system default-browser path only. |
| Deep-link / protocol-handler interstitial | desktop-file `x-scheme-handler` ownership plus widen-authority interstitial | `shell_command_system / @ahmeddyounis` | deep-link spoof-resistance drill plus reopen smoke | `stable_blocker` | Portable rows do not claim machine-global handler ownership; distro package-manager rows do not widen the handler claim. |
| Embedded-auth boundary and device-code fallback | system-browser-first auth with host-owned device-code fallback | `security_trust_review / @ahmeddyounis` | embedded-surface boundary audit plus browser/device-code drill | `stable_blocker` | Embedded password collection is outside the claim unless an exception row exists. |
| Proxy/PAC inheritance and route source | GNOME proxy settings plus launcher env where present; PAC honored only where the session/browser path exposes it | `security_trust_review / @ahmeddyounis` | proxy/PAC lab plus route-inspection smoke | `claimed_enterprise_support_blocker` | Non-GNOME session proxy behavior and ad hoc env-only launchers are best-effort, not claim-bearing. |
| Enterprise CA and client-certificate handling | distro trust bundle plus imported enterprise CA and brokered/system client-cert selection | `security_trust_review / @ahmeddyounis` | CA/client-cert lab plus certificate failure drill | `claimed_enterprise_support_blocker` | This row does not claim raw private-key export or every distro TLS stack variant. |
| Credential store lock/deny and session-only degrade | Secret Service via `libsecret` / GNOME Keyring with visible unavailable/session-only state | `security_trust_review / @ahmeddyounis` | secret-store failure drill plus session-only degradation audit | `claimed_enterprise_support_blocker` | KWallet or other secret-store implementations are best-effort or unclaimed unless separately named. |
| IME, bidi, and dead-key input path | IBus on the named GNOME rows with explicit Wayland/X11 coverage | `aureline-render / @ahmeddyounis` | IME/bidi corpus plus multilingual manual lab | `stable_blocker` | Fcitx is best-effort only until it has its own named profile row. |
| DPI, fractional scaling, multi-monitor, and Spaces/desktops | GNOME Wayland/X11 scaling, workspace, and monitor-topology behavior on the named rows | `aureline-render / @ahmeddyounis` | topology suite plus GNOME Wayland/X11 display drill | `stable_blocker` | Alternate compositors/window managers are unclaimed. |
| OS-notification click-through and lock-screen privacy | GNOME notification reopen target and privacy-safe summary | `shell_command_system / @ahmeddyounis` | notification privacy script plus reopen drill | `stable_blocker` | Non-GNOME lock-screen/notification-center behavior is best-effort only. |
| Badge semantics and quiet-hours alignment | in-product durable attention is normative; shell badge behavior is claimed only on the named GNOME rows | `design_system_seeds / @ahmeddyounis` | quiet-hours matrix audit plus GNOME badge semantics review | `claimed_profile_blocker` | Outside the named GNOME rows, the shell badge is unclaimed; the durable in-product truth remains the fallback. |
| Theme/contrast and density live apply | GNOME theme/contrast changes applied live where the desktop session exposes them | `aureline-render / @ahmeddyounis` | theme/contrast live-apply smoke | `stable_blocker` | Desktop-environment-specific theme packs outside the named rows are best-effort only. |
| Accessibility bridge | AT-SPI bridge for core editor, shell, and review surfaces | `aureline-render / @ahmeddyounis` | accessibility regression suite | `stable_blocker` | Any Linux surface without AT-SPI parity remains outside the stable claim. |
| Wake-from-sleep and display-reconnect revalidation | suspend/resume and display-reconnect revalidation on the named GNOME rows | `aureline-render / @ahmeddyounis` | suspend/resume plus display-reconnect drill | `stable_blocker` | Wake paths may not silently replay privileged or remote actions. |
| Removable-volume return and missing-root recovery | mount-loss placeholder and return path with locate/cached-context/close actions | `aureline-vfs / @ahmeddyounis` | missing-root plus removable-volume return drill | `stable_blocker` | The claim is truthful placeholder recovery, not silent path remap or data loss. |
| Install/update/rollback and deployment hooks | direct tarball/AppImage rows on the named profiles; managed/offline/customer-mirror widening only on Ubuntu 24.04 Wayland | `release_evidence / @ahmeddyounis` | packaging-lane report plus install/update/rollback drill | `release_candidate_blocker` | Distro-native package managers are best-effort only; KDE/KWallet, ARM64 Linux, and unnamed desktop environments remain outside the claim. |

## Explicitly unclaimed or degraded lanes

- Windows ARM64 and Linux ARM64 remain unclaimed until install-profile
  cards and per-OS conformance rows exist for them.
- Linux KDE, KWallet-backed secret storage, non-GNOME badge behavior,
  and non-GNOME deep-link/notification semantics are not implied by the
  named GNOME rows.
- macOS Homebrew and Linux distro-native package-manager paths are
  best-effort only; they do not widen the direct-install stable claim.
- Windows `winget` is intentionally unclaimed until a dedicated install
  card, support diagnostics path, and rollback evidence row land.
- macOS offline-bundle deployment is unclaimed.
- Portable rows on every OS are narrower by design: they do not claim
  machine-global file associations, shared protocol-handler ownership,
  machine-global recent-item registration, or package-manager-owned
  update semantics.
- Embedded password collection is prohibited on claimed rows unless a
  separately approved exception register row exists under the embedded
  surface boundary contract.
