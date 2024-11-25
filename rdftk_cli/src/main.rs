use clap::builder::FalseyValueParser;
use clap::{Parser, ValueEnum};
use rdftk_core::error::{unknown_error, Error};
use std::process::ExitCode;
use tracing::{error, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::filter::LevelFilter as TracingLevelFilter;
use tracing_subscriber::FmtSubscriber;

// ------------------------------------------------------------------------------------------------
// Sub-command definitions
// ------------------------------------------------------------------------------------------------

pub mod commands;
use commands::Command;

// ------------------------------------------------------------------------------------------------
// Command-Line Arguments
// ------------------------------------------------------------------------------------------------

/// Command-Line Interface (CLI) for the SDML language, primarily for verification and translation
/// from SDML source to other formats.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Level of logging to enable
    #[arg(long)]
    #[arg(value_enum)]
    #[arg(default_value_t = LogFilter::None)]
    log_filter: LogFilter,

    /// Turn off color for code emitters
    #[arg(
        long,
        action = clap::ArgAction::SetTrue,
        env = "NO_COLOR",
        value_parser = FalseyValueParser::new(),
    )]
    no_color: bool,

    #[command(subcommand)]
    command: commands::Commands,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum LogFilter {
    /// Turn off all logging
    None,
    /// Enable error logging only
    Errors,
    /// Enable warnings and above
    Warnings,
    /// Enable information and above
    Information,
    /// Enable debugging and above
    Debugging,
    /// Enable tracing (ALL) and above
    Tracing,
}

// ------------------------------------------------------------------------------------------------
// Main
// ------------------------------------------------------------------------------------------------

fn main() -> ExitCode {
    human_panic::setup_panic!();

    let cli = Cli::parse();

    init_color(cli.no_color);

    if let Err(e) = init_logging(cli.log_filter) {
        eprintln!("Could not initialize tracing; error: {e}");
        error!("init_logging failed, exiting. error: {e:?}");
        ExitCode::FAILURE
    } else {
        match cli.command.execute() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Command failed to execute; error: {e}");
                error!("command.execute failed, exiting. error: {e:?}");
                ExitCode::FAILURE
            }
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn init_logging(log_filter: LogFilter) -> Result<(), Error> {
    let log_level_filter = match log_filter {
        LogFilter::None => TracingLevelFilter::OFF,
        LogFilter::Errors => TracingLevelFilter::ERROR,
        LogFilter::Warnings => TracingLevelFilter::WARN,
        LogFilter::Information => TracingLevelFilter::INFO,
        LogFilter::Debugging => TracingLevelFilter::DEBUG,
        LogFilter::Tracing => TracingLevelFilter::TRACE,
    };

    let filter = EnvFilter::from_default_env().add_directive(
        format!("{}={}", module_path!(), log_level_filter)
            .parse()
            .map_err(unknown_error)?,
    );
    let subscriber = FmtSubscriber::builder().with_env_filter(filter).finish();

    tracing::subscriber::set_global_default(subscriber).map_err(unknown_error)?;

    info!("Log level set to `LevelFilter::{:?}`", log_filter);

    Ok(())
}

// ------------------------------------------------------------------------------------------------

fn init_color(no_color: bool) {
    if no_color {
        info!("Turning off color");
        //set_colorize(UseColor::Never);
    }
}
