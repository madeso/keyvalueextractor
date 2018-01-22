mod kve;

fn main() {
    let kv = kve::KeyValueExtractor::new("%album%/%artist%-%title%");
    println!("It is {:?}!", kv);
}
