
/// The KeypairType enum is used to allow handling multiple different 
/// types of keypairs being stored in a configuration file in a dependency
/// free manner
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum KeypairType {
    /// indicates this keypair type is a file stored at the given path
    FileBased(String),
    /// indicates this keypair type is an encoded private key. 
    /// the actual encoding method will depend on the library
    /// using this template, but is likely to be hex encoded
    Private(String),
}