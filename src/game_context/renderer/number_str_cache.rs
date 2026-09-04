use elsa::FrozenMap;

/// This capacity matches the largest `CoreDimension` size.
const DEFAULT_STORAGE_CAPACITY: usize = 8000;

/// `NumberStrCache` is responsible for building and cacheing string representation of any `usize` integer.
pub struct NumberStrCache {
    storage: [Box<str>; DEFAULT_STORAGE_CAPACITY],
    extra_storage: FrozenMap<usize, Box<str>>,
}

impl Default for NumberStrCache {
    fn default() -> Self {
        Self {
            storage: std::array::from_fn(|i| i.to_string().into_boxed_str()),
            extra_storage: FrozenMap::new(),
        }
    }
}

impl NumberStrCache {
    // Get a `&str` representation for input `num`.
    /// First look in `storage`, then look/insert in `extra_storage`.
    pub fn get_str(&self, num: usize) -> &str {
        if num < self.storage.len() {
            #[allow(clippy::indexing_slicing, reason = "The index is valid 👌")]
            return &self.storage[num];
        }

        self.extra_storage.get(&num).unwrap_or_else(|| {
            self.extra_storage
                .insert(num, num.to_string().into_boxed_str())
        })
    }
}
