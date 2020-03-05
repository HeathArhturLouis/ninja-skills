#![allow(dead_code)]
use rand::prelude::*;
use std::{
    cmp::{Ord, Ordering},
    collections::VecDeque,
    fmt,
};

#[derive(Debug)]
struct Node<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    key: K,
    value: V,
    descendants: Option<usize>, // Number of total children. If Some, has to be correct. Should be Some, after every pub fn.
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            descendants: Some(0),
            left: None,
            right: None,
        }
    }

    fn random(&self, rng: &mut ThreadRng) -> (&K, &V) {
        debug_assert_eq!(
            self.descendants,
            Some(self.left_children() + self.right_children())
        );

        let i = rng.gen_range(0, self.descendants.unwrap() + 1);

        // We only *need* to sample once, but this is easier to code.
        // Time *complexity* doesn't change.
        if i == 0 {
            (&self.key, &self.value)
        } else if i <= self.left_children() {
            self.left.as_ref().unwrap().random(rng)
        } else if i <= self.left_children() + self.right_children() {
            self.right.as_ref().unwrap().random(rng)
        } else {
            unreachable!()
        }
    }

    fn calc_descendants(&mut self) -> usize {
        if self.descendants.is_none() {
            self.left.as_mut().map(|child| child.calc_descendants());
            self.right.as_mut().map(|child| child.calc_descendants());
            self.descendants = Some(self.left_children() + self.right_children());
        }
        self.descendants.unwrap()
    }

    fn left_children(&self) -> usize {
        match &self.left {
            Some(boxed_node) => 1 + boxed_node.descendants.unwrap(),
            None => 0,
        }
    }

    fn right_children(&self) -> usize {
        match &self.right {
            Some(boxed_node) => 1 + boxed_node.descendants.unwrap(),
            None => 0,
        }
    }

    fn iter(&self) -> Iter<'_, K, V> {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        Iter { queue }
    }

    fn into_iter(self) -> IntoIter<K, V> {
        let mut queue = VecDeque::<Node<K, V>>::new();
        queue.push_back(self);
        IntoIter { queue }
    }
}

#[derive(Debug)]
struct RandomTree<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    root: Option<Box<Node<K, V>>>,
}

impl<K, V> RandomTree<K, V>
where
    K: Ord + fmt::Debug,
    V: fmt::Debug,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    // Returns None iff the tree is empty.
    // O(log(n))
    pub fn random(&self, rng: &mut ThreadRng) -> Option<(&K, &V)> {
        self.root.as_ref().map(|root_node| root_node.random(rng))
    }

    // Returns the old value associated with K, or None if the key is new.
    // O(log(n))
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.find_mut(&key) {
            Some(node) => Some(std::mem::replace(&mut node.value, value)),
            None => {
                Self::insert_aux(&mut self.root, key, value);
                None
            }
        }
    }

    // O(log(n))
    pub fn get(&self, key: &K) -> Option<&V> {
        self.find(key).map(|node| &node.value)
    }

    // O(log(n))
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.find_mut(key).map(|node| &mut node.value)
    }

    // O(log(n))
    pub fn has(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    // O(n)
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let branch = self.take_branch(key)?;
        let mut kv_pairs = branch.into_iter();
        let removed = kv_pairs.next().unwrap();
        for (k, v) in kv_pairs {
            self.insert(k, v);
        }
        Some(removed.1)
    }

    fn insert_aux(parents_ref: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        match parents_ref {
            None => {
                let node = Node::new(key, value);
                *parents_ref = Some(Box::new(node));
            }
            Some(new_parent) => {
                *new_parent.descendants.as_mut().unwrap() += 1;
                match (&key).cmp(&new_parent.key) {
                    Ordering::Less => Self::insert_aux(&mut new_parent.left, key, value),
                    Ordering::Greater => Self::insert_aux(&mut new_parent.right, key, value),
                    Ordering::Equal => unreachable!(),
                }
            }
        }
    }

    // Returns a reference to the entire node associated with key, if it
    // exists.
    fn find(&self, key: &K) -> Option<&Node<K, V>> {
        let mut current = &self.root;
        while let Some(current_node) = current.as_ref() {
            match key.cmp(&current_node.key) {
                Ordering::Equal => return Some(&current_node),
                Ordering::Less => current = &current_node.left,
                Ordering::Greater => current = &current_node.right,
            }
        }
        None
    }

    // Same as find, but mutable.
    fn find_mut(&mut self, key: &K) -> Option<&mut Node<K, V>> {
        let mut current = &mut self.root;
        while let Some(current_node) = current.as_mut() {
            match key.cmp(&current_node.key) {
                Ordering::Equal => return Some(current_node),
                Ordering::Less => current = &mut current_node.left,
                Ordering::Greater => current = &mut current_node.right,
            }
        }
        None
    }

    // Cuts a branch off the tree and returns it.
    fn take_branch(&mut self, key: &K) -> Option<Box<Node<K, V>>> {
        // reference to the pointer of the node we're inspecting.
        let mut current_ptr: &mut Option<Box<Node<K, V>>> = &mut self.root;
        let ret;
        loop {
            match current_ptr.take() {
                None => {
                    ret = None;
                    break;
                }
                Some(mut boxed_node) => match key.cmp(&boxed_node.key) {
                    Ordering::Equal => {
                        ret = Some(boxed_node);
                        break;
                    }
                    Ordering::Less => {
                        boxed_node.descendants = None;
                        *current_ptr = Some(boxed_node);
                        current_ptr = &mut current_ptr.as_mut().unwrap().left;
                    }
                    Ordering::Greater => {
                        boxed_node.descendants = None;
                        *current_ptr = Some(boxed_node);
                        current_ptr = &mut current_ptr.as_mut().unwrap().right;
                    }
                },
            }
        }
        self.root.as_mut().map(|node| node.calc_descendants());
        ret
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        let mut queue = VecDeque::new();
        if let Some(root) = self.root.as_ref() {
            queue.push_back(&**root);
        }
        Iter { queue }
    }

    pub fn into_iter(mut self) -> IntoIter<K, V> {
        let mut queue = VecDeque::new();
        if let Some(boxed_root) = self.root.take() {
            queue.push_back(*boxed_root);
        }
        IntoIter { queue }
    }
}

