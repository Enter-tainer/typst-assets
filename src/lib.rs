//! Assets for the Typst compiler.
//!
//! These are not part of the main compiler crate to keep its size down.

macro_rules! asset {
    ($path:literal) => {
        include_bytes!(concat!("../files/", $path)).as_slice()
    };
}

/// ICU data.
pub mod icu {
    /// Generated by the following command:
    ///
    /// ```sh
    /// icu4x-datagen --locales full \
    ///               --format blob \
    ///               --keys-for-bin target/debug/typst \
    ///               --out typst-assets/files/icu/icu.postcard \
    ///               --overwrite
    /// ```
    ///
    /// Install icu_datagen with
    /// `cargo install --git https://github.com/unicode-org/icu4x icu_datagen --locked`.
    pub const ICU: &[u8] = asset!("icu/icu.postcard");

    /// Generated by the following command:
    ///
    /// ```sh
    /// icu4x-datagen-cj --locales zh ja \
    ///                  --format blob \
    ///                  --keys segmenter/line@1 \
    ///                  --out typst-assets/files/icu/icu_cj_segment.postcard \
    ///                  --overwrite
    /// ```
    ///
    /// Install a separate, patched icu_datagen with
    /// `cargo install --git https://github.com/typst/icu4x icu_datagen --locked --branch cj-patch`
    ///
    /// Make sure that the `cj-patch` branch is up-to-date with the latest
    /// icu4x upstream changes!
    pub const ICU_CJ_SEGMENT: &[u8] = asset!("icu/icu_cj_segment.postcard");
}

/// ICC profiles.
pub mod icc {
    /// The ICC profile used to convert from CMYK to RGB.
    ///
    /// This is a minimal CMYK profile that only contains the necessary
    /// information to convert from CMYK to RGB. It is based on the CGATS TR
    /// 001-1995 specification. See
    /// <https://github.com/saucecontrol/Compact-ICC-Profiles#cmyk>.
    pub const CMYK_TO_XYZ: &[u8] = asset!("icc/CMYK-to-XYZ.icc");
    pub const S_GREY_V4: &[u8] = asset!("icc/sGrey-v4.icc");
    pub const S_RGB_V4: &[u8] = asset!("icc/sRGB-v4.icc");
}

/// Bundled fonts.
///
/// This is only available if the `fonts` feature is enabled.
#[cfg(feature = "fonts")]
pub fn fonts() -> impl Iterator<Item = &'static [u8]> {
    [
        asset!("fonts/LinLibertine_R.ttf"),
        asset!("fonts/LinLibertine_RB.ttf"),
        asset!("fonts/LinLibertine_RBI.ttf"),
        asset!("fonts/LinLibertine_RI.ttf"),
        asset!("fonts/NewCMMath-Book.otf"),
        asset!("fonts/NewCMMath-Regular.otf"),
        asset!("fonts/NewCM10-Regular.otf"),
        asset!("fonts/NewCM10-Bold.otf"),
        asset!("fonts/NewCM10-Italic.otf"),
        asset!("fonts/NewCM10-BoldItalic.otf"),
        asset!("fonts/DejaVuSansMono-Bold.ttf"),
        asset!("fonts/DejaVuSansMono-BoldOblique.ttf"),
        asset!("fonts/DejaVuSansMono-Oblique.ttf"),
        asset!("fonts/DejaVuSansMono.ttf"),
    ]
    .into_iter()
}
