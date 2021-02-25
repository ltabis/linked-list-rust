
// for the sake of being simple, we will skip genericity for this one
pub struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    value: i32,
    next: List
}