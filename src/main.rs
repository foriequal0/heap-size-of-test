use heapsize::HeapSizeOf;

trait Buf: HeapSizeOf {}

struct A {
    buf: [u8; 8],
}

impl A {
    fn new() -> A {
        A { buf: [1,2,3,4,1,2,3,4] }
    }
}

impl Buf for A{}

impl HeapSizeOf for A {
    fn heap_size_of_children(&self) -> usize { 0 }
}

struct B {
    buf: [u8; 15],
}

impl B {
    fn new() -> B {
        B { buf: [1,2,3,4,5, 1,2,3,4,5, 1,2,3,4,5, ], }
    }
}

impl Buf for B{}

impl HeapSizeOf for B {
    fn heap_size_of_children(&self) -> usize { 0 }
}

fn main() {
    let v = Box::new(vec![
        Box::new(A::new()) as Box<Buf>,
        Box::new(B::new()) as Box<Buf>,
    ]);

    dbg!(v.heap_size_of_children());
}
