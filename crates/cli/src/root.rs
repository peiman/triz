use clap::{Parser, Subcommand, ValueEnum};

/// A production-ready Rust CLI built with triz.
#[derive(Parser, Debug)]
#[command(name = "triz", version, about)]
pub struct Cli {
    /// Output format: text (human-readable) or json (machine-readable)
    #[arg(long, global = true, default_value = "text")]
    pub output: OutputFormat,

    /// Configuration file path
    #[arg(long, global = true)]
    pub config: Option<String>,

    /// Enable verbose output (debug level)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

/// Output format selection (CKSPEC-OUT-002).
/// Matches ckeletin-go convention: --output text|json
#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Text,
    Json,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check connectivity — returns pong
    Ping,

    /// Search the 39 TRIZ engineering parameters by free-text query
    ParameterSearch {
        /// Free-text term to rank against parameter names + aliases
        query: String,
    },

    /// Formulate a contradiction from an improving and worsening term
    FormulateContradiction {
        /// The parameter you want to improve
        #[arg(long)]
        improving: String,
        /// The parameter that worsens as a result
        #[arg(long)]
        worsening: String,
    },
}
