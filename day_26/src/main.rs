use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    let num = Box::new(5);
    println!("This num is allocated on the heap {}", num);

    let x = Rc::new(50);
    let y = x.clone();
    let count = Rc::strong_count(&x);
    println!("Value of x is {}", x);
    println!("Value of y is {}", y);
    println!("Reference count is {}", count);

    let data = RefCell::new(10);
    *data.borrow_mut() -= 5;
    println!("Data is {}", data.borrow());


    let cell = Cell::new(20);
    cell.set(2);
    println!("Cell is {}", cell.get());
}
