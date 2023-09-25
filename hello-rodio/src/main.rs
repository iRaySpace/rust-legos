use rodio::source::{SineWave};
use rodio::{OutputStream, Sink};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    let sine_wave = SineWave::new(200.0);
    sink.append(sine_wave);
    sink.sleep_until_end();
}
