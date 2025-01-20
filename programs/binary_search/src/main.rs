//@source http://codekata.com/kata/kata02-karate-chop/
//@reference https://www.cs.cmu.edu/~15122-archive/n17/lec/06-binsearch.pdf
//@reference https://matklad.github.io/2023/10/06/what-is-an-invariant.html


fn binary_search(target: i32, array: &[i32]) -> i32 {
    //@requires array to be sorted
    //@requires e in array to be between -2^31 and (2^31) - 1
    //@ensures (-1 == \result && !is_in(target, array, 0, n)) || ((0 <= \result && \result < array.len()) && array[\result] == target);
    let mut lo: usize = 0;
    let mut hi: usize = array.len();

    while lo < hi
    //@loop_invariant 0 <= lo && lo <= hi && hi <= array.len(); The first loop invariant, relating lo and hi to each other and the overall bound of the array
    //@loop_invariant (lo == 0 || array[lo-1] < x);
    //@loop_invariant (hi == array.len() - 1 || array[hi] > x);
    {
        let mid: usize = lo + (hi-lo)/2;
        //@assert lo <= mid && mid < hi;
        if array[mid] == target {
            return mid as i32;
        } else if array[mid] < target {
            lo = mid+1;
        } else /*@assert(array[mid] > x);@*/ {
            hi = mid
        }
    }
    -1 as i32
}

fn test_binary_search() {
    assert_eq!(binary_search(3, &[]), -1);
    assert_eq!(binary_search(3, &[1]), -1);
    assert_eq!(binary_search(1, &[1]), 0);

    assert_eq!(binary_search(1, &[1, 3, 5]), 0);
    assert_eq!(binary_search(3, &[1, 3, 5]), 1);
    assert_eq!(binary_search(5, &[1, 3, 5]), 2);
    assert_eq!(binary_search(0, &[1, 3, 5]), -1);
    assert_eq!(binary_search(2, &[1, 3, 5]), -1);
    assert_eq!(binary_search(4, &[1, 3, 5]), -1);
    assert_eq!(binary_search(6, &[1, 3, 5]), -1);

    assert_eq!(binary_search(1, &[1, 3, 5, 7]), 0);
    assert_eq!(binary_search(3, &[1, 3, 5, 7]), 1);
    assert_eq!(binary_search(5, &[1, 3, 5, 7]), 2);
    assert_eq!(binary_search(7, &[1, 3, 5, 7]), 3);
    assert_eq!(binary_search(0, &[1, 3, 5, 7]), -1);
    assert_eq!(binary_search(2, &[1, 3, 5, 7]), -1);
    assert_eq!(binary_search(4, &[1, 3, 5, 7]), -1);
    assert_eq!(binary_search(6, &[1, 3, 5, 7]), -1);
    assert_eq!(binary_search(8, &[1, 3, 5, 7]), -1);
}

fn main() {
    test_binary_search();
}
