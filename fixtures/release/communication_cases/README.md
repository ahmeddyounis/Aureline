# Release-communication card fixtures

Worked cases for the release-notes, what's-new, and service-health
communication contract frozen in
[`/docs/release/release_notes_whats_new_service_health_contract.md`](../../../docs/release/release_notes_whats_new_service_health_contract.md)
and the
[`/schemas/release/whats_new_card.schema.json`](../../../schemas/release/whats_new_card.schema.json)
boundary schema.

Each fixture is one `whats_new_card_record` projection. They cover the
acceptance fixtures the contract calls out:

| Fixture | `card_class` | What it pins |
|---|---|---|
| [`post_upgrade_whats_new_card.yaml`](./post_upgrade_whats_new_card.yaml) | `whats_new_post_upgrade` | The post-upgrade what's-new card with `new`, `changed`, `security`, and `deprecated` entries, an all-healthy service-health panel, current freshness, and reopen access from stable navigation. |
| [`breaking_change_notice_before_restart.yaml`](./breaking_change_notice_before_restart.yaml) | `breaking_change_notice` | A breaking-change / migration notice rendered before restart with `migration_required` and `changed` entries, irreversible-after-restart reversibility, and a release-candidate card ref. |
| [`stale_service_health_banner.yaml`](./stale_service_health_banner.yaml) | `stale_status_banner` | A stale service-health banner rendered when the snapshot is warm-cached after the control plane stopped responding; the four service tiers stay separate and the local row stays healthy. |
| [`cached_release_note_feed.yaml`](./cached_release_note_feed.yaml) | `update_detail_view` | An update-detail view rendered from a mirrored / offline-bundle release-note feed with `imported_historical` freshness and an explicit stale label. |
| [`admin_note_with_affected_groups.yaml`](./admin_note_with_affected_groups.yaml) | `admin_note` | An admin note routed to fleet and security-officer audiences naming affected fleet segments, tenants, and policy class, plus an `admin_action_required` change entry. |
