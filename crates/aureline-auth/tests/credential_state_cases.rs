//! Fixture-driven coverage for the seed credential-state row, locked /
//! unavailable store handling, and provider/account registry seed.
//!
//! Every JSON file matching `fixtures/auth/credential_state_cases/seed_*.json`
//! parses into the seed shape and projects to the same credential-state chip
//! a consumer would render. The failure-drill fixture additionally feeds the
//! seed registry's `lock_store` helper and asserts the saved alias survives
//! while the row flips to the typed locked state.

use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    AccountBoundaryClass, CredentialStateChip, CredentialStateClass, CredentialStateRow,
    CredentialUnavailableReason, IdentityModeAlias, LifetimeClass, ProviderAccountRegistry,
    RetryPathClass, RevokeActionClass, StorageModeClass, StoreSourceClass, TrustState,
    CREDENTIAL_STATE_ROW_RECORD_KIND, CREDENTIAL_STATE_SEED_SCHEMA_VERSION,
    PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/auth/credential_state_cases")
}

fn load_string(file_name: &str) -> String {
    let path = fixture_dir().join(file_name);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()))
}

fn load_row(file_name: &str) -> CredentialStateRow {
    serde_json::from_str(&load_string(file_name))
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a row: {err}"))
}

fn load_registry(file_name: &str) -> ProviderAccountRegistry {
    serde_json::from_str(&load_string(file_name))
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a registry: {err}"))
}

#[test]
fn account_free_local_byok_ai_fixture_parses_and_projects_handle_only_chip() {
    let row = load_row("seed_account_free_local_byok_ai.json");
    assert_eq!(row.record_kind, CREDENTIAL_STATE_ROW_RECORD_KIND);
    assert_eq!(row.schema_version, CREDENTIAL_STATE_SEED_SCHEMA_VERSION);
    assert_eq!(row.state_class, CredentialStateClass::HandleOnly);
    assert_eq!(row.identity_mode, IdentityModeAlias::AccountFreeLocal);
    assert_eq!(row.trust_state, TrustState::Trusted);
    assert_eq!(row.storage.store_source, StoreSourceClass::OsKeychain);
    assert_eq!(
        row.storage.storage_mode,
        StorageModeClass::SystemCredentialStore
    );
    assert!(row.local_work_continues());
    assert!(!row.is_unavailable());
    assert!(!row.requires_visible_recovery());

    let chip = CredentialStateChip::from_row(&row);
    assert_eq!(chip.state_class_token, "handle_only");
    assert_eq!(chip.storage_mode_token, "system_credential_store");
    assert_eq!(chip.store_source_token, "os_keychain");
    assert_eq!(
        chip.revoke_action,
        RevokeActionClass::RemoveSavedProviderSession
    );
    assert_eq!(
        chip.primary_recovery_action,
        RetryPathClass::ContinueLocalWithoutSignIn
    );
    assert!(chip.local_work_continues);
    assert!(!chip.visible_recovery_required);
    assert!(!chip.plaintext_fallback_allowed);
    assert!(!chip.raw_secret_material_present);
}

#[test]
fn managed_provider_session_fixture_parses_and_quotes_scope_and_revoke_verbatim() {
    let row = load_row("seed_managed_provider_session.json");
    assert_eq!(row.state_class, CredentialStateClass::HandleOnly);
    assert_eq!(row.identity_mode, IdentityModeAlias::ManagedConvenience);
    assert_eq!(row.scope.scope_label, "payments-prod workspace");
    assert_eq!(
        row.scope.bound_tenant_or_org_ref.as_deref(),
        Some("tenant.acme_prod"),
        "scope tenant ref travels verbatim",
    );
    assert_eq!(
        row.lifetime.revocation_path_label,
        "Remove saved provider session"
    );
    assert_eq!(
        row.lifetime.revoke_action,
        RevokeActionClass::RemoveSavedProviderSession
    );

    let chip = CredentialStateChip::from_row(&row);
    assert_eq!(chip.audience_label, "Managed sign-in refresh");
    assert_eq!(chip.revoke_action_token, "remove_saved_provider_session");
    assert_eq!(
        chip.primary_recovery_action,
        RetryPathClass::RetryInSystemBrowser
    );
}

#[test]
fn failure_drill_fixture_surfaces_locked_state_without_widening_local_path() {
    // Failure drill: the OS keychain locks. The fixture row carries the
    // typed `locked` state class, the `store_locked` unavailable reason, the
    // `resume_after_credential_store_unlock` recovery action, and the saved
    // alias survives so the unlock prompt can resolve the same handle. The
    // shell never silently flips the credential row into `Connected`,
    // never collapses to a generic warning chip, and never falls back to a
    // plaintext-file credential.
    let row = load_row("seed_failure_drill_locked_keychain.json");
    assert_eq!(row.state_class, CredentialStateClass::Locked);
    assert_eq!(
        row.unavailable_reason,
        Some(CredentialUnavailableReason::StoreLocked)
    );
    assert_eq!(
        row.primary_recovery_action,
        RetryPathClass::ResumeAfterCredentialStoreUnlock
    );
    assert_eq!(row.lifetime.lifetime_class, LifetimeClass::Unavailable);
    assert!(row.is_unavailable());
    assert!(row.requires_visible_recovery());
    assert!(
        row.local_work_continues(),
        "no-account local path stays usable when the keychain is locked",
    );
    assert_eq!(
        row.authority_alias_ref.as_deref(),
        Some("credential_alias.managed.payments_prod"),
        "alias survives the lock so the unlock prompt resolves the same handle",
    );
    assert!(!row.storage.plaintext_fallback_allowed);
    assert!(!row.storage.raw_secret_material_present);

    let chip = CredentialStateChip::from_row(&row);
    assert_eq!(chip.state_class_token, "locked");
    assert_eq!(
        chip.unavailable_reason_token.as_deref(),
        Some("store_locked")
    );
    assert!(chip.visible_recovery_required);
    assert!(chip.local_work_continues);
}

