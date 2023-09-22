use soloud::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sl = Soloud::default()?;
    let mut wav = audio::Wav::default();

    // Create an empty playlist (vector of audio file paths)
    let mut playlist: Vec<&str> = Vec::new();

    // Add audio files to the playlist (replace with your audio file)
    playlist.push("audiofile.mp3");
    playlist.push("audiofile.mp3");
    playlist.push("audiofile.mp3");

    // Loop through the playlist and play each audio file
    for audio_file in playlist {
        println!("Playing {audio_file}");
        wav.load(&std::path::Path::new(audio_file))?;
    
        sl.play(&wav);
        while sl.voice_count() > 0 {
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }

    println!("Playlist end.");

    Ok(())
}
