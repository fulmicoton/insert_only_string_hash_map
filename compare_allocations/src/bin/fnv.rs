
use dhat::{Dhat, DhatAlloc};
use fnv::FnvHashMap;
use std::io::Read;

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
        let mut map: FnvHashMap<String, u32> = FnvHashMap::with_capacity_and_hasher(1 << 10, Default::default());
        for text in contents.split_whitespace() {
            let data = get_or_insert_prefer_get(&mut map, text, || { 0 });
            *data  += 1;
        }
    }
}

fn get_or_insert_prefer_get<'a, T, F>(map: *mut FnvHashMap<String, T>, key: &str, mut constructor: F) -> &'a mut T
where
    F: FnMut() -> T,
{
    unsafe {
        if let Some(e) = (*map).get_mut(key) {
            return e;
        }

        (*map).insert(key.to_string(), constructor());
        (*map).get_mut(key).unwrap()
    }
}