use core::ops::Deref;
use std::sync::OnceLock;

use ibc::primitives::Timestamp;
use tendermint::Time;

/// Returns a `Timestamp` representation of beginning of year 2023.
///
/// This is introduced to initialize [`StoreGenericTestContext`](crate::context::StoreGenericTestContext)s
/// with the same latest timestamp by default.
/// If two [`StoreGenericTestContext`](crate::context::StoreGenericTestContext)
/// are initialized using [`Time::now()`], second one will have a greater timestamp than the first one.
/// So, the latest header of the second context can not be submitted to first one.
/// We can still set a custom timestamp via [`TestContextConfig`](crate::fixtures::core::context::TestContextConfig).
pub fn year_2023() -> Timestamp {
    // Sun Jan 01 2023 00:00:00 GMT+0000
    Time::from_unix_timestamp(1_672_531_200, 0)
        .expect("should be a valid time")
        .into()
}

/// In-house `LazyLock` implementation.
/// Because `std::sync::LazyLock` in not yet stabilized.
pub struct LazyLock<T, F = fn() -> T> {
    once: OnceLock<T>,
    factory: F,
}

impl<T, F> LazyLock<T, F>
where
    F: Fn() -> T,
{
    pub const fn new(factory: F) -> Self {
        Self {
            once: OnceLock::new(),
            factory,
        }
    }

    pub fn get(&self) -> &T {
        self.once.get_or_init(|| (self.factory)())
    }
}

/// Implement `Deref` for `LazyLock` to allow dereferencing.
impl<T> Deref for LazyLock<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}
