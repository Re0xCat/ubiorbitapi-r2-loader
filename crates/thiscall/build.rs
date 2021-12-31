fn main() {
    cc::Build::new().file("includes/cxx.c").compile("libcxx.a");
}