#[test]
fn provider_account_registry_fixture_parses_and_joins_rows_to_accounts() {
    let registry = load_registry("seed_provider_account_registry.json");
    assert_eq!(registry.record_kind, PROVIDER_ACCOUNT_REGISTRY_RECORD_KIND);
    assert_eq!(
        registry.schema_version,
        CREDENTIAL_STATE_SEED_SCHEMA_VERSION
    );
    assert_eq!(registry.accounts.len(), 2);
    assert_eq!(registry.credential_states.len(), 2);

    let local = registry
        .accounts
        .iter()
        .find(|account| account.provider_account_id == "provider_account.local.byok_ai")
        .expect("registry exposes the local BYOK AI account");
    assert_eq!(
        local.account_boundary_class,
        AccountBoundaryClass::LocalOnly
    );
    assert_eq!(local.identity_mode, IdentityModeAlias::AccountFreeLocal);

    let managed = registry
        .accounts
        .iter()
        .find(|account| account.provider_account_id == "provider_account.managed.payments_prod")
        .expect("registry exposes the managed payments-prod account");
    assert_eq!(
        managed.account_boundary_class,
        AccountBoundaryClass::Managed
    );
    assert_eq!(managed.identity_mode, IdentityModeAlias::ManagedConvenience);
    assert_eq!(
        managed.bound_tenant_or_org_ref.as_deref(),
        Some("tenant.acme_prod")
    );

    let local_rows: Vec<&CredentialStateRow> = registry
        .rows_for_account("provider_account.local.byok_ai")
        .collect();
    assert_eq!(local_rows.len(), 1);
    assert_eq!(
        local_rows[0].credential_state_id,
        "credential_state.local.byok_ai.0001"
    );
    let managed_rows: Vec<&CredentialStateRow> = registry
        .rows_for_account("provider_account.managed.payments_prod")
        .collect();
    assert_eq!(managed_rows.len(), 1);
    assert_eq!(
        managed_rows[0].credential_state_id,
        "credential_state.managed.payments_prod.0001"
    );
}

#[test]
fn lock_store_helper_drives_the_failure_drill_against_the_registry_fixture() {
    // Failure drill on the registry seed: the OS keychain locks. Every row
    // backed by `os_keychain` flips to the typed locked state class, the
    // `store_locked` unavailable reason fires, the alias survives so the
    // unlock prompt resolves the same handle, and local work continues.
    let mut registry = load_registry("seed_provider_account_registry.json");
    let affected = registry.lock_store(StoreSourceClass::OsKeychain);
    assert_eq!(affected, 2);

    for row in &registry.credential_states {
        assert_eq!(row.state_class, CredentialStateClass::Locked);
        assert_eq!(
            row.unavailable_reason,
            Some(CredentialUnavailableReason::StoreLocked)
        );
        assert_eq!(
            row.primary_recovery_action,
            RetryPathClass::ResumeAfterCredentialStoreUnlock
        );
        assert!(row.local_work_continues());
        assert!(!row.storage.plaintext_fallback_allowed);
        assert!(!row.storage.raw_secret_material_present);
        assert!(row.authority_alias_ref.is_some());
    }
}

#[test]
fn every_seed_fixture_in_credential_state_cases_parses_into_a_seed_record() {
    // Whole-directory sweep so a new `seed_*.json` fixture can never silently
    // miss the seed contract. Non-`seed_*.json` fixtures are owned by the
    // broader credential-state-and-secret-prompt contract suite and are not
    // inspected here.
    let dir = fixture_dir();
    let mut sweep_count = 0;
    for entry in fs::read_dir(&dir).expect("fixture directory must exist") {
        let entry = entry.expect("readable directory entry");
        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("json") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(OsStr::to_str)
            .expect("file name must be utf-8");
        if !file_name.starts_with("seed_") {
            continue;
        }
        let payload = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let value: serde_json::Value = serde_json::from_str(&payload)
            .unwrap_or_else(|err| panic!("fixture {file_name} must be valid JSON: {err}"));
        let record_kind = value
            .get("record_kind")
            .and_then(|kind| kind.as_str())
            .unwrap_or_else(|| panic!("fixture {file_name} must declare a record_kind"));
        match record_kind {
            "credential_state_row_seed_record" => {
                let _row: CredentialStateRow = serde_json::from_value(value)
                    .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a row: {err}"));
            }
            "provider_account_registry_seed_record" => {
                let _registry: ProviderAccountRegistry = serde_json::from_value(value)
                    .unwrap_or_else(|err| {
                        panic!("fixture {file_name} must parse as a registry: {err}")
                    });
            }
            other => panic!("fixture {file_name} declares unsupported record_kind {other}"),
        }
        sweep_count += 1;
    }
    assert!(
        sweep_count >= 4,
        "expected at least four seed_*.json fixtures, observed {sweep_count}",
    );
}
