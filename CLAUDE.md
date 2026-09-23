# CLAUDE.md

Rust client library for the WhatsApp Cloud API (Meta Graph API), published on crates.io as `whatsapp-cloud-api`.

## Workflow

- Never commit unless explicitly asked to.
- Solo-maintained repo: when asked to commit, commit directly to `main`. No feature branches, no PRs.
- Record user-facing changes under `## [Unreleased]` in `CHANGELOG.md` (Keep a Changelog style; prefix breaking changes with `**Breaking:**`).

## Layout

- `src/whatsapp_client.rs` — `WhatsappClient` (API methods, URL builders) and the private `http_client` module (reqwest GET/POST + error mapping).
- `src/models/` — serde request/response types (messages, templates, interactive, media, webhooks, phone number).
- `src/error.rs` — `WhatsappError`.
- `tests/` — integration tests that call the **real** Graph API.

## Commands

- `cargo build`, `cargo clippy --all-targets`, `cargo fmt`
- `cargo test` needs a `.env` (gitignored) with `WHATSAPP_ACCESS_TOKEN`, `WHATSAPP_PHONE_NUMBER_ID`, `WHATSAPP_SEND_TO`; tests send real messages, so don't run them casually.

## Features

- Default TLS: `native-tls`; `rustls` feature switches reqwest to rustls.
