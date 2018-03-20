// Assumes both vectors are sorted.
pub fn intersection_size<T>(a: &[T], b: &[T]) -> usize
where
    T: PartialOrd,
{
    // Count the length required in the intersection, to avoid
    // paying for reallocations while pushing onto the end.
    let mut count = 0;
    let mut ap = 0;
    let mut bp = 0;
    while ap < a.len() && bp < b.len() {
        if a[ap] < b[bp] {
            ap += 1;
        } else if b[bp] < a[ap] {
            bp += 1;
        } else {
            count += 1;
            ap += 1;
            bp += 1;
        }
    }
    count
}

// Assumes both vectors are sorted.
pub fn union<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    // Count the length required in the union, to avoid
    // paying for reallocations while pushing onto the end.
    let mut count = 0;
    let mut ap = 0;
    let mut bp = 0;
    while ap < a.len() && bp < b.len() {
        if a[ap] < b[bp] {
            count += 1;
            ap += 1;
        } else if b[bp] < a[ap] {
            count += 1;
            bp += 1;
        } else {
            count += 1;
            ap += 1;
            bp += 1;
        }
    }
    count += a.len() - ap;
    count += b.len() - bp;

    let mut c: Vec<T> = Vec::with_capacity(count);
    let mut ap = 0;
    let mut bp = 0;
    while ap < a.len() && bp < b.len() {
        if a[ap] < b[bp] {
            c.push(a[ap]);
            ap += 1;
        } else if b[bp] < a[ap] {
            c.push(b[bp]);
            bp += 1;
        } else {
            c.push(a[ap]);
            ap += 1;
            bp += 1;
        }
    }
    while ap < a.len() {
        c.push(a[ap]);
        ap += 1;
    }
    while bp < b.len() {
        c.push(b[bp]);
        bp += 1;
    }
    c
}

// Assumes both vectors are sorted.
pub fn intersection<T>(a: &[T], b: &[T]) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    // Count the length required in the intersection, to avoid
    // paying for reallocations while pushing onto the end.
    let count = intersection_size(a, b);

    let mut c: Vec<T> = Vec::with_capacity(count);
    let mut ap = 0;
    let mut bp = 0;
    while ap < a.len() && bp < b.len() {
        if a[ap] < b[bp] {
            ap += 1;
        } else if b[bp] < a[ap] {
            bp += 1;
        } else {
            c.push(a[ap]);
            ap += 1;
            bp += 1;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use item::Item;
    fn to_item_vec(nums: &[u32]) -> Vec<Item> {
        nums.iter().map(|i| Item::with_id(*i)).collect()
    }

    #[test]
    fn test_union() {
        use super::union;

        let test_cases: Vec<(Vec<Item>, Vec<Item>, Vec<Item>)> = [
            (vec![1, 2, 3], vec![4, 5, 6], vec![1, 2, 3, 4, 5, 6]),
            (vec![1, 2, 3], vec![3, 4, 5, 6], vec![1, 2, 3, 4, 5, 6]),
            (vec![], vec![1], vec![1]),
            (vec![1], vec![], vec![1]),
        ].iter()
            .map(|&(ref a, ref b, ref u)| (to_item_vec(a), to_item_vec(b), to_item_vec(u)))
            .collect();

        for &(ref a, ref b, ref c) in &test_cases {
            assert_eq!(&union(&a, &b), c);
        }
    }

    #[test]
    fn test_intersection() {
        use super::intersection;

        let test_cases: Vec<(Vec<Item>, Vec<Item>, Vec<Item>)> = [
            (vec![1], vec![1], vec![1]),
            (vec![1, 2, 3, 4, 5], vec![4, 5, 6], vec![4, 5]),
            (vec![1, 2, 3], vec![3, 4, 5, 6], vec![3]),
            (vec![], vec![1], vec![]),
            (vec![1], vec![], vec![]),
        ].iter()
            .map(|&(ref a, ref b, ref u)| (to_item_vec(a), to_item_vec(b), to_item_vec(u)))
            .collect();

        for &(ref a, ref b, ref c) in &test_cases {
            assert_eq!(&intersection(&a, &b), c);
        }
    }

    #[test]
    fn test_intersection_size() {
        use super::intersection_size;
        let cases: Vec<(Vec<Item>, Vec<Item>, usize)> = [
            (vec![1], vec![2], 0),
            (vec![1, 2, 3], vec![1, 2, 4], 2),
            (vec![1, 2, 3], vec![2, 3, 4], 2),
            (vec![1, 2, 3], vec![1, 3, 4], 2),
            (vec![1, 2, 3], vec![3, 4, 5], 1),
            (vec![3, 4, 5], vec![1, 2, 3], 1),
        ].iter()
            .map(|&(ref a, ref b, sz)| (to_item_vec(a), to_item_vec(b), sz))
            .collect();

        for &(ref a, ref b, sz) in cases.iter() {
            assert_eq!(intersection_size(a, b), sz);
        }
    }
}
