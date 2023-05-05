#![feature(test)]

extern crate test;

fn linear_search<T>(haystack: &[T], needle: T) -> bool 
where T: Eq{
    for item in haystack {
        if *item == needle {
            return true;
        }
    }
    false 
}

fn binary_search<T>(haystack: &[T], needle: T) -> bool where T:Eq+PartialOrd{
    let mut low = 0;
    let mut high = haystack.len() - 1;

    while low <= high {
        let middle = (low + high) / 2;
        if haystack[middle] > needle {
            high = middle - 1;
        } else if haystack[middle] < needle {
            low = middle + 1;
        } else {
            return true;
        }
    }
    false
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_binary_search() {
        let mut rng = rand::thread_rng();
        let haystack: Vec<i32> = (0..10000000).collect();
        assert_eq!(binary_search(&haystack, rng.gen_range(0..10000000)), true);
    }

    #[bench]
    fn bench_binary_search(b: &mut test::Bencher) {
        let mut rng = rand::thread_rng();
        let haystack: Vec<i32> = (0..10000000).collect();
        b.iter(|| binary_search(&haystack, rng.gen_range(0..10000000)));
    }

    #[test]
    fn test_linear_search() {
        let mut rng = rand::thread_rng();
        let haystack: Vec<i32> = (0..10000000).collect();
        assert_eq!(linear_search(&haystack, rng.gen_range(0..10000000)), true);
    }

    #[bench]
    fn bench_linear_search(b: &mut test::Bencher) {
        let mut rng = rand::thread_rng();
        let haystack: Vec<i32> = (0..10000000).collect();
        b.iter(|| linear_search(&haystack, rng.gen_range(0..10000000)));
    }
}


fn main() {
    println!("Hello, world!");
}
