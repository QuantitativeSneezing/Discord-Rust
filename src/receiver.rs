use serenity::async_trait;
use songbird::{
    Event,
    EventContext,
    EventHandler as VoiceEventHandler,
    model::payload::{
        Speaking,
        ClientDisconnect,
    },
};

use crate::stt_model::SttModel;

pub struct Receiver;

impl Receiver {
    pub fn new() -> Self {
        // manage state
        // e.g. buffer of audio packet bytes to later store in intervals
        Self {}
    }
}

#[async_trait]
impl VoiceEventHandler for Receiver {
    #[allow(unused_variables)]
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        use EventContext as Ctx;
        match ctx {
            Ctx::SpeakingStateUpdate(
                Speaking {
                    speaking,
                    ssrc,
                    user_id,
                    ..
                }
            ) => {
                // Discord voice calls use RTP, where every sender uses a randomly allocated
                // *Synchronisation Source* (SSRC) to allow receivers to tell which audio
                // stream a received packet belongs to. As this number is not derived from
                // the sender's user_id, only Discord Voice Gateway messages like this one
                // inform us about which random SSRC a user has been allocated. Future voice
                // packets will contain *only* the SSRC.
                //
                // You can implement logic here so that you can differentiate users'
                // SSRCs and map the SSRC to the User ID and maintain this state.
                // Using this map, you can map the `ssrc` in `voice_packet`
                // to the user ID and handle their audio packets separately.
                // println!(
                //     "speaking state update: user ID {:?} with SSRC {:?} and speaking state {:?}",
                //     user_id,
                //     ssrc,
                //     speaking,
                // );
            },

            Ctx::SpeakingUpdate(data) => {
                // user start or stop speaking
                // map SSRC to user ID
                // println!(
                //     "SSRC {} {} to speak",
                //     data.ssrc,
                //     if data.speaking { "began" } else { "ceased" },
                // );
            },

            Ctx::VoicePacket(data) => {
                // each received audio packet
                // decoded data
                if let Some(audio) = data.audio {
                    // println!("audio packet first 5 samples: {:?}", audio.get(..5.min(audio.len())));
                    // println!(
                    //     "audio packet sequence {:05} has {:04} bytes (decompressed from {}), SSRC {}",
                    //     data.packet.sequence.0,
                    //     audio.len() * std::mem::size_of::<i16>(),
                    //     data.packet.payload.len(),
                    //     data.packet.ssrc,
                    // );

                    let samples = audio;
                    let mut stt_model = SttModel::new();
                    let model = stt_model.model();
                    let recognizer = stt_model.recognizer();
                    for sample in samples.chunks(100) {
                        recognizer.accept_waveform(sample);
                        println!("error: Speech-To-Text partial recognition {:#?}", recognizer.partial_result());
                    }

                    println!("STT: {:#?}", recognizer.final_result().multiple().expect("error: Speech-To-Text recognition"));
                } else {
                    println!("RTP packet has no audio, driver may not be configured for decoding");
                }
            },

            Ctx::RtcpPacket(data) => {
                // each received rtcp packet
                // call statistics and reporting information
                // println!("RTCP packet received: {:?}", data.packet);
            },

            Ctx::ClientDisconnect(
                ClientDisconnect {user_id, ..}
            ) => {
                // map user ID to SSRC
                // found when user speaks for the first time

                println!("user {:?} disconnected", user_id);
            },

            _ => {
                unimplemented!()
            }
        }

        None
    }
}
