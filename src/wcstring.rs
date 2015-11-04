
use ::std;
use ::std::ffi::OsStr;
use ::std::os::windows::ffi::OsStrExt;

use ::error;
use ::{NulError, NoNulError};
use ::WCStr;
use ::split;
use ::Split;

/// A type representing an owned Win32 style "wide" string.
#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Clone)]
pub struct WCString {
    inner: Vec<u16>
}

impl WCString {
    /// Create an empty ```WCString```.
    /// # ```new()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::new();
    ///     assert!(s.len() == 0);
    pub fn new() -> WCString {
        WCString {
            inner: vec![0]
        }
    }

    /// Create a ```WCString``` from a ```Vec<u16>```.
    /// The string will be scanned for nul and NulError will be returned if a nul is found.
    /// # ```from_vec()``` example
    ///     use wcstr::WCString;
    ///     use std::os::windows::ffi::OsStrExt;
    ///     use std::ffi::OsStr;
    ///     let v: Vec<_> = OsStr::new("testing").encode_wide().collect();
    ///     let s = WCString::from_vec(v).unwrap();
    ///     assert!(s.len() == 7);
    pub fn from_vec<T>(v: T) -> Result<WCString, NulError>
        where T: Into<Vec<u16>> {
        let v = v.into();
        match v.iter().position(|&x| x == 0) {
            Some(i) => Err(error::nul(i, Some(v))),
            None => Ok(unsafe { WCString::from_vec_unchecked(v) }),
        }
    }

    /// Create a WCString from a Vec<u16> with a nul terminator.
    /// The string will be scanned for nul.
    /// The string will be truncated at the position where nul is found.
    /// NoNulError will be returned if a nul could not be found.
    /// # ```from_vec_with_nul()``` example
    ///     use wcstr::WCString;
    ///     use std::os::windows::ffi::OsStrExt;
    ///     use std::ffi::OsStr;
    ///     let v: Vec<_> = OsStr::new("testing\0").encode_wide().collect();
    ///     let s = WCString::from_vec_with_nul(v).unwrap();
    ///     assert!(s.len() == 7);
    pub fn from_vec_with_nul<T>(u16s: T) -> Result<WCString, NoNulError>
        where T: Into<Vec<u16>> {
        let mut v = u16s.into();
        match v.iter().position(|&x| x == 0) {
            None => Err(error::no_nul(Some(v))),
            Some(i) => {
                v.truncate(i + 1);
                Ok(unsafe { WCString::from_vec_with_nul_unchecked(v) })
            },
        }
    }

    /// Create a WCString from a Vec<u16> without checking for validity.
    /// This function is unsafe as it assumes that the string passed in has no nul in it.
    /// # ```from_vec_unchecked()``` example
    ///     use wcstr::WCString;
    ///     use std::os::windows::ffi::OsStrExt;
    ///     use std::ffi::OsStr;
    ///     let v: Vec<_> = OsStr::new("testing").encode_wide().collect();
    ///     let s = unsafe { WCString::from_vec_unchecked(v) };
    ///     assert!(s.len() == 7);
    pub unsafe fn from_vec_unchecked(v: Vec<u16>) -> WCString {
        let mut v = v;
        v.push(0);
        WCString::from_vec_with_nul_unchecked(v)
    }

    /// Create a WCString from a Vec<u16> with a nul terminator without checking for validity.
    /// This function is unsafe for the following reasons:
    ///  * This function assumes that the string passed in has no nul in it aside from the nul
    ///  terminator.
    ///  * This function assumes that the string passed in has a nul terminator at the end.
    /// # ```from_vec_with_nul_unchecked()``` example
    ///     use wcstr::WCString;
    ///     use std::os::windows::ffi::OsStrExt;
    ///     use std::ffi::OsStr;
    ///     let v: Vec<_> = OsStr::new("testing\0").encode_wide().collect();
    ///     let s = unsafe { WCString::from_vec_with_nul_unchecked(v) };
    ///     assert!(s.len() == 7);
    pub unsafe fn from_vec_with_nul_unchecked(v: Vec<u16>) -> WCString {
        WCString { inner: v }
    }

