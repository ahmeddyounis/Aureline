# Plain local folder with no archetype evidence

## Row binding

- Archetype row id: `archetype_row:misc_local_folder_no_archetype`
- Archetype id: `misc_folder`
- Initial support class: not applicable — excluded row
- Exclusion reason: `not_a_first_party_support_archetype`
- Inclusion target: `post_stable`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

A local folder with no detectable package manifest, no lockfile, and
no DVCS metadata. The corpus exercises this shape so the
unrecognised-archetype path stays inspectable; the row is not a
support claim.

## Required-mode rationale

Not applicable — the row is intentionally excluded from the support
ladder. The reference fixtures exist to validate the
`unrecognised_archetype` outcome, not to back any user-facing support
class.

## Evidence already on file

- Reference workspaces:
  `refws.micro_local_folder`
  ([fixture](../../workspaces/reference/micro_local_folder.json)) and
  `refws.misc_folder_unknown_archetype`
  ([fixture](../../workspaces/reference/misc_folder_unknown_archetype.json)).

## Open evidence questions

- Confirm at the next inventory review that no surface has begun
  rendering this row as a support class. The exclusion-row mechanism
  exists exactly to prevent that.
- Decide whether the inventory needs to grow additional misc-folder
  variants (read-only mounts, network drives, etc.) before
  `post_stable`, or whether the existing two fixtures cover the
  unrecognised path.
