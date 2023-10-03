use std::{
    collections::VecDeque,
    sync::Mutex,
};

use vosk::{
    Model,
    Recognizer,
};

lazy_static! {
    static ref MODEL: Model = Model::new("./stt_model").expect("error: loading model");
    pub static ref RECOGNIZER: Mutex<Recognizer> = Recognizer::new(&MODEL, 16000.).expect("error: building recognizer").into();
    static ref BUFFER: Mutex<VecDeque<i16>> = VecDeque::new().into();
}

pub fn recognize(samples: &Vec<i16>) {
    let mut recognizer = RECOGNIZER.lock().expect("error: acquiring lock");
    // for sample in samples.chunks(100) {
    //     recognizer.accept_waveform(sample);
    //     println!("partial voice recognition {:#?}", recognizer.partial_result());
    // }
    recognizer.accept_waveform(&samples);
    let partial_results = recognizer.partial_result();
    if !partial_results.partial.is_empty() {
        println!("partial voice recognition {:#?}", partial_results);
    }

    // println!("voice recognition: {:#?}", recognizer.final_result().multiple().expect("error: voice recognition"));
    let results = recognizer.final_result().multiple().expect("error: voice recognition");
    if !results.alternatives.first().unwrap().text.is_empty() {
        println!("voice recognition: {:#?}", results);
    }
}

pub fn push_samples(samples: &Vec<i16>) {
    let mut buffer = BUFFER.lock().expect("error: acquiring lock");
    let mut samples = samples.clone();
    // for sample in samples.drain(..) {
    //     if buffer.len() == buffer.capacity() {
    //         recognize(&buffer.drain(..).collect());
    //         buffer.push_back(sample);
    //     } else {
    //         buffer.push_back(sample);
    //     }
    // }
    buffer.extend(samples.drain(..));
    if buffer.len() == 32_768 {
        recognize(&buffer.drain(..).collect());
        buffer.clear();
    }
}
