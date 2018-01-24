mod kve;

use std::path::Path;

fn main() {
    match kve::KeyValueExtractor::new("%album%/%artist%-%title%") {
        Err(err) => println!("Failed to parse: {:?}", err),
        Ok(t) => {
            println!("//////////////////////////////////");

            println!("{:?}", t.extract(Path::new("songs/crap/Cannibal/Ke$ha-Crazy Beautiful Life.mp3")));
            println!("{:?}", t.extract(Path::new("songs/One Of The Boys/Katy Perry-I Kissed A Girl.mp3")));
            println!("{:?}", t.extract(Path::new("All I Ever Wanted/Kelly Clarkson-Long Shot.mp3")));
            println!("{:?}", t.extract(Path::new("music.mp3")));
            println!("{:?}", t.extract(Path::new("the las - There she goes again.mp3")));
        }
    }
}
