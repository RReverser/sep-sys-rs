const HEADER_PATH: &str = "../src/sep/src/sep.h";

fn main() {
    bindgen::builder()
        .raw_line("#![no_std]")
        .use_core()
        .raw_line("#![allow(non_camel_case_types)]")
        .header(HEADER_PATH)
        .allowlist_file(HEADER_PATH)
        .derive_default(true)
        .layout_tests(false)
        .parse_callbacks({
            use bindgen::callbacks::{IntKind, ParseCallbacks};

            #[derive(Debug)]
            struct Callbacks;

            impl ParseCallbacks for Callbacks {
                fn int_macro(&self, name: &str, _value: i64) -> Option<IntKind> {
                    Some(
                        if name.starts_with("SEP_NOISE_")
                            || name.starts_with("SEP_OBJ_")
                            || name.starts_with("SEP_APER_")
                            || name.starts_with("SEP_MASK_")
                        {
                            IntKind::Short
                        } else {
                            IntKind::Int
                        },
                    )
                }
            }

            Box::new(Callbacks)
        })
        .merge_extern_blocks(true)
        .generate()
        .unwrap()
        .write_to_file("../src/lib.rs")
        .unwrap();
}
