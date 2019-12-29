pub struct Permesan<T> {
    num_perm: usize,
    total_perm: usize,
    c: Vec<usize>,
    arr: Vec<T>,
    i: usize,
}

impl<T: Clone> Permesan<T> {
    /// Instantiates a new Permesan iterator given a collection to permute.
    ///
    /// # Arguments
    ///
    /// * `C` - A collection of `T`'s implementing the `IntoIterator` trait.
    ///
    /// # Results
    /// Returns a Permesan iterator that generates permutations of the original
    /// collection in a Vec<T>.
    pub fn new<C: IntoIterator>(col: C) -> Self
    where
        std::vec::Vec<T>: std::iter::FromIterator<<C as std::iter::IntoIterator>::Item>,
    {
        let arr: Vec<_> = col.into_iter().collect();
        Permesan {
            num_perm: 0,
            total_perm: (1..=arr.len()).product(),
            c: vec![0; arr.len()],
            arr: arr,
            i: 0,
        }
    }

    /// Generates the next permutation using Heap's algorithm.
    fn generate(&mut self) {
        if self.num_perm == 0 {
            self.num_perm += 1;
            return;
        }

        while self.i < self.arr.len() {
            if self.c[self.i] < self.i {
                if self.i % 2 == 0 {
                    self.arr.swap(0, self.i);
                } else {
                    self.arr.swap(self.c[self.i], self.i);
                }

                self.c[self.i] += 1;
                self.i = 0;

                self.num_perm += 1;
                return;
            } else {
                self.c[self.i] = 0;
                self.i += 1;
            }
        }
    }
}

impl<T: Clone> Iterator for Permesan<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_perm == self.total_perm {
            None
        } else {
            self.generate();
            Some(self.arr.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::Permesan;
    use std::collections::HashMap;

    #[test]
    fn empty_vector() {
        let empty: Vec<i32> = Vec::new();
        let mut v = Permesan::new(empty);

        assert_eq!(v.next(), Some(vec![]));
        assert_eq!(v.next(), None);
    }

    #[test]
    fn basic() {
        let mut v = Permesan::new(vec![1, 2, 3]);

        assert_eq!(v.next(), Some(vec![1, 2, 3]));
        assert_eq!(v.next(), Some(vec![2, 1, 3]));
        assert_eq!(v.next(), Some(vec![3, 1, 2]));
    }

    #[test]
    fn advanced() {
        let mut v = Permesan::new(vec![1, 2, 3, 4]);

        assert_eq!(v.next(), Some(vec![1, 2, 3, 4]));
        assert_eq!(v.next(), Some(vec![2, 1, 3, 4]));
        assert_eq!(v.next(), Some(vec![3, 1, 2, 4]));
        assert_eq!(v.next(), Some(vec![1, 3, 2, 4]));
        assert_eq!(v.next(), Some(vec![2, 3, 1, 4]));
        assert_eq!(v.next(), Some(vec![3, 2, 1, 4]));
        assert_eq!(v.next(), Some(vec![4, 2, 1, 3]));
        assert_eq!(v.next(), Some(vec![2, 4, 1, 3]));
        assert_eq!(v.next(), Some(vec![1, 4, 2, 3]));
        assert_eq!(v.next(), Some(vec![4, 1, 2, 3]));
        assert_eq!(v.next(), Some(vec![2, 1, 4, 3]));
        assert_eq!(v.next(), Some(vec![1, 2, 4, 3]));
        assert_eq!(v.next(), Some(vec![1, 3, 4, 2]));
        assert_eq!(v.next(), Some(vec![3, 1, 4, 2]));
        assert_eq!(v.next(), Some(vec![4, 1, 3, 2]));
        assert_eq!(v.next(), Some(vec![1, 4, 3, 2]));
        assert_eq!(v.next(), Some(vec![3, 4, 1, 2]));
        assert_eq!(v.next(), Some(vec![4, 3, 1, 2]));
        assert_eq!(v.next(), Some(vec![4, 3, 2, 1]));
        assert_eq!(v.next(), Some(vec![3, 4, 2, 1]));
        assert_eq!(v.next(), Some(vec![2, 4, 3, 1]));
        assert_eq!(v.next(), Some(vec![4, 2, 3, 1]));
        assert_eq!(v.next(), Some(vec![3, 2, 4, 1]));
        assert_eq!(v.next(), Some(vec![2, 3, 4, 1]));
        assert_eq!(v.next(), None);
        assert_eq!(v.next(), None);
    }

    #[test]
    fn collect_vector() {
        let v = vec![1, 2, 3];
        let perms: Vec<_> = Permesan::new(v).collect();

        let sol = vec![
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 1, 2],
            vec![1, 3, 2],
            vec![2, 3, 1],
            vec![3, 2, 1],
        ];

        assert_eq!(perms, sol);
    }
}
