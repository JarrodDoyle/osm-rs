fn main() {
    println!("cargo::rustc-check-cfg=cfg(game, values(\"t1\", \"t2\", \"ss2\"))");
}
