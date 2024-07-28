mod cli;
use cli::{
    enable_logging, init_ort_environment, process_synthesis_request,
    Cli, SynthesisMode, SynthesisRequest
};

use sonata_piper::PiperSynthesisConfig;
use sonata_synth::{
    AudioOutputConfig, AudioSamples, SonataModel, SonataResult, SonataSpeechSynthesizer
};

use std::path::PathBuf;
use std::io;

const VOICE_CONFIG_PATH: &str = "US.onnx.json";

fn main()
{
    enable_logging();
    init_ort_environment();

    let args = Cli{
        config: PathBuf::from(VOICE_CONFIG_PATH),
        input_file: None,
        output_file: None,
        // mode: Some(SynthesisMode::Realtime),
        mode: None,
        speaker_id: None,
        length_scale: None,
        noise_scale: None,
        noise_w: None,
        rate: None,
        pitch: None,
        volume: None,
        silence: None,
        chunk_size: None,
        chunk_padding: None,
    };

    let synth = {
        let voice = sonata_piper::from_config_path(&args.config)
            .expect("Could not find config");
        SonataSpeechSynthesizer::new(voice)
            .expect("Could not create synth")
    };

    let default_synth_config: PiperSynthesisConfig = *synth
        .get_default_synthesis_config()
        .expect("Could not get default synth config")
        .downcast()
        .expect("Invalid default synthesis config. Expected Piper config.");

    loop
    {
        let lines = io::stdin().lines();
        for line in lines
        {
            let input = line.unwrap();

            if input.is_empty()
            {
                continue;
            }

            let req = SynthesisRequest {
                text: input,
                // mode: Some(SynthesisMode::Realtime),
                // TODO: req and args seem redundant
                mode: None,
                speaker_id: None,
                length_scale: None,
                noise_scale: None,
                noise_w: None,
                rate: None,
                volume: None,
                pitch: None,
                appended_silence_ms: None,
                chunk_size: None,
                chunk_padding: None
            };

            // NOTE: for some reason doesn't work with small inputs
            process_synthesis_request(&args, &synth, &default_synth_config, req)
                .expect("Synthesis failed");
        }
    }
}
