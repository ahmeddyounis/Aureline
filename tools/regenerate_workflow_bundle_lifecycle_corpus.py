#!/usr/bin/env python3
"""Regenerate (or verify) the workflow-bundle lifecycle conformance corpus.

This generator is the single source for the workflow-bundle lifecycle corpus:
the positive/negative `WorkflowBundleReviewRecord` fixtures, the corpus manifest,
the certification freshness matrix, and the lifecycle compatibility report all
come from here so the harness expectations and the published evidence cannot
drift apart.

Usage:
    python3 tools/regenerate_workflow_bundle_lifecycle_corpus.py [--write | --check]

    --write   (default) write the generated files to the repository.
    --check   regenerate in memory and fail if any committed file differs.

The conformance test `cargo test -p aureline-qe --test
workflow_bundle_lifecycle_conformance` replays the committed corpus against the
real `aureline_workspace::bundles::WorkflowBundleReviewRecord` validator.
"""
import argparse
import json
import os
import sys

ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
CORPUS_REL = "fixtures/workspace/m3/workflow_bundle_lifecycle"
CERT_REL = "artifacts/cert/m3/workflow_bundle_certification_matrix.json"
COMPAT_REL = "artifacts/compat/m3/workflow_bundle_lifecycle_report.md"

SCHEMA_REL = "../../../../schemas/workspace/workflow_bundle_review.schema.json"
RANGE = ">=1.0.0 <2.0.0"
CORPUS_ID = "workspace.workflow_bundle_lifecycle.beta"

SOURCE_LEGEND = [
    {"source_class": "certified", "display_label": "Certified",
     "caveat_ref": "legend:workflow_bundle.source.certified"},
    {"source_class": "managed_approved", "display_label": "Managed approved",
     "caveat_ref": "legend:workflow_bundle.source.managed_approved"},
    {"source_class": "community", "display_label": "Community",
     "caveat_ref": "legend:workflow_bundle.source.community"},
    {"source_class": "imported", "display_label": "Imported",
     "caveat_ref": "legend:workflow_bundle.source.imported"},
    {"source_class": "local_draft", "display_label": "Local draft",
     "caveat_ref": "legend:workflow_bundle.source.local_draft"},
]

CONSUMER_SURFACES = ["start_center", "bundle_detail", "cli_headless",
                     "diagnostics", "support_export", "docs_workspace"]

GUARDRAILS_OK = {
    "providers_recommended_only": True,
    "remote_modes_recommended_only": True,
    "templates_recommended_only": True,
    "workspace_trust_widened": False,
    "network_egress_widened_without_review": False,
    "policy_scope_widened_without_review": False,
    "approval_defaults_widened_without_review": False,
}

INVARIANTS_OK = {
    "diff_before_apply": True,
    "rollback_reviewed": True,
    "removal_preserves_user_assets": True,
    "cli_headless_parity": True,
    "diagnostics_export_parity": True,
    "offline_mirror_truth_preserved": True,
    "no_hidden_imperative_hooks": True,
    "no_raw_secret_injection": True,
}


def item(ns, ref, cls, ownership, src):
    return {
        "item_ref": f"component:{cls}.{ns}.{ref}",
        "item_class": cls,
        "ownership_class": ownership,
        "source_class": src,
        "summary_ref": f"summary:{cls}.{ns}.{ref}",
        "revision_ref": f"revision:{cls}.{ns}.{ref}.r1",
        "mirrorable": True,
        "disclosure_required": True,
    }


def detail(ns, src):
    return {
        "extension_sets": [item(ns, "core", "extension_set", "bundle_owned", src)],
        "presets": [
            item(ns, "editor", "profile_preset", "shared_user_overlay_on_bundle", src),
            item(ns, "zone", "surface_preset", "bundle_owned", src),
        ],
        "task_launch_debug_recipes": [
            item(ns, "dev_server", "task_recipe", "bundle_owned", src),
            item(ns, "dev", "launch_recipe", "bundle_owned", src),
            item(ns, "attach", "debug_recipe", "bundle_owned", src),
        ],
        "docs_tour_packs": [
            item(ns, "guide", "docs_pack", "bundle_owned", src),
            item(ns, "first_edit", "tour_pack", "bundle_owned", src),
        ],
        "template_refs": [item(ns, "minimal", "template_or_scaffold_ref",
                               "shared_user_overlay_on_bundle", src)],
        "migration_mappings": [item(ns, "from_prior", "migration_mapping",
                                    "shared_user_overlay_on_bundle", src)],
        "certification_targets": [item(ns, "proof", "certification_target",
                                       "bundle_owned", src)],
    }


def diff(axis, kind, subject_kind, subject_ref, ownership, ns,
         before=None, after=None, local_override=None):
    e = {
        "change_axis": axis,
        "change_kind": kind,
        "subject_kind": subject_kind,
        "subject_ref": subject_ref,
        "ownership_class": ownership,
        "routes_through_preview_ref": f"bundle_change_preview:{ns}.update.001",
        "disclosure_required": True,
        "keyboard_reachable": True,
    }
    if before is not None:
        e["before_ref"] = before
    if after is not None:
        e["after_ref"] = after
    if local_override is not None:
        e["local_override_ref"] = local_override
    return e


