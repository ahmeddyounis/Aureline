// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Resolution of configured Aureline state-family roots.
//!
//! The normative state map names `$AURELINE_CONFIG` and `$AURELINE_STATE` as
//! location concepts. Explicit absolute overrides take precedence. Durable
//! mutation paths otherwise resolve channel-isolated platform roots, while the
//! older family selectors retain their repository-local compatibility paths
//! only for fixtures and not-yet-graduated disposable evidence writers.

use std::ffi::OsStr;
use std::fs::Metadata;
use std::path::{Component, Path, PathBuf};

/// Environment variable that selects the Aureline local-state root.
pub const AURELINE_STATE_ENV: &str = "AURELINE_STATE";

/// Environment variable that selects the user-readable configuration root.
pub const AURELINE_CONFIG_ENV: &str = "AURELINE_CONFIG";

#[cfg(not(windows))]
const HOME_ENV: &str = "HOME";
#[cfg(all(not(target_os = "macos"), not(windows)))]
const XDG_CONFIG_HOME_ENV: &str = "XDG_CONFIG_HOME";
#[cfg(all(not(target_os = "macos"), not(windows)))]
const XDG_STATE_HOME_ENV: &str = "XDG_STATE_HOME";
#[cfg(windows)]
const APPDATA_ENV: &str = "APPDATA";
#[cfg(windows)]
const LOCAL_APPDATA_ENV: &str = "LOCALAPPDATA";

/// State families currently consumed through the shared resolver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatePathFamily {
    /// Disposable diagnostic logs and traces.
    Logs,
    /// User-owned recent-work continuity metadata.
    RecentWork,
    /// User-owned session topology and restore metadata.
    Session,
    /// User-owned dirty-buffer recovery journals.
    RecoveryJournal,
    /// User-owned local history and mutation lineage.
    History,
}

impl StatePathFamily {
    const fn configured_component(self) -> &'static str {
        match self {
            Self::Logs => "logs",
            Self::RecentWork => "recent_work",
            Self::Session => "session",
            Self::RecoveryJournal => "history/recovery_journal",
            Self::History => "history",
        }
    }

    fn legacy_root(self) -> PathBuf {
        match self {
            Self::Logs => PathBuf::from(".logs"),
            Self::RecentWork => PathBuf::from(".logs").join("recent_work"),
            Self::Session | Self::RecoveryJournal => PathBuf::from(".logs").join("recovery"),
            Self::History => PathBuf::from(".logs").join("history"),
        }
    }
}

/// Resolves a compatibility state-family root from an environment value.
///
/// This pure helper exists so fixtures do not need to mutate the process
/// environment. An absent value preserves the legacy path. A present invalid
/// value returns `None`; it is never reinterpreted as a relative path and never
/// falls back to `.logs`. Durable product state should use the fallible
/// `durable_*_root` selectors below.
pub fn resolve_state_family_root_from(
    configured_state_root: Option<&OsStr>,
    family: StatePathFamily,
) -> Option<PathBuf> {
    match configured_state_root {
        Some(value) => {
            validate_durable_root_value(value).map(|root| root.join(family.configured_component()))
        }
        None => Some(family.legacy_root()),
    }
}

/// Resolves a compatibility state-family root from current configuration.
pub fn state_family_root(family: StatePathFamily) -> PathBuf {
    let configured = std::env::var_os(AURELINE_STATE_ENV);
    resolve_state_family_root_from(configured.as_deref(), family)
        .unwrap_or_else(|| panic!("present AURELINE_STATE override is not a safe absolute root"))
}

/// Returns the explicit or platform-resolved application-state root itself.
///
/// Callers should prefer a named family whenever the state-map row is known.
/// This base is retained for machine settings that have not yet gained a
/// narrower public family selector. Failure is explicit; no `.logs` fallback
/// is available from this durable selector.
pub fn application_state_root() -> Option<PathBuf> {
    durable_application_state_root()
}

