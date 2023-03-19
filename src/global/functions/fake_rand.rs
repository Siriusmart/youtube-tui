use chrono::Utc;
use std::ops::*;

/// generate a random number between range
pub fn fake_rand_range<T>(from: T, to: T) -> T
where T: From<i64> + Sub + Rem<<T as std::ops::Sub>::Output> + std::convert::From<<<T as std::ops::Rem<<T as std::ops::Sub>::Output>>::Output as std::ops::Add<T>>::Output> + Copy, <T as Rem<<T as Sub>::Output>>::Output: Add<T>
{
    let timestamp = Utc::now().timestamp();

    (<i64 as Into<T>>::into(timestamp) % (to - from) + from).into()
}
