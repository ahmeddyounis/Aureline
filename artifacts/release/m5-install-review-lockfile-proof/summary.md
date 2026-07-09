# Install-review sheets and lockfile-impact cards

- Packet: `install-review-lockfile:stable:0001`
- Surface: `Install-review sheets and lockfile-impact cards`
- Review sheets: 3 (1 broad)
- Lockfile-impact cards: 3 (1 broad churn)
- Proof freshness SLO: 168 hours (last refresh: 2026-07-08T00:00:00Z)

## Install-review sheets

- **left-pad** (install) — breadth `small_single`, 1 manifest(s), 2 transitive churn
- **eslint + plugins** (update) — breadth `grouped_change`, 2 manifest(s), 8 transitive churn
- **react (with peers)** (remove) — breadth `broad_change`, 1 manifest(s), 30 transitive churn

## Lockfile-impact cards

- **npm 10.8.2** — churn `broad_churn` [regenerate_whole_lockfile], 1 lockfile(s)
- **npm 10.8.2** — churn `narrow_churn` [edit_in_place_entries], 1 lockfile(s)
- **npm 10.8.2** — churn `no_churn` [no_lockfile_write], 1 lockfile(s)
