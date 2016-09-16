extern crate iron;

use iron::prelude::*;
use iron::status;
use std::fmt::*;

struct MyShit {
    my: i32,
    shit: i32,
}

impl Display for MyShit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.my, self.shit)
    }
}

impl Drop for MyShit {
    fn drop(&mut self) {
        println!("dropped {}", self);
    }
}

fn main() {
    fn hello_world(_: &mut Request) -> IronResult<Response> {
        let d = MyShit { my: 5, shit: 5 };
        println!("{}", d);
        Ok(Response::with(status::Ok, format!{"HOOOOO{}", d}))
    }

    let mut x = 5;
    {
        let y = &x;
    }
    x = x * 2;

    Iron::new(hello_world).http("localhost:3000").unwrap();
    println!("On 3000");
}
