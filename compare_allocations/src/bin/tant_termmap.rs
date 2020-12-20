
use tantivity_term_map::map::TermHashMap;
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
        let mut map = TermHashMap::new(10);
        for text in contents.split_whitespace() {
            map.mutate_or_create(text, |el| {
                if let Some(el) = el {
                    el+1
                }else{
                    0
                }
            });
        }
    }
}