/// Resolves the explicitly configured application-state root, if present.
///
/// Mutation paths that have no conforming repository-local fallback use this
/// selector instead of [`application_state_root`] and fail closed on `None`.
pub fn configured_application_state_root() -> Option<PathBuf> {
    resolve_application_state_root_from(std::env::var_os(AURELINE_STATE_ENV).as_deref())
}

/// Pure resolver for an explicitly configured application-state root.
pub fn resolve_application_state_root_from(
    configured_state_root: Option<&OsStr>,
) -> Option<PathBuf> {
    configured_state_root.and_then(validate_durable_root_value)
}

/// Resolves an explicitly configured user configuration root.
///
/// Unlike local state, user-authored configuration has no repository-local
/// fallback. Callers that need installed-desktop discovery should use
/// [`application_config_root`]; this selector intentionally reports only a
/// valid explicit override.
pub fn configured_application_config_root() -> Option<PathBuf> {
    resolve_application_config_root_from(std::env::var_os(AURELINE_CONFIG_ENV).as_deref())
}

/// Pure resolver for an explicitly configured user configuration root.
pub fn resolve_application_config_root_from(
    configured_config_root: Option<&OsStr>,
) -> Option<PathBuf> {
    configured_config_root.and_then(validate_durable_root_value)
}

/// Resolves the durable application-state root used by user-owned recovery
/// stores. A present but invalid explicit override fails closed rather than
/// silently falling back to a different root.
pub fn durable_application_state_root() -> Option<PathBuf> {
    match std::env::var_os(AURELINE_STATE_ENV) {
        Some(configured) => resolve_application_state_root_from(Some(&configured)),
        None => platform_application_roots().map(|roots| roots.state),
    }
}

/// Resolves the durable user-configuration root. A present but invalid
/// explicit override fails closed rather than silently writing elsewhere.
pub fn application_config_root() -> Option<PathBuf> {
    match std::env::var_os(AURELINE_CONFIG_ENV) {
        Some(configured) => resolve_application_config_root_from(Some(&configured)),
        None => platform_application_roots().map(|roots| roots.config),
    }
}

/// Returns the configured profile-library root, when available.
pub fn configured_profile_library_root() -> Option<PathBuf> {
    configured_application_config_root().map(|root| root.join("profiles"))
}

/// Returns the explicit or platform-resolved profile-library root.
pub fn profile_library_root() -> Option<PathBuf> {
    application_config_root().map(|root| root.join("profiles"))
}

/// Returns the explicitly configured local-history root, when available.
pub fn configured_history_root() -> Option<PathBuf> {
    configured_application_state_root().map(|root| root.join("history"))
}

/// Returns the explicit or platform-resolved durable local-history root.
pub fn durable_history_root() -> Option<PathBuf> {
    durable_application_state_root().map(|root| root.join("history"))
}

/// Returns the explicit or platform-resolved durable session-restore root.
pub fn durable_session_root() -> Option<PathBuf> {
    durable_application_state_root().map(|root| root.join("session"))
}

/// Returns the explicit or platform-resolved dirty-buffer recovery root.
pub fn durable_recovery_journal_root() -> Option<PathBuf> {
    durable_application_state_root().map(|root| root.join("history/recovery_journal"))
}

/// Returns the explicit or platform-resolved diagnostic log root.
pub fn durable_logs_root() -> Option<PathBuf> {
    durable_application_state_root().map(|root| root.join("logs"))
}

