//! Undo-class taxonomy backing the buffer prototype.
//!
//! Class ids and compensation postures are frozen in
//! `docs/adr/0003-buffer-undo-large-file.md` and
//! `artifacts/architecture/undo_class_rows.yaml`. The enum here is the
//! typed mirror; lanes that emit a transaction pick exactly one
//! variant. Adding a variant is an ADR amendment plus a yaml row.

/// Frozen undo-class vocabulary. One variant per class id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UndoClass {
    TextEdit,
    MultiCursorTextEdit,
    StructuralEdit,
    RefactorSingleFile,
    RefactorMultiFile,
    FormatterRun,
    SaveParticipantGroup,
    ImportedChange,
    MachineGeneratedChange,
    MigrationChange,
    ExternalReload,
    DecodeRecoveryChange,
}

impl UndoClass {
    pub const ALL: [Self; 12] = [
        Self::TextEdit,
        Self::MultiCursorTextEdit,
        Self::StructuralEdit,
        Self::RefactorSingleFile,
        Self::RefactorMultiFile,
        Self::FormatterRun,
        Self::SaveParticipantGroup,
        Self::ImportedChange,
        Self::MachineGeneratedChange,
        Self::MigrationChange,
        Self::ExternalReload,
        Self::DecodeRecoveryChange,
    ];

    /// The frozen `class_id` string from the ADR taxonomy.
    pub fn class_id(self) -> &'static str {
        match self {
            Self::TextEdit => "text_edit",
            Self::MultiCursorTextEdit => "multi_cursor_text_edit",
            Self::StructuralEdit => "structural_edit",
            Self::RefactorSingleFile => "refactor_single_file",
            Self::RefactorMultiFile => "refactor_multi_file",
            Self::FormatterRun => "formatter_run",
            Self::SaveParticipantGroup => "save_participant_group",
            Self::ImportedChange => "imported_change",
            Self::MachineGeneratedChange => "machine_generated_change",
            Self::MigrationChange => "migration_change",
            Self::ExternalReload => "external_reload",
            Self::DecodeRecoveryChange => "decode_recovery_change",
        }
    }

    /// Invariant compensation posture from the ADR taxonomy. The
    /// posture is frozen; promoting an only-revertible class to
    /// compensatable requires a new decision row, not a code change.
    pub fn compensation_posture(self) -> CompensationPosture {
        match self {
            Self::TextEdit
            | Self::MultiCursorTextEdit
            | Self::StructuralEdit
            | Self::RefactorSingleFile
            | Self::FormatterRun => CompensationPosture::Compensatable,
            Self::RefactorMultiFile
            | Self::SaveParticipantGroup
            | Self::ImportedChange
            | Self::MachineGeneratedChange
            | Self::MigrationChange
            | Self::ExternalReload
            | Self::DecodeRecoveryChange => CompensationPosture::OnlyRevertible,
        }
    }

    /// True when the class fires `text_edit_apply` in addition to
    /// `transaction_apply`.
    pub fn fires_text_edit_apply(self) -> bool {
        matches!(self, Self::TextEdit | Self::MultiCursorTextEdit)
    }

    /// True when the class opens a named undo group (classes that the
    /// ADR requires to carry a human-readable label).
    pub fn is_named_group(self) -> bool {
        matches!(
            self,
            Self::RefactorSingleFile
                | Self::RefactorMultiFile
                | Self::FormatterRun
                | Self::SaveParticipantGroup
                | Self::ImportedChange
                | Self::MachineGeneratedChange
                | Self::MigrationChange
        )
    }
}

/// Compensation posture vocabulary.
///
/// - `Compensatable`: the inverse can be expressed as a forward,
///   legal transaction (insert reverses delete). Redo survives a
///   divergent edit.
/// - `OnlyRevertible`: the inverse depends on a stored snapshot. A
///   divergent edit drops the redo stack for the group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompensationPosture {
    Compensatable,
    OnlyRevertible,
}

impl CompensationPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Compensatable => "compensatable",
            Self::OnlyRevertible => "only_revertible",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_class_id_is_stable() {
        let ids: Vec<&'static str> = UndoClass::ALL.iter().map(|c| c.class_id()).collect();
        assert_eq!(
            ids,
            vec![
                "text_edit",
                "multi_cursor_text_edit",
                "structural_edit",
                "refactor_single_file",
                "refactor_multi_file",
                "formatter_run",
                "save_participant_group",
                "imported_change",
                "machine_generated_change",
                "migration_change",
                "external_reload",
                "decode_recovery_change",
            ]
        );
    }

    #[test]
    fn postures_match_the_adr_split() {
        let compensatable: Vec<&'static str> = UndoClass::ALL
            .iter()
            .filter(|c| c.compensation_posture() == CompensationPosture::Compensatable)
            .map(|c| c.class_id())
            .collect();
        assert_eq!(
            compensatable,
            vec![
                "text_edit",
                "multi_cursor_text_edit",
                "structural_edit",
                "refactor_single_file",
                "formatter_run",
            ]
        );
    }

    #[test]
    fn text_edit_classes_fire_text_edit_apply() {
        for class in UndoClass::ALL {
            let expected = matches!(class, UndoClass::TextEdit | UndoClass::MultiCursorTextEdit);
            assert_eq!(class.fires_text_edit_apply(), expected, "{class:?}");
        }
    }
}
