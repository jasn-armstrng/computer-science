//@source: https://matklad.github.io/2023/10/06/what-is-an-invariant.html
//@description: Write a binary search variation which computes insertion point — the smallest index such that, if the new
//             element is inserted at this index, the array remains sorted.

fn insertion_point(arr: &[i32], new_element: i32) -> usize {
    //@requires array to be sorted
    //@requires e in array to be between -2^31 and (2^31) - 1
    let mut lo: usize = 0;
    let mut hi: usize = arr.len();

    while lo < hi {
        let mid: usize = lo + (hi - lo) / 2;

        // Assert for debugging purposes:
        assert!(lo <= mid && mid <= hi);

        if arr[mid] == new_element {
            return mid;
        } else if arr[mid] < new_element {
            lo = mid + 1;
        } else
        // assert(array[mid] >= x)
        {
            hi = mid
        }
    }

    lo
}


fn _test_insertion_point() {
    assert_eq!(insertion_point(&[], 5), 0); // Insert into empty array
    assert_eq!(insertion_point(&[1, 2, 3], 0), 0); // Insert at start
    assert_eq!(insertion_point(&[1, 2, 3], 4), 3); // Insert at end
}

fn main() {
    let n = 5;
    let m = 5;
    let mut g = exhaustigen::Gen::new();

    // test_insertion_point();

    while !g.done() {
        // Generate an arbitrary sorted array of length at most m.
        let mut xs = (0..g.gen(n)).map(|_| g.gen(m) as i32).collect::<Vec<_>>();
        xs.sort();

        let x = g.gen(m) as i32;
        let i = insertion_point(&xs, x);

        // Check boundary conditions:
        if i > 0 {
            assert!(xs[i - 1] < x, "Value at xs[i-1] is not less than x");
        }
        if i < xs.len() {
            assert!(x <= xs[i], "Value at xs[i] is not greater than or equal to x");
        }
    }
}
