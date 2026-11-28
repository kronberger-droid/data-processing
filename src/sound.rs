//! Sound in and out, the Rust side of `exercises/play.py` and
//! `exercises/record.py`.
//!
//! A signal is a slice of `f64` samples in `[-1, 1]`, one channel, [`RATE`]
//! samples per second. On its way to the sound card or into a WAV file it
//! becomes signed 16 bit little endian, the format of the Python templates.
//!
//! Playing and recording go through `aplay` and `arecord` from alsa-utils,
//! which the dev shell provides. Linking ALSA into the binary would bring in
//! a C library, and `scripts/bundle.nu` builds every binary static.
//!
//! ```no_run
//! use data_processing::sound;
//!
//! let tone: Vec<f64> = (0..sound::samples(1.0))
//!     .map(|i| (std::f64::consts::TAU * 440.0 * i as f64 / sound::RATE as f64).sin())
//!     .collect();
//! let heard = sound::play_and_record(&tone)?;
//! sound::write_wav("a440.wav", &heard)?;
//! # Ok::<(), std::io::Error>(())
//! ```

use std::{
    fs,
    io::{self, Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

#[cfg(test)]
mod tests;

/// Samples per second. Frequencies up to half of it can be represented.
pub const RATE: u32 = 22050;

/// How long `play_and_record` keeps listening after the last sample went to
/// the sound card, for the card's own delay and for the system to ring out.
const TAIL: Duration = Duration::from_millis(500);

/// Number of samples that make up `seconds` of signal.
pub fn samples(seconds: f64) -> usize {
    (seconds * RATE as f64).round() as usize
}

/// Plays `signal` on the default sound card and returns once it is through.
pub fn play(signal: &[f64]) -> io::Result<()> {
    let mut player = start(pcm("aplay").stdin(Stdio::piped()))?;
    // Dropping stdin closes it, which is what tells aplay the signal is over.
    let sent = player
        .stdin
        .take()
        .expect("stdin is piped")
        .write_all(&encode(signal));
    let status = player.wait()?;
    if !status.success() {
        return Err(io::Error::other(format!("aplay failed, {status}")));
    }
    sent
}

/// Records `seconds` from the default sound card.
pub fn record(seconds: f64) -> io::Result<Vec<f64>> {
    let mut recorder = start(pcm("arecord").stdout(Stdio::piped()))?;
    let mut bytes = vec![0; 2 * samples(seconds)];
    let read = recorder
        .stdout
        .take()
        .expect("stdout is piped")
        .read_exact(&mut bytes);
    // arecord would go on for ever, the count read above is what ends it.
    stop(&mut recorder)?;
    read.map_err(|e| io::Error::new(e.kind(), format!("arecord stopped early: {e}")))?;
    Ok(decode(&bytes))
}

/// Plays `signal` and returns what the microphone heard meanwhile, which is
/// the response of whatever sits between loudspeaker and microphone. The
/// recording starts shortly before the signal and outlasts it by half a
/// second, so it is a little longer than `signal`.
pub fn play_and_record(signal: &[f64]) -> io::Result<Vec<f64>> {
    let mut recorder = start(pcm("arecord").stdout(Stdio::piped()))?;
    let mut heard = recorder.stdout.take().expect("stdout is piped");

    // The first sample shows the sound card is open. Playing before that
    // would lose the start of the signal.
    let mut first = [0; 2];
    if let Err(e) = heard.read_exact(&mut first) {
        stop(&mut recorder)?;
        return Err(io::Error::new(
            e.kind(),
            format!("arecord stopped early: {e}"),
        ));
    }
    // The pipe holds less than two seconds, so it is drained while playing.
    let rest = thread::spawn(move || {
        let mut bytes = first.to_vec();
        heard.read_to_end(&mut bytes).map(|_| bytes)
    });

    let played = play(signal);
    if played.is_ok() {
        thread::sleep(TAIL);
    }
    stop(&mut recorder)?;
    let bytes = rest.join().expect("the reader does not panic")?;
    played?;
    Ok(decode(&bytes))
}

/// Writes `signal` as a mono 16 bit WAV file at [`RATE`].
pub fn write_wav(path: impl AsRef<Path>, signal: &[f64]) -> io::Result<()> {
    let data = encode(signal);
    let mut wav = Vec::with_capacity(44 + data.len());
    wav.extend(b"RIFF");
    wav.extend((36 + data.len() as u32).to_le_bytes());
    wav.extend(b"WAVE");
    wav.extend(b"fmt ");
    wav.extend(16u32.to_le_bytes()); // size of this chunk
    wav.extend(1u16.to_le_bytes()); // PCM
    wav.extend(1u16.to_le_bytes()); // channels
    wav.extend(RATE.to_le_bytes());
    wav.extend((2 * RATE).to_le_bytes()); // bytes per second
    wav.extend(2u16.to_le_bytes()); // bytes per sample
    wav.extend(16u16.to_le_bytes()); // bits per sample
    wav.extend(b"data");
    wav.extend((data.len() as u32).to_le_bytes());
    wav.extend(data);
    fs::write(path, wav)
}

/// Reads a mono 16 bit WAV file at [`RATE`], as `write_wav` and
/// `arecord -f S16_LE -c 1 -r 22050` write them. Any other format is an
/// error, since a different rate would shift every frequency read off it.
pub fn read_wav(path: impl AsRef<Path>) -> io::Result<Vec<f64>> {
    let path = path.as_ref();
    let invalid = |why: String| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("{}: {why}", path.display()),
        )
    };
    let wav = fs::read(path)?;
    if wav.len() < 12 || &wav[..4] != b"RIFF" || &wav[8..12] != b"WAVE" {
        return Err(invalid("not a WAV file".into()));
    }

    let u16_at = |bytes: &[u8], i: usize| u16::from_le_bytes([bytes[i], bytes[i + 1]]);
    let u32_at = |bytes: &[u8], i: usize| {
        u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]])
    };

    let mut format_seen = false;
    let mut rest = &wav[12..];
    while rest.len() >= 8 {
        let size = u32_at(rest, 4) as usize;
        // A WAV that was written to a pipe claims more data than it has.
        let body = &rest[8..rest.len().min(8 + size)];
        match &rest[..4] {
            b"fmt " if body.len() >= 16 => {
                let got = (
                    u16_at(body, 0),
                    u16_at(body, 2),
                    u32_at(body, 4),
                    u16_at(body, 14),
                );
                if got != (1, 1, RATE, 16) {
                    let (format, channels, rate, bits) = got;
                    return Err(invalid(format!(
                        "want mono 16 bit PCM at {RATE} Hz, got format {format}, \
                         {channels} channels, {bits} bit at {rate} Hz"
                    )));
                }
                format_seen = true;
            }
            b"data" if format_seen => return Ok(decode(body)),
            b"data" => return Err(invalid("data before the format".into())),
            _ => {}
        }
        // Chunks are padded to an even length.
        rest = &rest[rest.len().min(8 + size + size % 2)..];
    }
    Err(invalid("no samples in it".into()))
}

