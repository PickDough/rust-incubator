use btree_proc_macro::btree_proc;
use std::collections::BTreeMap;

macro_rules! btree_decl {
    ($(($k:expr, $v:expr)),*) => {
        {
            let mut b_tree = BTreeMap::new();
            $(
                b_tree.insert($k, $v);
            )*

            b_tree
        }
    };
}

fn main() {
    println!("Implement me!");

    let b = btree_decl![("1", 2), ("3", 4)];

    let x = btree_proc!((1, 2), (3, 5));

    println!("{:?}", x);
}
