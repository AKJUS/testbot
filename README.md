# testbot

Toying around with Rust and the [serenity library](https://github.com/serenity-rs/serenity). It's docs are [located here](https://docs.rs/serenity/).

![CI](https://github.com/tmcarr/testbot/workflows/CI/badge.svg?branch=master)

## Building Locally

1. Install Rust (<https://rustup.rs/>)
2. Install Diesel CLI:

   ```sh
   cargo install diesel_cli --no-default-features --features postgres
   ```

3. Set up your environment variables:
   - `DISCORD_TOKEN` (your Discord bot token)
   - `DATABASE_URL` (your Postgres connection string)
   - (Optional) `HISTORY_RETENTION_DAYS` (default: 30)
   - (Optional) `WEB_PORT` (port for the web interface, default: 8080)
4. Run database migrations (optional, for local development only — the bot will run migrations automatically on startup):

   ```sh
   diesel migration run
   ```

5. Build and run:

   ```sh
   cargo run --release
   ```

> **Note:** The bot will automatically run any new database migrations on startup, both locally and in production. You do not need to run migrations manually in Docker or Fly.io deployments.

## Running with Docker

1. Build the image:

   ```sh
   docker build -t testbot .
   ```

2. Run the container:

   ```sh
   docker run -e DISCORD_TOKEN=... -e DATABASE_URL=... -e WEB_PORT=5000 -p 5000:5000 testbot
   ```

   The bot will automatically run any new database migrations on startup. The web interface will be available on the port specified by `WEB_PORT`.

## Deploying to Fly.io

1. [Install Fly.io CLI](https://fly.io/docs/hands-on/installing/)
2. Authenticate:

   ```sh
   fly auth login
   ```

3. Create and configure your app:

   ```sh
   fly launch  # if new app
   fly postgres create --name testbot-db
   fly postgres attach --app <your-app-name> testbot-db
   # This sets DATABASE_URL automatically
   fly secrets set DISCORD_TOKEN=...  # and optionally HISTORY_RETENTION_DAYS=... and WEB_PORT=...
   ```

4. Deploy:

   ```sh
   fly deploy
   ```

   The bot will automatically run any new database migrations on startup.

5. Visit `https://<your-app-name>.fly.dev/history` (or the port you configured with `WEB_PORT`) for the web interface.

### Notes for M1 Macs

You need to use rustup to target x86 since some diesel deps don't really like ARM yet.

```sh
rustup default nightly-x86_64-apple-darwin
```
