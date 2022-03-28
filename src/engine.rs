use dasp::rms::Rms;
use dasp_ring_buffer as ring_buffer;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::sync::mpsc::{self, Receiver, Sender};

mod extensions;
use extensions::*;

// Find out how to dynamically import the "current mode" and switch on the fly.
// I think Eyesy actually just loads them all but only calls the current draw function.
// There might be a better way to do that.
use modes::example_mode as mode;

fn main() {
    nannou::app(model).update(update).run();
}

type SixteenChannels = [f32; 16];
type SampleRms = Rms<SixteenChannels, [SixteenChannels; 128]>;

struct Model {
    stream: audio::Stream<StreamParams>,
    receiver: Receiver<SixteenChannels>,
    rms: SampleRms,
    last_hit_at: SixteenChannels,
}

struct StreamParams {
    transmitter: Sender<SixteenChannels>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let rms_window = ring_buffer::Fixed::from([[0.0 as f32; 16]; 128]);
    let rms: SampleRms = Rms::new(rms_window);

    let (transmitter, receiver) = mpsc::channel();

    let audio_host = audio::Host::new();

    let capture_model = StreamParams { transmitter };

    // for device in audio_host.input_devices().unwrap() {
    //     dbg!(device.name().unwrap());
    // }

    let input_device = audio_host
        .input_devices()
        .unwrap()
        .find(|device| device.name().unwrap() == "BlackHole 16ch")
        .expect("Couldn't find specified audio device.");

    let stream = audio_host
        .new_input_stream(capture_model)
        .device(input_device)
        .capture(process_stream_samples)
        .build()
        .unwrap();

    stream.play().unwrap();

    let mode_model = mode.setup();

    Model {
        stream,
        receiver,
        rms,
        last_hit_at: [0.0; 16],
        mode_model,
    }
}

fn process_stream_samples(stream_params: &mut StreamParams, buffer: &Buffer) {
    for frame in buffer.frames() {
        dbg!(frame);

        let frame_array: SixteenChannels = frame.try_into().unwrap();
        stream_params.transmitter.send(frame_array).unwrap();
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    for frame in model.receiver.try_iter() {
        model.rms.next(frame);
    }

    for (index, sample) in model.rms.current().iter().enumerate() {
        if *sample > 0.15 {
            model.last_hit_at[index] = app.time;
        }
    }

    model.mode_model = mode.update(app, model.mode_model);
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            if model.stream.is_paused() {
                model.stream.play().unwrap();
            } else if model.stream.is_playing() {
                model.stream.pause().unwrap();
            }
        }
        // Eyesy controls include: trigger, 5 knobs, left and right through
        // modes, left and right through scenes, screenshot, and shift functions for all of it.
        // I think the engine implements all of this.
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let nannou_draw = app.draw();

    let seconds_since_last_hit: Vec<f32> = model
        .last_hit_at
        .iter()
        .map(|last_hit_at| app.time - last_hit_at)
        .collect();

    let rms = model.rms.current();

    let samples = model.receiver;

    fn audio_trigger(audio_channel: i32, animation: Function) {
        animation(seconds_since_last_hit[audio_channel]);
    }

    fn midi_trigger(audio_channel: i32, animation: Function) {
        animation(seconds_since_last_hit[audio_channel]);
    }

    fn meter(audio_channel: i32, animation: Function) {
        animation(rms[audio_channel]);
    }

    fn scope(audio_channel: i32, animation: Function) {
        animation(samples[audio_channel]);
    }

    Reyesy {
        draw: nannou_draw,
        window: app.window_rect(),
        model: model.mode_model,
        time: app.time(),
        meter,
        scope,
        audio_trigger,
        midi_trigger,
        // color_picker, knob values, other eyesy utilites, etc
    };

    mode.draw(Reyesy);

    nannou_draw.to_frame(app, &frame).unwrap();
}
