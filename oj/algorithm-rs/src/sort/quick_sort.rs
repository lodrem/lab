use std::cmp::Ordering;

fn partition<T>(coll: &mut [T], ordering: Ordering) -> usize
where
    T: Ord,
{
    let len = coll.len();
    let pivot_idx = 0;

    coll.swap(pivot_idx, len - 1);
    let mut store_idx = 0;

    for i in 0..len - 1 {
        if coll[i].cmp(&coll[len - 1]) == ordering {
            coll.swap(i, store_idx);
            store_idx += 1;
        }
    }

    coll.swap(store_idx, len - 1);

    store_idx
}

pub fn sort<T>(coll: &mut [T], ordering: Ordering)
where
    T: Ord,
{
    let len = coll.len();
    if len < 2 {
        return;
    }

    let pivot_idx = partition(coll, ordering);

    sort(&mut coll[..pivot_idx], ordering);
    sort(&mut coll[pivot_idx + 1..len], ordering);
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::sort;

    #[test]
    fn it_works() {
        {
            let mut actual = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        }

        {
            let mut actual = vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        }

        {
            let mut actual = vec![10, 9, 8, 7, 6, 1, 2, 3, 4, 5];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        }

        {
            let mut actual = vec![1, 2, 3, 4, 5, 10, 9, 8, 7, 6];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 5, 4, 3, 2, 1]);
        }

        {
            let mut actual = vec![1, 1, 1, 1, 1, 10, 9, 8, 7, 6];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 1, 1, 1, 1, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 1, 1, 1, 1, 1]);
        }

        {
            let mut actual = vec![10, 9, 8, 7, 6, 1, 1, 1, 1, 1];
            sort(&mut actual, Ordering::Less);
            assert_eq!(actual, vec![1, 1, 1, 1, 1, 6, 7, 8, 9, 10]);
            sort(&mut actual, Ordering::Greater);
            assert_eq!(actual, vec![10, 9, 8, 7, 6, 1, 1, 1, 1, 1]);
        }
    }
}
