use vosk::{
    Model,
    Recognizer,
};

pub struct SttModel {
    model: Option<Model>,
    recognizer: Option<Recognizer>,
}

impl SttModel {
    pub fn new() -> Self {
        let model_path = "./stt_model";

        let model = Model::new(model_path).expect("error: loading model");
        let mut recognizer = Recognizer::new(&model, 16000.0).unwrap();

        recognizer.set_max_alternatives(10);
        recognizer.set_words(true);
        recognizer.set_partial_words(true);

        let new_self = Self {
            model: Some(model),
            recognizer: Some(recognizer),
        };

        new_self
    }

    pub fn empty() -> Self {
        Self {
            model: None,
            recognizer: None,
        }
    }

    pub fn model(&self) -> Option<&Model> {
        self.model.as_ref()
    }

    pub fn recognizer_mut(&mut self) -> Option<&mut Recognizer> {
        self.recognizer.as_mut()
    }

    // pub fn recognize(&self, samples: &Vec<i16>) {
    //     // let model = stt.model().expect("error: getting voice recognition model");
    //     let recognizer = self.recognizer.as_mut().expect("error: getting voice recognizer");

    //     for sample in samples.chunks(100) {
    //         recognizer.accept_waveform(sample);
    //         println!("error: partial voice recognition {:#?}", recognizer.partial_result());
    //     }

    //     println!("voice recognition: {:#?}", recognizer.final_result().multiple().expect("error: voice recognition"));
    // }
}
