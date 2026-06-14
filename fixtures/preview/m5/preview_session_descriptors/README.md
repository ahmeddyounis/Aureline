# M5 Preview-Session Descriptor Fixtures

## session_set_switches_posture_and_downgrades_on_stale.json

A posture-switch and stale-downgrade drill fixture for the preview-session
descriptor set. The first real M5 consumer surfaces — framework-pack preview,
preview route, and notebook-adjacent preview — each carry one shared session
descriptor exposing source revision, runtime identity, device/viewport target,
data posture, freshness, and source-sync state through governed chips.

The set demonstrates a posture switch: the framework-pack session is `live`, the
preview-route session is `mock`, and the notebook-adjacent session is `captured`,
so switching data modes changes the governed `data` chip rather than bespoke
copy. One preview-route session is a `captured` view that has `drifted_from_source`
and is past its freshness SLO (`stale`); because a session may not advertise
current state by silence, it auto-downgrades, records a `stale_freshness`
trigger, and carries a precise non-generic degraded label. A second preview-route
session is `runtime_only_no_source`: it declares `runtime_backed` true while never
claiming to be saved source state and carries no canonical source revision. The
support/export projection session has an unidentified data posture and so
downgrades with an `unidentified_data_posture` trigger and a precise label. Every
non-downgraded session carries neither a trigger nor a degraded label.

The fixture validates against
`schemas/preview/preview_session_descriptor_set.schema.json` and is byte-aligned
with the in-crate builder via
`cargo run -p aureline-preview --example dump_m5_preview_session_descriptors`.
