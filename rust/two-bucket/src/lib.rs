use std::cmp::{max, min};
use std::collections::VecDeque;

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct BucketState {
    id: Bucket,
    capacity: u8,
    current: u8,
}

impl BucketState {
    fn new(id: Bucket, capacity: u8) -> Self {
        Self {
            id,
            capacity,
            current: 0,
        }
    }

    fn is_having(&self, amount: u8) -> bool {
        self.current == amount
    }

    fn is_full(&self) -> bool {
        self.current == self.capacity
    }

    fn is_empty(&self) -> bool {
        self.current == 0
    }

    fn free_space(&self) -> u8 {
        self.capacity - self.current
    }

    fn fill(&self, amount: u8) -> Self {
        Self {
            current: min(self.current + amount, self.capacity),
            ..self.to_owned()
        }
    }

    fn remove(&self, amount: u8) -> Self {
        Self {
            current: max(self.current - amount, 0),
            ..self.to_owned()
        }
    }

    fn fill_full(&self) -> Self {
        Self {
            current: self.capacity,
            ..self.to_owned()
        }
    }

    fn empty(&self) -> Self {
        Self {
            current: 0,
            ..self.to_owned()
        }
    }

    fn fill_from(&self, other: &BucketState) -> (Self, Self) {
        let transfer_amount = min(self.free_space(), other.current);
        (self.fill(transfer_amount), other.remove(transfer_amount))
    }
}

#[derive(Debug, Clone, Eq, Hash)]
struct SearchState {
    bucket1: BucketState,
    bucket2: BucketState,
    moves: u8,
    expectation: u8,
    must_keep: Bucket,
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.bucket1 == other.bucket1
            && self.bucket2 == other.bucket2
            && self.must_keep == other.must_keep
            && self.expectation == other.expectation
    }
}

impl SearchState {
    fn new(bucket1: BucketState, bucket2: BucketState, expectation: u8, must_keep: Bucket) -> Self {
        Self {
            bucket1,
            bucket2,
            must_keep,
            expectation,
            moves: 1,
        }
    }

    fn next_state(&self, bucket1: BucketState, bucket2: BucketState) -> Self {
        Self {
            bucket1,
            bucket2,
            moves: self.moves + 1,
            expectation: self.expectation,
            must_keep: self.must_keep.clone(),
        }
    }

    fn is_meet_expectation(&self) -> Option<BucketStats> {
        if self.bucket1.is_having(self.expectation) {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::One,
                other_bucket: self.bucket2.current,
            });
        }
        if self.bucket2.is_having(self.expectation) {
            return Some(BucketStats {
                moves: self.moves,
                goal_bucket: Bucket::Two,
                other_bucket: self.bucket1.current,
            });
        }
        None
    }

    fn generate_new_states(&self) -> (Option<BucketStats>, Vec<SearchState>) {
        let mut ret = vec![];
        ret.push(self.next_state(self.bucket1.fill_full(), self.bucket2.clone()));
        ret.push(self.next_state(self.bucket1.clone(), self.bucket2.fill_full()));
        ret.push(self.next_state(self.bucket1.clone(), self.bucket2.empty()));
        ret.push(self.next_state(self.bucket1.empty(), self.bucket2.clone()));
        let (new_bucket1, new_bucket2) = self.bucket1.fill_from(&self.bucket2);
        ret.push(self.next_state(new_bucket1, new_bucket2));
        let (new_bucket2, new_bucket1) = self.bucket2.fill_from(&self.bucket1);
        ret.push(self.next_state(new_bucket1, new_bucket2));

        ret.retain(|it| match self.must_keep {
            Bucket::One => !(it.bucket1.is_empty() && it.bucket2.is_full()),
            Bucket::Two => !(it.bucket1.is_full() && it.bucket2.is_empty()),
        });
        let found = ret
            .iter()
            .map(|it| it.is_meet_expectation())
            .filter(Option::is_some)
            .nth(0)
            .flatten();
        (found, ret)
    }
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let initial_state = match start_bucket {
        Bucket::One => SearchState::new(
            BucketState::new(Bucket::One, capacity_1).fill_full(),
            BucketState::new(Bucket::Two, capacity_2),
            goal,
            start_bucket.clone(),
        ),
        Bucket::Two => SearchState::new(
            BucketState::new(Bucket::One, capacity_1),
            BucketState::new(Bucket::Two, capacity_2).fill_full(),
            goal,
            start_bucket.clone(),
        ),
    };
    if let Some(stats) = initial_state.is_meet_expectation() {
        return Some(stats);
    }

    let mut history = vec![];
    history.push(initial_state.clone());
    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    while let Some(top) = queue.pop_front() {
        let (found, next_states) = top.generate_new_states();
        match found {
            Some(stats) => return Some(stats),
            None => next_states.iter().for_each(|it| {
                println!("{:?}", it);
                if history.contains(it) {
                    return;
                }
                history.push(it.clone());
                queue.push_back(it.to_owned());
            }),
        }
    }

    None
}
