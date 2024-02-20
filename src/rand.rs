use core::cell::UnsafeCell;
use nanorand::{RandomGen, RandomRange, Rng, WyRand};

static mut RAND: UnsafeCell<Option<WyRand>> = UnsafeCell::new(None);

unsafe fn get(seed: Option<u64>) -> &'static mut WyRand {
    let rand = unsafe { &mut *RAND.get() };
    if rand.is_none() {
        //  https://xkcd.com/221/
        *rand = Some(WyRand::new_seed(seed.unwrap_or(4)));
    }
    rand.as_mut().unwrap()
}

pub fn rand<T>() -> T
where
    T: RandomGen<WyRand, 8>,
{
    let rand = unsafe { get(None) };
    rand.generate::<T>()
}

pub fn rand_range<T, B>(bounds: B) -> T
where
    T: RandomRange<WyRand, 8>,
    B: core::ops::RangeBounds<T>,
{
    let rand = unsafe { get(None) };
    rand.generate_range(bounds)
}

pub unsafe fn set_seed(seed: u64) {
    let rand = unsafe { get(None) };
    *rand = WyRand::new_seed(seed);
}
