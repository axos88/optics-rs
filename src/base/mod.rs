mod total_getter;
mod getter;
mod reversible;
mod total_reversible;
mod setter;

pub use total_getter::HasTotalGetter;
pub use getter::HasGetter;
pub use reversible::HasReverseGet;
pub use total_reversible::HasTotalReverseGet;
pub use setter::HasSetter;
