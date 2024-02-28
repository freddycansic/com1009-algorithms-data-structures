#![feature(test)]

fn main() {
    let mut a = [9, 8, 7, 6, 5, 4, 3, 2, 1];
    
    merge_sort(&mut a);
    println!("{:?}", a);
}

fn merge_sort(a: &mut [u32]) {
    merge_sort_inner(a, 0, a.len() - 1);
}

fn merge_sort_inner(a: &mut [u32], l: usize, r: usize) {
    if l < r {
        let m = (l + r) / 2;
        merge_sort_inner(a, l, m);
        merge_sort_inner(a, m + 1, r);
        merge(a, l, m, r);
    }
}

fn merge(a: &mut [u32], l: usize, m: usize, r: usize) {
    let n1 = m - l + 1;
    let n2 = r - m;

    // this is faster than the below comment
    let mut lhs = Vec::<u32>::with_capacity(n1 + 1);
    unsafe { lhs.set_len(n1 + 1); }
    
    let mut rhs = vec![0; n2 + 1];
    unsafe { rhs.set_len(n2 + 1); }

    //let mut lhs = vec![0; n1 + 1];
    //let mut rhs = vec![0; n2 + 1];

    lhs[0..n1].clone_from_slice(&a[l..=m]);
    rhs[0..n2].clone_from_slice(&a[m + 1..=r]);

    lhs[n1] = std::u32::MAX;
    rhs[n2] = std::u32::MAX;

    let mut i = 0;
    let mut j = 0;

    for k in l..=r {
        if lhs[i] <= rhs[j] {
            a[k] = lhs[i];
            i += 1;
        } else {
            a[k] = rhs[j];
            j += 1;
        }
    }
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    const ARRAY_LENGTH: u32 = 100_000;

    #[test]
    fn merge_sort_works() {
        let mut array = (0..ARRAY_LENGTH).rev().collect::<Vec<u32>>();
        merge_sort(&mut array);
        
        assert_eq!((0..ARRAY_LENGTH).collect::<Vec<u32>>(), array);
    }

    #[bench]
    fn bench_merge_sort(bencher: &mut Bencher) {        
        bencher.iter(|| {
            let mut array = (0..ARRAY_LENGTH).rev().collect::<Vec<u32>>();
            merge_sort(&mut array);
        });
    }
    
    #[bench]
    fn bench_std_sort(bencher: &mut Bencher) {        
        bencher.iter(|| {
            let mut array = (0..ARRAY_LENGTH).rev().collect::<Vec<u32>>();
            array.sort();
        });
    }
}
