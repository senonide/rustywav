mod channels {
    pub mod pulse;
    pub mod triangle;
    pub mod noise;
}
mod utils {
    pub mod notes;
}

use std::error::Error;
use std::env;
use std::thread;
use std::time::Duration;
use rodio::{OutputStream, source::{Source, UniformSourceIterator}};
use csv::ReaderBuilder;
use utils::notes::get_note_frequency;
use channels::pulse::PulseWave;
use channels::triangle::TriangleWave;
use channels::noise::NoiseChannel;

type Melody = Vec<(String, u64)>;

fn read_csv_to_melodies(file_path: &str) -> Result<Vec<Melody>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(false).from_path(file_path)?;
    let mut melodies: Vec<Melody> = vec![vec![], vec![], vec![], vec![], vec![]];

    for record in reader.records() {
        let record = record?;
        for (i, value) in record.iter().enumerate() {
            if i % 2 == 0 {
                let note = value.trim().to_string();
                if !note.is_empty() {
                    melodies[i / 2].push((note, 0));
                }
            } else {
                if let Ok(duration) = value.trim().parse::<u64>() {
                    if let Some(last) = melodies[i / 2].last_mut() {
                        last.1 = duration;
                    }
                }
            }
        }
    }

    Ok(melodies)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <csv_file_path>", args[0]);
        std::process::exit(1);
    }

    let file_path = &args[1];
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let melodies = read_csv_to_melodies(file_path)?;

    let handles: Vec<_> = melodies.into_iter().enumerate().map(|(index, melody)| {
        let stream_handle = stream_handle.clone();
        thread::spawn(move || {
            for (note, duration_ms) in melody {
                if let Some(freq) = get_note_frequency(&note) {
                    let source: Box<dyn Source<Item = f32> + Send> = match index {
                        0 => Box::new(PulseWave::new(freq, 0.125)
                            .take_duration(Duration::from_millis(duration_ms))
                            .amplify(0.1)),
                        1 => Box::new(PulseWave::new(freq, 0.25)
                            .take_duration(Duration::from_millis(duration_ms))
                            .amplify(0.1)),
                        2 => Box::new(PulseWave::new(freq, 0.50)
                            .take_duration(Duration::from_millis(duration_ms))
                            .amplify(0.1)),
                        3 => Box::new(TriangleWave::new(freq)
                            .take_duration(Duration::from_millis(duration_ms))
                            .amplify(0.1)),
                        _ => Box::new(NoiseChannel::new(12345, false)
                            .take_duration(Duration::from_millis(duration_ms))
                            .amplify(0.1)),
                    };

                    let uniform_source = UniformSourceIterator::new(source, 2, 44100);
                    let _ = stream_handle.play_raw(uniform_source);
                    thread::sleep(Duration::from_millis(duration_ms));
                }
            }
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}
