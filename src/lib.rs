use core::fmt::Debug;
use crate::bytesref::BytesRef;
use std::convert::TryInto;
use crate::hasher::fnv32a_yoshimitsu_hasher;

mod hasher;
mod bytesref;

#[derive(Debug)]
pub struct HashMap<T> {
    string_data: Vec<u8>,
    /// pointer to string data
    table: Vec<TableEntry<T>>,
    bitshift: usize,
    occupied: usize,
}


#[derive(Debug, Clone, Default)]
pub(crate) struct TableEntry<T> {
    value: T,
    pointer: BytesRef
}

impl<T: Default + Clone + Debug> HashMap<T> {
    pub fn with_size(size: usize) -> Self {
        let mut table = vec![];
        table.resize(size, TableEntry::default());
        HashMap{
            string_data: vec![],
            table,
            bitshift: 25,
            occupied: 0,
        }
    }
    pub fn new() -> Self {
        Self::with_size(1<<24)
    }

    #[inline(never)]
    pub fn get_or_create(&mut self, el: &str, value: T) -> &mut T {
        // check load factor, resize when 0.5
        if self.occupied * 2 > self.table.len() {
            self.resize();
        }
        let hash = fnv32a_yoshimitsu_hasher(el.as_bytes()) >> self.bitshift;
        let mask = self.table.len() - 1;
        let mut probe = QuadraticProbing::compute(hash as usize, mask);
        let mut hash = probe.next_probe();
        loop {
            let entry = self.get_entry(hash);
            if entry.pointer.is_null() {
                self.occupied+=1;
                let inserted_value = self.put_in_bucket(hash as usize, el, value);
                return &mut inserted_value.value;
            }else if self.read_string(entry.pointer) == el {
                return &mut self.table[hash as usize].value;
            }
            hash = probe.next_probe();
        }
    }

    fn get_entry(&self, hash: usize) -> &TableEntry<T> {
        unsafe{self.table.get_unchecked(hash as usize)}
    }
    fn get_entry_mut(&mut self, hash: usize) -> &mut TableEntry<T> {
        unsafe{self.table.get_unchecked_mut(hash as usize)}
    }

    /// Double the size of the capacity
    pub fn resize(&mut self) {
        let mut table: Vec<TableEntry<T>> = vec![];
        table.resize(self.table.len() * 2, TableEntry::default());

        std::mem::swap(&mut self.table, &mut table);
        self.bitshift += 1;
        for entry in table.iter().filter(|x| !x.pointer.is_null()) {
            let text = self.read_string(entry.pointer).to_string(); // TODO remove copy
            self.get_or_create(&text, entry.value.clone());
        }
        
    }

    pub(crate) fn put_in_bucket(&mut self, hash: usize, el: &str, value: T) -> &mut TableEntry<T> {
        let pos = BytesRef(self.string_data.len() as u32);
        let len_as_bytes = (el.len() as u32).to_le_bytes();
        self.string_data.extend_from_slice(&len_as_bytes);
        self.string_data.extend_from_slice(el.as_bytes());
        let entry = self.get_entry_mut(hash);
        *entry = TableEntry{value, pointer: pos};
        entry
    }
    pub(crate) fn read_string(&self, pos: BytesRef) -> &str {
        let pos = pos.0 as usize;
        let length_string_bytes: [u8;4] = self.string_data[pos..pos + 4].try_into().unwrap();
        let length_string = u32::from_le_bytes(length_string_bytes);
        unsafe {
            std::str::from_utf8_unchecked(&self.string_data[pos + 4 .. pos + 4 + length_string as usize])
        }
    }

}


struct QuadraticProbing {
    hash: usize,
    i: usize,
    mask: usize,
}

impl QuadraticProbing {
    fn compute(hash: usize, mask: usize) -> QuadraticProbing {
        QuadraticProbing { hash, i: 1, mask }
    }

    #[inline]
    fn next_probe(&mut self) -> usize {
        self.i += 1;
        // (self.hash + (self.i + self.i * self.i) >> 1) & self.mask
        (self.hash + (self.i + self.i * self.i) >> 1) & self.mask
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_or_create() {
        let mut hashmap = HashMap::<u32>::new();
        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 0);
        *val += 1;

        let val = hashmap.get_or_create("blub", 0);
        assert_eq!(*val, 1);
    }
    #[test]
    fn test_resize() {
        let mut hashmap = HashMap::<u32>::with_size(4);
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

