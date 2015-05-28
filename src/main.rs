extern crate tabprint;
use tabprint::Table;

#[allow(dead_code)]
fn main() {
    let mut table = Table::new(vec!["ABC".to_string(), "DEFG".to_string(), "HIJKLMN".to_string()]);
    table.add_row(vec!["foobar".to_string(), "bar".to_string(), "foo".to_string()]).unwrap();
    table.add_row(vec!["foobar2".to_string(), "bar2".to_string(), "foo2".to_string()]).unwrap();
    table.print();
    println!("Modified : ");
    table.set_element("new_foo".to_string(), 2, 1).unwrap();
    table.print();
}