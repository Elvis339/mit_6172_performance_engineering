#[macro_export]
macro_rules! time_it {
    ($label:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let duration = start.elapsed();
        println!("{}: {} seconds", $label, duration.as_secs_f64());
        result
    }};
}