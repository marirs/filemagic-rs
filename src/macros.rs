///
/// Returns the Mime type of a file
///
///
/// ### Usage Example
///
/// ```ignore
/// use filemagic::magic;
///
/// let magic = magic!();
/// println!("{}", magic.file("path/to/file"));
/// ```
#[macro_export]
macro_rules! magic {
    () => {
        $crate::Magic::open(Default::default())
            .and_then(|magic| magic.load::<&str>(&[]).and_then(|_| Ok(magic)))
    };
    ($flags:expr) => {
        $crate::Magic::open($flags)
            .and_then(|magic| magic.load::<&str>(&[]).and_then(|_| Ok(magic)))
    };
    (,$magic_databases:expr) => {
        $crate::Magic::open(Default::default())
            .and_then(|magic| magic.load($magic_databases).and_then(|_| Ok(magic)))
    };
    ($flags:expr, $magic_databases:expr) => {
        $crate::Magic::open($flags)
            .and_then(|magic| magic.load($magic_databases).and_then(|_| Ok(magic)))
    };
}
