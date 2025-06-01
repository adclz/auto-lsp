# Default Database

The `default` crate provides default components for managing source files and a database in the db module:

- BaseDb: A basic database implementation using Salsa.
- File: A struct representing a source file.
- BaseDatabase: A trait for file retrieval.
- FileManager: A trait for file updates.

These components are designed to cover common use cases but can be extended or replaced to suit your project’s specific needs.

## File struct

The File struct is a `salsa::input` representing a source file, its `Url`, and a reference to its parser configuration.

```rust, ignore
#[salsa::input]
pub struct File {
    #[id]
    pub url: Url,
    pub parsers: &'static Parsers,
    #[return_ref]
    pub document: Arc<Document>,
}
```

## BaseDb struct

`BaseDb` is the default implementation of a database. It stores File inputs.

```rust
#[salsa::db]
#[derive(Default, Clone)]
pub struct BaseDb {
    storage: Storage<Self>,
    pub(crate) files: DashMap<Url, File>,
}
```

To enable logging, use the `with_logger` method to initialize the database with a logger closure.

## BaseDatabase trait

The `BaseDatabase` trait defines how to access stored files. It's meant to be implemented by any Salsa-compatible database. It is used to retrieve files from the database.

```rust, ignore
#[salsa::db]
pub trait BaseDatabase: Database {
    fn get_files(&self) -> &DashMap<Url, File>;

    fn get_file(&self, url: &Url) -> Option<File> {
        self.get_files().get(url).map(|file| *file)
    }
}
```

## FileManager trait

The `FileManager` trait provides high-level methods to manage files (add, update, remove).  It is implemented for any type that also implements `BaseDatabase`.

```rust, ignore
pub trait FileManager: BaseDatabase + salsa::Database {
    fn add_file_from_texter(
        &mut self,
        parsers: &'static Parsers,
        url: &Url,
        texter: Text,
    ) -> Result<(), DataBaseError>;

    fn update(
        &mut self,
        url: &Url,
        changes: &[lsp_types::TextDocumentContentChangeEvent],
    ) -> Result<(), DataBaseError>;

    fn remove_file(&mut self, url: &Url) -> Result<(), DataBaseError>;
}
```