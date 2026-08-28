use elsa::FrozenMap;

/// `NumberStrCache` is responsible for building and cacheing string representation of any number.
pub struct NumberStrCache {
    i32_str_cache: FrozenMap<i32, Box<str>>,
}

impl Default for NumberStrCache {
    fn default() -> Self {
        Self {
            i32_str_cache: FrozenMap::new(),
        }
    }
}

impl NumberStrCache {
    /// Given `num`, return its `&str` representation from cache, or build and insert a new entry in cache if needed.
    pub fn i32_to_str(&self, num: i32) -> &str {
        self.i32_str_cache.get(&num).unwrap_or_else(|| {
            self.i32_str_cache
                .insert(num, num.to_string().into_boxed_str())
        })
    }
}