def required_diff_entries(ns, src):
    """All twelve required install/update diff axes."""
    return [
        diff("extension_set", "revision_bumped", "extension_package_set",
             f"component:extension_set.{ns}.core", "bundle_owned", ns,
             before=f"revision:extension_set.{ns}.core.r0",
             after=f"revision:extension_set.{ns}.core.r1"),
        diff("profile_preset", "preserved_local", "profile_preset",
             f"component:profile_preset.{ns}.editor", "shared_user_overlay_on_bundle", ns,
             before=f"revision:profile_preset.{ns}.editor.r0",
             after=f"revision:profile_preset.{ns}.editor.r1",
             local_override=f"retained_override:{ns}.editor.format_on_save"),
        diff("surface_preset", "changed", "surface_preset",
             f"component:surface_preset.{ns}.zone", "bundle_owned", ns,
             before=f"revision:surface_preset.{ns}.zone.r0",
             after=f"revision:surface_preset.{ns}.zone.r1"),
        diff("settings_or_token", "preserved_local", "settings_field",
             f"settings_path:{ns}.editor.format_on_save", "shared_user_overlay_on_bundle", ns,
             before="value_class:present_workspace_set",
             after="value_class:present_user_set",
             local_override=f"retained_override:{ns}.editor.format_on_save"),
        diff("task_recipe", "changed", "task_recipe",
             f"component:task_recipe.{ns}.dev_server", "bundle_owned", ns,
             before=f"revision:task_recipe.{ns}.dev_server.r0",
             after=f"revision:task_recipe.{ns}.dev_server.r1"),
        diff("launch_recipe", "added", "launch_recipe",
             f"component:launch_recipe.{ns}.dev", "bundle_owned", ns,
             after=f"revision:launch_recipe.{ns}.dev.r1"),
        diff("debug_recipe", "unchanged_visible", "debug_recipe",
             f"component:debug_recipe.{ns}.attach", "bundle_owned", ns,
             before=f"revision:debug_recipe.{ns}.attach.r1",
             after=f"revision:debug_recipe.{ns}.attach.r1"),
        diff("docs_pack", "revision_bumped", "docs_pack",
             f"component:docs_pack.{ns}.guide", "bundle_owned", ns,
             before=f"revision:docs_pack.{ns}.guide.r0",
             after=f"revision:docs_pack.{ns}.guide.r1"),
        diff("tour_pack", "unchanged_visible", "tour_pack",
             f"component:tour_pack.{ns}.first_edit", "bundle_owned", ns,
             before=f"revision:tour_pack.{ns}.first_edit.r1",
             after=f"revision:tour_pack.{ns}.first_edit.r1"),
        diff("template_or_scaffold_ref", "unchanged_visible", "template_ref",
             f"component:template_or_scaffold_ref.{ns}.minimal", "shared_user_overlay_on_bundle", ns,
             before=f"revision:template_or_scaffold_ref.{ns}.minimal.r1",
             after=f"revision:template_or_scaffold_ref.{ns}.minimal.r1"),
        diff("migration_mapping", "changed", "migration_mapping",
             f"component:migration_mapping.{ns}.from_prior", "shared_user_overlay_on_bundle", ns,
             before=f"revision:migration_mapping.{ns}.from_prior.r0",
             after=f"revision:migration_mapping.{ns}.from_prior.r1"),
        diff("certification_target", "revision_bumped", "certification_target",
             f"component:certification_target.{ns}.proof", "bundle_owned", ns,
             before=f"revision:certification_target.{ns}.proof.r0",
             after=f"revision:certification_target.{ns}.proof.r1"),
    ]


def review_actions(ns, confirm_state="enabled", confirm_reason=None):
    confirm = {
        "action_id": "review.confirm",
        "rendered_state": confirm_state,
        "destination_ref": f"apply:{ns}.update",
        "keyboard_reachable": True,
    }
    if confirm_reason is not None:
        confirm["disabled_reason_code"] = confirm_reason
    return [
        {"action_id": "review.compare", "rendered_state": "enabled",
         "destination_ref": f"compare_report:{ns}.update", "keyboard_reachable": True},
        confirm,
        {"action_id": "review.cancel", "rendered_state": "enabled",
         "destination_ref": f"cancel:{ns}.update", "keyboard_reachable": True},
        {"action_id": "review.set_up_later", "rendered_state": "enabled",
         "destination_ref": f"defer:{ns}.update", "keyboard_reachable": True},
        {"action_id": "review.inspect_change_source", "rendered_state": "enabled",
         "destination_ref": f"inspect:{ns}.update", "keyboard_reachable": True},
        {"action_id": "review.create_rollback_checkpoint", "rendered_state": "enabled",
         "destination_ref": f"workspace_checkpoint:{ns}.pre_update", "keyboard_reachable": True},
    ]


def resolve_actions(ns, slug, include_ignore=True, adopt_state="enabled"):
    acts = [
        {"action_id": "resolve.keep_local", "rendered_state": "enabled",
         "destination_ref": f"ack:{ns}.keep_local.{slug}", "keyboard_reachable": True},
        {"action_id": "resolve.compare", "rendered_state": "enabled",
         "destination_ref": f"compare_report:{ns}.{slug}", "keyboard_reachable": True},
    ]
    adopt = {"action_id": "resolve.adopt_bundle", "rendered_state": adopt_state,
             "destination_ref": f"bundle_change_preview:{ns}.adopt.{slug}.001",
             "keyboard_reachable": True}
    if adopt_state == "visible_disabled":
        adopt["disabled_reason_code"] = "trust_review_required"
    acts.append(adopt)
    acts.append({"action_id": "resolve.rebase_to_bundle", "rendered_state": "enabled",
                 "destination_ref": f"bundle_change_preview:{ns}.rebase.{slug}.001",
                 "keyboard_reachable": True})
    if include_ignore:
        acts.append({"action_id": "resolve.ignore_this_drift", "rendered_state": "enabled",
                     "destination_ref": f"ack:{ns}.ignore.{slug}", "keyboard_reachable": True})
    return acts


def base_drift_entries(ns):
    return [
        {"drift_state_class": "local_override", "drift_axis": "settings_or_token",
         "subject_granularity_class": "field",
         "subject_ref": f"settings_path:{ns}.editor.format_on_save",
         "asset_ownership_class": "shared_user_overlay_on_bundle",
         "claim_narrowing_class": "no_narrowing_informational",
         "preserved_local_override_ref": f"retained_override:{ns}.editor.format_on_save",
         "resolve_actions": resolve_actions(ns, "editor_format_on_save")},
        {"drift_state_class": "missing_artifact", "drift_axis": "extension_set",
         "subject_granularity_class": "package",
         "subject_ref": f"component:extension_set.{ns}.core",
         "asset_ownership_class": "bundle_owned",
         "claim_narrowing_class": "narrows_certification_target_pending_retest",
         "resolve_actions": resolve_actions(ns, "missing_extension", include_ignore=False)},
    ]


def base_removable_assets(ns):
    return [
        {"asset_ref": f"removable_asset:{ns}.extension_set",
         "asset_kind": "extension_set", "ownership_class": "bundle_owned",
         "safe_to_remove_class": "safe_to_remove_no_user_data",
         "review_required": False, "explicit_reviewed": True},
        {"asset_ref": f"removable_asset:{ns}.editor_preset",
         "asset_kind": "profile_preset", "ownership_class": "shared_user_overlay_on_bundle",
         "safe_to_remove_class": "safe_to_remove_user_overlay_preserved",
         "review_required": False, "explicit_reviewed": True},
        {"asset_ref": f"removable_asset:{ns}.user_scaffold_output",
         "asset_kind": "template_or_scaffold_ref", "ownership_class": "user_owned",
         "safe_to_remove_class": "not_safe_to_remove_user_owned",
         "review_required": True, "explicit_reviewed": True},
    ]


def retained_overrides(ns):
    return [
        {"override_ref": f"retained_override:{ns}.editor.format_on_save",
         "retained_class": "override_retained_in_user_scope", "target_scope_class": "user_scope"},
        {"override_ref": f"retained_override:{ns}.user_authored_files",
         "retained_class": "override_inlined_to_user_authored_record",
         "target_scope_class": "workspace_scope"},
    ]


