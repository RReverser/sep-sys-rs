fn main() {
    cc::Build::new()
        .define("_USE_MATH_DEFINES", "")
        .files(
            std::fs::read_dir("src/sep/src")
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| path.extension().is_some_and(|ext| ext == "c")),
        )
        .compile("sep");
}
