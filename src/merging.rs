// core merging algorithm
fn merge(list: &mut [u32]) {
    assert!(list.len() >= 2);
    let mut start = 0;

    // if can still expand right and current is still zero
    while start + 1 < list.len() && list[start] == 0 {
        start += 1;
    }
    list[0] = list[start];

    let mut write_ptr = 0;
    for i in (start + 1)..list.len() {
        if list[i] == 0 {
            continue;
        }

        if list[i] == list[write_ptr] {
            list[write_ptr] *= 2;

            // merge backwards if possible
            while write_ptr >= 1 && list[write_ptr] == list[write_ptr - 1] {
                list[write_ptr - 1] *= 2;
                write_ptr -= 1;
            }
        } else {
            write_ptr += 1;
            list[write_ptr] = list[i];
        }
    }
    for i in write_ptr + 1..list.len() {
        list[i] = 0;
    }
}

#[cfg(test)]
mod tests {
    use crate::merging::merge;

    #[test]
    fn test_merge() {
        let mut list = [2, 4, 0, 6, 8, 2, 2, 3, 0, 0, 4, 4, 4];
        let correct = [2, 4, 6, 8, 4, 3, 8, 4, 0, 0, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_2() {
        let mut list = [4, 4, 4];
        let correct = [8, 4, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_3() {
        let mut list = [9, 0, 9];
        let correct = [18, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_4() {
        let mut list = [2, 2, 4];
        let correct = [8, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_5() {
        let mut list = [2, 2, 2, 2, 2, 2, 2, 2];
        let correct = [16, 0, 0, 0, 0, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_6() {
        let mut list = [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
        let correct = [32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_7() {
        let mut list = [4, 4, 16, 16, 2, 2, 2, 2, 2, 2, 2, 2, 16];
        let correct = [8, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_8() {
        let mut list = [4, 4, 4, 4, 4, 4];
        let correct = [16, 8, 0, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }

    #[test]
    fn test_merge_9() {
        let mut list = [0, 0, 16, 16];
        let correct = [32, 0, 0, 0];
        merge(&mut list);
        assert_eq!(&list, &correct);
    }
}
