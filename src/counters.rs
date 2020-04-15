/*
 *  Copyright 2017 Gianmarco Garrisi
 *
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU Lesser General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

use crate::{Assignments, Literal, Variable};
use ordered_float::OrderedFloat;
use rand::{rngs::ThreadRng, Rng};
use std::{marker::PhantomData, ops::Index};

pub type Count = f64;

const VAR_DECAY: f64 = 0.95;
const RESCALE_THRESH: f64 = 1e100;

#[derive(Clone, Debug)]
pub struct Counters<T> {
    priorities: Vec<Count>,        // Item index -> priority of item
    heap: Vec<usize>,              // Heap of item indices
    positions: Vec<Option<usize>>, // Item index -> Index of item in heap
    bump: Count,                   // Quantity to increment count with
    key: PhantomData<T>,
}

impl<T> Counters<T> {
    pub fn new(size: usize) -> Self {
        let counters = Self {
            priorities: vec![Default::default(); size],
            heap: (0..size).collect(),
            positions: (0..size).map(Option::Some).collect(),
            bump: 1.0,
            key: PhantomData,
        };
        debug_assert!(counters.valid());
        counters
    }

    pub fn decay_activity(&mut self) {
        self.bump /= VAR_DECAY;
    }

    fn valid_positions(&self) -> bool {
        let from_positions = self
            .positions
            .iter()
            .enumerate()
            .all(|(item, pos)| pos.map(|pos| self.heap[pos] == item).unwrap_or(true));
        let from_heap = self
            .heap
            .iter()
            .enumerate()
            .all(|(pos, item)| self.positions[*item] == Some(pos));
        from_positions && from_heap
    }

    fn valid(&self) -> bool {
        let ordered = self.heap.iter().enumerate().all(|(idx, item)| {
            let mut greater_than_children = true;
            if Self::left(idx) < self.heap.len() {
                greater_than_children &=
                    self.priorities[self.heap[Self::left(idx)]] <= self.priorities[*item];
            }
            if Self::right(idx) < self.heap.len() {
                greater_than_children &=
                    self.priorities[self.heap[Self::right(idx)]] <= self.priorities[*item];
            }
            greater_than_children
        });
        self.valid_positions() && ordered
    }

    /// Compute the index of the left child of an item from its index
    fn left(i: usize) -> usize {
        (i * 2) + 1
    }

    /// Compute the index of the right child of an item from its index
    fn right(i: usize) -> usize {
        (i * 2) + 2
    }

    /// Compute the index of the parent element in the heap from its index
    fn parent(i: usize) -> usize {
        (i - 1) / 2
    }

    fn priority(&self, item: usize) -> OrderedFloat<Count> {
        OrderedFloat(self.priorities[item])
    }

    fn update_largest(&self, largest: &mut usize, other: usize) {
        if other < self.heap.len() {
            *largest =
                std::cmp::max_by_key(*largest, other, |x| self.priority(self.heap[*x].into()));
        }
    }

    /// Restore the functional property of the heap
    fn sift_down(&mut self, mut idx: usize) {
        let mut largest = idx;
        self.update_largest(&mut largest, Self::left(idx));
        self.update_largest(&mut largest, Self::right(idx));

        while largest != idx {
            // One of idx's children is larger than it
            self.swap(idx, largest);
            idx = largest;
            self.update_largest(&mut largest, Self::left(idx));
            self.update_largest(&mut largest, Self::right(idx));
        }
        debug_assert!(self.valid_positions());
    }

    fn bubble_up(&mut self, mut idx: usize) -> usize {
        let priority = self.priorities[self.heap[idx]];
        while idx > 0 && self.priorities[self.heap[Self::parent(idx)]] < priority {
            self.swap(idx, Self::parent(idx));
            idx = Self::parent(idx);
        }
        debug_assert!(self.valid_positions());
        idx
    }

    /// Swap two elements keeping a consistent state.
    fn swap(&mut self, a: usize, b: usize) {
        let (i, j) = (self.heap[a], self.heap[b]);
        self.heap.swap(a, b);
        self.positions.swap(i, j);
        debug_assert!(self.valid_positions());
    }
}

impl Counters<Variable> {
    #[must_use]
    pub fn next_decision(&mut self, assignments: &Assignments) -> Option<Literal> {
        match self.pop().map(|(item, _)| item) {
            Some(x) if assignments[x].is_some() => self.next_decision(assignments),
            Some(x) => Some(Literal::new(x, assignments.last_sign(x))),
            None => None,
        }
    }

    #[must_use]
    pub fn random_decision(
        &mut self,
        rng: &mut ThreadRng,
        assignments: &Assignments,
    ) -> Option<Literal> {
        let size = self.heap.len();
        if size == 0 {
            return None;
        }

        let var = self.heap[rng.gen_range(0, size)];
        if assignments[var].is_some() {
            self.remove_from_heap(var);
            self.random_decision(rng, assignments)
        } else {
            Some(Literal::new(var, rng.gen_bool(0.5)))
        }
    }
}

impl<T> Counters<T>
where
    T: From<usize> + Into<usize> + Copy,
{
    /// Removes the item with the greatest priority from
    /// the priority counters and returns the pair (item index, priority),
    /// or None if the counters is empty.
    pub fn pop(&mut self) -> Option<(T, Count)> {
        self.remove_idx(0)
    }

    pub fn remove_from_heap(&mut self, item: T) -> Option<(T, Count)> {
        self.positions[item.into()].and_then(|idx| self.remove_idx(idx))
    }

    fn remove_idx(&mut self, idx: usize) -> Option<(T, Count)> {
        self.swap_remove(idx).map(|removed| {
            if self.heap.len() > idx {
                self.bubble_up(idx);
                self.sift_down(idx);
            }
            debug_assert!(self.valid());
            removed
        })
    }

    pub fn add_to_heap(&mut self, item: T) {
        let item: usize = item.into();
        if self.positions[item].is_some() {
            return;
        }

        // Add the item to the end of the heap
        let pos = self.heap.len();
        self.heap.push(item);
        self.positions[item] = Some(pos);

        let pos = self.bubble_up(pos);
        self.sift_down(pos);
    }

    pub fn bump(&mut self, item: T) {
        self.increase_priority(item, self.bump);
    }

    fn increase_priority(&mut self, item: T, quantity: Count) {
        let item: usize = item.into();

        // Increase priority of item
        self.priorities[item] += quantity;

        // Rescale if necessary
        if self.priorities[item] > RESCALE_THRESH {
            self.priorities.iter_mut().for_each(|priority| {
                *priority /= RESCALE_THRESH;
            });
            self.bump /= RESCALE_THRESH;
        }

        // If element is currently in the heap, move it to its new position
        if let Some(pos) = self.positions[item] {
            let pos = self.bubble_up(pos);
            self.sift_down(pos);
        }

        debug_assert!(self.valid());
    }

    /// Remove and return the item at idx and swap it with the last
    fn swap_remove(&mut self, idx: usize) -> Option<(T, Count)> {
        if idx < self.heap.len() {
            let removed = self.heap.swap_remove(idx);

            self.positions[removed] = None;
            if let Some(replacement) = self.heap.get(idx) {
                self.positions[*replacement] = Some(idx);
            }
            debug_assert!(self.valid_positions());
            Some((removed.into(), self.priorities[removed].into()))
        } else {
            None
        }
    }
}

impl<T: Into<usize>> Index<T> for Counters<T> {
    type Output = Count;

    #[inline]
    fn index(&self, item: T) -> &Self::Output {
        &self.priorities[item.into()]
    }
}

#[cfg(test)]
mod tests {
    use super::{Count, Counters};
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;
    use rand::Rng;

    impl Arbitrary for Counters<usize> {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut rng = rand::thread_rng();
            let size = usize::arbitrary(g);

            let mut counters = Counters::new(size);
            let mut priorities = vec![0.0; size];

            for _ in 0..size {
                // Pick an arbitrary item to bump
                let item = rng.gen_range(0, size);
                // Pick an arbitrary increase
                let increase = rng.gen::<u16>() as Count;
                priorities[item] += increase;
                counters.increase_priority(item, increase);
            }

            counters
        }
    }

    impl<T> Counters<T>
    where
        T: From<usize> + Into<usize> + Copy,
    {
        fn into_ordered_vec(mut self) -> Vec<(T, Count)> {
            (0..self.heap.len()).map(|_| self.pop().unwrap()).collect()
        }
    }

    #[quickcheck]
    fn all_priorities_equal(size: usize) {
        let counters = Counters::new(size);
        let mut out: Vec<(usize, Count)> = counters.into_ordered_vec();
        out.sort_by_key(|pair| pair.0);
        assert_eq!(
            out,
            (0..size)
                .map(|idx| (idx, 0.0))
                .collect::<Vec<(usize, Count)>>()
        );
    }

    #[test]
    fn incrementing() {
        let mut counters = Counters::new(3);
        counters.increase_priority(0usize, 2.0);
        counters.increase_priority(1usize, 3.0);
        counters.increase_priority(1usize, 1.0);
        counters.increase_priority(2usize, 1.0);
        let out = counters.into_ordered_vec();
        assert_eq!(out, vec![(1, 4.0), (0, 2.0), (2, 1.0)]);
    }

    #[quickcheck]
    fn pop_order(counters: Counters<usize>) {
        let size = counters.heap.len();
        let out: Vec<(usize, Count)> = counters.into_ordered_vec();
        assert!(out.is_sorted_by_key(|pair| std::cmp::Reverse(pair.1)));

        let mut keys = out.into_iter().map(|(key, _)| key).collect::<Vec<_>>();
        keys.sort();
        assert_eq!(keys, (0..size).collect::<Vec<_>>());
    }

    #[quickcheck]
    fn pop_push_pop(mut counters: Counters<usize>) {
        if let Some((item, priority)) = counters.pop() {
            counters.increase_priority(item, 5.0);
            counters.add_to_heap(item);
            assert_eq!(counters.pop(), Some((item, priority + 5.0)));
        }
    }

    #[quickcheck]
    fn pop_pop_push_push_pop_pop(mut counters: Counters<usize>) {
        if let Some((item, priority)) = counters.pop() {
            counters.increase_priority(item, 5.0);
            if let Some((item2, priority2)) = counters.pop() {
                counters.increase_priority(item2, 5.0);
                counters.add_to_heap(item2);
                counters.add_to_heap(item);
                assert_eq!(counters.pop(), Some((item, priority + 5.0)));
                assert_eq!(counters.pop(), Some((item2, priority2 + 5.0)));
            }
        }
    }

    #[quickcheck]
    fn pop_pop_push_push_pop_pop2(mut counters: Counters<usize>) {
        if let Some((item, priority)) = counters.pop() {
            if let Some((item2, priority2)) = counters.pop() {
                counters.increase_priority(item2, priority + 1.0);
                counters.add_to_heap(item2);
                counters.add_to_heap(item);
                assert_eq!(counters.pop(), Some((item2, priority2 + priority + 1.0)));
                assert_eq!(counters.pop(), Some((item, priority)));
            }
        }
    }
}