    /// Create a WCString from a &OsStr (or anything that can be cast to &OsStr, including OsString, &str and String)
    /// The string will be scanned for nul and NulError will be returned if a nul is found.
    /// # ```from_str()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     assert!(s.len() == 7);
    pub fn from_str<T>(s: T) -> Result<WCString, NulError>
        where T: AsRef<OsStr> {
        let v: Vec<u16> = s.as_ref().encode_wide().collect();
        WCString::from_vec(v)
    }

    /// Create a WCString from a &OsStr with a nul terminator (or anything that can be cast to &OsStr, including OsString, &str and String)
    /// The string will be scanned for nul and NoNulError will be returned if a nul could not be
    /// found. The string will be truncated at the position where nul is found.
    /// # ```from_str_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str_with_nul("testing\0").unwrap();
    ///     assert!(s.len() == 7);
    pub fn from_str_with_nul<T>(s: T) -> Result<WCString, NoNulError>
        where T: AsRef<OsStr> {
        let v: Vec<u16> = s.as_ref().encode_wide().collect();
        WCString::from_vec_with_nul(v)
    }

    /// Return the underlying buffer as a Vec<u16>.
    /// The WCString will be consumed.
    /// The returned buffer does not contain the nul terminator.
    /// The returned buffer does not contain any nul.
    /// # ```into_vec()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     let v = s.into_vec();
    ///     assert!(*v.last().unwrap() != 0);
    pub fn into_vec(self) -> Vec<u16> {
        let mut v = self.inner;
        let _nul = v.pop();
        debug_assert_eq!(_nul, Some(0u16));
        v
    }

    /// Return the underlying buffer as a Vec<u16> with a nul terminator.
    /// The WCString will be consumed.
    /// The returned buffer does not contain any nul aside from the nul terminator.
    /// # ```into_vec_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     let v = s.into_vec_with_nul();
    ///     assert!(*v.last().unwrap() == 0);
    pub fn into_vec_with_nul(self) -> Vec<u16> {
        self.inner
    }

    /// Return the underlying buffer as a u16 slice.
    /// The returned slice does not contain the nul terminator.
    /// The returned slice does not contain any nul.
    /// # ```as_slice()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     let w = s.as_slice();
    ///     assert!(*w.last().unwrap() != 0);
    pub fn as_slice(&self) -> &[u16] {
        &self.inner[..self.len()]
    }

    /// Return the underlying buffer as a u16 slice with a nul terminator.
    /// The returned slice does not contain any nul aside from the nul terminator.
    /// # ```as_slice_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     let w = s.as_slice_with_nul();
    ///     assert!(*w.last().unwrap() == 0);
    pub fn as_slice_with_nul(&self) -> &[u16] {
        &self.inner
    }

    /// Return this string as a &WCStr
    /// # ```as_wcstr()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("testing").unwrap();
    ///     let w = s.as_wcstr();
    pub fn as_wcstr(&self) -> &WCStr {
        &self
    }

    /// Push/Append a &WCStr (or anything that can cast to a &WCStr, like another WCString).
    /// # ```push()``` example
    ///     use wcstr::WCString;
    ///     let mut s = WCString::new();
    ///     let t = WCString::from_str("test").unwrap();
    ///     s.push(&t);
    ///     s.push(&t);
    pub fn push<T>(&mut self, s: T)
        where T: AsRef<WCStr> {
        let _nul = self.inner.pop();
        debug_assert_eq!(_nul, Some(0u16));
        self.inner.extend(s.as_ref().to_slice_with_nul());
    }

    /// Push/Append a u16 slice.
    /// The slice will be scanned for nul, and the push will fail with NulError if a nul is found.
    /// # ```push_slice()``` example
    ///     use wcstr::WCString;
    ///     let mut s = WCString::new();
    ///     let t = WCString::from_str("test").unwrap();
    ///     let t = t.as_slice();
    ///     s.push_slice(t).unwrap();
    ///     s.push_slice(t).unwrap();
    pub fn push_slice<T>(&mut self, s: T) -> Result<(), NulError>
        where T: AsRef<[u16]> {
        let s = s.as_ref();
        match s.iter().position(|&w| w == 0) {
            Some(i) => Err(error::nul(i, None)),
            None => {
                let _nul = self.inner.pop();
                debug_assert_eq!(_nul, Some(0u16));
                self.inner.extend(s);
                self.inner.push(0);
                Ok(())
            },
        }
    }

