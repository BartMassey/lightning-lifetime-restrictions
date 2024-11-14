pub fn static_hello(x: &str) -> &'static str {
    println!("{}", x);
    "hello"
}

pub struct Thing<'a, 'b> {
    _init_data: &'a str,
    run_data: &'b str,
}

impl<'a, 'b> Thing<'a, 'b> {
    pub fn new(_init_data: &'a str, run_data: &'b str) -> Self {
        Thing { _init_data, run_data }
    }

    pub fn extract(self) -> &'b str {
        self.run_data
    }
}
