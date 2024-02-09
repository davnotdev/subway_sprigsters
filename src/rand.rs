use core::cell::UnsafeCell;
use nanorand::{RandomGen, RandomRange, Rng, WyRand};

static mut RAND: UnsafeCell<Option<WyRand>> = UnsafeCell::new(None);

pub fn rand<T>() -> T
where
    T: RandomGen<WyRand, 8>,
{
    let rand = unsafe { &mut *RAND.get() };
    if rand.is_none() {
        *rand = Some(WyRand::new_seed(9999999999));
    }
    let rand = rand.as_mut().unwrap();
    rand.generate::<T>()
}

pub fn rand_range<T, B>(bounds: B) -> T
where
    T: RandomRange<WyRand, 8>,
    B: std::ops::RangeBounds<T>,
{
    let rand = unsafe { &mut *RAND.get() };
    if rand.is_none() {
        *rand = Some(WyRand::new_seed(9999999999));
    }
    let rand = rand.as_mut().unwrap();
    rand.generate_range(bounds)
}
