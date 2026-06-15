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
//!
//! The [`m5_chargeback_scope_views`] module renders the chargeback surface for
//! each claimed managed lane: who owns the cost and whether it is charged
//! directly or inherited from a broader scope. It freezes one view per service
//! family, each keeping one cost truth per offered chargeback scope so personal,
//! workspace, team, and organization never collapse into one ambiguous owner
//! bucket, and each separating a direct cost line from an inherited share that
//! names its parent scope. A single scope switcher holds the active scope across
//! the views while preserving the active scope, the inherited-versus-direct
//! separation, and each scope's owner identity; the set exports at CSV/JSON
//! parity, projects its control-plane lane, narrows the marketed claim under the
//! active managed state, and keeps a non-empty local-safe baseline.
//!
//! The [`m5_metering_degradation_rules`] module freezes the runtime degradation
//! behavior when a metering or rating path goes stale or unreachable. It pins one
//! rule per service family and degradation trigger — a stale meter, an
//! unreachable metering service, and an unavailable rating path — across the AI
//! gateway, settings sync, the companion relay, the registry/mirror surface,
//! support ingest, and the managed workspace. Each rule projects its
//! control-plane lane's fail posture so the local core fails open and keeps
//! running, gates exactly one named spend-bearing optional action with its
//! blocking reason when the lane fails closed, names the affected service family,
//! the local-safe promise, and the retry and details actions, discloses any
//! number bound to its unit, as-of time, and scope owner or suppressed, keeps a
//! metering degradation distinct from a seat loss, an org switch, a grace window,
//! and a sign-in failure, and narrows the marketed claim to managed-narrowed
//! without ever collapsing the local core to local-safe-only.
//!
//! The [`m5_commercial_boundary_cards`] module renders the commercial-boundary
//! surface a user, admin, or procurement reviewer sees on Help/About, the release
//! center, diagnostics, and in a procurement/support packet. It freezes one card
//! for the local open core plus one per managed service family, and one
//! binding per surface. Each card states its open-versus-paid boundary class,
//! discloses the residual vendor-hosted dependencies a managed lane carries —
//! naming whether each remains vendor-hosted and whether self-hosting eliminates
//! it — names the deployment profiles its boundary actually holds in so no open
//! boundary is overstated, links the procurement/support evidence at export
//! parity that both buyer and support packets reuse, keeps a non-empty local-safe
//! baseline above any upsell prompt, defers every spend/quota number to the
//! metering surfaces, and narrows its marketed claim automatically when the
//! backing boundary evidence is stale, missing, or downgraded. It cross-checks
//! each managed card against the control-plane lane for its service family.
//!
//! The [`m5_offboarding_cards`] module renders the humane offboarding surface a
//! user or admin sees when a managed entitlement is winding down. It freezes one
//! card per lifecycle event — a grace period, a seat loss, a cancellation, and an
//! org switch — and each card states the event type, the effective date, the
//! impacted managed features, the export rights, the local-safe continuation, the
//! deletion timeline, and the owner/contact handoff, separates local artifacts
//! from tenant-scoped managed state so a seat loss or an org switch never blurs
//! the two, keeps export and local continuation above any upgrade or renewal
//! prompt, stays distinct from the other three events and a sign-in failure, and
//! narrows the marketed claim from the lifecycle event's cap — all without ever
//! deleting or blocking the local core.
//!
//! The [`m5_commercial_honesty_certification`] module certifies the whole lane:
//! it freezes one row per honesty dimension — entitlement, metering, forecast,
//! chargeback, downgrade/offboarding, and commercial-boundary honesty — and rides
//! the sibling packets above as its backing consumers. Each row names the
//! deployment profiles it is certified in and the profiles its managed lane is
//! honestly not offered in (never certifying from one vendor-managed online
//! profile alone), and runs the certification drills the contract requires — the
//! stale-meter drill, the fail-open-local-core and fail-closed-managed-action
//! drills, the seat-loss, org-switch, and grace-period drills, the export-rights
//! validation, the chargeback-scope export check, and the residual-dependency
//! disclosure review. A failed drill or stale evidence narrows the row's marketed
//! claim from the weaker of its declared claim and every drill cap, so the
//! release center, Help/About, diagnostics, service health, support/admin packets,
//! and claim/public-truth automation all narrow automatically instead of
//! inheriting broader managed marketing language, while every row keeps a
//! non-empty local-safe baseline.

pub mod m5_chargeback_scope_views;
pub mod m5_commercial_boundary_cards;
pub mod m5_commercial_control_plane;
pub mod m5_commercial_honesty_certification;
pub mod m5_entitlement_summary;
pub mod m5_metering_degradation_rules;
pub mod m5_offboarding_cards;
pub mod m5_usage_forecast_views;
