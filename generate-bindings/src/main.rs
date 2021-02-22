fn main() {
    bindgen::builder()
        .raw_line("#![allow(non_camel_case_types)]")
        .header("../src/sep/src/sep.h")
        .derive_default(true)
        .layout_tests(false)
        .parse_callbacks({
            use bindgen::callbacks::{IntKind, ParseCallbacks};

            #[derive(Debug)]
            struct Callbacks;

            impl ParseCallbacks for Callbacks {
                fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
                    if name.starts_with("SEP_T")
                        || name.starts_with("SEP_THRESH_")
                        || name.starts_with("SEP_FILTER_")
                    {
                        Some(IntKind::Int)
                    } else if name.starts_with("SEP_NOISE_")
                        || name.starts_with("SEP_OBJ_")
                        || name.starts_with("SEP_APER_")
                        || name.starts_with("SEP_MASK_")
                    {
                        Some(IntKind::Short)
                    } else {
                        None
                    }
                }
            }

            Box::new(Callbacks)
        })
        .generate()
        .unwrap()
        .write_to_file("../src/lib.rs")
        .unwrap();
}
