use std::io::Write;
use std::{fs::File, io::BufWriter};

use crate::Result;

use crate::billiards::Table;
use crate::billiards::{HOLE_RADIUS, HOLE_VARIANTS};

pub fn output_snapshot(file: &mut File, table: &Table) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let balls = table.get_balls();

    // NOTE: Write the number of balls and holes
    let particles_count = balls.iter().flatten().count() + HOLE_VARIANTS.len();
    writeln!(writer, "{particles_count}")?;

    writeln!(
        writer,
        "Properties=pos:R:2:velo:R:2:radius:R:1:type:I:1 pbc=\"F F\"",
    )?;

    // NOTE: Write the balls
    for ball in balls.iter().flatten() {
        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.12} {}",
            ball.get_x(),
            ball.get_y(),
            ball.get_velocity_x(),
            ball.get_velocity_y(),
            ball.get_radius(),
            0
        )?;
    }

    // NOTE: Write the holes
    let holes = &HOLE_VARIANTS;
    for hole in holes.iter() {
        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.12} {}",
            hole.coordinates().0,
            hole.coordinates().1,
            0.0,
            0.0,
            HOLE_RADIUS,
            1
        )?;
    }
    Ok(())
}