/// Returns the explicit or platform-resolved recent-work continuity root.
pub fn durable_recent_work_root() -> Option<PathBuf> {
    durable_application_state_root().map(|root| root.join("recent_work"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlatformPathClass {
    #[cfg(any(target_os = "macos", test))]
    MacOs,
    #[cfg(any(windows, test))]
    Windows,
    Unix,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlatformApplicationRoots {
    config: PathBuf,
    state: PathBuf,
}

fn platform_application_roots() -> Option<PlatformApplicationRoots> {
    let channel = option_env!("AURELINE_RELEASE_CHANNEL_CLASS").unwrap_or("dev_local");

    #[cfg(target_os = "macos")]
    let roots = resolve_platform_application_roots_from(
        PlatformPathClass::MacOs,
        channel,
        std::env::var_os(HOME_ENV).as_deref(),
        None,
        None,
        None,
        None,
    );

    #[cfg(windows)]
    let roots = resolve_platform_application_roots_from(
        PlatformPathClass::Windows,
        channel,
        None,
        None,
        None,
        std::env::var_os(APPDATA_ENV).as_deref(),
        std::env::var_os(LOCAL_APPDATA_ENV).as_deref(),
    );

    #[cfg(all(not(target_os = "macos"), not(windows)))]
    let roots = resolve_platform_application_roots_from(
        PlatformPathClass::Unix,
        channel,
        std::env::var_os(HOME_ENV).as_deref(),
        std::env::var_os(XDG_CONFIG_HOME_ENV).as_deref(),
        std::env::var_os(XDG_STATE_HOME_ENV).as_deref(),
        None,
        None,
    );

    let roots = roots?;
    Some(PlatformApplicationRoots {
        config: validate_durable_root_path(roots.config)?,
        state: validate_durable_root_path(roots.state)?,
    })
}

#[allow(clippy::too_many_arguments)]
fn resolve_platform_application_roots_from(
    platform: PlatformPathClass,
    channel: &str,
    home: Option<&OsStr>,
    xdg_config_home: Option<&OsStr>,
    xdg_state_home: Option<&OsStr>,
    _appdata: Option<&OsStr>,
    _local_appdata: Option<&OsStr>,
) -> Option<PlatformApplicationRoots> {
    let product_component = platform_product_component(platform, channel)?;
    let roots = match platform {
        #[cfg(any(target_os = "macos", test))]
        PlatformPathClass::MacOs => {
            let product_root = absolute_environment_path(home)?
                .join("Library")
                .join("Application Support")
                .join(product_component);
            PlatformApplicationRoots {
                config: product_root.join("config"),
                state: product_root.join("state"),
            }
        }
        #[cfg(any(windows, test))]
        PlatformPathClass::Windows => PlatformApplicationRoots {
            config: absolute_environment_path(_appdata)?.join(product_component),
            state: absolute_environment_path(_local_appdata)?
                .join(product_component)
                .join("state"),
        },
        PlatformPathClass::Unix => PlatformApplicationRoots {
            config: xdg_or_home_root(xdg_config_home, home, ".config")?.join(product_component),
            state: xdg_or_home_root(xdg_state_home, home, ".local/state")?.join(product_component),
        },
    };
    Some(roots)
}

fn platform_product_component(platform: PlatformPathClass, channel: &str) -> Option<&'static str> {
    let title_case = platform != PlatformPathClass::Unix;
    match (title_case, channel) {
        (_, "portable_stable" | "portable_preview") => None,
        (true, "stable") => Some("Aureline"),
        (true, "preview") => Some("Aureline Preview"),
        (true, "beta") => Some("Aureline Beta"),
        (true, "lts") => Some("Aureline LTS"),
        (true, "dev_local") => Some("Aureline Dev"),
        (false, "stable") => Some("aureline"),
        (false, "preview") => Some("aureline-preview"),
        (false, "beta") => Some("aureline-beta"),
        (false, "lts") => Some("aureline-lts"),
        (false, "dev_local") => Some("aureline-dev"),
        _ => None,
    }
}

fn xdg_or_home_root(
    xdg_value: Option<&OsStr>,
    home: Option<&OsStr>,
    home_relative_fallback: &str,
) -> Option<PathBuf> {
    match xdg_value {
        Some(value) => absolute_environment_path(Some(value)),
        None => Some(absolute_environment_path(home)?.join(home_relative_fallback)),
    }
}

fn absolute_environment_path(value: Option<&OsStr>) -> Option<PathBuf> {
    let value = value.filter(|value| !value.is_empty())?;
    let path = PathBuf::from(value);
    (path.is_absolute()
        // A filesystem/volume root is syntactically absolute but is far too
        // broad to be an application-owned mutation root. Requiring one
        // normal component also rejects bare Windows drive and UNC roots.
        && path
            .components()
            .any(|component| matches!(component, Component::Normal(_)))
        && !path
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir)))
    .then_some(path)
}

fn validate_durable_root_value(value: &OsStr) -> Option<PathBuf> {
    validate_durable_root_path(absolute_environment_path(Some(value))?)
}

fn validate_durable_root_path(path: PathBuf) -> Option<PathBuf> {
    if existing_ancestor_chain_is_safe(&path) {
        Some(path)
    } else {
        None
    }
}

fn existing_ancestor_chain_is_safe(path: &Path) -> bool {
    let mut current = PathBuf::new();
    let mut normal_component_depth = 0usize;
    for component in path.components() {
        match component {
            Component::Prefix(prefix) => current.push(prefix.as_os_str()),
            Component::RootDir => current.push(component.as_os_str()),
            Component::Normal(segment) => current.push(segment),
            Component::CurDir | Component::ParentDir => return false,
        }
        if !matches!(component, Component::Normal(_)) {
            continue;
        }
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) if metadata_is_path_redirect(&metadata) => {
                if !allow_trusted_platform_root_alias(&current, &metadata, normal_component_depth)
                    || !std::fs::metadata(&current).is_ok_and(|followed| followed.is_dir())
                {
                    return false;
                }
            }
            Ok(metadata) if !metadata.is_dir() => return false,
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return true,
            Err(_) => return false,
        }
        normal_component_depth = normal_component_depth.saturating_add(1);
    }
    true
}

