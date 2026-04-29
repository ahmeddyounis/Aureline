# Privacy-history fixture cases

These fixtures exercise the privacy-history and export/delete lifecycle
contract:

- `consent_toggle_telemetry_opt_in.yaml` records a telemetry consent
  toggle and resolves uploaded signal classes with no applicable AI
  memory class.
- `policy_locked_crash_upload.yaml` records a policy-enforced crash
  upload override where the crash payload remains local-only and
  redacted.
- `support_bundle_exported_with_redaction.yaml` records a support
  bundle export event with redacted and uploaded lifecycle terms.
- `delete_blocked_by_hold.yaml` records a delete request blocked by
  legal hold.
- `offboarding_export_excluded_classes.yaml` records an offboarding
  export with included classes, excluded classes, remaining local
  artifacts, and AI memory-class resolution.

The cases are YAML for readability. Each file starts with a
`yaml-language-server` schema comment and carries a `__fixture__`
annotation block that is not part of the runtime record.
