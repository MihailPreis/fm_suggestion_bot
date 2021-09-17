# Suggestion Bot

Channel suggestions bot for Telegram

# Deploy

1. Create `.env` file with:
   ```dotenv
   TELOXIDE_TOKEN=<telegram bot token>
   CHANNEL_ID=<telegram channel id>
   ADMINS_CHAT_ID=<telegram suggestion chat id>
   DATABASE_URL=sqlite:<db name>.db
   ACCEPT_FILES=<path to folder or mp4 file | optional>
   DECLINE_FILES=<path to folder or mp4 file | optional>
   ```
   P.S. examples of gifs are in `responses/accept` and `responses/decline`, respectively.

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