def remove_actions(ns):
    acts = review_actions(ns)
    for a in acts:
        a["destination_ref"] = a["destination_ref"].replace("update", "remove").replace("apply:", "remove:")
    return acts


def rollback_checkpoint(ns):
    return {
        "linkage_class": "single_attributable_workspace_checkpoint",
        "rollback_path_class": "single_checkpoint_revert",
        "checkpoint_ref": f"workspace_checkpoint:{ns}.pre_update",
        "restorable_axes": [
            "extensions_installed_or_activated",
            "settings_or_token_overlays",
            "task_run_test_debug_recipes",
            "filesystem_scaffolded_files",
            "compatibility_or_runtime_pinning",
        ],
        "attributable_to_review": True,
    }


def side_effects(ns):
    return [
        {"side_effect_class": "extension_install_or_activate", "scope_class": "workspace_scope",
         "summary_ref": f"side_effect:{ns}.extension_install", "reversible_in_rollback": True},
        {"side_effect_class": "settings_write_workspace_scope", "scope_class": "workspace_scope",
         "summary_ref": f"side_effect:{ns}.settings_write", "reversible_in_rollback": True},
        {"side_effect_class": "rollback_checkpoint_creation", "scope_class": "workspace_scope",
         "summary_ref": f"side_effect:{ns}.rollback_checkpoint", "reversible_in_rollback": True},
    ]


def build_record(*, review_id, ns, bundle_id, bundle_class, source_class, status_class,
                 support_class, signer_source_class, signer_ref, channel,
                 evidence_freshness_class, certification_state_class, effective_badge_class,
                 support_claim_class, retest_required, posture_class,
                 source_registry_ref, mirror_ref=None, offline_pack_ref=None,
                 reference_workspace_ref=None, imported_source_ref=None,
                 confirm_state="enabled", confirm_reason=None,
                 extra_compat_refs=None, extra_diff_entries=None, extra_drift_entries=None,
                 removable_assets=None, extra_support_refs=None, minted_at="2026-05-20T12:00:00Z"):
    compat_refs = [
        f"certification_sheet:{ns}",
        f"compatibility_report:{ns}.beta",
        f"benchmark_packet:{ns}.first_useful_edit",
    ]
    if extra_compat_refs:
        compat_refs += extra_compat_refs

    cert = {
        "source_badge_class": source_class,
        "evidence_freshness_class": evidence_freshness_class,
        "certification_state_class": certification_state_class,
        "effective_badge_class": effective_badge_class,
        "support_claim_class": support_claim_class,
        "compatibility_evidence_refs": compat_refs,
        "retest_required": retest_required,
    }
    if reference_workspace_ref:
        cert["reference_workspace_ref"] = reference_workspace_ref

    identity = {
        "bundle_id": bundle_id,
        "bundle_revision": 2,
        "bundle_revision_semver": "1.1.0",
        "bundle_class": bundle_class,
        "bundle_source_class": source_class,
        "bundle_status_class": status_class,
        "support_class": support_class,
        "signer_source_class": signer_source_class,
        "signer_ref": signer_ref,
        "manifest_ref": f"manifest:{ns}.bundle",
        "manifest_digest_ref": f"digest:{ns}.bundle.r2",
        "compatible_aureline_range": RANGE,
        "channel": channel,
    }
    if imported_source_ref:
        identity["imported_source_ref"] = imported_source_ref

    diff_entries = required_diff_entries(ns, source_class)
    if extra_diff_entries:
        diff_entries += extra_diff_entries

    drift_entries = base_drift_entries(ns)
    if extra_drift_entries:
        drift_entries += extra_drift_entries

    mirror = {
        "posture_class": posture_class,
        "source_registry_ref": source_registry_ref,
        "signer_ref": signer_ref,
        "compatible_aureline_range": RANGE,
        "retest_needed_preserved": True,
        "offline_restore_review_ref": f"offline_restore_review:{ns}.{posture_class}",
    }
    if mirror_ref:
        mirror["mirror_ref"] = mirror_ref
    if offline_pack_ref:
        mirror["offline_pack_ref"] = offline_pack_ref

    support_export = {
        "export_packet_refs": [
            f"support_export:{ns}.review",
            f"support_export:{ns}.drift",
            f"support_export:{ns}.remove_review",
        ],
        "diagnostics_refs": [f"diagnostics:{ns}.review"],
        "cli_headless_refs": [f"cli:aureline.bundle.review.{ns}"],
        "raw_secret_export_allowed": False,
        "raw_user_content_export_allowed": False,
        "raw_paths_export_allowed": False,
        "redaction_class": "metadata_safe_default",
    }
    if extra_support_refs:
        support_export["diagnostics_refs"] += extra_support_refs

    return {
        "$schema": SCHEMA_REL,
        "record_kind": "workflow_bundle_review_beta_record",
        "schema_version": 1,
        "review_id": review_id,
        "minted_at": minted_at,
        "bundle_identity": identity,
        "source_class_legend": SOURCE_LEGEND,
        "detail": detail(ns, source_class),
        "certification": cert,
        "install_update_review": {
            "review_state_class": "preview_validated_ready_to_confirm"
            if confirm_state == "enabled" else "preview_blocked_review_required",
            "preview_refs": [
                f"bundle_change_preview:{ns}.fresh_install.001",
                f"bundle_change_preview:{ns}.update.001",
            ],
            "diff_entries": diff_entries,
            "actions": review_actions(ns, confirm_state, confirm_reason),
            "rollback_checkpoint": rollback_checkpoint(ns),
            "side_effects": side_effects(ns),
        },
        "drift_override_review": {
            "drift_state_class": "local_override_and_missing_artifact",
            "field_package_task_granular": True,
            "drift_entries": drift_entries,
        },
        "remove_rollback_review": {
            "review_state_class": "remove_review_classified_ready_to_confirm",
            "remove_review_ref": f"remove_bundle_review:{ns}.001",
            "rollback_target_ref": f"revision:{ns}.bundle.r1",
            "rollback_checkpoint_ref": f"workspace_checkpoint:{ns}.pre_update",
            "removable_assets": removable_assets if removable_assets else base_removable_assets(ns),
            "retained_local_overrides": retained_overrides(ns),
            "actions": remove_actions(ns),
        },
        "mirror_offline": mirror,
        "support_export": support_export,
        "consumer_surfaces": CONSUMER_SURFACES,
        "guardrails": dict(GUARDRAILS_OK),
        "review_invariants": dict(INVARIANTS_OK),
    }


def marker_diff(ns, axis, marker):
    return diff(axis, "changed", "capability_dependency_marker", marker, "bundle_owned", ns,
                before=f"{marker}.r0", after=f"{marker}.r1")


