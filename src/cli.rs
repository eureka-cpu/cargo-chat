use std::path::PathBuf;

pub use clap::Parser;

// TODO: This should not be a hard-coded string. We should be setting this
// dynamically using detection or allow the user to optionally specify the path.
const RUST_ANALYZER: &str = "rust-analyzer";

// TODO: We should probably have a dry run to show what would have been stored
// or changed in the db.
#[derive(Debug, Clone, Parser)]
pub(crate) struct CargoChat {
    // TODO: We should make it possible to connect over SSH so the user doesn't
    // have to keep the database locally, although that is the original intent.
    // We can make it so that if the db is local to the project it is automatically
    // picked up, and if the env var is set we will use that, which may be more
    // convenient for those using non-local caches.
    //
    /// The path to the database containing project and dependency symbols.
    #[arg(
        long,
        env = "CARGO_CHAT_DB_URL",
        default_value = "~/.cargo/cargo-chat/db.sqlite"
    )]
    // TODO: We need some intermediate type to handle parsing a local or remote url
    pub(crate) db_url: String,

    // TODO: use value_parser to locate the binary and validate the path
    /// Override rust-analyzer binary path.
    #[arg(long, env = "RUST_ANALYZER_PATH", default_value = RUST_ANALYZER)]
    pub(crate) rust_analyzer: PathBuf,
}
