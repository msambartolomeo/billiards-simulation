use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::billiards::Table;
use crate::billiards::{HOLE_RADIUS, HOLE_VARIANTS};
use crate::Result;

const COLORS: [[f64; 3]; 8] = [
    [0.0, 0.0, 0.0], // Black
    [1.0, 1.0, 0.0], // Yellow
    [0.0, 0.0, 1.0], // Blue
    [1.0, 0.0, 0.0], // Red
    [1.0, 0.0, 1.0], // Purple
    [1.0, 0.5, 0.0], // Orange
    [0.0, 0.5, 0.0], // Green
    [0.5, 0.0, 0.0], // Maroon
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
            let color = if id == 0 {
                [1.0, 1.0, 1.0]
            } else {
                COLORS[id % COLORS.len()]
            };

            writeln!(
                writer,
                "{} {:.12} {:.12} {:.12} {:.12} {:.12} {} {} {}",
                id,
                ball.get_x(),
                ball.get_y(),
                ball.get_velocity_x(),
                ball.get_velocity_y(),
                ball.get_radius(),
                color[0],
                color[1],
                color[2],
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

pub fn output_event_times(path: &str, events_times_list: &[f64]) -> Result<()> {
    let mut output = File::create(path)?;

    writeln!(output, "{}", events_times_list.len())?;

    for time in events_times_list {
        writeln!(output, "{time}")?;
    }
    Ok(())
}
