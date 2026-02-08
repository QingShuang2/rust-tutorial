use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use std::thread;

fn main() {
    // Box<T> for heap allocation
    let b = Box::new(5);
    println!("Box value: {}", b);

    // Recursive types with Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("List: {:?}", list);

    // Rc<T> for multiple ownership
    let a = Rc::new(List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil)))));
    println!("Reference count after creating a: {}", Rc::strong_count(&a));

    let b = List::Cons(3, Box::new(a.clone()));
    println!("Reference count after creating b: {}", Rc::strong_count(&a));

    {
        let c = List::Cons(4, Box::new(a.clone()));
        println!("Reference count after creating c: {}", Rc::strong_count(&a));
    }
    println!("Reference count after c goes out of scope: {}", Rc::strong_count(&a));

    // RefCell<T> for interior mutability
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(ListRef::Cons(Rc::clone(&value), Rc::new(ListRef::Nil)));
    let b = ListRef::Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = ListRef::Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Arc<T> for thread-safe reference counting
    let counter = Arc::new(RefCell::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.borrow_mut();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.borrow());
}

#[derive(Debug)]
enum ListRef {
    Cons(Rc<RefCell<i32>>, Rc<ListRef>),
    Nil,
}