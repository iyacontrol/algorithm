pub fn binary_search(vec: Vec<i32>, val: i32) -> i32 {
    let mut low = 0;
    let mut high = vec.len() - 1;
    let mut mid = 0;

    while low <= high {
        mid = (high - low) / 2;
        if vec[mid] == val {
            let ret = mid as i32;
            return ret;
        } else if vec[mid] > val {
            high = mid - 1;
        } else if vec[mid] < val {
            low = mid + 1;
        }
    }

    -1
}
