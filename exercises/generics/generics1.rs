// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

// I NOT DONE

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // 这算用泛型解决吗？虽然Vec本身就是泛型的
    shopping_list.push("milk");
}
