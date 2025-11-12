// ===== UI imports =====
#[cfg(feature = "ui")]
use crate::ui::viewer::run_native::run_native;
#[cfg(feature = "cli")]
use {
    crate::cli::command::to_blp::to_blp,
    crate::cli::command::to_png::to_png,
    crate::core::image::MAX_MIPS,
    crate::error::error::BlpError,
    clap::{Parser, Subcommand, error::ErrorKind},
};

// ======================= Entry point ========================

// Unified entry point for both CLI-only and UI+CLI builds
#[cfg(any(feature = "cli", feature = "ui"))]
pub fn run() -> Result<(), BlpError> {
    // Unified CLI parsing:
    // - Help/Version → print and return Ok(())
    // - Other errors → print and exit with code 2
    let Some(cli) = (match Cli::try_parse() {
        Ok(cli) => Some(cli),
        Err(e) => {
            match e.kind() {
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion => {
                    let _ = e.print(); // graceful 0
                    None
                }
                _ => {
                    let _ = e.print();
                    std::process::exit(e.exit_code()); // usually 2
                }
            }
        }
    }) else {
        return Ok(());
    };

    // ===== UI + CLI build =====
    #[cfg(all(feature = "cli", feature = "ui"))]
    {
        return if let Some(cmd) = cli.command { run_cli_command(cmd) } else { run_native(cli.path) };
    }

    // ===== CLI-only build =====
    #[cfg(all(feature = "cli", not(feature = "ui")))]
    {
        match (cli.path, cli.command) {
            // Single PATH → sanity decode (process exits inside helper)
            (Some(p), None) => {
                sanity_decode_or_exit(p);
            }
            // Subcommand without PATH
            (None, Some(cmd)) => run_cli_command(cmd),
            // Both PATH and subcommand → prefer subcommand (ignore PATH)
            (Some(_), Some(cmd)) => run_cli_command(cmd),
            // Neither PATH nor subcommand → print error and exit with code 2
            (None, None) => {
                eprintln!("error: a PATH or a subcommand is required\n\nUse --help for more information.");
                std::process::exit(2);
            }
        }
    }
}
