mod maths;

use quill::prelude::*;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut before = Instant::now();
    let data = (0..=3000)
        .map(|k| {
            before = Instant::now();
            let n = maths::smallest_divisible::find(k);
            println!(
                "Time elapsed for k={} : {:.2?}. Total elapsed time: {:.2?}",
                k,
                before.elapsed(),
                start.elapsed()
            );
            (k, n)
        })
        .filter(|(_, len)| *len != -1)
        .collect();

    let before_plotting = Instant::now();
    let line_plot = Plot::builder()
        .dimensions((1980, 1600))
        .title("Distribution of the smallest repunit number (y) divisible by (x)")
        .x_label("divisor")
        .y_label("smallest repunit num")
        .legend(Legend::TopRightOutside)
        .grid(Grid::None)
        .data([Series::builder()
            .name("divisors")
            .color(Color::Purple)
            .data(data)
            .marker(Marker::Circle)
            .marker_size(9.0)
            .line(Line::None)
            .build()])
        .build();
    line_plot
        .to_svg("./smallest_repunit_divisible_distribution.svg")
        .unwrap();
    println!(
        "Time elapsed for plotting : {:.2?}. Total elapsed time: {:.2?}",
        before_plotting.elapsed(),
        start.elapsed()
    );
}
