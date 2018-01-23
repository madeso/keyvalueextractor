mod kve;

use std::path::Path;

fn main() {
    match kve::KeyValueExtractor::new("%album%/%artist%-%title%") {
        Err(err) => println!("Failed to parse: {:?}", err),
        Ok(kv) => {
            kv.extract(Path::new("./foo/bar.txt"));
            println!("It is {:?}!", kv);
        }
    }
}
