use std::fmt;
use std::u64;

#[derive(Clone, Copy, Eq, PartialEq)]
pub(crate) struct TinySet(u64);

impl fmt::Debug for TinySet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.into_iter().collect::<Vec<u32>>().fmt(f)
    }
}

pub struct TinySetIterator(TinySet);
impl Iterator for TinySetIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_lowest()
    }
}

impl IntoIterator for TinySet {
    type Item = u32;
    type IntoIter = TinySetIterator;
    fn into_iter(self) -> Self::IntoIter {
        TinySetIterator(self)
    }
}

impl TinySet {
    /// Returns an empty `TinySet`.
    pub fn empty() -> TinySet {
        TinySet(0u64)
    }

    pub fn clear(&mut self) {
        self.0 = 0u64;
    }

    /// Returns the complement of the set in `[0, 64[`.
    fn complement(self) -> TinySet {
        TinySet(!self.0)
    }

    /// Returns true iff the `TinySet` contains the element `el`.
    pub fn contains(self, el: u32) -> bool {
        !self.intersect(TinySet::singleton(el)).is_empty()
    }

    /// Returns the number of elements in the TinySet.
    pub fn len(self) -> u32 {
        self.0.count_ones()
    }

    /// Returns the intersection of `self` and `other`
    pub fn intersect(self, other: TinySet) -> TinySet {
        TinySet(self.0 & other.0)
    }

    /// Creates a new `TinySet` containing only one element
    /// within `[0; 64[`
    #[inline(always)]
    pub fn singleton(el: u32) -> TinySet {
        TinySet(1u64 << u64::from(el))
    }

    /// Insert a new element within [0..64[
    #[inline(always)]
    pub fn insert(self, el: u32) -> TinySet {
        self.union(TinySet::singleton(el))
    }

    /// Insert a new element within [0..64[
    #[inline(always)]
    pub fn insert_mut(&mut self, el: u32) -> bool {
        let old = *self;
        *self = old.insert(el);
        old != *self
    }

    /// Returns the union of two tinysets
    #[inline(always)]
    pub fn union(self, other: TinySet) -> TinySet {
        TinySet(self.0 | other.0)
    }

    /// Returns true iff the `TinySet` is empty.
    #[inline(always)]
    pub fn is_empty(self) -> bool {
        self.0 == 0u64
    }

    /// Returns the lowest element in the `TinySet`
    /// and removes it.
    #[inline(always)]
    pub fn pop_lowest(&mut self) -> Option<u32> {
        if self.is_empty() {
            None
        } else {
            let lowest = self.0.trailing_zeros() as u32;
            self.0 ^= TinySet::singleton(lowest).0;
            Some(lowest)
        }
    }

    /// Returns a `TinySet` than contains all values up
    /// to limit excluded.
    ///
    /// The limit is assumed to be strictly lower than 64.
    pub fn range_lower(upper_bound: u32) -> TinySet {
        TinySet((1u64 << u64::from(upper_bound % 64u32)) - 1u64)
    }

    /// Returns a `TinySet` that contains all values greater
    /// or equal to the given limit, included. (and up to 63)
    ///
    /// The limit is assumed to be strictly lower than 64.
    pub fn range_greater_or_equal(from_included: u32) -> TinySet {
        TinySet::range_lower(from_included).complement()
    }
}

#[derive(Clone)]
pub struct BitSet {
    tinysets: Box<[TinySet]>,
    len: usize,
    max_value: u32,
}

fn num_buckets(max_val: u32) -> u32 {
    (max_val + 63u32) / 64u32
}

impl BitSet {
    /// Create a new `BitSet` that may contain elements
    /// within `[0, max_val[`.
    pub fn with_max_value(max_value: u32) -> BitSet {
        let num_buckets = num_buckets(max_value);
        let tinybisets = vec![TinySet::empty(); num_buckets as usize].into_boxed_slice();
        BitSet {
            tinysets: tinybisets,
            len: 0,
            max_value,
        }
    }

    /// Removes all elements from the `BitSet`.
    pub fn clear(&mut self) {
        for tinyset in self.tinysets.iter_mut() {
            *tinyset = TinySet::empty();
        }
    }

    /// Returns the number of elements in the `BitSet`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Inserts an element in the `BitSet`
    pub fn insert(&mut self, el: u32) {
        // we do not check saturated els.
        let higher = el / 64u32;
        let lower = el % 64u32;
        self.len += if self.tinysets[higher as usize].insert_mut(lower) {
            1
        } else {
            0
        };
    }

    /// Returns true iff the elements is in the `BitSet`.
    pub fn contains(&self, el: u32) -> bool {
        self.tinyset(el / 64u32).contains(el % 64)
    }

    /// Returns the first non-empty `TinySet` associated to a bucket lower
    /// or greater than bucket.
    ///
    /// Reminder: the tiny set with the bucket `bucket`, represents the
    /// elements from `bucket * 64` to `(bucket+1) * 64`.
    pub(crate) fn first_non_empty_bucket(&self, bucket: u32) -> Option<u32> {
        self.tinysets[bucket as usize..]
            .iter()
            .cloned()
            .position(|tinyset| !tinyset.is_empty())
            .map(|delta_bucket| bucket + delta_bucket as u32)
    }

    pub fn max_value(&self) -> u32 {
        self.max_value
    }

    /// Returns the tiny bitset representing the
    /// the set restricted to the number range from
    /// `bucket * 64` to `(bucket + 1) * 64`.
    pub(crate) fn tinyset(&self, bucket: u32) -> TinySet {
        self.tinysets[bucket as usize]
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item=u32> + 'a {
        self.tinysets
            .iter()
            .cloned()
            .enumerate()
            .flat_map(|(ord, tinyset)| {
                let offset = (ord *64) as  u32;
                tinyset.into_iter().map(move |el| offset + el)
            })
    }
}
