mod kve;

use std::path::Path;

fn main() {
    match kve::KeyValueExtractor::new("%album%/%artist%-%title%") {
        Err(err) => println!("Failed to parse: {:?}", err),
        Ok(kv) => {
            println!("Pattern is {:?}!", kv);
            let extracted = kv.extract(Path::new("./data/foo/bar.txt"));
            println!("Extracted is {:?}!", extracted);
        }
    }
}
