# How to run

## Environment variables

Populate `.env` file with the following:

- `DISCORD_TOKEN` : Your discord bot token
- `RUST_LOG` : `trace`, `debug` etc.
- `PHRASE` : Phrase of our faucet supply account.

## Prepare sqlx database

1. `cargo install sqlx-cli`
2. `sqlx db create`
3. `sqlx migrate run`
4. `export DATABASE_URL="sqlite:faucet.db"`
5. `cargo run`
