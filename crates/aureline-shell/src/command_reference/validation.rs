//! Validation for the command-reference catalog.
//!
//! The validator enforces the acceptance invariants from M3-198:
//! the catalog must publish a structured projection for every claimed
//! stable/beta command, alias/deprecation truth must be present, and
//! every claimed command must point back to a canonical docs/help
//! anchor with a non-empty argument schema where the descriptor has
//! typed arguments. The CLI/docs help, palette, and onboarding
//! surfaces consume the same record, so a missing field is a release
//! blocker.

use serde::{Deserialize, Serialize};

use super::{
    AliasLifecycleState, CommandReferenceCatalog, CommandReferenceEntry,
    COMMAND_REFERENCE_CATALOG_RECORD_KIND, COMMAND_REFERENCE_ENTRY_RECORD_KIND,
    COMMAND_REFERENCE_SCHEMA_VERSION, COMMAND_REFERENCE_SHARED_CONTRACT_REF,
};

/// One validation error returned by [`validate_command_reference_catalog`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CommandReferenceValidationError {
    EmptyCatalog,
    EntryRecordKindMismatch { command_id: String, actual: String },
    CatalogRecordKindMismatch { actual: String },
    SchemaVersionMismatch { actual: u32 },
    SharedContractRefMismatch { actual: String },
    DuplicateCommandId { command_id: String },
    MissingTitle { command_id: String },
    MissingSummary { command_id: String },
    MissingDocsHelpAnchor { command_id: String },
    MissingPrimaryLabelRef { command_id: String },
    MissingCanonicalVerb { command_id: String },
    MissingCommandRevisionRef { command_id: String },
    EmptySearchIndex { command_id: String },
    MissingHumanLabelToken { command_id: String },
    MissingCommandIdToken { command_id: String },
    AvailabilityListsNoSupportedSurfaces { command_id: String },
    DeprecatedEntryMissingReplacement { command_id: String },
    DeprecatedAliasMissingRetirementMetadata { command_id: String, alias_id: String },
    SearchTokenEmpty { command_id: String },
}

