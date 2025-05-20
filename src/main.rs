use clap::Parser;
use std::{error::Error, io, io::Write, process::Command};

mod bandcamp;

#[derive(Parser, Debug)]
#[command(name = "make-ghost-album")]
#[command(about = "Create \"ghost album\" (1s empty audio with correct metadata) representing album not on streaming", long_about = None)]
struct Args {
    /// Bandcamp album URL
    #[arg(short = 'b', long = "bandcamp")]
    bandcamp_link: Option<String>,
    //TODO last.fm? rym?
}

pub struct Album {
    album_name: String,
    artist_name: String,
    album_url: String,
}

impl Album {
    pub fn new() -> Album {
        Album {
            album_name: String::new(),
            artist_name: String::new(),
            album_url: String::new(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let album;

    if let Some(link) = args.bandcamp_link {
        album = bandcamp::scrape_album(&link)
            .await
            .expect("Bandcamp link should be valid.");
    } else {
        album = manual_import();
    }

    run_commands(album);

    Ok(())
}

fn run_commands(album: Album) {
    let mp3_file = make_file_name(&album.album_name, &album.artist_name);
    let temp_cover = mp3_file.clone() + ".jpg";

    let mut _res = Command::new("curl")
        .arg("-L")
        .arg("-o")
        .arg(&temp_cover)
        .arg(album.album_url)
        .output()
        .expect("error downloading image");

    _res = Command::new("ffmpeg")
        .arg("-f")
        .arg("lavfi")
        .arg("-t")
        .arg("1")
        .arg("-i")
        .arg("anullsrc=r=44100:cl=stereo")
        .arg("-i")
        .arg(&temp_cover)
        .arg("-map")
        .arg("0:a")
        .arg("-map")
        .arg("1")
        .arg("-id3v2_version")
        .arg("3")
        .arg("-metadata")
        .arg(format!("title={}", album.album_name))
        .arg("-metadata")
        .arg(format!("artist={}", album.artist_name))
        .arg("-metadata")
        .arg(format!("album={}", album.album_name))
        .arg("-disposition:v:0")
        .arg("attached_pic")
        .arg("-c:a")
        .arg("libmp3lame")
        .arg("-b:a")
        .arg("192k")
        .arg("-c:v")
        .arg("mjpeg")
        .arg(mp3_file)
        .output()
        .expect("error creating mp3");

    _res = Command::new("rm")
        .arg(&temp_cover)
        .output()
        .expect("error removing cover");

    println!(
        "Generated \"{0} - {1}\"!",
        album.album_name, album.artist_name
    );
}

fn manual_import() -> Album {
    let mut album_url = String::new();
    let mut album_name = String::new();
    let mut artist_name = String::new();

    print!("Enter album name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut album_name)
        .expect("error: unable to read user input");

    print!("Enter artist name: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut artist_name)
        .expect("error: unable to read user input");

    print!("Enter album cover URL: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut album_url)
        .expect("error: unable to read user input");

    trim_newline(&mut album_url);
    trim_newline(&mut album_name);
    trim_newline(&mut artist_name);

    Album {
        album_name,
        artist_name,
        album_url,
    }
}

fn make_file_name(album_name: &String, artist_name: &String) -> String {
    let mut ret: String = format!("{album_name}_{artist_name}.mp3");

    ret = ret
        .chars()
        .filter(|c| *c != ' ')
        .filter(|c| *c != '/') // Breaks with filepath stuff
        .collect();

    return ret;
}

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}
