mod basket;
mod stack;
mod container;

use container::Container;
use basket::Basket;
use stack::Stack;

fn add_string<T:Container<String>>(c:&mut T, s:String){
    c.put(s);
}

fn main() {
    let mut b1 = Basket::new(String::from("hi there!"));

    let b2 = Basket::new(10);

    let b3 = Basket::new(true);

    let mut s1 = Stack::new(
        vec![String::from("hi")]
    );

    let s2 = Stack::new(vec![1,2,4]);

    add_string(&mut b1, String::from("hi"));
    add_string(&mut s1, String::from("hi"));
}
