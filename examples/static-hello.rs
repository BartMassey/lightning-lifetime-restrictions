fn main() {
    let world = "world".to_string();
    let hello = llr::static_hello(&world);
    #[cfg(feature = "appfail")]
    drop(world);
    println!("{}", hello);
}
