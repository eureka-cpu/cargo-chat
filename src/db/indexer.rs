//! Indexes the project and computes a hash for each crate, referencing the database to check if
//! any crates need to be updated.

use std::path;

use sqlx::{Pool, Sqlite};

use crate::util::{CargoMetadata, Crate};

/// Contains the new toplevel hash and metadata to be processed for the target.
/// `Indexer` is a producers which returns events on each advance.
#[derive(Debug)]
pub(crate) struct Indexer<'a> {
    /// The database to store indexed values.
    pool: &'a Pool<Sqlite>,
    /// The project being indexed. If this is the first
    /// time the project is indexed, target will be empty.
    target: Option<String>,
    /// The latest toplevel hash.
    toplevel: blake3::Hash,
    /// The dependencies being indexed of the project.
    packages: Vec<Crate>,
    /// The workspace members of the project.
    workspace_members: Vec<path::PathBuf>,
}
impl<'a> Indexer<'a> {
    pub(crate) fn new(
        pool: &'a Pool<Sqlite>,
        target: Option<String>,
        toplevel: blake3::Hash,
        meta: &[u8],
    ) -> anyhow::Result<Self> {
        let CargoMetadata {
            packages,
            workspace_members,
        } = meta.try_into()?;

        Ok(Self {
            pool,
            target,
            toplevel,
            packages,
            workspace_members,
        })
    }

    // TODO: hash crate info and check the db if the hash has changed
    // pub(crate) fn index<I>(self) -> I
    //     where
    //         I: IntoIterator<Item = Entry>
    // {
    //     self.packages.into_iter().map(|pkg| pkg)
    // }
}

/// An entry in the cache.
pub struct Entry {
    /// A hash representing the id of the entry in the cache.
    key: blake3::Hash,
    /// A type containing values stored as part of the entry.
    val: EntryType,
}

/// A type containing values stored as part of the entry.
pub enum EntryType {
    Crate,
    WorkspaceMember,
}