#[cfg(target_os = "macos")]
fn allow_trusted_platform_root_alias(
    path: &Path,
    metadata: &Metadata,
    normal_component_depth: usize,
) -> bool {
    use std::os::unix::fs::MetadataExt;

    if path != Path::new("/var")
        || normal_component_depth != 0
        || !metadata.file_type().is_symlink()
        || metadata.uid() != 0
    {
        return false;
    }
    let Some(parent) = path.parent() else {
        return false;
    };
    let Ok(parent_metadata) = std::fs::symlink_metadata(parent) else {
        return false;
    };
    parent_metadata.is_dir()
        && parent_metadata.uid() == 0
        && parent_metadata.mode() & 0o022 == 0
        && std::fs::canonicalize(path).is_ok_and(|target| target == Path::new("/private/var"))
}

#[cfg(not(target_os = "macos"))]
fn allow_trusted_platform_root_alias(
    _path: &Path,
    _metadata: &Metadata,
    _normal_component_depth: usize,
) -> bool {
    false
}

fn metadata_is_path_redirect(metadata: &Metadata) -> bool {
    metadata.file_type().is_symlink() || metadata_is_platform_redirect(metadata)
}

#[cfg(windows)]
fn metadata_is_platform_redirect(metadata: &Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;

    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
    metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
}

#[cfg(not(windows))]
fn metadata_is_platform_redirect(_metadata: &Metadata) -> bool {
    false
}

/// Returns the installed/channel-resolved logs root when available.
///
/// The legacy path remains only for compatibility callers running without a
/// durable channel root (primarily focused fixtures). Native shell startup
/// resolves the durable root before any production log writer can run.
pub fn logs_root() -> PathBuf {
    match std::env::var_os(AURELINE_STATE_ENV) {
        Some(_) => durable_logs_root().unwrap_or_else(|| {
            panic!("present AURELINE_STATE override is not a safe absolute root")
        }),
        None => durable_logs_root().unwrap_or_else(|| StatePathFamily::Logs.legacy_root()),
    }
}

/// Returns the configured recent-work root, or its legacy location when unset.
pub fn recent_work_root() -> PathBuf {
    state_family_root(StatePathFamily::RecentWork)
}

/// Returns the session-restore state root.
pub fn session_root() -> PathBuf {
    state_family_root(StatePathFamily::Session)
}

/// Returns the dirty-buffer recovery-journal root.
pub fn recovery_journal_root() -> PathBuf {
    state_family_root(StatePathFamily::RecoveryJournal)
}

