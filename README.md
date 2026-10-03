# share

Simple file sharing client and server written in Rust.

## Usage

```bash
share upload <path>
share ls
share get <id>
share rm <id>
```

## Build

```bash
cargo build --release
```

The CLI binary will be at:

```text
target/release/share
```

## Structure

```text
share/
├── share-cli/
├── share-core/
└── share-server/
```

* `share-cli` — CLI client
* `share-core` — shared types
* `share-server` — server

## Status

Personal project.
