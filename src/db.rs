use sqlx::{Pool, Sqlite, SqlitePool, migrate};

mod indexer;
use indexer::Indexer;

/// Represents the project pool containing the toplevel metadata
/// hashes and dependency symbols.
pub struct SymbolCache(Pool<Sqlite>);
impl SymbolCache {
    /// Connect to the db and run migrations.
    pub async fn init(url: &str) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(url).await?;

        migrate!("./migrations").run(&pool).await?;

        Ok(Self(pool))
    }

    /// Computes the toplevel hash from `cargo metadata`, and returns a new [`Indexer`] if it differs from the target.
    ///
    /// `meta` in this case is the `stdout` of `cargo metadata`'s output which comes from [`crate::util::CargoMetadata::output`].
    pub async fn compare_toplevel(&self, meta: &[u8]) -> anyhow::Result<Option<Indexer<'_>>> {
        let hash = blake3::hash(meta);

        let target: Option<String> =
            sqlx::query_scalar("SELECT metadata_hash FROM index_state ORDER BY id DESC LIMIT 1")
                .fetch_optional(&self.0)
                .await?;

        Ok(target
            .as_ref()
            .is_none_or(|target| target.as_bytes() != hash.as_bytes())
            .then_some(Indexer::new(&self.0, target, hash, meta)?))
    }

    // pub async fn write_batch(&self, batch: Batch) -> anyhow::Result<()> {
    //     Ok(())
    // }
}
