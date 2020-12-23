/// Bitmask flags which control `libmagic` behaviour
use libc::c_int;

bitflags! {
    #[doc = "Bitmask flags that specify how `Cookie` functions should behave\n\nNOTE: The descriptions are taken from `man libmagic 3`."]
    #[derive(Default)]
    pub struct Flags: c_int {
        #[doc = "No special handling"]
        const NONE              = 0x000000;

        #[doc = "Print debugging messages to `stderr`\n\nNOTE: Those messages are printed by `libmagic` itself, no this Rust crate."]
        const DEBUG             = 0x000001;

        #[doc = "If the file queried is a symlink, follow it"]
        const SYMLINK           = 0x000002;

        #[doc = "If the file is compressed, unpack it and look at the contents"]
        const COMPRESS          = 0x000004;

        #[doc = "If the file is a block or character special device, then open the device and try to look in its contents"]
        const DEVICES           = 0x000008;

        #[doc = "Return a MIME type string, instead of a textual description"]
        const MIME_TYPE         = 0x000010;

        #[doc = "Return all matches, not just the first"]
        const CONTINUE          = 0x000020;

        #[doc = "Check the magic database for consistency and print warnings to `stderr`\n\nNOTE: Those warnings are printed by `libmagic` itself, no this Rust crate."]
        const CHECK             = 0x000040;

        #[doc = "On systems that support `utime(2)` or `utimes(2)`, attempt to preserve the access time of files analyzed"]
        const PRESERVE_ATIME    = 0x000080;

        #[doc = "Don't translate unprintable characters to a `\\ooo` octal representation"]
        const RAW               = 0x000100;

        #[doc = "Treat operating system errors while trying to open files and follow symlinks as real errors, instead of printing them in the magic buffer"]
        const ERROR             = 0x000200;

        #[doc = "Return a MIME encoding, instead of a textual description"]
        const MIME_ENCODING     = 0x000400;

        #[doc = "A shorthand for `MIME_TYPE | MIME_ENCODING`"]
        const MIME              = Self::MIME_TYPE.bits
                                 | Self::MIME_ENCODING.bits;

        #[doc = "Return the Apple creator and type"]
        const APPLE             = 0x000800;

        #[doc = "Return a slash-separated list of extensions"]
        const EXTENSION         = 0x1000000;

        #[doc = "Check inside compressed files but do not report compression"]
        const COMPRESS_TRANSP   = 0x2000000;

        #[doc = "Don't look inside compressed files"]
        const NO_CHECK_COMPRESS = 0x001000;

        #[doc = "Don't examine tar files"]
        const NO_CHECK_TAR      = 0x002000;

        #[doc = "Don't consult magic files"]
        const NO_CHECK_SOFT     = 0x004000;

        #[doc = "Check for EMX application type (only on EMX)"]
        const NO_CHECK_APPTYPE  = 0x008000;

        #[doc = "Don't print ELF details"]
        const NO_CHECK_ELF      = 0x010000;

        #[doc = "Don't check for various types of text files"]
        const NO_CHECK_TEXT     = 0x020000;

        #[doc = "Don't get extra information on MS Composite Document Files"]
        const NO_CHECK_CDF      = 0x040000;

        #[doc = "Don't look for known tokens inside ascii files"]
        const NO_CHECK_TOKENS   = 0x100000;

        #[doc = "Don't check text encodings"]
        const NO_CHECK_ENCODING = 0x200000;

        #[doc = "No built-in tests; only consult the magic file"]
        const NO_CHECK_BUILTIN  = Self::NO_CHECK_COMPRESS.bits
                                 | Self::NO_CHECK_TAR.bits
                                 | Self::NO_CHECK_APPTYPE.bits
                                 | Self::NO_CHECK_ELF.bits
                                 | Self::NO_CHECK_TEXT.bits
                                 | Self::NO_CHECK_CDF.bits
                                 | Self::NO_CHECK_TOKENS.bits
                                 | Self::NO_CHECK_ENCODING.bits;

        #[doc = "Don't look inside ascii files"]
        const NO_CHECK_ASCII = Self::NO_CHECK_TEXT.bits;

        #[doc = "Don't check ascii/fortran"]
        const NO_CHECK_FORTRAN = 0x000000;

        #[doc = "Don't check ascii/troff"]
        const NO_CHECK_TROFF = 0x000000;
    }
}
