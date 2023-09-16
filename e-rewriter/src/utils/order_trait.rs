use std::cmp::Ordering;
use rand::Rng;

pub trait OrdRandom {
    fn max_random(self, other: i32) -> i32;
    fn min_random(self, other: i32) -> i32;
}

impl OrdRandom for i32 {
    fn max_random(self, other: i32) -> i32 {
        match self.cmp(&other) {
            Ordering::Equal => {
                let mut rng = rand::thread_rng();

                if rng.gen::<bool>() {
                    self
                } else {
                    other
                }
            }
            Ordering::Less | Ordering::Greater => {
                if self < other {
                    other
                } else {
                    self
                }
            }
        }
    }
    fn min_random(self, other: i32) -> i32 {
        match self.cmp(&other) {
            Ordering::Equal => {
                let mut rng = rand::thread_rng();
                if rng.gen::<bool>() {
                    self
                } else {
                    other
                }
            }
            Ordering::Less | Ordering::Greater => {
                if self < other {
                    self
                } else {
                    other
                }
            }
        }
    }
}

pub fn min_random_cmp<T, F>(v1: T, v2: T, compare: F) -> T
where
    F: FnOnce(&T, &T) -> Ordering,
{
    match compare(&v1, &v2) {
        Ordering::Less | Ordering::Equal => {
            let mut rng = rand::thread_rng();
            if rng.gen::<bool>() {
                v1
            } else {
                v2
            }
        }
        Ordering::Greater => v2,
    }
}


pub trait MyIteratorExt: Iterator {
    fn min_by_random<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> std::cmp::Ordering;
}


impl<I> MyIteratorExt for I
where
    I: Iterator,
{
    fn min_by_random<F>(self, compare: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        #[inline]
        fn fold<T>(mut compare: impl FnMut(&T, &T) -> Ordering) -> impl FnMut(T, T) -> T {
            move |x, y| min_random_cmp(x, y, &mut compare)
        }
    
        self.reduce(fold(compare))
    }
}

