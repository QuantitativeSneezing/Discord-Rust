use vosk::{
    Model,
    Recognizer,
};

pub struct SttModel {
    model: Model,
    recognizer: Recognizer,
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
            model: model,
            recognizer: recognizer,
        };

        new_self
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn recognizer(&mut self) -> &mut Recognizer {
        &mut self.recognizer
    }
}
