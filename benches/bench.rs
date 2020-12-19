#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use insert_only_string_hash_map::HashMap;
    use std::fs::File;
    use super::*;
    use std::io::Read;
    use test::Bencher;

    #[bench]
    fn bench_hasmap(b: &mut Bencher) {
        let contents = std::str::from_utf8(include_bytes!("../test.txt")).unwrap();
        // let mut contents = String::new();
        // include_bytes
        // File::open("test.txt")
        //     .unwrap()
        //     .read_to_string(&mut contents)
        //     .unwrap();

        let mut map = HashMap::<u32>::new();
        b.iter(|| {

            for text in contents.split_whitespace() {
                let value = map.get_or_create(text, 0);
                *value += 1;
            }

        });
    }

    // #[bench]
    // fn bench_fnv(b: &mut Bencher) {
    //     use fnv::FnvHashMap;
    //     let mut contents = String::new();
    //     File::open("test.txt")
    //         .unwrap()
    //         .read_to_string(&mut contents)
    //         .unwrap();

    //     let mut map:FnvHashMap<String, u32> = FnvHashMap::with_capacity_and_hasher(1<<24, Default::default());
    //     b.iter(|| {

    //         for text in contents.split_whitespace() {
    //             *map.entry(text.to_string()).or_insert(0) += 1;
    //         }

    //     });
    // }
}