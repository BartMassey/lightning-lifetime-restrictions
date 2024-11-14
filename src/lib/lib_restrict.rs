pub fn static_hello(x: &str) -> &str {
    println!("{}", x);
    "hello"
}

pub struct Thing<'a> {
    _init_data: &'a str,
    run_data: &'a str,
}

impl<'a> Thing<'a> {
    pub fn new(_init_data: &'a str, run_data: &'a str) -> Self {
        Thing { _init_data, run_data }
    }

    pub fn extract(self) -> &'a str {
        self.run_data
    }
}
