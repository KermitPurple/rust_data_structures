mod binary_tree;
use binary_tree::{make_tree_vals, Tree};

mod stack;
use stack::Stack;

fn print_heading(title: &str) {
    let sep = "-".repeat(30);
    println!("{} {} {}", sep, title, sep);
}

fn main() {
    // Binary Tree
    print_heading("Binary Tree");
    let t = Tree::from_vec(make_tree_vals(50));
    t.print_val();
    t.print_lnr();
    t.print_fs();
    println!("{}", t.contains(&5));
    println!("{:?}", make_tree_vals(10));
    // Stack
    print_heading("Stack");
    let mut s = Stack::<i32>::new();
    for i in 0..15 {
        s.push(i);
    }
    for _i in 0..16 {
        if let Some(val) = s.pop() {
            println!("{}", val);
        } else {
            println!("Empty!");
        }
    }
    s = Stack::from_vec(vec![1, 2, 3, 4]);
    for _i in 0..5 {
        if let Some(val) = s.pop() {
            println!("{}", val);
        } else {
            println!("Empty!");
        }
    }
    println!("{}, {}", s.is_empty(), Stack::<String>::new().is_empty());
}
