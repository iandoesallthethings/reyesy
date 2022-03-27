use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::f64::consts::PI;
mod extensions;
use extensions::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    stream: audio::Stream<AudioParams>,
    center: Point2,
    circle_color: Hsl,
}

struct AudioParams {
    phase: f64,
    frequency: f64,
    center_frequency: f64,
}

fn model(app: &App) -> Model {
    app.new_window()
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();

    let audio_host = audio::Host::new();
    let audio_params = AudioParams {
        phase: 0.0,
        frequency: 440.0,
        center_frequency: 440.0,
    };
    let stream = audio_host
        .new_output_stream(audio_params)
        .render(fill_audio_buffer)
        .build()
        .unwrap();

    stream.play().unwrap();

    Model {
        stream,
        center: pt2(100.0, 100.0),
        circle_color: hsl(0.0, 0.0, 0.0),
    }
}

fn fill_audio_buffer(audio_params: &mut AudioParams, buffer: &mut Buffer) {
    let sample_rate = buffer.sample_rate() as f64;
    let volume = 0.5;

    for frame in buffer.frames_mut() {
        let sine_amp = audio_params.phase.times(2.0).times(PI).sin() as f32;
        audio_params.phase += audio_params.frequency / sample_rate;
        audio_params.phase %= sample_rate;
        for channel in frame {
            *channel = sine_amp * volume;
        }
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            if model.stream.is_playing() {
                model.stream.pause().unwrap();
            } else {
                model.stream.play().unwrap();
            }
        }

        Key::Up => {
            model
                .stream
                .send(|audio_params| {
                    audio_params.center_frequency += 10.0;
                })
                .unwrap();
        }

        Key::Down => {
            model
                .stream
                .send(|audio_params| {
                    audio_params.center_frequency -= 10.0;
                })
                .unwrap();
        }

        _ => {}
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let x = app.time.cos() * 100.0;
    let y = app.time.times(0.5).sin() * 100.0;
    let circle_color = hsl(app.time.times(0.2).tan(), 0.2, 0.5);

    let normalized_vibrato = app.time.times(40.0).cos().times(0.05) as f64;

    model.circle_color = circle_color;
    model.center = pt2(x, y);
    model
        .stream
        .send(move |audio_params| {
            audio_params.frequency = audio_params.center_frequency
                + normalized_vibrato.times(audio_params.center_frequency);
        })
        .unwrap();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let background_color = hsl(0.1, 0.3, 0.8);
    let circle_color = model.circle_color;

    draw.background().color(background_color);
    draw.ellipse().xy(model.center).color(circle_color);
    draw.to_frame(app, &frame).unwrap();
}
