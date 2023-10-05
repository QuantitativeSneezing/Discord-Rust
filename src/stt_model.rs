use std::{
    collections::VecDeque,
    ops::Not,
    sync::Mutex,
};

use rodio::{OutputStream, Source, buffer::SamplesBuffer};
use vosk::{
    Model,
    Recognizer,
};

lazy_static! {
    static ref SAMPLE_RATE: u32 = 48_000;
    static ref BUFFER_SIZE: usize = 524_288;
    static ref MODEL: Model = Model::new("./stt_model").expect("error: loading model");
    pub static ref RECOGNIZER: Mutex<Recognizer> = Recognizer::new(&MODEL, *SAMPLE_RATE as f32).expect("error: building recognizer").into();
    static ref BUFFER: Mutex<VecDeque<i16>> = VecDeque::with_capacity(*BUFFER_SIZE).into();
    pub static ref RESULTS: Mutex<String> = String::new().into();
}

pub fn recognize(samples: &Vec<i16>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let buf = SamplesBuffer::new(2, 48_000, samples.clone());
    stream_handle.play_raw(buf.convert_samples()).unwrap();

    let mut recognizer = RECOGNIZER.lock().expect("error: acquiring lock");
    // for sample in samples.chunks(100) {
    //     recognizer.accept_waveform(sample);
    //     // println!("partial voice recognition {:#?}", recognizer.partial_result());
    //     let partial_results = recognizer.partial_result();
    //     if !partial_results.partial.is_empty() {
    //         println!("partial voice recognition {:#?}", partial_results);
    //     }
    // }
    recognizer.accept_waveform(&samples);

    // println!("voice recognition: {:#?}", recognizer.final_result().multiple().expect("error: voice recognition"));
    let results = recognizer.final_result().multiple().expect("error: voice recognition");
    let s = results.alternatives.iter().filter_map(|result| result.text.is_empty().not().then(|| result.text)).collect::<Vec<&str>>().join(" | ");
    println!("voice recognition: {s}");
    let mut new_results = RESULTS.lock().expect("error: saving results");
    new_results.clear();
    new_results.push_str(&s);
}

pub fn push_samples(samples: &Vec<i16>) {
    let mut buffer = BUFFER.lock().expect("error: acquiring lock");
    // for sample in samples.drain(..) {
    //     if buffer.len() == buffer.capacity() {
    //         recognize(&buffer.drain(..).collect());
    //         buffer.push_back(sample);
    //     } else {
    //         buffer.push_back(sample);
    //     }
    // }
    buffer.extend(samples);
    if buffer.len() > *BUFFER_SIZE / 10_000 * 10_000 {
        recognize(&buffer.drain(..).collect());
    }
}
