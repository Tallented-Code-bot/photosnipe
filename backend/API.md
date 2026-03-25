# REST API Documentation

## Base URL
`http://localhost:8000/api`

---

## Persons Endpoints

### List All Persons
**GET** `/persons?limit=50&offset=0`

Returns a paginated list of all persons.

**Query Parameters:**
- `limit` (optional, default: 50, max: 100) - Number of results per page
- `offset` (optional, default: 0) - Number of results to skip

**Example Response:**
```json
{
  "success": true,
  "data": [
    {
      "_id": "507f1f77bcf86cd799439011",
      "id": 123456789,
      "username": "john_doe",
      "display_name": "John"
    }
  ],
  "meta": {
    "total": 150,
    "limit": 50,
    "offset": 0,
    "has_more": true
  }
}
```

---

### Get Person by ID
**GET** `/persons/:id`

Get details of a specific person by their Discord ID.

**Path Parameters:**
- `id` - Discord user ID (integer)

**Example Response:**
```json
{
  "success": true,
  "data": {
    "_id": "507f1f77bcf86cd799439011",
    "id": 123456789,
    "username": "john_doe",
    "display_name": "John"
  }
}
```

---

### Get Snipes Taken By Person
**GET** `/persons/:id/snipes-by?limit=50&offset=0`

Get all snipes taken BY this person (where they are the sniper).

**Path Parameters:**
- `id` - Discord user ID

**Query Parameters:**
- `limit`, `offset` - Pagination parameters

---

### Get Snipes Of Person
**GET** `/persons/:id/snipes-of?limit=50&offset=0`

Get all snipes OF this person (where they are the snipee).

**Path Parameters:**
- `id` - Discord user ID

**Query Parameters:**
- `limit`, `offset` - Pagination parameters

---

### Get Person Statistics
**GET** `/persons/:id/stats`

Get statistics for a specific person.

**Example Response:**
```json
{
  "success": true,
  "data": {
    "person": {
      "_id": "507f1f77bcf86cd799439011",
      "id": 123456789,
      "username": "john_doe",
      "display_name": "John"
    },
    "snipes_taken": 42,
    "snipes_received": 15
  }
}
```

---

### Create Person
**POST** `/persons`

Create a new person.

**Request Body:**
```json
{
  "id": 123456789,
  "username": "john_doe",
  "display_name": "John"
}
```

**Response:** Returns the created person with MongoDB `_id`.

---

### Update Person
**PUT** `/persons/:id`

Update a person's information.

**Request Body:**
```json
{
  "username": "new_username",
  "display_name": "New Name"
}
```

All fields are optional. Only provided fields will be updated.

---

### Delete Person
**DELETE** `/persons/:id?cascade=false`

Delete a person.

**Query Parameters:**
- `cascade` (optional, default: false) - If true, also delete all snipes involving this person

**Example Response:**
```json
{
  "success": true,
  "data": "Person with id 123456789 deleted successfully"
}
```

---

## Snipes Endpoints

### List All Snipes
**GET** `/snipes?limit=50&offset=0`

Returns a paginated list of all snipes.

**Example Response:**
```json
{
  "success": true,
  "data": [
    {
      "_id": "507f1f77bcf86cd799439011",
      "sniper_id": 123456789,
      "snipee_id": 987654321,
      "picture_url": "https://cdn.discordapp.com/...",
      "text": "Got you!",
      "channel_id": 111222333,
      "guild_id": 444555666
    }
  ],
  "meta": {
    "total": 500,
    "limit": 50,
    "offset": 0,
    "has_more": true
  }
}
```

---

### Get Snipe by ID
**GET** `/snipes/:object_id`

Get a specific snipe by its MongoDB ObjectId.

**Path Parameters:**
- `object_id` - MongoDB ObjectId (24-character hex string)

---

### Get Snipes by Channel
**GET** `/snipes/by-channel/:channel_id?limit=50&offset=0`

Get all snipes from a specific Discord channel.

**Path Parameters:**
- `channel_id` - Discord channel ID (integer)

---

### Get Snipes by Guild
**GET** `/snipes/by-guild/:guild_id?limit=50&offset=0`

Get all snipes from a specific Discord guild.

**Path Parameters:**
- `guild_id` - Discord guild ID (integer)

---

### Search Snipes
**GET** `/snipes/search?sniper_id=&snipee_id=&channel_id=&guild_id=&has_text=&limit=50&offset=0`

Search snipes with various filters.

