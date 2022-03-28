use dasp::rms::Rms;
use dasp_ring_buffer as ring_buffer;
use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::sync::mpsc::{self, Receiver, Sender};

mod extensions;
use extensions::*;

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

    Model {
        stream,
        receiver,
        rms,
        last_hit_at: [0.0; 16],
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
        _ => {}
    }
}

// audio_trigger_animation
fn render_snare_animation(draw: &Draw, seconds_since_last_hit: f32) {
    let saturation_slope = -1.0;
    let saturation_max = 1.0;
    let saturation = (saturation_slope * seconds_since_last_hit + saturation_max).max(0.0);

    let background_color = hsl(0.1, saturation, 0.5);
    draw.background().color(background_color);
}

// fn midi_trigger_animation() {}

// meter_animation
fn render_kick_animation(draw: &Draw, seconds_since_last_hit: f32) {
    let radius_slope = -100.0;
    let radius_max = 50.0;
    let radius = (radius_slope * seconds_since_last_hit + radius_max).max(0.0);

    let ellipse_color = hsl(0.8, 0.4, 2.0);
    draw.ellipse().radius(radius).color(ellipse_color);
}

// Scope - Eyesy simply gives you the last ~100 samples to display.
// Not sure if it does any smoothing or anything. Real scopes scan. Maybe we
// can model that? XY scopes would be cool, too - Plug in more than one signal, default to scan otherwise.
// fn scope_animation() {}

fn render_meter(app: &App, draw: &Draw, current_rms: f32) {
    let meter_color = hsl(0.2, 0.3, 0.2);
    let meter_height = current_rms;

    let container = app.window_rect();
    let bottom_left = pt2(-25.0, -container.h() / 2.0);
    let top_right = pt2(
        25.0,
        meter_height.denormalize(-container.h() / 2.0, container.h() / 2.0),
    );
    let meter_rect = Rect::from_corners(bottom_left, top_right);

    draw.rect()
        .xy(meter_rect.xy())
        .wh(meter_rect.wh())
        .color(meter_color);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let seconds_since_last_hit: Vec<f32> = model
        .last_hit_at
        .iter()
        .map(|last_hit_at| app.time - last_hit_at)
        .collect();

    render_kick_animation(&draw, seconds_since_last_hit[2]);
    render_snare_animation(&draw, seconds_since_last_hit[3]);
    render_meter(app, &draw, model.rms.current()[3]);

    draw.to_frame(app, &frame).unwrap();
}
