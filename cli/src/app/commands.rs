use clap::{Args, Subcommand};

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
pub struct RequireArgs {
    pub packages: Vec<String>,
}