**Query Parameters:**
- `sniper_id` (optional) - Filter by sniper's Discord ID
- `snipee_id` (optional) - Filter by snipee's Discord ID
- `channel_id` (optional) - Filter by channel ID
- `guild_id` (optional) - Filter by guild ID
- `has_text` (optional, boolean) - Filter by presence of text
- `limit`, `offset` - Pagination parameters

**Example:**
```
GET /api/snipes/search?sniper_id=123456789&has_text=true&limit=20
```

---

### Create Snipe
**POST** `/snipes`

Create a new snipe.

**Request Body:**
```json
{
  "sniper_id": 123456789,
  "snipee_id": 987654321,
  "picture_url": "https://cdn.discordapp.com/...",
  "text": "Got you!",
  "channel_id": 111222333,
  "guild_id": 444555666
}
```

**Note:** Both sniper and snipee must exist in the persons collection.

---

### Update Snipe
**PUT** `/snipes/:object_id`

Update a snipe's information.

**Request Body:**
```json
{
  "picture_url": "https://new-url.com/...",
  "text": "Updated text"
}
```

All fields are optional.

---

### Delete Snipe
**DELETE** `/snipes/:object_id`

Delete a specific snipe.

**Example Response:**
```json
{
  "success": true,
  "data": "Snipe with id 507f1f77bcf86cd799439011 deleted successfully"
}
```

---

## Statistics Endpoints

### Get Global Statistics
**GET** `/stats`

Get overall statistics for the application.

**Example Response:**
```json
{
  "success": true,
  "data": {
    "total_persons": 150,
    "total_snipes": 500,
    "top_sniper": {
      "person": {
        "_id": "507f1f77bcf86cd799439011",
        "id": 123456789,
        "username": "top_sniper",
        "display_name": "Top Sniper"
      },
      "count": 42
    },
    "top_snipee": {
      "person": {
        "_id": "507f1f77bcf86cd799439012",
        "id": 987654321,
        "username": "most_sniped",
        "display_name": "Most Sniped"
      },
      "count": 35
    }
  }
}
```

---

### Get Top Snipers Leaderboard
**GET** `/leaderboard/snipers?limit=10`

Get the top snipers (persons who have taken the most snipes).

**Query Parameters:**
- `limit` (optional, default: 50, max: 100) - Number of results
- `offset` (optional, default: 0) - Results to skip

**Example Response:**
```json
{
  "success": true,
  "data": [
    {
      "person": {
        "_id": "507f1f77bcf86cd799439011",
        "id": 123456789,
        "username": "john_doe",
        "display_name": "John"
      },
      "count": 42
    },
    {
      "person": {
        "_id": "507f1f77bcf86cd799439012",
        "id": 987654321,
        "username": "jane_doe",
        "display_name": "Jane"
      },
      "count": 38
    }
  ]
}
```

---

### Get Top Snipees Leaderboard
**GET** `/leaderboard/snipees?limit=10`

Get the top snipees (persons who have been sniped the most).

**Query Parameters:** Same as snipers leaderboard

---

## Error Responses

All errors follow this format:

```json
{
  "success": false,
  "error": {
    "code": "ERROR_CODE",
    "message": "Detailed error message"
  }
}
```

### Error Codes

- **NOT_FOUND** (404) - Resource doesn't exist
- **BAD_REQUEST** (400) - Invalid input/parameters
- **VALIDATION_ERROR** (422) - Data validation failed
- **CONFLICT** (409) - Resource already exists
- **INTERNAL_ERROR** (500) - Database or server error

---

## Testing the API

### Using curl

```bash
# List persons
curl http://localhost:8000/api/persons

# Get a specific person
curl http://localhost:8000/api/persons/123456789

# Create a person
curl -X POST http://localhost:8000/api/persons \
  -H "Content-Type: application/json" \
  -d '{"id": 123456789, "username": "test_user", "display_name": "Test"}'

# Search snipes
curl "http://localhost:8000/api/snipes/search?sniper_id=123456789&limit=10"

# Get global stats
curl http://localhost:8000/api/stats

# Get leaderboard
curl "http://localhost:8000/api/leaderboard/snipers?limit=5"
```

---

## Notes

- All list endpoints support pagination with `limit` and `offset` parameters
- Maximum `limit` is 100, default is 50
- All responses include a `success` boolean field
- List responses include a `meta` object with pagination information
- The Discord bot automatically creates persons and snipes; manual creation is optional
- When deleting a person, use `cascade=true` to also delete their snipes
