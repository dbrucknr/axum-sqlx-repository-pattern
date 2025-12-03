# axum-sqlx-repository-pattern

## Running via Docker + Docker Compose

Notable commands:
- docker compose --profile dev ps
- docker compose --profile dev logs -f app-dev
- docker compose exec app-dev bash
  - cargo sqlx migrate info
- docker compose exec -u postgres -it postgres psql -U postgres -h postgres -p 5432 -d postgres
  - /dt

There are 2 targets (dev and prod)
To run a dev context: 
- `docker compose --profile dev up -d --build` or `docker compose --profile dev up --build` (For logs)
- `docker compose --profile dev down`

To run a prod context: 
-`docker compose --profile prod up -d`


I wonder if docker could help with testing contexts?:
```yaml
  # Test Target: docker compose --profile test up --build -d
  app-test:
    profiles: ["test"]
    build:
      context: .
      target: test
    ports:
      - "8000:8000"
```

I switched from SQLite to PostgreSQL
- I needed to update the repository to use PgPool instead of SqlitePool - I changed all the references 
- I think I'll need to update the SQL dialects in the /migrations to be compatible with PostgreSQL
- I'm not sure yet, but I suspect I may need to change the Device struct in models.rs

 psql -U postgres -h hostname -p port_number -d database_name

- `cargo run`

- `cargo build --release`
- `./target/release/axum-sqlx-repository-pattern`

I needed to stop tracking accidentally committed files:
- git rm --cached database.db
- git rm --cached database.db-wal
- git rm --cached database.db-shm
- git add .
- git commit -m "Remove database files"
- git push

## Dependencies
- Base: `cargo add axum tokio -F tokio/full serde -F serde/derive`
- Database: `cargo add sqlx --features runtime-tokio,tls-native-tls,sqlite`
- Environment Variables: `cargo add dotenvy`
- Tracing + Logging: `cargo add tower-http tracing tracing-subscriber -F tower-http/trace -F tracing-subscriber/env-filter`

## Database Preparation Steps
1. `cargo install sqlx-cli`
2. `sqlx db create`
3. `sqlx migrate add -r create_devices_table`

I used a database viewer to add data to the devices table.
```sql
INSERT INTO devices (
	serial_number
)
VALUES (
	"1234567890"
);
```

### Comparing to a simple Python FastAPI app:

This app appears to use approximately 1.2 MB of memory.
Typical Response Latency on GET Requests: 0-1 ms
Typical Response Latency on POST Requests: 5-15 ms

I compared these metrics to a simple FastAPI app with one handler, and middleware to track response latency:

The FastAPI app using uvicorn uses approximately 32.9 MB of memory.

Typical Response Latency was above 80 ms for the route below.
- Range: 66 ms to 1.19 ms
- NOTE that this endpoint doesn't make any calls to a database

```python
from time import perf_counter

from fastapi import FastAPI, Request
from fastapi.responses import ORJSONResponse

app = FastAPI(
    default_response_class=ORJSONResponse,
)


@app.middleware("http")
async def add_process_time_header(request: Request, call_next):
    start_time = perf_counter()
    response = await call_next(request)
    process_time_ms = (perf_counter() - start_time) * 1000
    print(f"Process time: {process_time_ms:.2f} ms")
    response.headers["X-Process-Time"] = f"{process_time_ms:.2f} ms"
    return response


@app.get("/")
async def root():
    return {"message": "Hello World"}

```
