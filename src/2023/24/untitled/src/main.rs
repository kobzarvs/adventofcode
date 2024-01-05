use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    line1: Line,
    line2: Line,
}

fn model(app: &App) -> Model {
    let line1 = Line::new(Vec3::new(-0.5, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0));
    let line2 = Line::new(Vec3::new(0.0, -0.5, 0.0), Vec3::new(0.0, 0.5, 0.0));

    Model { line1, line2 }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let draw = app.draw();

    draw.line().start(model.line1.start.into()).end(model.line1.end.into());
    draw.line().start(model.line2.start.into()).end(model.line2.end.into());
}

struct Line {
    start: Vec3,
    end: Vec3
}

impl Line {
    fn new(start: Vec3, end: Vec3) -> Self {
        Self { start, end }
    }
}
