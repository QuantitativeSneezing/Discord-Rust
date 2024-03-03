# Rust
This is a small rust bot we built, both to get some experience with the language and to try out voice recognition using Vosk, a technology we both encountered while playing Phasmophobia. The discord end uses songbird/serenity for functionality. Vosk is interesting for this because it does support local models over using an API, making it convenient for lightweight use on one's computer.


# Installation:

# Usage:

# A note on VOSK models
Vosk uses a variety of offline models of varying sizes, with matching memory demands to run them. generally speaking, you can achieve *reasonably* accurate transcription using only the lightweight models, and if this is installed on a personal machine, for memory reasons that is probably ideal.


Note for jason:
Remember to use "RUSTFLAGS=-L/usr/lib/vosk cargo build" to compile
