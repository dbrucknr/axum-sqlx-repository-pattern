# axum-sqlx-repository-pattern

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

## Steps
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
