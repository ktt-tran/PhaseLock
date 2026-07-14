use std::{
    fs::File,
    path::Path,
};

use zeroize::{Zeroize, ZeroizeOnDrop};

use symphonia::{
    core::{
        audio::{AudioBuffer, Signal},
        io::MediaSourceStream,
        probe::Hint,
    },
    default::{get_codecs, get_probe},
};

// zeroize key when out of scope.
#[derive(Zeroize, ZeroizeOnDrop)]

pub struct DecodedAudio {
    signal: Vec<f32>,
    pub sample_rate: u32,
}

impl DecodedAudio {
    pub fn get_signal(&self) -> &[f32] {
        &self.signal
    }
}

pub fn decode_audio_key<P: AsRef<Path>>(
    path: P,
) -> Result<DecodedAudio, Box<dyn std::error::Error>> {

    let file = File::open(&path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(extension) = path.as_ref().extension().and_then(|e| e.to_str()) {
        hint.with_extension(extension);
    }

    let probed = get_probe().format(
        &hint,
        mss,
        &Default::default(),
        &Default::default(),
    )?;

    let mut format_reader = probed.format;
    let track = format_reader.default_track().ok_or("No supported audio track found")?;
    let track_id = track.id;
    let sample_rate = track.codec_params.sample_rate.ok_or("Audio file has no sample rate")?;

    let mut decoder = get_codecs().make(&track.codec_params, &Default::default())?;

    let mut pcm_signal = Vec::new();

    while let Ok(packet) = format_reader.next_packet() {

        if packet.track_id() != track_id {
            continue;
        }

        let decoded = decoder.decode(&packet)?;
        
        let mut f32_buf = AudioBuffer::<f32>::new(decoded.capacity() as u64, decoded.spec().clone());
        decoded.convert(&mut f32_buf);

        let channels = f32_buf.spec().channels.count();

        // Convert all channels to mono.
        for frame in 0..f32_buf.frames() {
            let mut sum = 0.0;

            for channel in 0..channels {
                sum += f32_buf.chan(channel)[frame];
            }

            pcm_signal.push(sum / channels as f32);
        }
    }

    Ok(DecodedAudio {signal: pcm_signal, sample_rate} )

}