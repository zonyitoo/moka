use moka::{future::Cache, Entry};

/// This example demonstrates how to increment a cached `u64` counter. It uses the
/// `and_upsert_with` method of `Cache`.
#[tokio::main]
async fn main() {
    let cache: Cache<String, u64> = Cache::new(100);
    let key = "key".to_string();

    let entry = inclement_counter(&cache, &key).await;
    assert!(entry.is_fresh());
    assert!(!entry.is_updated());
    assert_eq!(entry.into_value(), 1);

    let entry = inclement_counter(&cache, &key).await;
    assert!(entry.is_fresh());
    assert!(entry.is_updated());
    assert_eq!(entry.into_value(), 2);

    let entry = inclement_counter(&cache, &key).await;
    assert!(entry.is_fresh());
    assert!(entry.is_updated());
    assert_eq!(entry.into_value(), 3);
}

async fn inclement_counter(cache: &Cache<String, u64>, key: &str) -> Entry<String, u64> {
    cache
        .entry_by_ref(key)
        .and_upsert_with(|maybe_entry| {
            let v = if let Some(entry) = maybe_entry {
                // The entry exists, increment the value by 1.
                entry.into_value().saturating_add(1)
            } else {
                // The entry does not exist, insert a new value of 1.
                1
            };
            // Return a Future that is resolved to `v` immediately.
            std::future::ready(v)
        })
        .await
}
