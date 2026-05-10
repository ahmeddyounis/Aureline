//! Setting scope vocabulary and precedence ranks.
//!
//! The scope token set mirrors the ADR vocabulary in
//! `docs/settings/precedence_lock_and_write_scope_contract.md`.
//! Adding or repurposing a scope is a breaking change and requires a
//! new ADR row.

use serde::{Deserialize, Serialize};

/// Canonical settings scope. Ordinary scopes contribute layered
/// value candidates; `AdminPolicyNarrowing` is a ceiling that may
/// only narrow, lock, or constrain a layered winner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SettingScope {
    BuiltInDefault,
    ChannelOrExperimentDefault,
    ImportedProfileDefault,
    UserGlobal,
    MachineSpecific,
    Workspace,
    FolderOrModuleOverride,
    LanguageOverride,
    SessionOverride,
    AdminPolicyNarrowing,
}

impl SettingScope {
    /// Returns the stable snake-case token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuiltInDefault => "built_in_default",
            Self::ChannelOrExperimentDefault => "channel_or_experiment_default",
            Self::ImportedProfileDefault => "imported_profile_default",
            Self::UserGlobal => "user_global",
            Self::MachineSpecific => "machine_specific",
            Self::Workspace => "workspace",
            Self::FolderOrModuleOverride => "folder_or_module_override",
            Self::LanguageOverride => "language_override",
            Self::SessionOverride => "session_override",
            Self::AdminPolicyNarrowing => "admin_policy_narrowing",
        }
    }

    /// Parse a scope from its canonical token. Returns `None` for
    /// unknown tokens; callers MUST surface that as a typed error
    /// rather than collapsing the scope silently.
    pub fn from_token(token: &str) -> Option<Self> {
        Some(match token {
            "built_in_default" => Self::BuiltInDefault,
            "channel_or_experiment_default" => Self::ChannelOrExperimentDefault,
            "imported_profile_default" => Self::ImportedProfileDefault,
            "user_global" => Self::UserGlobal,
            "machine_specific" => Self::MachineSpecific,
            "workspace" => Self::Workspace,
            "folder_or_module_override" => Self::FolderOrModuleOverride,
            "language_override" => Self::LanguageOverride,
            "session_override" => Self::SessionOverride,
            "admin_policy_narrowing" => Self::AdminPolicyNarrowing,
            _ => return None,
        })
    }

    /// Stable precedence rank. Higher rank wins for ordinary
    /// candidates; `AdminPolicyNarrowing` carries the highest rank
    /// because it caps the layered winner.
    pub const fn precedence_rank(self) -> u32 {
        match self {
            Self::BuiltInDefault => 10,
            Self::ChannelOrExperimentDefault => 15,
            Self::ImportedProfileDefault => 40,
            Self::UserGlobal => 60,
            Self::MachineSpecific => 70,
            Self::Workspace => 80,
            Self::FolderOrModuleOverride => 90,
            Self::LanguageOverride => 95,
            Self::SessionOverride => 120,
            Self::AdminPolicyNarrowing => 900,
        }
    }

    /// True for ordinary value-bearing scopes that participate in the
    /// layered tie-break. `AdminPolicyNarrowing` returns false; it
    /// applies as a ceiling after the layered winner is chosen.
    pub const fn is_ordinary(self) -> bool {
        !matches!(self, Self::AdminPolicyNarrowing)
    }

    /// Every ordinary scope, ordered low to high. The order mirrors
    /// the precedence ladder used by the resolver.
    pub const fn ordinary_scopes() -> &'static [Self] {
        &[
            Self::BuiltInDefault,
            Self::ChannelOrExperimentDefault,
            Self::ImportedProfileDefault,
            Self::UserGlobal,
            Self::MachineSpecific,
            Self::Workspace,
            Self::FolderOrModuleOverride,
            Self::LanguageOverride,
            Self::SessionOverride,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_round_trip_covers_every_variant() {
        let cases = [
            SettingScope::BuiltInDefault,
            SettingScope::ChannelOrExperimentDefault,
            SettingScope::ImportedProfileDefault,
            SettingScope::UserGlobal,
            SettingScope::MachineSpecific,
            SettingScope::Workspace,
            SettingScope::FolderOrModuleOverride,
            SettingScope::LanguageOverride,
            SettingScope::SessionOverride,
            SettingScope::AdminPolicyNarrowing,
        ];
        for scope in cases {
            assert_eq!(SettingScope::from_token(scope.as_str()), Some(scope));
        }
        assert_eq!(SettingScope::from_token("nope"), None);
    }

    #[test]
    fn precedence_ranks_strictly_increase_for_ordinary_scopes() {
        let mut last = 0u32;
        for scope in SettingScope::ordinary_scopes() {
            let rank = scope.precedence_rank();
            assert!(
                rank > last,
                "ordinary scope {} rank {} did not strictly exceed previous {}",
                scope.as_str(),
                rank,
                last
            );
            last = rank;
            assert!(scope.is_ordinary());
        }
        assert!(!SettingScope::AdminPolicyNarrowing.is_ordinary());
        assert!(SettingScope::AdminPolicyNarrowing.precedence_rank() > last);
    }
}