/// `aplay` or `arecord` set up for raw samples in the format of this module.
fn pcm(program: &str) -> Command {
    let mut command = Command::new(program);
    command
        .args(["-q", "-t", "raw", "-f", "S16_LE", "-c", "1", "-r"])
        .arg(RATE.to_string());
    command
}

fn start(command: &mut Command) -> io::Result<Child> {
    command.spawn().map_err(|e| {
        let program = command.get_program().to_string_lossy();
        io::Error::new(
            e.kind(),
            format!("{program}: {e}, is alsa-utils on the PATH?"),
        )
    })
}

/// Ends a recorder. It may be gone already, which is not an error here.
fn stop(recorder: &mut Child) -> io::Result<()> {
    let _ = recorder.kill();
    recorder.wait().map(drop)
}

/// Samples as signed 16 bit little endian. Values outside `[-1, 1)` clip.
fn encode(signal: &[f64]) -> Vec<u8> {
    signal
        .iter()
        .flat_map(|&x| ((x * 32768.0).round() as i16).to_le_bytes())
        .collect()
}

/// The reverse of `encode`. A trailing odd byte is dropped.
fn decode(bytes: &[u8]) -> Vec<f64> {
    bytes
        .chunks_exact(2)
        .map(|b| i16::from_le_bytes([b[0], b[1]]) as f64 / 32768.0)
        .collect()
}
