extern crate test1;
// extern crate test2; test2はdependenciesに追加する必要がある

fn main() {
    println!("{}", test1::add(2, 2));
}
