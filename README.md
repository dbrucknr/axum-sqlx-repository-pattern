# axum-sqlx-repository-pattern


## Dependencies
- Base: `cargo add axum tokio -F tokio/full serde -F serde/derive`
- Database: `cargo add sqlx --features runtime-tokio,tls-native-tls,sqlite`
- Environment Variables: `cargo add dotenvy`

## Steps
1. `cargo install sqlx-cli`
2. `sqlx db create`
