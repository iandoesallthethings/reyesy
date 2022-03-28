// This is the dream implementation if possible. It's the closest

pub fn setup() {
    // Return a partial ModeModel (Called Model to abstract the concept away from the dev)
    // import this in model() and append to actual Model
    // Might need an update function in the modes themselves to call in the engine's update()

    ModeModel {
			// Whatever data the user wants to track. 
			// Maybe custom timers or vectors of images and a current_image index
		}
}

pub fn update(mode_model: &ModeModel) {
    // Modify ModeModel however
    mode_model
}

pub fn draw(Reyesy: Reyesy) {
    let background_color = hsl(0.1, saturation, 0.5);
    Reyesy.draw.background().color(background_color);

    Reyesy.meter(3, render_meter);
    Reyesy.audio_trigger(4, render_kick_ellipse);
}

fn render_meter(current_rms: f32) {
    let meter_color = hsl(0.2, 0.3, 0.2);
    let meter_height = current_rms;
    let window = Eyesy.window;

    let bottom_left = pt2(-25.0, -window.h() / 2.0);
    let top_right = pt2(
        25.0,
        meter_height.denormalize(-window.h() / 2.0, window.h() / 2.0),
    );
    let meter_rect = Rect::from_corners(bottom_left, top_right);

    Reyesy
        .draw
        .rect()
        .xy(meter_rect.xy())
        .wh(meter_rect.wh())
        .color(meter_color);
}

fn render_kick_ellipse(seconds_since_last_hit: f32) {
    let radius_slope = -100.0;
    let radius_max = 50.0;
    let radius = (radius_slope * seconds_since_last_hit + radius_max).max(0.0);

    let ellipse_color = hsl(0.8, 0.4, 2.0);
    Reyesy.draw.ellipse().radius(radius).color(ellipse_color);
}
