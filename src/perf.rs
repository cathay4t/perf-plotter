// SPDX-License-Identifier: Apache-2.0

use std::time::{Duration, Instant};

use plotters::prelude::*;

use crate::PerfPlotterError;

pub fn generate_performance_png<F, E, I>(
    func: F,
    mut input_iter: I,
    output: &std::path::Path,
    caption: &str,
) -> Result<(), PerfPlotterError>
where
    F: Fn(E) -> Option<()>,
    I: Iterator<Item = E>,
{
    let mut perf_data: Vec<Duration> = Vec::new();
    let mut i = 0usize;
    while let Some(input) = input_iter.next() {
        i += 1;
        let now = Instant::now();
        if func(input).is_some() {
            let elapsed = now.elapsed();
            println!("Round {}: {} secs", i, elapsed.as_secs_f32());
            perf_data.push(elapsed);
        } else {
            break;
        }
    }

    gen_png(&perf_data, output, caption)
}

fn gen_png(
    data: &[Duration],
    output: &std::path::Path,
    caption: &str,
) -> Result<(), PerfPlotterError> {
    let root = BitMapBackend::new(output, (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)
        .map_err(|e| PerfPlotterError::new(e.to_string()))?;
    if data.is_empty() {
        return Err(PerfPlotterError::new("Empty performance data".into()));
    }

    // Save to unwrap, we already checked the data is not empty;
    let min_dur = data.iter().min().unwrap();
    let max_dur = data.iter().max().unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption(caption, ("sans-serif", 50).into_font())
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(
            0.0f32..(data.len() as f32),
            min_dur.as_secs_f32()..max_dur.as_secs_f32(),
        )
        .map_err(|e| PerfPlotterError::new(e.to_string()))?;

    chart
        .configure_mesh()
        .x_desc("Round")
        .y_desc("Sec")
        .axis_desc_style(("sans-serif", 15))
        .draw()
        .map_err(|e| PerfPlotterError::new(e.to_string()))?;

    chart
        .draw_series(
            LineSeries::new(
                data.iter()
                    .enumerate()
                    .map(|(i, d)| (i as f32, d.as_secs_f32())),
                RED,
            )
            .point_size(2),
        )
        .map_err(|e| PerfPlotterError::new(e.to_string()))?;

    Ok(())
}
