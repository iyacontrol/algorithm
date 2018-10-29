// 快速排序
pub fn quick_sort(vec: &mut Vec<u32>, low: usize, high: usize) {
    if low < high {
        let mut i = low;
        let mut j = high;
        let base = vec[low];

        loop {
            while vec[j] >= base && i < j {
                j -= 1;
            }
            while vec[i] <= base && i < j {
                i += 1;
            }
            if i >= j {
                break;
            }
            vec.swap(i, j);
        }

        vec.swap(low, i);
        if i >= 1 {
            quick_sort(vec, low, i - 1);
        }
        quick_sort(vec, i + 1, high);
    }
}

// 冒泡排序
pub fn bupple(vec: &mut Vec<u32>) {
    let len = vec.len();
    let mut j = 0;
    let mut i = 0;

    while j < len - 1 {
        while i < len - 1 - j {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }
            i += 1;
        }
        j += 1;
        i = 0;
    }
}
