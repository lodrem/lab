use std::ptr::NonNull;

struct Node<K, V, const N: usize> {
    key: K,
    value: V,
    children: [Option<NonNull<Node<K, V, N>>>; N],
}

pub struct BTreeMap<K, V, const N: usize> {
    root: Option<NonNull<Node<K, V, N>>>,
}

impl<K, V, const N: usize> BTreeMap<K, V, N> {
    pub fn new() -> Self {
        todo!()
    }
}
