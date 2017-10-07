use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::{Read, Write, Error, ErrorKind};
use std::path::PathBuf;
use std::hash::Hash;
use std::collections::hash_map::Entry::*;
use std::collections::*;
use serde_json;
use std::time::{SystemTime, Duration};

/// Caches precomputed values to avoid wasting resources.
#[derive(Serialize, Deserialize)]
pub struct Cache<K, V>
    where K: Hash + Eq
{
    store: HashMap<K, CachedItem<V>>

    #[serde(default = "default_store_path")]
    #[serde(skip)]
    path: PathBuf,
}

/// Holds a value and its expiration date.
#[derive(Serialize, Deserialize)]
struct CachedItem<V> {
    value: V,
    expires: SystemTime,
}

impl<K, V> Cache<K, V>
    where K: Hash + Eq
{
    /// Constructs a new empty Cache
    pub fn new() -> Self {
        Cache {
            path: default_store_path(),
            store: HashMap::new(),
        }
    }

    /// Returns the value corresponding to the given key if it's present in the cache
    /// and not expired. Otherwise uses the compute function to build the value and
    /// store it in the cache for later use.
    pub fn get_or_compute<F>(&mut self, key: K, compute: F) -> &V
        where F: FnOnce() -> V
    {
        let build_value  = || CachedItem {
            value: compute(),
            expires: SystemTime::now() + Duration::from_secs(3600)
        };

        let item = match self.store.entry(key) {
            Occupied(entry) => {
                let item = entry.into_mut();

                if item.expires < SystemTime::now() {
                    *item = build_value();
                }

                item
            },
            Vacant(entry) => entry.insert(build_value())
        };

        &item.value
    }

    /// Writes to disk the cache.
    pub fn save(&self)
        where K: Serialize,
              V: Serialize
    {
        let _ = File::create(&self.path)
            .and_then(|mut file| {
                serde_json::to_string(&self.store)
                    .map_err(|err| Error::new(ErrorKind::InvalidData, format!("Can't serialize cache: {}", err)))
                    .and_then(move |toml| file.write_all(toml.as_bytes()))
            });
    }

    /// Recreates a cache from the disk.
    pub fn load() -> Self
        where K: DeserializeOwned,
              V: DeserializeOwned
    {
        File::open(&default_store_path())
            .and_then(|mut file| {
                let mut string = String::new();
                let _          = file.read_to_string(&mut string)?;
                serde_json::from_str(&string)
                    .or_else(|err| Err(Error::new(ErrorKind::InvalidData, format!("Can't read configuration file: {}", err))))
            })
            .unwrap_or(Cache::new())
    }
}

fn default_store_path() -> PathBuf {
    PathBuf::from("/tmp/ran-git-cache.json")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_or_compute_computes_when_the_value_is_not_cached() {
        let mut cache = Cache::new();
        let result = cache.get_or_compute(1, || 1);
        assert!(1 == *result);
    }

    #[test]
    fn get_or_compute_dont_compute_when_the_value_is_cached() {
        let mut cache = Cache::new();
        cache.store.insert(1, CachedItem { value: 1 });

        let result = cache.get_or_compute(1, || panic!());
        assert!(1 == *result);
    }
}
