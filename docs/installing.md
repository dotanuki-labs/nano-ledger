# Installing and running nano-ledger

## Docker

Installing with Docker (macOS/Linux)

```bash
docker pull ghcr.io/dotanuki-labs/nano-ledger
```

## Pre-compiled binaries

> [!NOTE]
> This tool is compatible with `macOS` and `Linux` boxes, running over `x86_64` or `aarch64` hardware

Find a pre-compiled binary for your platform directly from
[the latest CI run](https://github.com/dotanuki-labs/nano-ledger/actions/workflows/ci.yml?query=branch%3Amain)
and drop it on your `$PATH`.

## Building from source

Installing from sources (requires [Rust 1.87+](https://rustup.rs/)):

```bash
cargo install --git https://github.com/dotanuki-labs/nano-ledger nano-ledger
```

## Running nano-ledger

> [!NOTE]
> This service binds to port `3000` by default.

- Running with a pre-compiled binary

```bash
# which nano-ledger
nano-ledger
```

- Running with Docker

```bash
docker run --rm -p 3000:3000 ghcr.io/dotanuki-labs/nano-ledger
```

When running with success, you should see something like this:

```text
2025-06-06T11:18:23.497891Z DEBUG nano_ledger: Listening on 127.0.0.1:3000
```
