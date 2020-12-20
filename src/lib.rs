/*! Stores values for strings in a Hashmap in a fast and compact way.

Good to count strings and assign ids to them or similar. Address space of string data is limited to u32::MAX (4GB).
string data is size in bytes of all uniquely inserted strings + string length metadata per string.

# Examples
```
use inohashmap::StringHashMap;
let mut hashmap = StringHashMap::<u32>::new();
let val = hashmap.get_or_create("blub1", 0);
assert_eq!(*val, 0);
*val += 1;

let val = hashmap.get_or_create("blub2", 2);
assert_eq!(*val, 2);

```

*/

use crate::bytesref::BytesRef;
use crate::hasher::fnv32a_yoshimitsu_hasher;
use core::fmt::Debug;
use vint32::{encode_varint_into, decode_varint_slice};
mod bytesref;
pub mod hasher;

#[derive(Debug)]
pub struct StringHashMap<T> {
    /// contains string in compressed format
    pub(crate) string_data: Vec<u8>,
    /// pointer to string data and value
    pub(crate) table: Vec<TableEntry<T>>,
    bitshift: usize,
    pub occupied: usize,
    mask: u32,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct TableEntry<T> {
    value: T,
    pointer: BytesRef,
}

impl<T: Default + Clone + Debug> StringHashMap<T> {
    #[inline]
    pub fn with_power_of_two_size(power_of_two: usize) -> Self {
        let shift = power_of_two - 1;
        let mut table = vec![];
        table.resize(1 << shift, TableEntry::default());
        StringHashMap {
            string_data: Vec::with_capacity((1 << shift) * 2),
            mask: table.len() as u32 - 1,
            table,
            bitshift: 32 - power_of_two,
            occupied: 0,
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self::with_power_of_two_size(10)
    }

    #[inline]
    pub fn get_or_create(&mut self, el: &str, value: T) -> &mut T {
        // check load factor, resize when 0.5
        // if self.occupied as f32 * 1.5 > self.table.len() as f32 {
        if self.occupied as f32 * 1.5 > self.table.len() as f32 {
            self.resize();
        }
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe() as usize;

        loop {
            let entry = self.get_entry(hash);
            if entry.pointer.is_null() {
                self.occupied += 1;
                let inserted_value = self.put_in_bucket(hash as usize, el, value);
                return &mut inserted_value.value;
            } else if self.read_string(entry.pointer) == el {
                return &mut self.get_entry_mut(hash as usize).value;
            }
            hash = probe.next_probe() as usize;
        }
    }

    #[inline]
    fn get_probe(&self, el: &str) -> QuadraticProbing {
        let hash = fnv32a_yoshimitsu_hasher(el.as_bytes());
        let hash = hash >> self.bitshift;
        let probe = QuadraticProbing::compute(hash, self.mask);
        probe
    }

    #[inline]
    fn put_entry_resize(&mut self, el: &str, new_entry: TableEntry<T>) {
        let mut probe = self.get_probe(el);
        let mut hash = probe.next_probe();
        loop {
            let entry = self.get_entry_mut(hash as usize);
            if entry.pointer.is_null() {
                entry.pointer = new_entry.pointer;
                entry.value = new_entry.value;
                return;
            }
            hash = probe.next_probe();
        }
    }

    #[inline]
    pub fn get_values(&self) -> impl Iterator<Item = &T> {
        self.table
            .iter()
            .filter(|entry| !entry.pointer.is_null())
            .map(|entry| &entry.value)
    }

    #[inline]
    fn get_entry(&self, hash: usize) -> &TableEntry<T> {
        unsafe { self.table.get_unchecked(hash) }
    }
    #[inline]
    fn get_entry_mut(&mut self, hash: usize) -> &mut TableEntry<T> {
        unsafe { self.table.get_unchecked_mut(hash as usize) }
    }

    /// Doubles the size of the table
    #[cold]
    pub fn resize(&mut self) {
        let mut table: Vec<TableEntry<T>> = vec![];
        table.resize(self.table.len() * 2, TableEntry::default());
        self.mask = table.len() as u32 - 1;

        std::mem::swap(&mut self.table, &mut table);
        self.bitshift -= 1;
        for entry in table.into_iter().filter(|x| !x.pointer.is_null()) {
            let text = self.read_string(entry.pointer);
            // casting away lifetime of text
            // Since string_data will not be altered in put_entry_resize
            let text = unsafe { std::mem::transmute::<&str, &'static str>(text) };
            self.put_entry_resize(text, entry);
        }
    }

    #[inline]
    pub(crate) fn put_in_bucket(&mut self, hash: usize, el: &str, value: T) -> &mut TableEntry<T> {
        let pos = BytesRef(self.string_data.len() as u32);

        encode_varint_into(&mut self.string_data, el.len() as u32);    

        self.string_data.extend_from_slice(el.as_bytes());
        // unsafe {
        //     self.string_data.reserve(el.len());
        //     let target = self.string_data.as_mut_ptr().add(self.string_data.len());
        //     std::ptr::copy_nonoverlapping(el.as_bytes().as_ptr(), target, el.as_bytes().len());
        //     self.string_data.set_len(self.string_data.len()+ el.len() );
        // };

        let entry = self.get_entry_mut(hash);
        *entry = TableEntry {
            value,
            pointer: pos,
        };
        entry
    }

    #[inline]
    pub(crate) fn read_string(&self, pos: BytesRef) -> &str {
        let mut pos = pos.addr() as usize;
        let length_string = decode_varint_slice(&self.string_data, &mut pos).unwrap();
        unsafe {
            std::str::from_utf8_unchecked(
                &self.string_data.get_unchecked(pos..pos + length_string as usize),
            )
        }
    }
}

struct QuadraticProbing {
    hash: u32,
    i: u32,
    mask: u32,
}

impl QuadraticProbing {
    #[inline]
    fn compute(hash: u32, mask: u32) -> QuadraticProbing {
        QuadraticProbing { hash, i: 1, mask }
    }

    #[inline]
    fn next_probe(&mut self) -> u32 {
        self.i += 1;
        (self.hash + (self.i + self.i * self.i) >> 1) & self.mask
        // (self.hash + (self.i * self.i)) & self.mask
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_values_big() {
        use std::io::Read;

        let mut contents = String::new();
        std::fs::File::open("1342-0.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        let mut map = StringHashMap::<u32>::new();
        let mut counter = 0;
        for text in contents.split_whitespace() {
            let value = map.get_or_create(text, 0);
            *value += 1;
            counter += 1;
        }

        let sum: u32 = map.get_values().sum();
        assert_eq!(sum, counter);
        assert_eq!(map.string_data.len() < 1_000_000, true);

        dbg!(counter);

        // let num_one_time_probe= map.num_probes.iter().filter(|el| *el == &1).cloned().sum::<u32>();
        // let num_two_time_probe= map.num_probes.iter().filter(|el| *el == &2).cloned().sum::<u32>();
        // let num_more_than_one_time_probe= map.num_probes.iter().filter(|el| *el != &1).cloned().sum::<u32>();
        // dbg!(num_one_time_probe);
        // dbg!(num_two_time_probe);
        // dbg!(num_more_than_one_time_probe);
        // dbg!(map.existing);

    }
    #[test]
    fn get_values() {
        let mut hashmap = StringHashMap::<u32>::new();
        hashmap.get_or_create("blub", 1);

        let val: u32 = hashmap.get_values().sum();
        assert_eq!(val, 1);
    }
    #[test]
    fn simple() {
        let mut hashmap = StringHashMap::<u32>::new();
        let val = hashmap.get_or_create("blub1", 0);
        assert_eq!(*val, 0);
        *val += 1;

        let val = hashmap.get_or_create("blub2", 2);
        assert_eq!(*val, 2);
    }
    #[test]
    fn get_or_create() {
        let mut hashmap = StringHashMap::<u32>::new();
        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 0);
        *val += 1;

        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 1);
    }
    #[test]
    fn test_resize() {
        let mut hashmap = StringHashMap::<u32>::with_power_of_two_size(1);
        hashmap.get_or_create("blub1", 3);
        hashmap.get_or_create("blub2", 4);

        assert_eq!(hashmap.get_or_create("blub1", 3), &3);

        //should resize
        let val = hashmap.get_or_create("blub3", 5);
        assert_eq!(*val, 5);

        // // check values after resize
        assert_eq!(hashmap.get_or_create("blub1", 0), &3);
        assert_eq!(hashmap.get_or_create("blub2", 0), &4);
        assert_eq!(hashmap.get_or_create("blub3", 0), &5);
    }
}
