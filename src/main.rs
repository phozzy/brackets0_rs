use brackets0_rs::brackets::Brackets;
fn main() {
    let mut br = Brackets::new(3);
    if let Some(plist) = br.next() {
        println!("{}", plist);
    }
}