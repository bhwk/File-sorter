# Simple file sorter

Creates folders for classifying MIME types and sorts files accordingly. Ignores directories. If parent folder shares name of MIME type, no folders will be created.

# MIME Types

List of types come from the [infer](https://crates.io/crates/infer) rust crate.

- Image
- Video
- Audio
- Archive
- Book
- Documents
- Font
- Application

## Crates used

- [infer](https://crates.io/crates/infer) for inferring MIME types
- [anyhow](https://crates.io/crates/anyhow) for easier error handling