def marker_drift(ns, marker, narrowing):
    return {
        "drift_state_class": "dependency_gated", "drift_axis": "compatibility_or_runtime",
        "subject_granularity_class": "component", "subject_ref": marker,
        "asset_ownership_class": "bundle_owned", "claim_narrowing_class": narrowing,
        "resolve_actions": resolve_actions(ns, "dependency", include_ignore=False),
    }


def build_corpus():
    """Returns (positive, negative) drill tuples."""
    positive = []
    negative = []

    def add_pos(drill_id, filename, record, exp, meta):
        positive.append((drill_id, filename, record, exp, meta))

    # 1. Certified launch wedge — full lifecycle, live or mirror, fresh evidence.
    ns = "launch_bundle.typescript_web_app"
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.certified_full_lifecycle",
        ns=ns, bundle_id=f"launch_bundle:{ns}.seed", bundle_class="launch_bundle",
        source_class="certified", status_class="certified_current",
        support_class="officially_supported", signer_source_class="core_signing_root",
        signer_ref="signer:aureline.core.workflow_bundles", channel="beta",
        evidence_freshness_class="fresh_current", certification_state_class="certified_current",
        effective_badge_class="certified", support_claim_class="stable_launch_wedge_claim",
        retest_required=False, posture_class="live_or_mirror",
        source_registry_ref="registry:aureline.public.workflow_bundles",
        mirror_ref="mirror:corp.workflow_bundles.approved",
        reference_workspace_ref="reference_workspace:ts_web_app_or_service.beta")
    add_pos("certified.full_lifecycle_live_or_mirror",
            "positive/certified_install_update_rebase_keep_local.json", rec,
            {"bundle_class": "launch_bundle", "source_class": "certified",
             "status_class": "certified_current", "support_class": "officially_supported",
             "effective_badge_class": "certified", "support_claim_class": "stable_launch_wedge_claim",
             "evidence_freshness_class": "fresh_current", "certification_state_class": "certified_current",
             "retest_required": False, "mirror_posture_class": "live_or_mirror",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 2, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm",
                                        "review.create_rollback_checkpoint"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.adopt_bundle",
                                         "resolve.rebase_to_bundle", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [], "lifecycle_sensitive_dependencies": []},
            {"lifecycle_flows": ["install", "update", "rebase_adopt", "keep_local",
                                 "remove_rollback", "drift_banner"]})

    # 2. Managed approved — mirror-only catalog, fresh evidence.
    ns = "org_bundle.platform_baseline"
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.managed_mirror_only",
        ns=ns, bundle_id=f"org_approved_bundle:{ns}.seed", bundle_class="org_approved_bundle",
        source_class="managed_approved", status_class="managed_approved_current",
        support_class="officially_supported", signer_source_class="org_signing_root",
        signer_ref="signer:corp.platform.workflow_bundles", channel="managed",
        evidence_freshness_class="fresh_current",
        certification_state_class="managed_approved_current",
        effective_badge_class="managed_approved", support_claim_class="managed_org_claim",
        retest_required=False, posture_class="mirror_only",
        source_registry_ref="registry:corp.platform.workflow_bundles",
        mirror_ref="mirror:corp.platform.workflow_bundles.approved")
    add_pos("managed_approved.mirror_only_update",
            "positive/managed_approved_mirror_only_update.json", rec,
            {"bundle_class": "org_approved_bundle", "source_class": "managed_approved",
             "status_class": "managed_approved_current", "support_class": "officially_supported",
             "effective_badge_class": "managed_approved", "support_claim_class": "managed_org_claim",
             "evidence_freshness_class": "fresh_current",
             "certification_state_class": "managed_approved_current",
             "retest_required": False, "mirror_posture_class": "mirror_only",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 2, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare",
                                         "resolve.rebase_to_bundle"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [], "lifecycle_sensitive_dependencies": []},
            {"lifecycle_flows": ["install", "update", "keep_local", "remove_rollback",
                                 "drift_banner", "mirror_only"]})

    # 3. Community — offline install, experimental support, evidence unknown.
    ns = "community_bundle.rust_cli_starter"
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.community_offline",
        ns=ns, bundle_id=f"community_bundle:{ns}.seed", bundle_class="design_partner_bundle",
        source_class="community", status_class="community_unreviewed",
        support_class="experimental", signer_source_class="community_publisher",
        signer_ref="signer:community.rust_cli_starter.publisher", channel="community",
        evidence_freshness_class="evidence_unknown",
        certification_state_class="community_unverified",
        effective_badge_class="community", support_claim_class="community_no_certification_claim",
        retest_required=False, posture_class="signed_offline_bundle",
        source_registry_ref="registry:community.workflow_bundles",
        offline_pack_ref="offline_pack:community.rust_cli_starter.signed")
    add_pos("community.offline_install_experimental",
            "positive/community_offline_install.json", rec,
            {"bundle_class": "design_partner_bundle", "source_class": "community",
             "status_class": "community_unreviewed", "support_class": "experimental",
             "effective_badge_class": "community",
             "support_claim_class": "community_no_certification_claim",
             "evidence_freshness_class": "evidence_unknown",
             "certification_state_class": "community_unverified",
             "retest_required": False, "mirror_posture_class": "signed_offline_bundle",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 2, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [], "lifecycle_sensitive_dependencies": []},
            {"lifecycle_flows": ["install", "keep_local", "remove_rollback", "drift_banner",
                                 "offline_install"]})

    # 4. Imported user bundle — round-trip, preserves user-owned assets, carries markers.
    ns = "imported_bundle.vscode_react_user"
    imp_markers = [
        "capability_marker:imported_keymap.community_supported",
        "capability_marker:imported_webview.host_specific",
    ]
    imp_lifecycle = ["lifecycle_dependency:imported_extension.block_apply_preserve_data"]
    imp_removable = [
        {"asset_ref": f"removable_asset:{ns}.keymap", "asset_kind": "profile_preset",
         "ownership_class": "user_owned", "safe_to_remove_class": "not_safe_to_remove_user_owned",
         "review_required": True, "explicit_reviewed": True},
        {"asset_ref": f"removable_asset:{ns}.migration_report", "asset_kind": "migration_mapping",
         "ownership_class": "user_owned", "safe_to_remove_class": "not_safe_to_remove_user_owned",
         "review_required": True, "explicit_reviewed": True},
        {"asset_ref": f"removable_asset:{ns}.native_recommendation",
         "asset_kind": "template_or_scaffold_ref", "ownership_class": "shared_user_overlay_on_bundle",
         "safe_to_remove_class": "safe_to_remove_user_overlay_preserved",
         "review_required": False, "explicit_reviewed": True},
    ]
    extra_diff = [marker_diff(ns, "trust_or_permission", imp_markers[0]),
                  marker_diff(ns, "compatibility_or_runtime", imp_markers[1]),
                  marker_diff(ns, "compatibility_or_runtime", imp_lifecycle[0])]
    extra_drift = [marker_drift(ns, imp_lifecycle[0], "narrows_to_imported_pending_review")]
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.imported_round_trip",
        ns=ns, bundle_id=f"imported_bundle:{ns}.seed", bundle_class="imported_user_bundle",
        source_class="imported", status_class="imported_pending_review",
        support_class="support_unknown", signer_source_class="local_user_trust_only",
        signer_ref="signer:imported.vscode_react_user.local", channel="imported",
        evidence_freshness_class="imported_evidence",
        certification_state_class="imported_pending_review",
        effective_badge_class="imported", support_claim_class="imported_pending_review_claim",
        retest_required=True, posture_class="signed_offline_bundle",
        source_registry_ref="migration_source:vscode_react_user",
        offline_pack_ref="offline_pack:imported.vscode_react_user.review_only",
        imported_source_ref="migration_source:vscode_react_user",
        confirm_state="visible_disabled", confirm_reason="trust_review_required",
        extra_compat_refs=imp_markers + imp_lifecycle,
        extra_diff_entries=extra_diff, extra_drift_entries=extra_drift,
        removable_assets=imp_removable,
        extra_support_refs=imp_markers + imp_lifecycle)
    add_pos("imported.round_trip_preserves_markers",
            "positive/imported_user_round_trip.json", rec,
            {"bundle_class": "imported_user_bundle", "source_class": "imported",
             "status_class": "imported_pending_review", "support_class": "support_unknown",
             "effective_badge_class": "imported",
             "support_claim_class": "imported_pending_review_claim",
             "evidence_freshness_class": "imported_evidence",
             "certification_state_class": "imported_pending_review",
             "retest_required": True, "mirror_posture_class": "signed_offline_bundle",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 3, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": imp_markers,
             "lifecycle_sensitive_dependencies": imp_lifecycle},
            {"lifecycle_flows": ["install", "keep_local", "remove_rollback", "drift_banner",
                                 "offline_install", "imported_round_trip"]})

    # 5. Local draft — keep-local, no claim, live origin only.
    ns = "local_draft.my_workflow"
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.local_draft_keep_local",
        ns=ns, bundle_id=f"local_draft_bundle:{ns}.seed", bundle_class="local_draft_bundle",
        source_class="local_draft", status_class="local_draft",
        support_class="support_unknown", signer_source_class="local_user_trust_only",
        signer_ref="signer:local_draft.my_workflow.local", channel="local",
        evidence_freshness_class="evidence_unknown", certification_state_class="local_draft",
        effective_badge_class="local_draft", support_claim_class="local_draft_no_claim",
        retest_required=False, posture_class="live_origin_only",
        source_registry_ref="local_workspace:my_workflow.draft")
    add_pos("local_draft.keep_local_no_claim",
            "positive/local_draft_keep_local.json", rec,
            {"bundle_class": "local_draft_bundle", "source_class": "local_draft",
             "status_class": "local_draft", "support_class": "support_unknown",
             "effective_badge_class": "local_draft", "support_claim_class": "local_draft_no_claim",
             "evidence_freshness_class": "evidence_unknown", "certification_state_class": "local_draft",
             "retest_required": False, "mirror_posture_class": "live_origin_only",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 2, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [], "lifecycle_sensitive_dependencies": []},
            {"lifecycle_flows": ["install", "keep_local", "remove_rollback", "drift_banner"]})

    # 6. Certified, stale evidence -> Retest pending downgrade.
    ns = "launch_bundle.python_service"
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.certified_stale_retest_pending",
        ns=ns, bundle_id=f"launch_bundle:{ns}.seed", bundle_class="launch_bundle",
        source_class="certified", status_class="certified_retest_pending",
        support_class="officially_supported", signer_source_class="core_signing_root",
        signer_ref="signer:aureline.core.workflow_bundles", channel="beta",
        evidence_freshness_class="stale_past_window", certification_state_class="retest_pending",
        effective_badge_class="retest_pending", support_claim_class="limited_retest_pending_claim",
        retest_required=True, posture_class="live_or_mirror",
        source_registry_ref="registry:aureline.public.workflow_bundles",
        mirror_ref="mirror:corp.workflow_bundles.approved",
        reference_workspace_ref="reference_workspace:python_service.beta")
    add_pos("certified.stale_evidence_retest_pending",
            "positive/certified_stale_evidence_retest_pending.json", rec,
            {"bundle_class": "launch_bundle", "source_class": "certified",
             "status_class": "certified_retest_pending", "support_class": "officially_supported",
             "effective_badge_class": "retest_pending",
             "support_claim_class": "limited_retest_pending_claim",
             "evidence_freshness_class": "stale_past_window", "certification_state_class": "retest_pending",
             "retest_required": True, "mirror_posture_class": "live_or_mirror",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 2, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [], "lifecycle_sensitive_dependencies": []},
            {"lifecycle_flows": ["update", "drift_banner", "remove_rollback",
                                 "stale_evidence_downgrade"]})

    # 7. Managed approved, stale evidence + stale mirror + stale dependency -> Limited.
    ns = "org_bundle.data_platform"
    dep_marker = "capability_marker:remote_indexing.beta_only"
    lifecycle_dep = "lifecycle_dependency:certification_evidence.retest_window"
    extra_diff = [marker_diff(ns, "compatibility_or_runtime", dep_marker),
                  marker_diff(ns, "compatibility_or_runtime", lifecycle_dep)]
    extra_drift = [marker_drift(ns, lifecycle_dep, "narrows_until_dependency_satisfied")]
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.managed_stale_limited",
        ns=ns, bundle_id=f"org_approved_bundle:{ns}.seed", bundle_class="org_approved_bundle",
        source_class="managed_approved", status_class="managed_approved_current",
        support_class="legacy_deprecated", signer_source_class="org_signing_root",
        signer_ref="signer:corp.data_platform.workflow_bundles", channel="managed",
        evidence_freshness_class="stale_past_window", certification_state_class="evidence_stale",
        effective_badge_class="limited", support_claim_class="managed_org_claim",
        retest_required=True, posture_class="mirror_only",
        source_registry_ref="registry:corp.data_platform.workflow_bundles",
        mirror_ref="mirror:corp.data_platform.workflow_bundles.stale",
        extra_compat_refs=[dep_marker, lifecycle_dep],
        extra_diff_entries=extra_diff, extra_drift_entries=extra_drift,
        extra_support_refs=[dep_marker, lifecycle_dep])
    add_pos("managed_approved.stale_dependency_limited",
            "positive/managed_approved_stale_limited.json", rec,
            {"bundle_class": "org_approved_bundle", "source_class": "managed_approved",
             "status_class": "managed_approved_current", "support_class": "legacy_deprecated",
             "effective_badge_class": "limited", "support_claim_class": "managed_org_claim",
             "evidence_freshness_class": "stale_past_window", "certification_state_class": "evidence_stale",
             "retest_required": True, "mirror_posture_class": "mirror_only",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 3, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": [dep_marker],
             "lifecycle_sensitive_dependencies": [lifecycle_dep]},
            {"lifecycle_flows": ["update", "drift_banner", "remove_rollback",
                                 "mirror_only", "stale_dependency_downgrade", "stale_mirror_downgrade"]})

    # 8. Community — capability/lifecycle dependency-marker propagation across surfaces.
    ns = "community_bundle.ai_assist_pack"
    markers = [
        "capability_marker:provider_completions.policy_gated",
        "capability_marker:companion_pairing.host_specific",
        "capability_marker:labs_inline_chat.labs",
    ]
    lifecycle_deps = [
        "lifecycle_dependency:extension_set.update_sensitive",
        "lifecycle_dependency:provider_link.remove_sensitive",
    ]
    extra_diff = ([marker_diff(ns, "trust_or_permission", markers[0]),
                   marker_diff(ns, "compatibility_or_runtime", markers[1]),
                   marker_diff(ns, "compatibility_or_runtime", markers[2])]
                  + [marker_diff(ns, "compatibility_or_runtime", d) for d in lifecycle_deps])
    extra_drift = [marker_drift(ns, lifecycle_deps[0], "narrows_until_dependency_satisfied"),
                   marker_drift(ns, lifecycle_deps[1], "narrows_until_dependency_satisfied")]
    rec = build_record(
        review_id=f"workflow_bundle_review:{ns}.dependency_marker_propagation",
        ns=ns, bundle_id=f"community_bundle:{ns}.seed", bundle_class="design_partner_bundle",
        source_class="community", status_class="community_reviewed",
        support_class="community_supported", signer_source_class="community_publisher",
        signer_ref="signer:community.ai_assist_pack.publisher", channel="community",
        evidence_freshness_class="aging_within_window",
        certification_state_class="community_unverified",
        effective_badge_class="community", support_claim_class="community_no_certification_claim",
        retest_required=False, posture_class="live_or_mirror",
        source_registry_ref="registry:community.workflow_bundles",
        mirror_ref="mirror:community.workflow_bundles.cache",
        extra_compat_refs=markers + lifecycle_deps,
        extra_diff_entries=extra_diff, extra_drift_entries=extra_drift,
        extra_support_refs=markers + lifecycle_deps)
    add_pos("community.dependency_marker_propagation",
            "positive/community_dependency_marker_propagation.json", rec,
            {"bundle_class": "design_partner_bundle", "source_class": "community",
             "status_class": "community_reviewed", "support_class": "community_supported",
             "effective_badge_class": "community",
             "support_claim_class": "community_no_certification_claim",
             "evidence_freshness_class": "aging_within_window",
             "certification_state_class": "community_unverified",
             "retest_required": False, "mirror_posture_class": "live_or_mirror",
             "required_diff_axes_complete": True, "guardrails_pass": True, "raw_export_allowed": False,
             "drift_entry_count": 4, "removable_asset_count": 3, "retained_override_count": 2,
             "review_actions_present": ["review.compare", "review.confirm"],
             "resolve_actions_present": ["resolve.keep_local", "resolve.compare"],
             "preserves_user_owned_assets": True, "rollback_restores_bundle_owned": True,
             "capability_dependency_markers": markers,
             "lifecycle_sensitive_dependencies": lifecycle_deps},
            {"lifecycle_flows": ["install", "update", "keep_local", "remove_rollback",
                                 "drift_banner", "dependency_markers"]})

    # ---- Negative drills: a valid base record with exactly one rule broken. ----
    def base_certified(ns, review_id):
        return build_record(
            review_id=review_id, ns=ns, bundle_id=f"launch_bundle:{ns}.seed",
            bundle_class="launch_bundle", source_class="certified",
            status_class="certified_current", support_class="officially_supported",
            signer_source_class="core_signing_root",
            signer_ref="signer:aureline.core.workflow_bundles", channel="beta",
            evidence_freshness_class="fresh_current",
            certification_state_class="certified_current", effective_badge_class="certified",
            support_claim_class="stable_launch_wedge_claim", retest_required=False,
            posture_class="live_or_mirror",
            source_registry_ref="registry:aureline.public.workflow_bundles",
            mirror_ref="mirror:corp.workflow_bundles.approved",
            reference_workspace_ref="reference_workspace:negative.beta")

    def base_imported(ns, review_id):
        return build_record(
            review_id=review_id, ns=ns, bundle_id=f"imported_bundle:{ns}.seed",
            bundle_class="imported_user_bundle", source_class="imported",
            status_class="imported_pending_review", support_class="support_unknown",
            signer_source_class="local_user_trust_only",
            signer_ref="signer:imported.negative.local", channel="imported",
            evidence_freshness_class="imported_evidence",
            certification_state_class="imported_pending_review", effective_badge_class="imported",
            support_claim_class="imported_pending_review_claim", retest_required=False,
            posture_class="signed_offline_bundle",
            source_registry_ref="migration_source:negative",
            offline_pack_ref="offline_pack:imported.negative.review_only",
            imported_source_ref="migration_source:negative",
            confirm_state="visible_disabled", confirm_reason="trust_review_required",
            removable_assets=[
                {"asset_ref": "removable_asset:negative.keymap", "asset_kind": "profile_preset",
                 "ownership_class": "user_owned", "safe_to_remove_class": "not_safe_to_remove_user_owned",
                 "review_required": True, "explicit_reviewed": True},
                {"asset_ref": "removable_asset:negative.overlay",
                 "asset_kind": "template_or_scaffold_ref",
                 "ownership_class": "shared_user_overlay_on_bundle",
                 "safe_to_remove_class": "safe_to_remove_user_overlay_preserved",
                 "review_required": False, "explicit_reviewed": True},
            ])

    r = base_certified("negative.stale_badge", "workflow_bundle_review:negative.stale_badge")
    r["certification"]["evidence_freshness_class"] = "stale_past_window"
    r["certification"]["retest_required"] = True
    r["bundle_identity"]["bundle_status_class"] = "certified_retest_pending"
    negative.append(("negative.certified_badge_on_stale_evidence",
                     "negative/certified_badge_on_stale_evidence.json", r,
                     "stale or retest-required evidence cannot render certified",
                     ["badge_downgrade_honesty", "certification_freshness"]))

    r = base_imported("negative.delete_user_asset", "workflow_bundle_review:negative.delete_user_asset")
    r["remove_rollback_review"]["removable_assets"][0]["safe_to_remove_class"] = "safe_to_remove_no_user_data"
    negative.append(("negative.removal_marks_user_asset_safe",
                     "negative/removal_marks_user_asset_safe.json", r,
                     "user_owned removable assets must be not_safe_to_remove_user_owned",
                     ["removal_preserves_user_assets", "ownership_truth"]))

    r = base_certified("negative.widen_trust", "workflow_bundle_review:negative.widen_trust")
    r["guardrails"]["workspace_trust_widened"] = True
    negative.append(("negative.guardrail_widens_workspace_trust",
                     "negative/guardrail_widens_workspace_trust.json", r,
                     "guardrails forbid silent",
                     ["no_silent_trust_widening", "guardrails"]))

    r = base_imported("negative.imported_overclaim", "workflow_bundle_review:negative.imported_overclaim")
    r["certification"]["effective_badge_class"] = "certified"
    negative.append(("negative.imported_overclaims_certified",
                     "negative/imported_overclaims_certified.json", r,
                     "imported source must remain imported pending review",
                     ["imported_provenance", "certification_truth"]))

    r = base_certified("negative.adopt_no_preview", "workflow_bundle_review:negative.adopt_no_preview")
    for entry in r["drift_override_review"]["drift_entries"]:
        for act in entry["resolve_actions"]:
            if act["action_id"] == "resolve.adopt_bundle":
                act["destination_ref"] = "ack:not_a_preview"
    negative.append(("negative.adopt_skips_change_preview",
                     "negative/adopt_skips_change_preview.json", r,
                     "route through bundle_change_preview",
                     ["reviewable_before_apply", "drift_resolution"]))

    r = base_certified("negative.missing_axis", "workflow_bundle_review:negative.missing_axis")
    r["install_update_review"]["diff_entries"] = [
        e for e in r["install_update_review"]["diff_entries"]
        if e["change_axis"] != "certification_target"]
    negative.append(("negative.install_missing_certification_axis",
                     "negative/install_missing_certification_axis.json", r,
                     "missing required axes",
                     ["diff_completeness", "reviewable_before_apply"]))

    r = base_certified("negative.raw_secret", "workflow_bundle_review:negative.raw_secret")
    r["support_export"]["raw_secret_export_allowed"] = True
    negative.append(("negative.support_export_enables_raw_secret",
                     "negative/support_export_enables_raw_secret.json", r,
                     "support_export raw export booleans must remain false",
                     ["no_raw_secret_export", "support_safe"]))

    return positive, negative


