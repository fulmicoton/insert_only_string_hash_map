#![feature(test)]

extern crate test;

use fnv::FnvHashMap;




use tantivity_term_map::map::TermHashMap;



#[cfg(test)]
mod tests {

    #[derive(Debug, Default, Clone, Copy)]
    struct MoreMetaData {
        counter1: usize,
        counter2: usize,
        counter3: usize,
        counter4: usize,
        counter5: usize,
    }

    use super::*;
    use inohashmap::StringHashMap;
    
    use std::io::Read;
    use test::Bencher;

    fn get_test_string() -> String {
        let mut contents = String::new();
        std::fs::File::open("test.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        contents
    }
    fn get_test_string_full() -> String {
        let mut contents = String::new();
        std::fs::File::open("1342-0.txt")
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        contents
    }

    #[bench]
    fn bench_tant_termmap(b: &mut Bencher) {
        let contents = get_test_string();

        b.iter(|| {
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
        });
    }

    #[bench]
    fn bench_tant_termmap_full_large_struct(b: &mut Bencher) {
        let contents = get_test_string_full();

        b.iter(|| {
            let mut map = TermHashMap::new(10);
            for text in contents.split_whitespace() {
                map.mutate_or_create(text, |el: Option<MoreMetaData>| {
                    if let Some(mut el) = el {
                        el.counter1 += 1;
                        el
                    }else{
                        MoreMetaData::default()
                    }
                });
            }
        });
    }

    #[bench]
    fn bench_tant_termmap_full(b: &mut Bencher) {
        let contents = get_test_string_full();

        b.iter(|| {
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
        });
    }
    #[bench]
    fn bench_hasmap_full_large_struct(b: &mut Bencher) {
        let contents = get_test_string_full();

        b.iter(|| {
            let mut map = StringHashMap::<MoreMetaData>::with_power_of_two_size(10);
            for text in contents.split_whitespace() {
                let value = map.get_or_create(text, MoreMetaData::default());
                value.counter1 += 1;
            }
        });
    }
    #[bench]
    fn bench_hasmap(b: &mut Bencher) {
        let contents = get_test_string();

        b.iter(|| {
            let mut map = StringHashMap::<u32>::with_power_of_two_size(10);
            for text in contents.split_whitespace() {
                let value = map.get_or_create(text, 0);
                *value += 1;
            }
        });
    }
    #[bench]
    fn bench_hasmap_full(b: &mut Bencher) {
        let contents = get_test_string_full();

        b.iter(|| {
            let mut map = StringHashMap::<u32>::with_power_of_two_size(10);
            for text in contents.split_whitespace() {
                let value = map.get_or_create(text, 0);
                *value += 1;
            }
        });
    }

    #[bench]
    fn bench_fnv(b: &mut Bencher) {
        let contents = get_test_string();

        b.iter(|| {
            let mut map: FnvHashMap<String, u32> = FnvHashMap::with_capacity_and_hasher(1 << 10, Default::default());
            for text in contents.split_whitespace() {
                let data = get_or_insert_prefer_get(&mut map, text, || { 0 });
                *data  += 1;
            }
        });
    }
    #[bench]
    fn bench_fnv_full(b: &mut Bencher) {
        let contents = get_test_string_full();

        b.iter(|| {
            let mut map: FnvHashMap<String, u32> = FnvHashMap::with_capacity_and_hasher(1 << 10, Default::default());
            for text in contents.split_whitespace() {
                let data = get_or_insert_prefer_get(&mut map, text, || { 0 });
                *data  += 1;
            }
        });
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