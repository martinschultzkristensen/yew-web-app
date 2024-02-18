
use yew_router::prelude::*;

fn main() {
    let location = Location::current();
    println!("Current URL path: {}", location.pathname());
    println!("Current URL query parameters: {:?}", location.query());
    // You can access other properties of the Location struct as needed
}

