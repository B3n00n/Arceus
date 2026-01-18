# Arceus Central Server - Architecture & Implementation Guide

## Context

### What is Arceus?
Arceus is a desktop application running on PCs in VR arcades worldwide. It:
- Serves as a launcher for Unity games on the local PC
- Manages Oculus Quest devices connecting via TCP (port 43572)
- Can download and update games from a central source (when connected)

**Offline-First Design:** Arceus must function fully without server connectivity. If authentication fails or there's no internet connection, Arceus continues operating normally - launching existing games and managing Quest devices. It simply cannot receive updates or server-managed data. The server sees unauthenticated clients as "ghosts" - they exist locally but aren't tracked or managed.

### What is the Central Server?
A Rust-based REST API deployed on GCP that:
1. Authenticates Arceus clients (VR arcade installations)
2. Manages game versions stored on Google Cloud Storage (GCS)
3. Provides smart delta updates via manifest comparison
4. Acts as source of truth for connected clients - but clients operate independently when offline

### How Everything Works Together

```
[Arceus Client]  <--REST API-->  [Central Server]  <--Queries-->  [PostgreSQL]
       |                                |
       |                                v
       +------------GCS Signed URLs---> [Google Cloud Storage]
                                        (Game files + Manifests)
```

**Client Connectivity States:**

1. **Connected & Authenticated** - Full functionality
   - Can check for updates
   - Can download new game versions
   - Server tracks client activity (last_seen)
   - Receives assigned games list

2. **Disconnected / Auth Failed** - Gracefully degraded
   - All local features work (game launching, Quest management)
   - Cannot check for or receive updates
   - Invisible to server ("ghost mode")
   - No data synced

**Update Flow (when connected):**
1. Client attempts authentication with API key → receives JWT (or fails gracefully)
2. Client requests assigned games list
3. Server returns game info + signed URL to manifest
4. Client downloads manifest, compares with local copy
5. Client identifies changed files, requests signed download URLs
6. Client downloads only what changed directly from GCS

**Manifest Approach:**
- Each game version has a `manifest.json` in GCS
- Manifest lists all files with SHA256 hashes and sizes
- Generated once per version publish (CLI tool)
- Client keeps local manifest copy, compares on update
- Integrity check = scan local files vs local manifest

---

## Non-Negotiable Architecture Principles

### 1. Separation of Concerns - Layered Architecture

```
API Layer (Handlers)
    ↓ receives HTTP requests, validates input, returns responses
Service Layer (Business Logic)
    ↓ orchestrates operations, enforces rules, coordinates
Repository Layer (Data Access)
    ↓ CRUD operations, queries, no business logic
Domain Layer (Core Types)
    ← pure data structures, no dependencies
```

**Rules:**
- Handlers NEVER contain business logic
- Services NEVER know about HTTP (no status codes, headers)
- Repositories NEVER enforce business rules
- Domain types are pure - no database annotations, no serialization logic in core structs

**Example violation to avoid:**
```rust
// BAD - business logic in handler
async fn create_game(Json(input): Json<CreateGame>) -> Response {
    if input.name.len() < 3 {  // Business rule in handler!
        return StatusCode::BAD_REQUEST;
    }
    // ...
}

// GOOD - handler delegates to service
async fn create_game(
    State(service): State<GameService>,
    Json(input): Json<CreateGame>
) -> Result<Json<Game>, AppError> {
    let game = service.create_game(input).await?;  // Service validates
    Ok(Json(game))
}
```

### 2. Open-Closed Principle - Extend, Don't Modify

**Use traits for abstractions:**
```rust
// Storage can be GCS, S3, local filesystem - code doesn't change
trait StorageProvider {
    async fn generate_signed_url(&self, path: &str, expiry: Duration) -> Result<String>;
    async fn list_files(&self, prefix: &str) -> Result<Vec<FileInfo>>;
    async fn get_file(&self, path: &str) -> Result<Bytes>;
}

// New storage backend = implement trait, not modify existing code
struct GcsStorage { /* ... */ }
impl StorageProvider for GcsStorage { /* ... */ }
```

**New features via extension:**
- Adding new game metadata? Add field to domain type, migration, done
- New auth method? Implement new auth strategy, plug into existing middleware
- Different manifest format? New parser implementation behind trait

### 3. No Duplicate Code - DRY Everything

**Common patterns:**
- Error mapping → single `AppError` enum with `IntoResponse`
- Input validation → shared validation module or validator trait
- Response wrapping → middleware or generic response type
- Repository queries → consider generic repository trait for CRUD

**Extract shared behavior:**
```rust
// Instead of copy-pasting pagination logic
trait Paginated {
    fn paginate(&self, page: u32, per_page: u32) -> Self;
}

// Instead of duplicating auth checks
// Use middleware that applies to route groups
```

### 4. Strong Typing - Make Invalid States Unrepresentable

**Newtypes for IDs:**
```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GameId(Uuid);

// Compiler prevents mixing up IDs
fn get_game(id: GameId) -> Game;  // Can't pass ClientId here
```

**Enums for states:**
```rust
enum VersionStatus {
    Draft,
    Published,
    Deprecated,
}
// Not strings that could be misspelled
```

### 5. Comprehensive Error Handling

**Custom error type:**
```rust
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("Client not found")]
    ClientNotFound,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    // ...
}

// Map to HTTP responses in ONE place
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code) = match &self {
            AppError::ClientNotFound => (StatusCode::NOT_FOUND, "CLIENT_NOT_FOUND"),
            AppError::InvalidApiKey => (StatusCode::UNAUTHORIZED, "INVALID_API_KEY"),
            // ...
        };
        // Return consistent error JSON
    }
}
```

