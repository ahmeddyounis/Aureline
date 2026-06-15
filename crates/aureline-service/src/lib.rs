//! Commercial-control-plane truth for Aureline's optional managed lanes.
//!
//! This crate owns the canonical, inspectable truth packets for the
//! managed-service economics boundary — the open-source local core never
//! depends on it. The packets here are metadata-only: every field is a typed
//! state, a closed-vocabulary token, or an opaque ref. They carry no credential
//! bodies, raw provider payloads, raw billing values, raw tenant names, or raw
//! account identifiers.
//!
//! The [`m5_commercial_control_plane`] module freezes the entitlement,
//! meter-family, chargeback-scope, org-switch, and grace-period matrix that
//! covers every claimed managed lane — the AI gateway, settings sync, the
//! companion relay, the registry/mirror surface, support ingest, and the
//! managed workspace. Each lane row pins its service family, meter family, meter
//! unit, aggregation window, as-of-time requirement, scope owner, chargeback
//! scopes, fail-open/fail-closed posture, forecast confidence, grace-period
//! rights, export guarantee, and the local-safe baseline that always continues
//! when the managed lane narrows. Alongside the lanes, the matrix locks the
//! user-visible managed-state vocabulary — signed in, local only, reauth
//! required, managed blocked, grace period, seat removed, plan downgrade, org
//! switched, forecast threshold, and meter stale — each bound to its frozen
//! entitlement state, posture origin, marketed-claim cap, and the distinctness
//! rule that keeps seat loss, an org switch, a grace window, and a sign-in
//! failure from collapsing into one generic account error.
//!
//! A managed lane's effective marketed claim is recomputed from the active
//! managed state's cap, so a stale meter, an exhausted forecast, a grace window,
//! or a removed seat narrows the marketed claim automatically rather than
//! leaving it as an optimistic constant; the stored value must equal that
//! recomputation or validation fails. Every lane keeps a non-empty local-safe
//! baseline, so narrowing a managed lane never blocks local editing, search,
//! Git, or already-authorized local automation. Account, diagnostics, Help/About,
//! support/admin, and claim/public-truth consumers all project the same matrix
//! rather than parallel spreadsheets.
//!
//! The [`m5_entitlement_summary`] module renders the account-context view a
//! surface shows a user — the plan, role, seat owner, org/tenant scope,
//! entitlement label, quota-snapshot age, and the local-only continuation notes
//! — reusing the control-plane vocabulary. It freezes one summary per managed
//! state and one binding per surface (account, diagnostics, support/admin,
//! Help/About, and feature entry points), recomputes each summary's degradation,
//! claim, and posture origin from the state, and degrades a seat loss or an
//! expiry to an explicit managed-blocked state rather than a generic sign-in
//! error.
//!
//! The [`m5_usage_forecast_views`] module renders the customer-visible usage and
//! forecast surface for each claimed managed lane. It freezes one view per
//! service family — the AI gateway, settings sync, the companion relay, the
//! registry/mirror surface, support ingest, and the managed workspace — that
//! pins the meter unit, the month-to-date measurement (bound to its unit, as-of
//! time, and scope owner, never a raw number), the forecast threshold status and
//! a banner that explains what changes next, and a CSV/JSON export-parity
//! guarantee, then narrows the marketed usage claim under the active managed
//! state while keeping a non-empty local-safe baseline. Each view projects its
//! control-plane lane rather than a parallel spreadsheet, and unlike service
//! families never merge into one opaque total.

pub mod m5_commercial_control_plane;
pub mod m5_entitlement_summary;
pub mod m5_usage_forecast_views;
