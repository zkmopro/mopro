#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub instance_size: u32,
    pub num_instance: u32,
    pub avg_processing_time: f64,
}
