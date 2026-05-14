# Admin Delete, Hold, Chronology, and Policy-Diff Alpha Fixtures

These fixtures exercise the admin/enterprise alpha packet that joins
delete/export outcomes, legal-hold visibility, timezone-aware chronology,
and policy-diff preview before apply.

The packet is intentionally bounded. It proves the shell and support
surfaces can preserve the shared result vocabulary and link to existing
governance records; it does not implement retention, deletion, legal-hold,
or policy-evaluation backends.

Protected coverage:

- every delete/export row surfaces archive-search posture, hold scope,
  redaction boundary, chain of custody, destruction-receipt availability,
  and export-before-delete posture;
- all chronology fields use the same UTC + local time + IANA timezone
  representation;
- the policy diff is generated before apply and tied to the current
  policy source;
- the support export preserves `completed`, `partial`, `blocked_by_hold`,
  `policy_retained`, `outside_platform_scope`,
  `manual_local_capture_required`, and `omitted_by_redaction`.
