mod binary_tree;
use binary_tree::{make_tree_vals, Tree};

fn print_heading(title: &str) {
    let sep = "-".repeat(30);
    println!("{} {} {}", sep, title, sep);
}

fn main() {
    //Binary Tree
    print_heading("Binary Tree");
    let t = Tree::from_vec(make_tree_vals(100));
    t.print_val();
    t.print_lnr();
    t.print_fs();
    println!("{}", t.contains(&5));
    println!("{:?}", make_tree_vals(10));
}
