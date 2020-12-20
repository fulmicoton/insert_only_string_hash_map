
use inohashmap::StringHashMap;
use std::io::Read;
use dhat::{Dhat, DhatAlloc};

#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() {
    let mut contents = String::new();
        std::fs::File::open("../1342-0.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

    let _dhat = Dhat::start_heap_profiling();
    for _ in 0..10 {
        let mut map = StringHashMap::<u32>::with_power_of_two_size(10);
        for text in contents.split_whitespace() {
            let value = map.get_or_create(text, 0);
            *value += 1;
        }
    }
}