**No unwrap() in production code. No expect() without good reason. Handle errors explicitly.**

### 6. Horizontal Scalability

**Stateless servers:**
- No in-memory state between requests
- No local file caching
- All state in PostgreSQL or GCS
- Any instance can handle any request

**Database design:**
- Connection pooling (SQLx handles this)
- Indexes on frequently queried columns
- Avoid N+1 queries - use JOINs or batch fetches

---

## Core Data Model

```sql
-- Arcade installations
clients (
    id UUID PRIMARY KEY,
    api_key VARCHAR(64) UNIQUE NOT NULL,
    name VARCHAR(255) NOT NULL,
    last_seen TIMESTAMP WITH TIME ZONE
)

-- Games in system
games (
    id UUID PRIMARY KEY,
    name VARCHAR(255) UNIQUE NOT NULL,
    gcs_bucket VARCHAR(255) NOT NULL,
    gcs_base_path VARCHAR(255) NOT NULL
)

-- Specific versions
game_versions (
    id UUID PRIMARY KEY,
    game_id UUID REFERENCES games(id),
    version VARCHAR(50) NOT NULL,
    gcs_path VARCHAR(512) NOT NULL,
    manifest JSONB NOT NULL,
    is_latest BOOLEAN DEFAULT FALSE,
    UNIQUE(game_id, version)
)

-- Client-game assignments
client_game_assignments (
    client_id UUID REFERENCES clients(id),
    game_id UUID REFERENCES games(id),
    target_version_id UUID REFERENCES game_versions(id),  -- NULL = latest
    PRIMARY KEY (client_id, game_id)
)
```

---

## API Design

### Authentication
- `POST /auth/token` - Exchange API key for JWT
- JWT expires in 1 hour, contains client_id
- All other endpoints require valid JWT in Authorization header

### Client-Facing (what Arceus calls)
- `GET /client/games` - List assigned games with manifest URLs
- `POST /client/games/{game_id}/download-urls` - Get signed URLs for specific files

### Admin (separate project later, but API exists)
- CRUD for clients, games, versions, assignments
- Protected by different auth mechanism

---

## Manifest Structure

Located at: `gs://{bucket}/{game}/{version}/manifest.json`

```json
{
  "version": "1.3.0",
  "files": {
    "Assets/scene.unity": {
      "hash": "sha256:abc123...",
      "size": 1048576
    }
  }
}
```

**Generated by CLI tool:**
```bash
./manifest-generator --bucket X --path Y --output manifest.json
```

Scans GCS folder, computes hashes, outputs JSON. Run once per version publish.

---

## Tech Stack

- **Framework**: Axum (modern, async-first, Tokio-based)
- **Database**: PostgreSQL with SQLx (compile-time query checking)
- **Auth**: JWT with `jsonwebtoken` crate
- **Storage**: GCS with signed URLs
- **Serialization**: Serde
- **Error handling**: thiserror + anyhow
- **Logging**: tracing

---

## What to Start With

**Recommended order:**

### Phase 1: Foundation (Start Here)
1. **Domain types** - Define your core entities with strong typing
2. **Error handling** - Set up AppError enum and response mapping
3. **Configuration** - Environment variables, database connection
4. **Database migrations** - Create tables with SQLx

**Why start here:** These are the building blocks everything else depends on. Get them right, and the rest is easier.

### Phase 2: Data Layer
1. **Repository traits** - Define interfaces for data access
2. **Repository implementations** - Implement with SQLx
3. **Unit tests** - Test queries work correctly

### Phase 3: Business Logic
1. **Service layer** - Implement core operations
2. **Auth service** - API key validation, JWT generation
3. **Game service** - Version management logic
4. **Update service** - Manifest comparison, URL generation

### Phase 4: API Layer
1. **Axum setup** - Router, middleware, extractors
2. **Auth middleware** - JWT validation
3. **Handlers** - Wire up endpoints to services
4. **Integration tests** - Test full request/response cycles

### Phase 5: Storage Integration
1. **GCS client** - Implement StorageProvider trait
2. **Signed URL generation** - Time-limited access
3. **Manifest generator tool** - Separate binary

### Phase 6: Polish
1. **Logging/tracing** - Observability
2. **Input validation** - Sanitize all inputs
3. **Rate limiting** - Protect against abuse
4. **Deployment configs** - Docker, Cloud Run

---

## Key Questions to Answer as You Build

1. How will admin authentication work initially? (Basic auth? Separate JWT?)
2. How long should signed URLs be valid? (1 hour recommended)
3. What's the max game size you expect? (Affects timeouts, chunk sizes)
4. Do you need to support multiple GCS buckets per game?
5. Should version strings be validated as semver?

---

## Success Metrics

- **Clean architecture**: Each layer testable in isolation
- **Type safety**: Compile-time catches most errors
- **Performance**: <100ms response for update checks
- **Scalability**: Handles 50+ concurrent clients
- **Maintainability**: New developer understands structure in <30 min
- **Extensibility**: Adding new feature doesn't require modifying existing code

---

## Common Pitfalls to Avoid

1. **Business logic in handlers** - Always delegate to services
2. **Leaking HTTP concerns** - Services shouldn't know about StatusCode
3. **String typing** - Use enums and newtypes, not raw strings
4. **Ignoring errors** - Handle every Result explicitly
5. **Tight coupling** - Use traits to abstract external dependencies
6. **Premature optimization** - Get it correct first, optimize later
7. **Skipping tests** - Test services thoroughly, they contain the logic
8. **Monolithic services** - Keep services focused, split if they grow too large

---

## Final Note

This server is the backbone of your VR arcade network. Invest in the architecture now. Every shortcut will cost 10x later when you have 50+ arcades depending on it. Build it right, build it clean, build it to last.
