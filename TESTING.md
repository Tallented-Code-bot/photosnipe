# Testing the Discord Bot Snipe Detection

## Prerequisites
- Backend is running (`cargo run` in the `backend/` directory)
- Discord bot is online (you should see "Discord bot connected as [bot name]" in logs)
- MongoDB is running

## How to Test Snipe Detection

### 1. Post a Snipe in Discord

The bot detects "snipes" when ALL of these conditions are met:

✅ **Message from a human** (not another bot)
✅ **Has an image attachment** (any image with width/height)
✅ **Mentions at least one user** (@username)

**Example test message:**
```
Hey @YourFriend check out this photo!
[attach any image]
```

### 2. Check the Backend Logs

After posting the message, you should see logs like:

```
[discord_bot] 📸 Processing snipe from YourName with 1 image(s) and 1 mention(s)
[discord_bot] ✅ Upserted sniper: YourName (123456789)
[discord_bot] ✅ Upserted snipee: YourFriend (987654321)
[discord_bot] ✅ Inserted snipe: YourName -> YourFriend (https://cdn.discordapp.com/...)
```

**If you DON'T see these logs, check for:**
- `Message from X has no image attachments, skipping` - Add an image
- `Message from X has images but no mentions, skipping` - Mention someone with @

### 3. Verify Data in MongoDB

Run the check script:
```bash
./check_mongodb.sh
```

Or manually check with mongosh:
```bash
mongosh mongodb://localhost:27017/photosnipe

# View all persons
db.persons.find()

# View all snipes
db.snipes.find()

# Count documents
db.persons.countDocuments({})
db.snipes.countDocuments({})
```

### 4. Test via the API Endpoint

Check the database connection:
```bash
curl http://localhost:8000/api/test_db
```

Expected response:
```
MongoDB connection succeeded! Collections: ["persons", "snipes"]
```

## Common Issues

### Bot is online but not detecting messages
- Check the bot has permission to read messages in the channel
- Verify GatewayIntents includes MESSAGE_CONTENT (we use `all()`)
- Ensure the bot role has "Read Messages" and "View Channel" permissions

### No logs appear at all
- The message might be from a bot (bot messages are ignored)
- Missing image attachment
- Missing user mention

### Database connection errors
- Verify MongoDB is running: `systemctl status mongodb` or `docker ps`
- Check `.env` file has correct `MONGODB_URI` and `MONGODB_DB`
- Test connection: `mongosh mongodb://localhost:27017`

## Example Test Sequence

1. Start backend:
   ```bash
   cd backend
   cargo run
   ```

2. Wait for "Discord bot connected as..." message

3. In Discord, post:
   ```
   @friend Look at this!
   [attach image.jpg]
   ```

4. Check backend terminal for success logs

5. Verify in MongoDB:
   ```bash
   ./check_mongodb.sh
   ```

6. You should see:
   - 2 entries in `persons` collection (you and your friend)
   - 1 entry in `snipes` collection with the image URL

## Expected Data Structure

**persons collection:**
```json
{
  "_id": ObjectId("..."),
  "id": 123456789,
  "username": "yourname",
  "display_name": "Your Nickname"
}
```

**snipes collection:**
```json
{
  "_id": ObjectId("..."),
  "sniper_id": 123456789,
  "snipee_id": 987654321,
  "picture_url": "https://cdn.discordapp.com/attachments/...",
  "text": "@friend Look at this!",
  "channel_id": 555555555,
  "guild_id": 444444444
}
```