def json_bytes(obj):
    return json.dumps(obj, indent=2) + "\n"


def build_outputs():
    """Returns a dict of repo-relative path -> file content string."""
    positive, negative = build_corpus()
    outputs = {}

    for drill_id, filename, record, exp, meta in positive:
        outputs[f"{CORPUS_REL}/{filename}"] = json_bytes(record)
    for drill_id, filename, record, substring, covers in negative:
        outputs[f"{CORPUS_REL}/{filename}"] = json_bytes(record)

    manifest = {
        "$schema": "../../../../schemas/workspace/workflow_bundle_conformance.schema.json",
        "corpus_id": CORPUS_ID,
        "schema_version": 1,
        "description": (
            "Conformance, interoperability, certification-freshness, and failure / "
            "recovery drill corpus for the M3 workflow-bundle lifecycle beta boundary owned "
            "by aureline-workspace::bundles (WorkflowBundleReviewRecord). This manifest is the "
            "single source of truth: every positive drill MUST parse, validate, project, and "
            "match every expected_* field below; every negative drill MUST FAIL validation with "
            "an error whose message contains expected_failure_substring. The corpus proves the "
            "M3 workflow-bundle exit-gate: Certified, Managed approved, Community, Imported, and "
            "Local draft bundles stay declarative, reviewable, reversible, and certification-honest "
            "across install, update, rebase/adopt, keep-local, remove/rollback, drift-banner, "
            "mirror-only, and offline lanes; badges downgrade automatically to Limited, Retest "
            "pending, or Experimental support when evidence, dependencies, or mirrors are stale; "
            "capability-dependency and lifecycle-sensitive markers propagate onto bundle cards, "
            "install/update review sheets, export/diagnostics artifacts, and claim-manifest rows; "
            "removal preserves user-created assets while bundle-owned state stays restorable; and "
            "no bundle path silently widens workspace trust, network egress, policy scope, or "
            "approval defaults."),
        "boundary": {
            "runtime_model": "aureline_workspace::bundles::WorkflowBundleReviewRecord",
            "record_schema_ref": "schemas/workspace/workflow_bundle_review.schema.json",
            "certification_matrix_ref": "artifacts/cert/m3/workflow_bundle_certification_matrix.json",
            "compatibility_report_ref": "artifacts/compat/m3/workflow_bundle_lifecycle_report.md",
        },
        "positive_drills": [],
        "negative_drills": [],
    }
    for drill_id, filename, record, exp, meta in positive:
        row = {
            "drill_id": drill_id,
            "fixture": filename,
            "kind": "workflow_bundle_review",
            "source_class": exp["source_class"],
            "bundle_class": exp["bundle_class"],
            "lifecycle_flows": meta["lifecycle_flows"],
        }
        for key in ["bundle_class", "source_class", "status_class", "support_class",
                    "effective_badge_class", "support_claim_class", "evidence_freshness_class",
                    "certification_state_class", "retest_required", "mirror_posture_class",
                    "required_diff_axes_complete", "guardrails_pass", "raw_export_allowed",
                    "drift_entry_count", "removable_asset_count", "retained_override_count",
                    "review_actions_present", "resolve_actions_present",
                    "preserves_user_owned_assets", "rollback_restores_bundle_owned",
                    "capability_dependency_markers", "lifecycle_sensitive_dependencies"]:
            row[f"expected_{key}"] = exp[key]
        manifest["positive_drills"].append(row)
    for drill_id, filename, record, substring, covers in negative:
        manifest["negative_drills"].append({
            "drill_id": drill_id,
            "fixture": filename,
            "kind": "workflow_bundle_review",
            "expected_failure_substring": substring,
            "covers": covers,
        })
    outputs[f"{CORPUS_REL}/manifest.json"] = json_bytes(manifest)

    cert_matrix = {
        "record_kind": "workflow_bundle_certification_matrix",
        "schema_version": 1,
        "corpus_id": CORPUS_ID,
        "minted_at": "2026-05-20T12:00:00Z",
        "description": (
            "Certification freshness matrix for every claimed beta workflow-bundle row in the "
            "workflow-bundle lifecycle conformance corpus. Each row pins the source badge, the "
            "evidence freshness, the effective badge after freshness/dependency/mirror checks, the "
            "support claim it is allowed to imply, and the capability-dependency and "
            "lifecycle-sensitive markers it carries. Rows are generated from the same source as the "
            "corpus fixtures, so the published certification truth cannot drift from the harness."),
        "badge_downgrade_legend": {
            "limited": "Effective badge narrowed because evidence, a dependency, or a mirror is stale.",
            "retest_pending": "Certification evidence is past its window; a retest is required.",
            "experimental": "Community / design-partner support promise; not a certified claim.",
        },
        "rows": [],
    }
    for drill_id, filename, record, exp, meta in positive:
        cert_matrix["rows"].append({
            "drill_id": drill_id,
            "bundle_id": record["bundle_identity"]["bundle_id"],
            "bundle_class": exp["bundle_class"],
            "source_class": exp["source_class"],
            "status_class": exp["status_class"],
            "support_class": exp["support_class"],
            "evidence_freshness_class": exp["evidence_freshness_class"],
            "certification_state_class": exp["certification_state_class"],
            "effective_badge_class": exp["effective_badge_class"],
            "support_claim_class": exp["support_claim_class"],
            "retest_required": exp["retest_required"],
            "mirror_posture_class": exp["mirror_posture_class"],
            "downgraded": exp["effective_badge_class"] in ("limited", "retest_pending")
            or exp["support_class"] == "experimental",
            "capability_dependency_markers": exp["capability_dependency_markers"],
            "lifecycle_sensitive_dependencies": exp["lifecycle_sensitive_dependencies"],
            "fixture_ref": f"fixtures/workspace/m3/workflow_bundle_lifecycle/{filename}",
        })
    outputs[CERT_REL] = json_bytes(cert_matrix)

    lines = []
    lines.append("# Workflow-bundle lifecycle compatibility report")
    lines.append("")
    lines.append(f"- Corpus: `{CORPUS_ID}`")
    lines.append("- Boundary: `aureline_workspace::bundles::WorkflowBundleReviewRecord`")
    lines.append("- Replay: `cargo test -p aureline-qe --test workflow_bundle_lifecycle_conformance`")
    lines.append("- Generated from the same source as the corpus fixtures and the certification")
    lines.append("  freshness matrix; do not hand-edit drill rows.")
    lines.append("")
    lines.append("This report is the published evidence that every claimed beta workflow-bundle row")
    lines.append("stays declarative, reviewable, reversible, and certification-honest across the")
    lines.append("install, update, rebase/adopt, keep-local, remove/rollback, drift-banner,")
    lines.append("mirror-only, and offline lanes. The harness fails CI if any row drifts from the")
    lines.append("pinned truth, if a badge over-claims stale evidence, if removal endangers a")
    lines.append("user-owned asset, if a guardrail widens trust, or if this report or the")
    lines.append("certification matrix stops covering a drill.")
    lines.append("")
    lines.append("## Positive lifecycle rows")
    lines.append("")
    lines.append("| Drill | Source | Effective badge | Support claim | Evidence | Mirror posture | Lifecycle flows |")
    lines.append("| --- | --- | --- | --- | --- | --- | --- |")
    for drill_id, filename, record, exp, meta in positive:
        lines.append(
            f"| `{drill_id}` | {exp['source_class']} | {exp['effective_badge_class']} | "
            f"{exp['support_claim_class']} | {exp['evidence_freshness_class']} | "
            f"{exp['mirror_posture_class']} | {', '.join(meta['lifecycle_flows'])} |")
    lines.append("")
    lines.append("## Automatic badge downgrades")
    lines.append("")
    lines.append("| Drill | Trigger | Effective badge | Support |")
    lines.append("| --- | --- | --- | --- |")
    for drill_id, filename, record, exp, meta in positive:
        if exp["effective_badge_class"] in ("limited", "retest_pending") or exp["support_class"] == "experimental":
            trig = []
            if exp["evidence_freshness_class"] in ("stale_past_window", "evidence_unknown"):
                trig.append("stale/unknown evidence")
            if exp["lifecycle_sensitive_dependencies"]:
                trig.append("lifecycle-sensitive dependency")
            if "stale" in record["mirror_offline"].get("mirror_ref", ""):
                trig.append("stale mirror")
            if exp["support_class"] == "experimental":
                trig.append("experimental support promise")
            lines.append(
                f"| `{drill_id}` | {', '.join(trig) or 'n/a'} | "
                f"{exp['effective_badge_class']} | {exp['support_class']} |")
    lines.append("")
    lines.append("## Dependency-marker propagation")
    lines.append("")
    lines.append("| Drill | Capability markers | Lifecycle-sensitive dependencies |")
    lines.append("| --- | --- | --- |")
    for drill_id, filename, record, exp, meta in positive:
        if exp["capability_dependency_markers"] or exp["lifecycle_sensitive_dependencies"]:
            lines.append(
                f"| `{drill_id}` | {', '.join(f'`{m}`' for m in exp['capability_dependency_markers']) or '—'} | "
                f"{', '.join(f'`{d}`' for d in exp['lifecycle_sensitive_dependencies']) or '—'} |")
    lines.append("")
    lines.append("## Negative drills")
    lines.append("")
    lines.append("| Drill | Rejected because the message contains |")
    lines.append("| --- | --- |")
    for drill_id, filename, record, substring, covers in negative:
        lines.append(f"| `{drill_id}` | `{substring}` |")
    lines.append("")
    outputs[COMPAT_REL] = "\n".join(lines)

    return outputs


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    group = parser.add_mutually_exclusive_group()
    group.add_argument("--write", action="store_true", help="write generated files (default)")
    group.add_argument("--check", action="store_true",
                       help="fail if any committed file differs from the generated output")
    args = parser.parse_args()

    outputs = build_outputs()

    if args.check:
        drift = []
        for rel, content in sorted(outputs.items()):
            path = os.path.join(ROOT, rel)
            try:
                with open(path, "r", encoding="utf-8") as fh:
                    current = fh.read()
            except FileNotFoundError:
                drift.append(f"missing: {rel}")
                continue
            if current != content:
                drift.append(f"out of date: {rel}")
        if drift:
            print("workflow-bundle lifecycle corpus is out of sync with its generator:")
            for line in drift:
                print(f"  - {line}")
            print("run: python3 tools/regenerate_workflow_bundle_lifecycle_corpus.py --write")
            return 1
        print(f"workflow-bundle lifecycle corpus matches generator ({len(outputs)} files)")
        return 0

    for rel, content in sorted(outputs.items()):
        path = os.path.join(ROOT, rel)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w", encoding="utf-8") as fh:
            fh.write(content)
    print(f"wrote {len(outputs)} files for the workflow-bundle lifecycle corpus")
    return 0


if __name__ == "__main__":
    sys.exit(main())
