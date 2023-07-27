/// The position of a page in a scrollable context.
#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(crate) enum PagePosition {
    /// This is the only page.
    Only,
    /// The first page of n where n > 1.
    First,
    /// Somewhere between 1 inclusive and len() exclusive.
    Mid,
    /// The last page of n where n > 2.
    Last,
}
