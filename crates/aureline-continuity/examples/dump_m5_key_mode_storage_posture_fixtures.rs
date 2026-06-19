//! Emits the canonical key-mode and storage-posture fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- page
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- summary
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- support-export
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-customer-key-unavailable-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-trust-root-mismatch-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-key-material-lost-withdrawn
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-store-locked-preview
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-encryption-opaque-beta
//! cargo run -q -p aureline-continuity --example dump_m5_key_mode_storage_posture_fixtures -- case-profile-key-mode-mismatch-preview
//! ```

use aureline_continuity::{
    seeded_key_mode_storage_posture_input, seeded_key_mode_storage_posture_page,
    KeyAvailabilityState, KeyModeClass, KeyModeStorageEntry, KeyModeStoragePostureInput,
    KeyModeStoragePosturePage, KeyModeStoragePostureSupportExport, StorageEncryptionClass,
    StoreLockState,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_key_mode_storage_posture_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = KeyModeStoragePostureSupportExport::from_page(
                "continuity:key-posture:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("case-customer-key-unavailable-withdrawn") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(&mut input, "continuity-row:self-hosted-restore", |entry| {
                entry.key_availability = KeyAvailabilityState::CustomerKeyUnavailable;
                entry.key_availability_token = KeyAvailabilityState::CustomerKeyUnavailable
                    .as_str()
                    .to_owned();
            });
            print_json(&case_page(
                "continuity:key-posture:case:customer-key-unavailable",
                "Case - customer-managed key unavailable, managed lane fails closed (withdrawn)",
                input,
            ))?;
        }
        Some("case-trust-root-mismatch-withdrawn") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(
                &mut input,
                "continuity-row:sovereign-airgap-snapshot",
                |entry| {
                    entry.key_availability = KeyAvailabilityState::TrustRootMismatch;
                    entry.key_availability_token =
                        KeyAvailabilityState::TrustRootMismatch.as_str().to_owned();
                },
            );
            print_json(&case_page(
                "continuity:key-posture:case:trust-root-mismatch",
                "Case - offline trust root mismatch, managed lane fails closed (withdrawn)",
                input,
            ))?;
        }
        Some("case-key-material-lost-withdrawn") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(&mut input, "continuity-row:managed-cloud-sync", |entry| {
                entry.key_availability = KeyAvailabilityState::KeyMaterialLost;
                entry.key_availability_token =
                    KeyAvailabilityState::KeyMaterialLost.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:key-posture:case:key-material-lost",
                "Case - durable key material lost, managed lane fails closed (withdrawn)",
                input,
            ))?;
        }
        Some("case-store-locked-preview") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(&mut input, "continuity-row:managed-cloud-sync", |entry| {
                entry.store_lock = StoreLockState::Locked;
                entry.store_lock_token = StoreLockState::Locked.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:key-posture:case:store-locked",
                "Case - local store locked on the managed lane (preview)",
                input,
            ))?;
        }
        Some("case-encryption-opaque-beta") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(
                &mut input,
                "continuity-row:managed-relay-failover",
                |entry| {
                    entry.storage_encryption = StorageEncryptionClass::EncryptedKeyModeOpaque;
                    entry.storage_encryption_token = StorageEncryptionClass::EncryptedKeyModeOpaque
                        .as_str()
                        .to_owned();
                },
            );
            print_json(&case_page(
                "continuity:key-posture:case:encryption-opaque",
                "Case - 'encrypted' claim does not name its key mode (beta)",
                input,
            ))?;
        }
        Some("case-profile-key-mode-mismatch-preview") => {
            let mut input = seeded_key_mode_storage_posture_input();
            with_entry(&mut input, "continuity-row:self-hosted-restore", |entry| {
                entry.key_mode = KeyModeClass::VendorManagedKeys;
                entry.key_mode_token = KeyModeClass::VendorManagedKeys.as_str().to_owned();
            });
            print_json(&case_page(
                "continuity:key-posture:case:profile-key-mode-mismatch",
                "Case - self-hosted row leans on vendor-managed keys (preview)",
                input,
            ))?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }

    Ok(())
}

fn with_entry(
    input: &mut KeyModeStoragePostureInput,
    row_id: &str,
    mutate: impl FnOnce(&mut KeyModeStorageEntry),
) {
    let entry = input
        .entries
        .iter_mut()
        .find(|entry| entry.row_id == row_id)
        .unwrap_or_else(|| panic!("missing seeded entry: {row_id}"));
    mutate(entry);
}

fn case_page(
    page_id: &str,
    page_label: &str,
    input: KeyModeStoragePostureInput,
) -> KeyModeStoragePosturePage {
    KeyModeStoragePosturePage::new(page_id, page_label, "2026-06-01T00:00:00Z", input)
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
