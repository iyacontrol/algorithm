// 二分查找
pub fn binary_search(vec: Vec<i32>, val: i32) -> i32 {
    let mut low = 0;
    let mut high = vec.len() - 1;
    let mut mid = 0;

    while low <= high {
        mid = (high + low) / 2;
        if vec[mid] == val {
            return mid as i32;
        } else if vec[mid] > val {
            high = mid - 1;
        } else {
            low = mid + 1;
        } 
    }

    -1
}

// 顺序查找
pub fn sequence_search(vec: Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    let len = vec.len();
    while index < len {
        if vec[index] == val {
            return index as i32;
        }
        index += 1;
    }
    -1
}
