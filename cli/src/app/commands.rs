use clap::{Args, Subcommand, ValueEnum};
use composer::config::PreferredInstallMethod;

#[derive(Subcommand)]
pub enum Commands {
    /// Shows a short information about Composer
    About,
    /// Creates an archive of this composer package
    Archive,
    /// Checks for security vulnerability advisories for installed packages
    Audit,
    /// Opens the package's repository URL or homepage in your browser
    #[command(visible_alias = "home")]
    Browse,
    /// Increases the lower limit of your composer.json requirements to the currently installed versions
    Bump,
    /// Check that platform requirements are satisfied
    CheckPlatformReqs,
    /// Clears composer's internal package cache
    #[command(visible_alias = "clearcache")]
    #[command(visible_alias = "cc")]
    ClearCache,
    /// Dump the shell completion script
    Completion,
    /// Sets config options
    Config,
    /// Creates new project from a package into given directory
    CreateProject,
    /// Shows which packages cause the given package to be installed
    #[command(visible_alias = "why")]
    Depends,
    /// Diagnoses the system to identify common errors
    Diagnose,
    /// Dumps the autoloader
    #[command(visible_alias = "dumpautoload")]
    #[command(visible_alias = "du")]
    DumpAutoload,
    /// Executes a vendored binary/script
    Exec,
    /// Discover how to help fund the maintenance of your dependencies
    Fund,
    /// Allows running commands in the global composer dir ($COMPOSER_HOME)
    Global,
    /// Creates a basic composer.json file in current directory
    Init,
    /// Installs the project dependencies from the composer.lock file if present, or falls back on the composer.json
    #[command(visible_alias = "i")]
    Install,
    /// Shows information about licenses of dependencies
    Licenses,
    /// List commands
    List,
    /// Shows a list of installed packages that have updates available, including their latest version
    Outdated,
    /// Shows which packages prevent the given package from being installed
    #[command(visible_alias = "why-not")]
    Prohibits,
    /// Uninstalls and reinstalls the given package names
    Reinstall,
    /// Removes a package from the require or require-dev
    Remove,
    /// Adds required packages to your composer.json and installs them
    #[command(visible_alias = "r")]
    Require(RequireArgs),
    /// Runs the scripts defined in composer.json
    #[command(visible_alias = "run")]
    RunScript,
    /// Searches for packages
    Search,
    /// Updates composer.phar to the latest version
    #[command(visible_alias = "selfupdate")]
    SelfUpdate,
    /// Shows information about packages
    #[command(visible_alias = "info")]
    Show,
    /// Shows a list of locally modified packages
    Status,
    /// Shows package suggestions
    Suggests,
    /// Updates your dependencies to the latest version according to composer.json, and updates the composer.lock file
    #[command(visible_alias = "u")]
    #[command(visible_alias = "upgrade")]
    Update,
    /// Validates a composer.json and composer.lock
    Validate,
}

#[derive(Args, Debug)]
#[clap(rename_all = "kebab-case")]
pub struct RequireArgs {
    pub packages: Option<Vec<String>>,
    /// Add requirement to require-dev.
    #[arg(long, default_value_t = false)]
    pub dev: bool,
    /// Outputs the operations but will not execute anything (implicitly enables --verbose).
    #[arg(long, default_value_t = false)]
    pub dry_run: bool,
    /// Forces installation from package sources when possible, including VCS information.
    #[arg(long, default_value_t = false)]
    pub prefer_source: bool,
    /// Forces installation from package dist (default behavior).
    #[arg(long, default_value_t = false)]
    pub prefer_dist: bool,
    /// Forces installation from package (auto chooses source for dev versions, dist for the rest).
    #[arg(long, value_enum, default_value_t = PreferredInstallMethod::Dist)]
    pub prefer_install: PreferredInstallMethod,
    /// Write fixed version to the composer.json.
    #[arg(long, default_value_t = false)]
    pub fixed: bool,
    /// Do not output download progress.
    #[arg(long, default_value_t = false)]
    pub no_progress: bool,
    /// Disables the automatic update of the dependencies (implies --no-install).
    #[arg(long, default_value_t = false)]
    pub no_update: bool,
    /// Skip the install step after updating the composer.lock file.
    #[arg(long, default_value_t = false)]
    pub no_install: bool,
    /// Skip the audit step after updating the composer.lock file (can also be set via the COMPOSER_NO_AUDIT=1 env var).
    #[arg(long, default_value_t = false)]
    pub no_audit: bool,
    /// Audit output format.
    #[arg(long, value_enum, default_value_t = AuditFormat::Summary)]
    pub audit_format: AuditFormat,
    /// Run the dependency update with the --no-dev option.
    #[arg(long, default_value_t = false)]
    pub update_no_dev: bool,
    /// Allows inherited dependencies to be updated, except those that are root requirements.
    #[arg(long, short = 'w', default_value_t = false)]
    pub update_with_dependencies: bool,
    /// Allows all inherited dependencies to be updated, including those that are root requirements.
    #[arg(long, short = 'W', default_value_t = false)]
    pub update_with_all_dependencies: bool,
    /// Alias for --update-with-dependencies
    #[arg(long, default_value_t = false)]
    pub with_dependencies: bool,
    /// Alias for --update-with-all-dependencies
    #[arg(long, default_value_t = false)]
    pub with_all_dependencies: bool,
    /// Ignore a specific platform requirement (php & ext- packages).
    #[arg(long, default_value_t = false)]
    pub ignore_platform: bool,
    /// Ignore all platform requirements (php & ext- packages).
    #[arg(long, default_value_t = false)]
    pub ignore_platform_reqs: bool,
    /// Prefer stable versions of dependencies (can also be set via the COMPOSER_PREFER_STABLE=1 env var).
    #[arg(long, default_value_t = false)]
    pub prefer_stable: bool,
    /// Prefer lowest versions of dependencies (can also be set via the COMPOSER_PREFER_LOWEST=1 env var).
    #[arg(long)]
    pub prefer_lowest: bool,
    /// Sorts packages when adding/updating a new dependency
    #[arg(long, default_value_t = false)]
    pub sort_packages: bool,
    /// Optimize autoloader during autoloader dump
    #[arg(long, short = 'o', default_value_t = false)]
    pub optimize_autoloader: bool,
    /// Autoload classes from the classmap only. Implicitly enables `--optimize-autoloader`.
    #[arg(long, short = 'a', default_value_t = false)]
    pub classmap_authoritative: bool,
    /// Use APCu to cache found/not-found classes.
    #[arg(long, default_value_t = false)]
    pub apcu_autoloader: bool,
    /// Use a custom prefix for the APCu autoloader cache. Implicitly enables --apcu-autoloader
    #[arg(long)]
    pub apcu_autoloader_prefix: Option<String>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AuditFormat {
    Table,
    Plain,
    Json,
    Summary,
}