/// Returns the local-history and mutation-lineage root.
pub fn history_root() -> PathBuf {
    state_family_root(StatePathFamily::History)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEST_DIRECTORY_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    struct TestDirectory(PathBuf);

    impl TestDirectory {
        fn new(label: &str) -> Self {
            let sequence = TEST_DIRECTORY_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "aureline-state-paths-{label}-{}-{sequence}",
                std::process::id()
            ));
            std::fs::create_dir(&path).expect("create test directory");
            Self(path)
        }

        fn path(&self) -> &Path {
            &self.0
        }
    }

    impl Drop for TestDirectory {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn explicit_state_root_routes_each_family_without_environment_mutation() {
        let temp = TestDirectory::new("compatibility-family-root");
        let configured = temp.path().join("configured-state");

        assert_eq!(
            resolve_state_family_root_from(Some(configured.as_os_str()), StatePathFamily::Logs),
            Some(configured.join("logs"))
        );
        assert_eq!(
            resolve_state_family_root_from(
                Some(configured.as_os_str()),
                StatePathFamily::RecentWork
            ),
            Some(configured.join("recent_work"))
        );
        assert_eq!(
            resolve_state_family_root_from(Some(configured.as_os_str()), StatePathFamily::Session),
            Some(configured.join("session"))
        );
        assert_eq!(
            resolve_state_family_root_from(
                Some(configured.as_os_str()),
                StatePathFamily::RecoveryJournal
            ),
            Some(configured.join("history").join("recovery_journal"))
        );
        assert_eq!(
            resolve_state_family_root_from(Some(configured.as_os_str()), StatePathFamily::History),
            Some(configured.join("history"))
        );
    }

    #[test]
    fn absent_state_root_preserves_legacy_paths_but_present_invalid_values_fail_closed() {
        assert_eq!(
            resolve_state_family_root_from(None, StatePathFamily::Logs),
            Some(PathBuf::from(".logs"))
        );
        assert_eq!(
            resolve_state_family_root_from(Some(OsStr::new("")), StatePathFamily::RecentWork),
            None
        );
        assert_eq!(
            resolve_state_family_root_from(
                Some(OsStr::new("relative-state")),
                StatePathFamily::History
            ),
            None
        );
        assert_eq!(
            resolve_state_family_root_from(None, StatePathFamily::Session),
            Some(PathBuf::from(".logs").join("recovery"))
        );
        assert_eq!(
            resolve_state_family_root_from(None, StatePathFamily::RecoveryJournal),
            Some(PathBuf::from(".logs").join("recovery"))
        );
        assert_eq!(
            resolve_state_family_root_from(None, StatePathFamily::History),
            Some(PathBuf::from(".logs").join("history"))
        );
    }

    #[test]
    fn configured_durable_roots_have_no_implicit_fallback() {
        let temp = TestDirectory::new("configured-roots");
        let configured_state = temp.path().join("configured-state");
        let configured_config = temp.path().join("configured-config");
        assert_eq!(resolve_application_state_root_from(None), None);
        assert_eq!(
            resolve_application_state_root_from(Some(OsStr::new(""))),
            None
        );
        assert_eq!(
            resolve_application_state_root_from(Some(configured_state.as_os_str())),
            Some(configured_state)
        );
        assert_eq!(resolve_application_config_root_from(None), None);
        assert_eq!(
            resolve_application_config_root_from(Some(OsStr::new(""))),
            None
        );
        assert_eq!(
            resolve_application_config_root_from(Some(configured_config.as_os_str())),
            Some(configured_config)
        );
        assert_eq!(
            resolve_application_state_root_from(Some(OsStr::new("relative-state"))),
            None
        );
        assert_eq!(
            resolve_application_config_root_from(Some(OsStr::new("relative-config"))),
            None
        );
        assert_eq!(
            resolve_application_state_root_from(Some(Path::new("/").as_os_str())),
            None
        );
        assert_eq!(
            resolve_application_config_root_from(Some(Path::new("/").as_os_str())),
            None
        );
    }

    #[test]
    fn channel_components_are_isolated_and_portable_channels_fail_closed() {
        assert_eq!(
            platform_product_component(PlatformPathClass::MacOs, "stable"),
            Some("Aureline")
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::MacOs, "preview"),
            Some("Aureline Preview")
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Windows, "dev_local"),
            Some("Aureline Dev")
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Unix, "stable"),
            Some("aureline")
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Unix, "lts"),
            Some("aureline-lts")
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Unix, "portable_stable"),
            None
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::MacOs, "nightly"),
            None
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Windows, "hotfix"),
            None
        );
        assert_eq!(
            platform_product_component(PlatformPathClass::Windows, "unknown"),
            None
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_platform_layout_separates_config_and_state() {
        let roots = resolve_platform_application_roots_from(
            PlatformPathClass::MacOs,
            "preview",
            Some(OsStr::new("/Users/example")),
            None,
            None,
            None,
            None,
        )
        .expect("macOS roots");
        let product = Path::new("/Users/example")
            .join("Library")
            .join("Application Support")
            .join("Aureline Preview");
        assert_eq!(roots.config, product.join("config"));
        assert_eq!(roots.state, product.join("state"));
    }

    #[cfg(windows)]
    #[test]
    fn windows_platform_layout_separates_roaming_config_and_local_state() {
        let roots = resolve_platform_application_roots_from(
            PlatformPathClass::Windows,
            "stable",
            None,
            None,
            None,
            Some(OsStr::new(r"C:\Users\example\AppData\Roaming")),
            Some(OsStr::new(r"C:\Users\example\AppData\Local")),
        )
        .expect("Windows roots");
        assert_eq!(
            roots.config,
            Path::new(r"C:\Users\example\AppData\Roaming").join("Aureline")
        );
        assert_eq!(
            roots.state,
            Path::new(r"C:\Users\example\AppData\Local")
                .join("Aureline")
                .join("state")
        );
    }

    #[cfg(all(not(target_os = "macos"), not(windows)))]
    #[test]
    fn unix_platform_layout_honors_absolute_xdg_roots() {
        let roots = resolve_platform_application_roots_from(
            PlatformPathClass::Unix,
            "beta",
            Some(OsStr::new("/home/example")),
            Some(OsStr::new("/srv/example-config")),
            Some(OsStr::new("/srv/example-state")),
            None,
            None,
        )
        .expect("Unix roots");
        assert_eq!(
            roots.config,
            Path::new("/srv/example-config").join("aureline-beta")
        );
        assert_eq!(
            roots.state,
            Path::new("/srv/example-state").join("aureline-beta")
        );
        assert!(resolve_platform_application_roots_from(
            PlatformPathClass::Unix,
            "stable",
            Some(OsStr::new("/home/example")),
            Some(OsStr::new("relative-config")),
            Some(OsStr::new("/srv/example-state")),
            None,
            None,
        )
        .is_none());
    }

    #[cfg(unix)]
    #[test]
    fn user_controlled_redirect_in_explicit_root_fails_closed() {
        use std::os::unix::fs::symlink;

        let temp = TestDirectory::new("redirect");
        let destination = temp.path().join("destination");
        std::fs::create_dir(&destination).expect("destination");
        let redirect = temp.path().join("redirect");
        symlink(&destination, &redirect).expect("symlink");
        assert_eq!(
            resolve_application_config_root_from(Some(redirect.join("config").as_os_str())),
            None
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn trusted_macos_var_alias_is_accepted() {
        let temp = TestDirectory::new("macos-var-alias");
        let canonical = temp.path().canonicalize().expect("canonical tempdir");
        let suffix = canonical
            .strip_prefix("/private/var")
            .expect("macOS tempdir below /private/var");
        let alias_spelling = Path::new("/var").join(suffix).join("config");
        assert_eq!(
            resolve_application_config_root_from(Some(alias_spelling.as_os_str())),
            Some(alias_spelling)
        );
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn other_macos_root_aliases_remain_rejected() {
        assert!(std::fs::symlink_metadata("/tmp")
            .ok()
            .is_some_and(|metadata| metadata_is_path_redirect(&metadata)));
        assert_eq!(
            resolve_application_state_root_from(Some(OsStr::new("/tmp/aureline-state"))),
            None
        );
    }
}
