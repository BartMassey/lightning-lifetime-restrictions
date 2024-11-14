fn main() {
    let init = "hello".to_string();
    let run = "world".to_string();
    let t = llr::Thing::new(&init, &run);
    let r = t.extract();
    #[cfg(feature = "appfail")]
    drop(init);
    println!("{}", r);
}
