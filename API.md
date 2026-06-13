# Social Service API Reference

Externally via gateway at `/api/social/*`.  
Full URL: `https://host/api/social/friend-request` → gateway → `POST /friend-request`

---

## Auth

All endpoints require `X-User-Id` header (gateway injects from session).

---

## Error format

```json
{ "error": "error_code", "message": "Human readable message" }
```

| Status | error_code | Meaning |
|---|---|---|
| 400 | `CannotFriendSelf` | Cannot send friend request to yourself |
| 401 | `Unauthorized` | Not authorized (e.g. accepting someone else's request) |
| 404 | `FriendRequestNotFound` | Request does not exist |
| 409 | `UniqueViolation` | Request already exists |
| 409 | `AlreadyFriends` | Already friends with this user |

---

## Endpoints

### `GET /health` — Health check (public)

Response `200`:
```json
{ "status": "ok", "service": "social" }
```

---

### `POST /friend-request` — Send friend request (protected)

Body:
```json
{ "receiver_id": 99 }
```

Response `201`:
```json
{
  "id": 1,
  "sender_id": 42,
  "receiver_id": 99,
  "status": "pending",
  "created_at": "2026-06-11T12:00:00Z",
  "updated_at": "2026-06-11T12:00:00Z"
}
```

Errors: `CannotFriendSelf` (400), `UniqueViolation` (409), `AlreadyFriends` (409)

---

### `POST /friend-accept` — Accept incoming friend request (protected)

Body:
```json
{ "request_id": 1 }
```

Only the `receiver_id` can accept. Response `201`:
```json
{
  "id": 1,
  "user_id": 42,
  "friend_id": 99,
  "created_at": "2026-06-11T12:00:00Z"
}
```

`user_id` is the smaller of the two user IDs, `friend_id` the larger.

Errors: `FriendRequestNotFound` (404), `Unauthorized` (401)

---

### `GET /friend-requests/incoming` — List pending requests sent to me (protected)

Response `200`:
```json
[
  {
    "id": 1,
    "sender_id": 99,
    "receiver_id": 42,
    "status": "pending",
    "created_at": "...",
    "updated_at": "..."
  }
]
```

Only requests with `status = "pending"` where current user is `receiver_id`.

---

### `GET /friend-requests/outgoing` — List pending requests I sent (protected)

Same shape, filtered by current user as `sender_id`.

---

### `GET /friends` — List my friends (protected)

Response `200`:
```json
[
  { "user_id": 99 },
  { "user_id": 101 }
]
```

Each entry's `user_id` is the **other** user's ID in the friendship.
