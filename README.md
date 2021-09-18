# Suggestion Bot

Channel suggestions bot for Telegram

# Deploy

1. Install [Rust](https://www.rust-lang.org/learn/get-started):
   ```shell
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. Install [sqlx-cli](https://crates.io/crates/sqlx-cli):
   ```shell
   $ cargo install sqlx-cli
   ```
3. Create `.env` file with:
   ```dotenv
   TELOXIDE_TOKEN=<telegram bot token>
   CHANNEL_ID=<telegram channel id>
   ADMINS_CHAT_ID=<telegram suggestion chat id>
   DATABASE_URL=sqlite:<db name>.db
   ACCEPT_FILES=<path to folder or mp4 file | optional>
   DECLINE_FILES=<path to folder or mp4 file | optional>
   ```
   P.S. examples of gifs (mp4 file without audio for telegram) are in `responses/accept` and `responses/decline`, respectively.
4. `cargo build` or `cargo build --release --locked --verbose` for release build.
5. `cargo run`

---
- **License:** © 2021 M.Price.<br>See the [LICENSE file](LICENSE) for license rights and limitations (MIT).
