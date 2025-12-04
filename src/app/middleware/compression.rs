use tower_http::compression::CompressionLayer;

// GET request before compression: 325B
// After compression: 230B
// After setting headers: Accept-Encoding: br, gzip, zstd, deflate
// This is micro-optimization, but interesting to implement...
// It will help once payload responses become larger. Can add a
// predicate check for advanced conditional compression:
// https://docs.rs/tower-http/latest/tower_http/compression/struct.Compression.html#method.compress_when
pub fn compress_responses() -> CompressionLayer {
    CompressionLayer::new()
        .br(true)
        .deflate(true)
        .gzip(true)
        .zstd(true)
}