/// Validates one entry. Surfaces should call this whenever they
/// receive an entry from a network or fixture source.
pub fn validate_command_reference_entry(
    entry: &CommandReferenceEntry,
) -> Result<(), Vec<CommandReferenceValidationError>> {
    let mut errors = Vec::new();
    let command_id = entry.command_id.clone();

    if entry.record_kind != COMMAND_REFERENCE_ENTRY_RECORD_KIND {
        errors.push(CommandReferenceValidationError::EntryRecordKindMismatch {
            command_id: command_id.clone(),
            actual: entry.record_kind.clone(),
        });
    }
    if entry.schema_version != COMMAND_REFERENCE_SCHEMA_VERSION {
        errors.push(CommandReferenceValidationError::SchemaVersionMismatch {
            actual: entry.schema_version,
        });
    }
    if entry.shared_contract_ref != COMMAND_REFERENCE_SHARED_CONTRACT_REF {
        errors.push(CommandReferenceValidationError::SharedContractRefMismatch {
            actual: entry.shared_contract_ref.clone(),
        });
    }
    if entry.title.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingTitle {
            command_id: command_id.clone(),
        });
    }
    if entry.summary.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingSummary {
            command_id: command_id.clone(),
        });
    }
    if entry.docs_help_anchor_ref.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingDocsHelpAnchor {
            command_id: command_id.clone(),
        });
    }
    if entry.primary_label_ref.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingPrimaryLabelRef {
            command_id: command_id.clone(),
        });
    }
    if entry.canonical_verb.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingCanonicalVerb {
            command_id: command_id.clone(),
        });
    }
    if entry.command_revision_ref.trim().is_empty() {
        errors.push(CommandReferenceValidationError::MissingCommandRevisionRef {
            command_id: command_id.clone(),
        });
    }
    if entry.search_index.is_empty() {
        errors.push(CommandReferenceValidationError::EmptySearchIndex {
            command_id: command_id.clone(),
        });
    }
    for token in &entry.search_index {
        if token.value.trim().is_empty() {
            errors.push(CommandReferenceValidationError::SearchTokenEmpty {
                command_id: command_id.clone(),
            });
        }
    }
    let has_human_label = entry
        .search_index
        .iter()
        .any(|token| matches!(token.token_class, super::SearchTokenClass::HumanLabel));
    if !has_human_label {
        errors.push(CommandReferenceValidationError::MissingHumanLabelToken {
            command_id: command_id.clone(),
        });
    }
    let has_command_id = entry
        .search_index
        .iter()
        .any(|token| matches!(token.token_class, super::SearchTokenClass::CommandId));
    if !has_command_id {
        errors.push(CommandReferenceValidationError::MissingCommandIdToken {
            command_id: command_id.clone(),
        });
    }
    if entry.availability.supported_surfaces.is_empty() {
        errors.push(
            CommandReferenceValidationError::AvailabilityListsNoSupportedSurfaces {
                command_id: command_id.clone(),
            },
        );
    }
    if entry.deprecation.state != AliasLifecycleState::Active
        && entry.deprecation.replacement_command_id.is_none()
    {
        errors.push(
            CommandReferenceValidationError::DeprecatedEntryMissingReplacement {
                command_id: command_id.clone(),
            },
        );
    }
    for alias in &entry.aliases {
        if alias.lifecycle_state != AliasLifecycleState::Active
            && (alias.retirement_version.is_none() || alias.replacement_command_id.is_none())
        {
            errors.push(
                CommandReferenceValidationError::DeprecatedAliasMissingRetirementMetadata {
                    command_id: command_id.clone(),
                    alias_id: alias.alias_id.clone(),
                },
            );
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates the catalog and every entry inside it.
pub fn validate_command_reference_catalog(
    catalog: &CommandReferenceCatalog,
) -> Result<(), Vec<CommandReferenceValidationError>> {
    let mut errors = Vec::new();

    if catalog.record_kind != COMMAND_REFERENCE_CATALOG_RECORD_KIND {
        errors.push(CommandReferenceValidationError::CatalogRecordKindMismatch {
            actual: catalog.record_kind.clone(),
        });
    }
    if catalog.schema_version != COMMAND_REFERENCE_SCHEMA_VERSION {
        errors.push(CommandReferenceValidationError::SchemaVersionMismatch {
            actual: catalog.schema_version,
        });
    }
    if catalog.shared_contract_ref != COMMAND_REFERENCE_SHARED_CONTRACT_REF {
        errors.push(CommandReferenceValidationError::SharedContractRefMismatch {
            actual: catalog.shared_contract_ref.clone(),
        });
    }
    if catalog.entries.is_empty() {
        errors.push(CommandReferenceValidationError::EmptyCatalog);
    }

    let mut seen = std::collections::BTreeSet::new();
    for entry in &catalog.entries {
        if !seen.insert(entry.command_id.clone()) {
            errors.push(CommandReferenceValidationError::DuplicateCommandId {
                command_id: entry.command_id.clone(),
            });
        }
        if let Err(entry_errors) = validate_command_reference_entry(entry) {
            errors.extend(entry_errors);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

#[cfg(test)]
mod tests {
    use super::super::seeded_command_reference_catalog;
    use super::*;

    #[test]
    fn seeded_catalog_passes_validation() {
        let catalog = seeded_command_reference_catalog();
        validate_command_reference_catalog(&catalog).expect("seeded catalog must validate");
    }

    #[test]
    fn deprecated_entry_without_replacement_is_rejected() {
        let mut catalog = seeded_command_reference_catalog();
        let entry = catalog.entries.first_mut().expect("at least one entry");
        entry.deprecation.state = AliasLifecycleState::Deprecated;
        entry.deprecation.replacement_command_id = None;
        let errors =
            validate_command_reference_catalog(&catalog).expect_err("must flag deprecation");
        assert!(errors.iter().any(|err| matches!(
            err,
            CommandReferenceValidationError::DeprecatedEntryMissingReplacement { .. }
        )));
    }

    #[test]
    fn missing_human_label_token_is_rejected() {
        let mut catalog = seeded_command_reference_catalog();
        let entry = catalog.entries.first_mut().expect("at least one entry");
        entry
            .search_index
            .retain(|token| !matches!(token.token_class, super::super::SearchTokenClass::HumanLabel));
        let errors =
            validate_command_reference_catalog(&catalog).expect_err("must flag missing label");
        assert!(errors.iter().any(|err| matches!(
            err,
            CommandReferenceValidationError::MissingHumanLabelToken { .. }
        )));
    }
}