    /// Push/Append a u16 slice with a nul terminator.
    /// The slice will be scanned for nul, and the push will fail with NoNulError if a nul is not
    /// found.
    /// The push will stop at the first nul found in the slice.
    /// # ```push_slice_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let mut s = WCString::new();
    ///     let t = WCString::from_str("test").unwrap().into_vec_with_nul();
    ///     s.push_slice_with_nul(&t).unwrap();
    ///     s.push_slice_with_nul(&t).unwrap();
    pub fn push_slice_with_nul<T>(&mut self, s: T) -> Result<(), NoNulError>
        where T: AsRef<[u16]> {
        let s = s.as_ref();
        match s.iter().position(|&w| w == 0) {
            None => Err(error::no_nul(None)),
            Some(i) => {
                let _nul = self.inner.pop();
                debug_assert_eq!(_nul, Some(0u16));
                self.inner.extend(&s[.. i + 1]);
                Ok(())
            },
        }
    }

    /// Push/Append a &OsStr (or anything that can be cast to &OsStr)
    /// The string will be scanned for nul, and the push will fail with NulError if a nul is found.
    /// # ```push_ce_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let mut s = WCString::new();
    ///     s.push_str("test1").unwrap();
    ///     s.push_str("test2").unwrap();
    pub fn push_str<T>(&mut self, s: T) -> Result<(), NulError>
        where T: AsRef<OsStr> {
        let _nul = self.inner.pop();
        debug_assert_eq!(_nul, Some(0u16));

        let len = self.inner.len();
        let s = s.as_ref();
        let mut not_nuled = true;
        self.inner.extend(s.encode_wide().take_while(|&w| { not_nuled = w != 0; not_nuled }));

        if not_nuled {
            self.inner.push(0);
            Ok(())
        }
        else {
            let pos = self.inner.len() - len;
            self.inner.truncate(len);
            self.inner.push(0);
            Err(error::nul(pos, None))
        }
    }

    /// Push/Append a &OsStr (or anything that can be cast to &OsStr)
    /// The string will be scanned for nul, and the push will fail with NoNulError if a nul is not
    /// found.
    /// The push will stop at the first nul found in the string.
    /// # ```push_ce_with_nul()``` example
    ///     use wcstr::WCString;
    ///     let mut s = WCString::new();
    ///     s.push_str_with_nul("test1\0everything after nul will be ignored").unwrap();
    ///     s.push_str_with_nul("test2\0").unwrap();
    pub fn push_str_with_nul<T>(&mut self, s: T) -> Result<(), NoNulError>
        where T: AsRef<OsStr> {
        let _nul = self.inner.pop();
        debug_assert_eq!(_nul, Some(0u16));

        let len = self.inner.len();
        let s = s.as_ref();
        let mut not_nuled = true;
        self.inner.extend(s.encode_wide().take_while(|&w| { not_nuled = w != 0; not_nuled }));
        if not_nuled {
            self.inner.truncate(len);
            self.inner.push(0);
            Err(error::no_nul(None))
        }
        else {
            self.inner.push(0);
            Ok(())
        }
    }

    /// Split the string into multiple ```&mut WCStr``` using a delimiter.
    ///
    /// * This returns an iterator that creates a ```&mut WCStr``` for each part of the string
    /// separated by the delimiter.
    /// * This will consume the string.
    ///
    /// # ```split()``` example
    ///     use wcstr::WCString;
    ///     let s = WCString::from_str("a;b;c;d;e").unwrap();
    ///     let mut count = 0;
    ///     for w in s.split(b';' as u16).iter() {
    ///         count += 1;
    ///         assert!(w.len() == 1);
    ///     }
    ///     assert!(count == 5);
    pub fn split(self, delimiter: u16) -> Split {
        split::new(self.inner, delimiter)
    }
}

impl std::ops::Deref for WCString {
    type Target = WCStr;

    fn deref(&self) -> &WCStr {
        unsafe { std::mem::transmute(self.as_slice_with_nul()) }
    }
}

impl std::fmt::Debug for WCString {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Debug::fmt(&self, formatter)
    }
}

impl AsRef<WCStr> for WCString {
    fn as_ref(&self) -> &WCStr {
        self
    }
}

impl AsRef<[u16]> for WCString {
    fn as_ref(&self) -> &[u16] {
        use std::ops::Deref;
        Deref::deref(self).as_ref()
    }
}

impl std::borrow::Borrow<WCStr> for WCString {
    fn borrow(&self) -> &WCStr {
        self
    }
}

