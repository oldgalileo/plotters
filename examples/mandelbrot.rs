use plotters::prelude::*;
use std::ops::Range;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut backend = BitMapBackend::new("examples/outputs/mandelbrot.png", (800, 600));

    backend.open()?;
    let root: DrawingArea<_, _> = backend.into();
    root.fill(&White)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_ranged(-2.1f64..0.6f64, -1.2f64..1.2f64);

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let plotting_area = chart.plotting_area();

    let range = plotting_area.get_pixel_range();
    let (pw, ph) = (range.0.end - range.0.start, range.1.end - range.1.start);

    for (x, y, c) in mandelbrot_set(chart.x_range(), chart.y_range(), (pw as usize, ph as usize), 200) {
        if c != 200 {
            plotting_area
                .draw_pixel((x, y), &Palette99::pick(c / 10).mix((c % 10) as f64 / 10.0))?;
        } else {
            plotting_area.draw_pixel((x, y), &Black)?;
        }
    }

    root.close()?;
    return Ok(());
}

fn mandelbrot_set(
    real: Range<f64>,
    complex: Range<f64>,
    samples: (usize, usize),
    max_iter: usize,
) -> impl Iterator<Item = (f64, f64, usize)> {
    let step = (
        (real.end - real.start) / samples.0 as f64,
        (complex.end - complex.start) / samples.1 as f64,
    );
    return (0..(samples.0 * samples.1))
        .map(move |k| {
            (
                real.start + step.0 * (k % samples.0) as f64,
                complex.start + step.1 * (k / samples.0) as f64,
            )
        })
        .map(move |c| {
            let mut z = (0.0, 0.0);
            let mut cnt = 0;
            while cnt < max_iter && z.0 * z.0 + z.1 * z.1 <= 1e10 {
                z = (z.0 * z.0 - z.1 * z.1 + c.0, 2.0 * z.0 * z.1 + c.1);
                cnt += 1;
            }
            return (c.0, c.1, cnt);
        });
}