// These are BFS iterators.
// The benefit of that is that, when reinserting nodes after taking
// a branch, we reinsert them in almost the same order they had
// originally.
// A shuffling iterator would possibly be better.
// A DFS iterator would definitely be worse.
struct Iter<'a, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    queue: VecDeque<&'a Node<K, V>>,
}

struct IntoIter<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    queue: VecDeque<Node<K, V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    type Item = (&'a K, &'a V);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|current_node| {
            if let Some(left) = current_node.left.as_ref() {
                self.queue.push_back(left);
            }
            if let Some(right) = current_node.right.as_ref() {
                self.queue.push_back(right);
            }
            (&current_node.key, &current_node.value)
        })
    }
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: fmt::Debug,
    V: fmt::Debug,
{
    type Item = (K, V);
    fn next(&mut self) -> Option<Self::Item> {
        self.queue.pop_front().map(|mut current_node| {
            if let Some(boxed_left) = current_node.left.take() {
                self.queue.push_back(*boxed_left);
            }
            if let Some(boxed_right) = current_node.right.take() {
                self.queue.push_back(*boxed_right);
            }
            (current_node.key, current_node.value)
        })
    }
}

// O(n) time to pick one of n items.
fn cheeze_it<T>(source: impl Iterator<Item = T>, rng: &mut ThreadRng) -> Option<T> {
    let mut ret: Option<T> = None;
    let mut count: usize = 0;
    for item in source {
        count += 1;
        if rng.gen_range(0, count) == 0 {
            ret = Some(item);
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;

    #[test]
    fn insert_unique() {
        let mut tree = RandomTree::<i32, String>::new();
        let table = [(10, "Aardvark"), (5, "Zebra"), (6, "What?")];
        for (i, s) in table.iter() {
            assert_eq!(tree.insert(*i, s.to_string()), None);
        }
        for (i, s) in table.iter() {
            assert_eq!(tree.get(i).map(|out| out.as_str()), Some(*s));
        }
    }

    #[test]
    fn insert_replacement() {
        let mut tree = RandomTree::<i32, &str>::new();
        assert_eq!(tree.insert(100, "Original"), None);
        assert_eq!(tree.insert(100, "Replacement"), Some("Original"));
        assert_eq!(tree.get(&100), Some(&"Replacement"));
    }

    #[test]
    fn birthday_paradox() {
        let mut bday_set = RandomTree::new();
        let mut rng = thread_rng();

        assert!(bday_set.random(&mut rng).is_none());

        let mut days = (1..366).collect::<Vec<_>>();
        days.shuffle(&mut rng);
        for day in days {
            bday_set.insert(day, ());
        }
        let mut selection = (0..27)
            .map(|_| bday_set.random(&mut rng).unwrap().0)
            .collect::<Vec<_>>();
        selection.sort_unstable();
        println!("{:?}", selection);
    }

    #[test]
    fn birthday_with_cheeze() {
        let mut bday_set = RandomTree::new();
        let mut rng = thread_rng();

        assert!(cheeze_it(bday_set.iter(), &mut rng).is_none());

        let mut days = (1..366).collect::<Vec<_>>();
        days.shuffle(&mut rng);
        for day in days {
            bday_set.insert(day, ());
        }
        let mut selection = (0..27)
            .map(|_| cheeze_it(bday_set.iter(), &mut rng).unwrap().0)
            .copied()
            .collect::<Vec<_>>();
        selection.sort_unstable();
        println!("{:?}", selection);
    }

    #[test]
    fn iter() {
        let mut rng = thread_rng();

        let mut original = [0, 1, 58, 120, 130];
        original.shuffle(&mut rng);

        let mut set = RandomTree::new();
        for i in original.iter() {
            set.insert(*i, ());
        }

        let mut recovered = set.iter().map(|(i, _)| *i).collect::<Vec<_>>();

        original.sort_unstable();
        recovered.sort_unstable();

        assert_eq!(original[..], recovered[..]);
    }

    #[test]
    fn into_iter() {
        let mut rng = thread_rng();

        let mut original = [0, 1, 58, 120, 130];
        original.shuffle(&mut rng);

        let mut set = RandomTree::new();
        for i in original.iter() {
            set.insert(*i, ());
        }

        let mut recovered = set.into_iter().map(|(i, _)| i).collect::<Vec<_>>();

        original.sort_unstable();
        recovered.sort_unstable();

        assert_eq!(original[..], recovered[..]);
    }

    #[test]
    fn has() {
        let mut map = RandomTree::new();
        for i in -9_i8..9 {
            if i % 2 == 1 {
                map.insert(i, i as f64);
            }
        }
        for i in -9_i8..9 {
            assert_eq!(map.has(&i), i % 2 == 1);
        }
    }

    #[test]
    fn get_mut() {
        let mut counter = RandomTree::new();
        assert_eq!(counter.get_mut(&0), None);

        counter.insert(0, 0);
        counter.insert(1, 0);
        counter.insert(2, 0);
        for i in 0..100 {
            *counter.get_mut(&(i % 3)).unwrap() += 1;
        }
        assert_eq!(counter.get(&0), Some(&34));
        assert_eq!(counter.get(&1), Some(&33));
        assert_eq!(counter.get(&2), Some(&33));
    }

    impl<K, V> Node<K, V>
    where
        K: fmt::Debug,
        V: fmt::Debug,
    {
        fn validate_children(&self) -> bool {
            let actual_descendants = self.iter().map(|_| 1).sum::<usize>() - 1;
            Some(actual_descendants) == self.descendants
        }
    }

    #[test]
    fn descendant_invariant() {
        let mut tree = RandomTree::new();
        let mut rng = thread_rng();

        for _ in 0..100_000 {
            tree.insert(rng.gen_range(-1_000, 0), rng.gen_range(0, 1_000_000));
        }

        for (k, _) in tree.iter() {
            assert!(tree.find(&k).unwrap().validate_children());
        }
    }

    #[test]
    fn take_branch() {
        let mut tree = RandomTree::new();
        tree.insert(4, ());
        tree.insert(2, ());
        tree.insert(1, ());
        tree.insert(3, ());
        tree.insert(6, ());
        tree.insert(5, ());
        tree.insert(7, ());
        dbg!(&tree);
        let branch = tree.take_branch(&2);
        dbg!(&branch);
        dbg!(&tree);
    }

    #[test]
    fn remove() {
        let mut tree = RandomTree::new();
        tree.insert(4, ());
        tree.insert(2, ());
        tree.insert(1, ());
        tree.insert(3, ());

        tree.remove(&2);

        assert!(tree.has(&4));
        assert!(tree.has(&1));
        assert!(tree.has(&3));

        for (k, _) in tree.iter() {
            assert!(tree.find(k).unwrap().validate_children());
        }
    }
}
