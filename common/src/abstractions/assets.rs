/// Represents an asset that can be loaded from a file or a folder.
///
/// This is a high-level abstraction that allows assets to be loaded from a
/// variety of sources, without having to know about the specifics of how those
/// assets are loaded.
///
/// For example, a texture can be loaded from a file, or from a folder
/// containing multiple files. The asset loader will know how to handle both
/// cases, and will return a texture object that can be used by the engine.
pub trait Asset {}
