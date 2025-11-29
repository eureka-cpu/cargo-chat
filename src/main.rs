//! I dislike that ChatGPT never seems to have up-to-date knowledge
//! of libraries that I want to use in rust, and even if I tell it
//! what version I am using it still hallucinates stuff about the API.
//!
//! I also hate that it is now biased towards Israel.
//!
//! To fix these two things for my use cases, I'm writing a context aware
//! CLI agent that can use either a local LLM like Ollama or subscription
//! like Anthropic's Claude AI.
//!
//! I'd also like it to be as small and portable as possible, and not force
//! the user to be locked in to a specific LLM, as well as maintain a local
//! cache to speed up some of the processes needed.
//!
//! The general idea is that it will analyze the rust project and dependency
//! graph and have a backend SQL database that it can use to determine how
//! to best answer questions related to the project. For instance, if there
//! is a dependency `tokio` at version `1.48.0`, it should only be suggesting
//! usage from that version of the crate, not a newer or older version.
//!
//! Similarly, it should be able to tell you things about the crate itself, e.g
//! you should be able to ask about why something works in the project and it
//! will be able to give you a better response because it has access to the source.
//! This removes the need for copy-pasting code snippets to and from chats.
//!
//! To accomplish this task my thoughts are that it will handle things in this order:
//!
//! 1. Create the DB if it doesn't already exist, maybe using `sqlx`.
//! 2. Analyze the crate structure, then use `rust-analyzer` to generate the symbols
//! and tokens, and any other data that may be useful (not sure yet).
//! 3. Store this data along with content hashes in the database.
//! 4. Provide this information to the LLM as part of prompts, only re-indexing
//! the project if the hashes are out of date.
//! 5. Process the prompts in a stream using incremental markdown parsing.
//! 6. Print the markdown stream to the terminal.
//!
//! Eventually we will want to support non-local agents but for the time being
//! this is all I want to achieve, until the tool either picks up steam or
//! becomes irrelevant due to some major unforseen advancement.

mod cli;
mod db;
mod util;

use crate::{
    cli::{CargoChat, Parser},
    db::SymbolCache,
    util::CargoMetadata,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let CargoChat {
        rust_analyzer: _,
        db_url,
    } = CargoChat::parse();

    let cache = SymbolCache::init(&db_url).await?;

    match cache
        .compare_toplevel(&CargoMetadata::output()?.stdout)
        .await
    {
        Ok(Some(indexer)) => {
            dbg!(&indexer);
            Ok(())
        }
        Err(err) => anyhow::bail!(err),
        _ => Ok(()),
    }
}
