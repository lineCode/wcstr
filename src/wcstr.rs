
use ::std;
use ::std::ffi::OsString;
use ::std::os::windows::ffi::OsStringExt;

use ::WCString;
use ::error;
use error::NoNulError;

/// Representation of a borrowed Win32 style "wide" string.
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WCStr {
    inner: [u16]
}

impl WCStr {
    /// Create a ```&WCStr``` from a raw pointer and a length.
    ///
    /// This function is unsafe for the reasons mentioned below.
    ///
    /// This function assumes that the pointer passed in has these properties:
    ///
    /// * It is not null.
    /// * It is a valid pointer.
    /// * It points to an array of ```u16```'s that does not contain any ```nul``` values.
    /// * It points to an array of ```u16```'s that is terminated with a ```nul``` at exactly the offset "```len```".
    ///
    /// This function will assert/panic when ```nul``` is not found at offset "```len```".
    ///
    /// The lifetime of the ```&WCStr``` returned from this function is not guranteed to be correct and
    /// it is up to the caller to determine the appropriate lifetime.
    ///
    /// # ```from_raw_parts()``` example
    ///
    ///     use wcstr::WCStr;
    ///     static a : &'static [u16] = &[116u16, 101u16, 115u16, 116u16, 0];
    ///     let s = unsafe { WCStr::from_raw_parts(a.as_ptr(), a.len() - 1) };
    ///     assert!(s.len() == (a.len() - 1));
    pub unsafe fn from_raw_parts<'a>(ptr: *const u16, len: usize) -> &'a WCStr {
        assert!(*ptr.offset(len as isize) == 0u16);
        std::mem::transmute(std::slice::from_raw_parts(ptr, len + 1))
    }

    /// Create a ```&WCStr``` from a slice of ```u16```'s.
    /// This function will scan the slice for ```nul``` and assume that ```nul``` terminates the string.
    /// If no ```nul``` is found in the slice, it will return ```Err(NoNulError(None))```
    /// # ```frm_slice_with_nul()``` example
    ///
    ///     use wcstr::WCStr;
    ///     static a : &'static [u16] = &[116u16, 101u16, 115u16, 116u16, 0];
    ///     let s = WCStr::from_slice_with_nul(a);
    ///     assert!(s.len() == (a.len() - 1));
    pub fn from_slice_with_nul<'a>(slice: &'a [u16]) -> Result<&'a WCStr, NoNulError> {
        match slice.iter().position(|x| *x == 0) {
            None => Err(error::no_nul(None)),
            Some(i) => Ok(unsafe { std::mem::transmute(&slice[..i + 1]) }),
        }
    }

    /// length of the string in u16 units
    pub fn len(&self) -> usize {
        self.inner.len() - 1
    }

    /// Return a raw pointer to this "wide" string.
    ///
    ///  * The pointer remains valid only as long as this string is valid.
    ///  * The pointer points to a contiguous region of memory terminated with nul.
    pub fn as_ptr(&self) -> *const u16 {
        self.inner.as_ptr()
    }

    /// Return this "wide" string as a slice of ```u16```s without a nul terminator.
    pub fn to_slice(&self) -> &[u16] {
        &self.inner[..self.len()]
    }

    /// Return this "wide" string as a slice of ```u16```s with a nul terminator.
    pub fn to_slice_with_nul(&self) -> &[u16] {
        &self.inner
    }

    /// Convert this "wide" string to a ```String``` by using ```String::from_utf16```
    pub fn to_string(&self) -> Result<String, std::string::FromUtf16Error> {
        String::from_utf16(&self.inner)
    }

    /// Convert this "wide" string to a ```String``` by using ```String::from_utf16_lossy```
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(&self.inner)
    }

    /// Convert this "wide" string to an ```OsString``` by using OsString::from_wide
    pub fn to_os_string(&self) -> OsString {
        OsString::from_wide(self.to_slice())
    }
}

impl std::fmt::Debug for WCStr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "\""));
        for &w in self.to_slice().iter() {
            if w < 0xD800 || w >= 0xE000 {
                for c in unsafe { std::char::from_u32_unchecked(w as u32) }.escape_default() {
                    use std::fmt::Write;
                    try!(f.write_char(c));
                }
            }
            else {
                try!(write!(f, "\\u{{{:X}}}", w));
            }
        }
        write!(f, "\"")
    }
}


impl AsRef<WCStr> for WCStr {
    fn as_ref(&self) -> &WCStr {
        self
    }
}

impl AsRef<[u16]> for WCStr {
    fn as_ref(&self) -> &[u16] {
        &self.inner[..self.len()]
    }
}

impl ToOwned for WCStr {
    type Owned = WCString;
    fn to_owned(&self) -> WCString {
        unsafe {
            WCString::from_vec_with_nul_unchecked(self.inner.to_owned())
        }
    }
}

