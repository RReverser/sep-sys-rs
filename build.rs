fn main() {
    cc::Build::new()
        .define("_USE_MATH_DEFINES", "")
        .files(
            std::fs::read_dir("src/sep/src")
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| match path.extension() {
                    Some(ext) => ext == "c",
                    None => false,
                }),
        )
        .compile("sep");
}
