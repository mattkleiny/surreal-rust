/// A handle for a resource created by one of the platform servers.
///
/// Resource IDs can be passed around the application to represent shared platform components,
/// without having to have a pointer back to the original provider.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RID(pub(crate) u32);

// TODO: custom drop implementation for RIDs?
