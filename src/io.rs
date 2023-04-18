use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::billiards::Table;
use crate::billiards::{HOLE_RADIUS, HOLE_VARIANTS};
use crate::Result;

struct RGB {
    r: f64,
    g: f64,
    b: f64,
}

impl RGB {
    fn new(r: f64, g: f64, b: f64) -> Self {
        RGB { r, g, b }
    }
}

enum Color {
    White,
    Black,
    Yellow,
    Red,
    Green,
    Blue,
    Purple,
    Orange,
    Maroon,
}

impl Color {
    fn get_rgb(&self) -> RGB {
        match self {
            Color::White => RGB::new(1.0, 1.0, 1.0),
            Color::Black => RGB::new(0.0, 0.0, 0.0),
            Color::Yellow => RGB::new(1.0, 1.0, 0.0),
            Color::Red => RGB::new(1.0, 0.0, 0.0),
            Color::Green => RGB::new(0.0, 0.5, 0.0),
            Color::Blue => RGB::new(0.0, 0.0, 1.0),
            Color::Purple => RGB::new(1.0, 0.0, 1.0),
            Color::Orange => RGB::new(1.0, 0.5, 0.0),
            Color::Maroon => RGB::new(0.5, 0.0, 0.0),
        }
    }
}

const COLORS: [Color; 16] = [
    Color::White,
    Color::Yellow,
    Color::Blue,
    Color::Red,
    Color::Purple,
    Color::Black,
    Color::Orange,
    Color::Green,
    Color::Maroon,
    Color::Yellow,
    Color::Blue,
    Color::Red,
    Color::Purple,
    Color::Orange,
    Color::Green,
    Color::Maroon,
];

pub fn output_snapshot(file: &mut File, table: &Table) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let balls = table.get_balls();

    // NOTE: Write the number of balls and holes
    let particles_count = balls.iter().flatten().count() + HOLE_VARIANTS.len();
    writeln!(writer, "{particles_count}")?;

    writeln!(
        writer,
        "Properties=id:I:1:pos:R:2:velo:R:2:radius:R:1:color:R:3 pbc=\"F F\"",
    )?;

    // NOTE: Write the balls
    for (id, ball) in balls.iter().enumerate() {
        if let Some(ball) = ball {
            let color = COLORS[id].get_rgb();

            writeln!(
                writer,
                "{} {:.12} {:.12} {:.12} {:.12} {:.12} {} {} {}",
                id,
                ball.get_x(),
                ball.get_y(),
                ball.get_velocity_x(),
                ball.get_velocity_y(),
                ball.get_radius(),
                color.r,
                color.g,
                color.b,
            )?;
        }
    }
    // NOTE: Write the holes
    let holes = &HOLE_VARIANTS;
    for (idx, hole) in holes.iter().enumerate() {
        writeln!(
            writer,
            "{} {:.12} {:.12} {:.12} {:.12} {:.12} {} {} {}",
            idx + balls.len(),
            hole.coordinates().0,
            hole.coordinates().1,
            0.0,
            0.0,
            HOLE_RADIUS,
            1.0,
            1.0,
            1.0
        )?;
    }
    Ok(())
}

pub fn output_event_times(path: &str, events_times_list: &[(f32, usize)]) -> Result<()> {
    let mut output = File::create(path)?;

    writeln!(output, "{}", events_times_list.len())?;

    for (time, ball_count) in events_times_list {
        writeln!(output, "{time} {ball_count}")?;
    }
    Ok(())
}
