# nano-ledger

## What

`nano-ledger` is a simple double-entry ledger system, built with Rust.

Main features:

- In-memory data stores (for the sake of simplicity)
- Async-first implementation on top of [axum](https://github.com/tokio-rs/axum)
- Minimalistic API covering accounts, transactions, and journaling
- Easily deployable (standalone binaries or Docker)

The structure of this project is built on top of some ideas from my previous open-source
Rust projects, like
[gradle-wrapper-validator](https://github.com/dotanuki-labs/gradle-wrapper-validator)
and
[gradle-wiper](https://github.com/dotanuki-labs/gradle-wiper)
