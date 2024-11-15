#![feature(closure_lifetime_binder)]

fn main() {
    #[cfg(feature = "appfail")]
    let f = |s: &str| -> Result<&str, std::convert::Infallible> {
        if s.len() & 1 == 1 {
            Ok(s)
        } else {
            Ok("!")
        }
    };

    #[cfg(not(feature = "appfail"))]
    let f = for <'a> |s: &'a str| -> Result<&'a str, std::convert::Infallible> {
        if s.len() & 1 == 1 {
            Ok(s)
        } else {
            Ok("!")
        }
    };

    let h = "hello".to_string();
    let w = "world".to_string();

    let l = [h.as_ref(), w.as_ref()];
    llr::select(f, l.as_ref());
}
