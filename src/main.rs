extern crate keyvalueextractor;

use std::path::Path;

fn print(t: &keyvalueextractor::KeyValueExtractor, path: &str)
{
    let extracted = t.extract(Path::new(path));
    println!("{}", path);
    println!("{:?}", extracted);
    println!("");
}

fn main() {
    match keyvalueextractor::KeyValueExtractor::new("%album%/%artist%-%title%") {
        Err(err) => println!("Failed to parse: {:?}", err),
        Ok(t) => {
            println!("//////////////////////////////////");

            print(&t, "songs/crap/Cannibal/Ke$ha-Crazy Beautiful Life.mp3");
            print(&t, "songs/One Of The Boys/Katy Perry-I Kissed A Girl.mp3");
            print(&t, "All I Ever Wanted/Kelly Clarkson-Long Shot.mp3");
            print(&t, "music.mp3");
            print(&t, "the las - There she goes again.mp3");
        }
    }
}
