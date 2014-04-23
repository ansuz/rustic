fn main() {
    let list = ~Node(1, ~Node(2, ~Node(3, ~Empty)));
    println!("Sum of all values in the list: {:i}.", list.sum());
}

enum IntList {
    Node(int, ~IntList),
    Empty
}

impl IntList {
    fn sum(~self) -> int {
        // As in C and C++, pointers are dereferenced with the asterisk `*` operator.
        match *self {
            Node(value, next) => value + next.sum(),
            Empty => 0
        }
    }
}
