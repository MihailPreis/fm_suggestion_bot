# Suggestion Bot

Channel suggestions bot for Telegram

# Deploy

1. Create `.env` file with:
   ```dotenv
   TELOXIDE_TOKEN=<telegram bot token>
   CHANNEL_ID=<telegram channel id>
   ADMINS_CHAT_ID=<telegram suggestion chat id>
   DATABASE_URL=sqlite:<db name>.db
   ```

2. Configure database:
   ```shell
   $ cargo install sqlx-cli
   $ export DATABASE_URL="sqlite:<db name>.db"
   $ sqlx db create
   $ sqlx migrate run
   ```

3. `cargo build`
4. `cargo run`

---
- **License:** © 2021 M.Price.<br>See the [LICENSE file](LICENSE) for license rights and limitations (MIT).
