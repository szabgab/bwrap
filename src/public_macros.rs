// Copyright (C) 2023 Michael Lee <micl2e2@proton.me>
//
// Licensed under the MIT License <LICENSE-MIT or
// https://opensource.org/license/mit> or the GNU General Public License,
// Version 3.0 or any later version <LICENSE-GPL or
// https://www.gnu.org/licenses/gpl-3.0.txt>, at your option.
//
// This file may not be copied, modified, or distributed except except in
// compliance with either of the licenses.
//

#[cfg_attr(doc_cfg, doc(cfg(feature = "use_std")))]
#[cfg(feature = "use_std")]
mod stdonly {
    ///
    /// Wrap text in [`Non-Break`] style.
    ///
    /// [`Non-Break`]: crate::WrapStyle::NoBrk
    ///
    /// Note that the style `WrapStyle::NoBrk` is in use, assuming no
    /// appending or prepending, and max width defaults to 80 if second
    /// argument is omitted. This suits for the space-sensitive
    /// languages, such as English, French, German, etc.
    ///
    /// # Panics
    ///
    /// Panics if input string consists of invalid UTF8 bytes.
    #[macro_export]
    macro_rules! wrap {
        ($s:expr) => {{
            use bwrap::EasyWrapper;
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
        ($s:expr, $mw:expr) => {{
            use bwrap::EasyWrapper;
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper.wrap().expect("bwrap wrap");
            String::from(w)
        }};
    }

    ///
    /// Wrap text in [`May-Break`] style.
    ///
    /// [`May-Break`]: crate::WrapStyle::MayBrk
    ///
    /// Note that the style `WrapStyle::MayBrk` is in use, assuming no
    /// appending or prepending, and max width defaults to 80 if second
    /// argument is omitted. This suits for the space-insensitive
    /// languages, such as Chinese, Japanese, Thai, etc.
    ///
    /// # Panics
    ///
    /// Panics if input string consists of invalid UTF8 bytes.
    #[macro_export]
    macro_rules! wrap_maybrk {
        ($s:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, 80).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(None, None))
                .expect("bwrap wrap");
            String::from(w)
        }};

        ($s:expr, $mw:expr) => {{
            use bwrap::{EasyWrapper, WrapStyle};
            let mut wrapper = EasyWrapper::new($s, $mw).expect("bwrap init");
            let w = wrapper
                .wrap_use_style(WrapStyle::MayBrk(None, None))
                .expect("bwrap wrap");
            String::from(w)
        }};
    }
}
