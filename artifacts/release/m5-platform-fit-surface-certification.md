# M5 Platform-Fit Surface Certification

- Packet: `m5-platform-fit-surface-certification:stable:0001`
- As of: `2026-07-13T00:00:00Z`
- Canonical bundle: `artifacts/release/m5-platform-fit-proof/support_export.json`
- Profiles: 6 / 6 certified (2 green, 4 yellow, 0 red)
- Families covered: true
- Invariants held: true
- Auto-narrowed profiles: 4
- Report clean: true

## Profiles

- **cert:live-trusted-platform-fit-surface** — profile=live_trusted_platform_fit_surface claimed=trusted_platform_fit_surface certified=trusted_platform_fit_surface status=green narrowed_axes=0
- **cert:reviewable-platform-fit-structure** — profile=reviewable_platform_fit_structure claimed=reviewable_platform_fit_surface certified=reviewable_platform_fit_surface status=green narrowed_axes=0
- **cert:disclosed-path-terminology-profile** — profile=disclosed_path_terminology_profile claimed=reviewable_platform_fit_surface certified=path_terminology_disclosed_projection status=yellow narrowed_axes=1
- **cert:unverified-appearance-response-profile** — profile=unverified_appearance_response_profile claimed=reviewable_platform_fit_surface certified=appearance_response_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-credential-wording-profile** — profile=unverified_credential_wording_profile claimed=reviewable_platform_fit_surface certified=credential_wording_unverified_projection status=yellow narrowed_axes=1
- **cert:unverified-input-fidelity-profile** — profile=unverified_input_fidelity_profile claimed=reviewable_platform_fit_surface certified=input_fidelity_unverified_projection status=yellow narrowed_axes=1
