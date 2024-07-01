use crate::middleware::gpu_explorations::utils::preprocess::{self, Point};
use ark_bn254::{Fr as ScalarField, G1Projective as G};
use ark_ec::Group;
use ark_ff::PrimeField;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use rayon::prelude::*;
use std::fs::File;
use std::time::{Duration, Instant};

// ref: https://github.com/ingonyama-zk/icicle/blob/de25b6e203df0ca70b71dcb77e19da156a8b9ff1/icicle/src/msm/msm.cu#L27C1-L36C6
fn left_shift_points(points: &mut [Point], shift: u32) {
    points.par_iter_mut().for_each(|point| {
        let mut shifted_point = G::from(*point);
        // 2^{shift} * P_i
        for _ in 0..shift {
            shifted_point = shifted_point.double();
        }
        *point = Point::from(shifted_point);
    });
}

// ref: https://github.com/ingonyama-zk/icicle/blob/de25b6e203df0ca70b71dcb77e19da156a8b9ff1/icicle/src/msm/msm.cu#L889C1-L913C4
pub fn precompute_msm_points(
    points: &[Point],       // original points
    precompute_factor: u32, // number of precomputed points
    c: u32,                 // window_size
) -> Result<Vec<Point>, preprocess::HarnessError> {
    let points_size = points.len();
    let shift = c;
    // generating an array composed of original and extended points (size: msm_size * precompute_factor)
    // pf = precompute_factor
    //  l = shift
    // [
    //                P_1,             P_2,  ...,             P_n,
    //         2^c  * P_1,      2^c  * P_2,  ...,      2^c  * P_n,
    //         2^2c * P_1,      2^2c * P_2,  ...,      2^2c * P_n,
    //                ...,             ...,                   ...,
    //    2^(w-1)c * P_1, 2^(w-1)l * P_2,  ..., 2^(w-1)l * P_n,
    // ]
    let mut output_points = Vec::with_capacity(points_size * precompute_factor as usize);
    output_points.extend_from_slice(points);

    // now we compute all the points: 2^{1c}*P_i..2^{(w-1)c}*P_i
    for i in 1..precompute_factor {
        let mut shifted_points = points.to_vec();
        left_shift_points(&mut shifted_points, shift * i);
        output_points.extend_from_slice(&shifted_points);
    }
    Ok(output_points)
}

pub fn precompute_points_from_instances<I>(
    instances: I,
    instance_size: u32,
    num_instance: u32,
    precompute_factor: u32,
    c: u32,
    output_dir: &str,
) -> Result<(), preprocess::HarnessError>
where
    I: Iterator<Item = preprocess::Instance>,
{
    println!("Generating precomputed points...");
    let mut total_duration = Duration::ZERO;
    let start = Instant::now();

    // precomputation for each instance
    for (i, instance) in instances.enumerate() {
        let points = &instance.0;
        let precomputed_points = precompute_msm_points(points, precompute_factor, c)?;
        serialize_precomputed_points(output_dir, &precomputed_points, i != 0)?;
        total_duration += start.elapsed();
    }
    println!("Precomputation Completed!");
    println!(
        "Precomputation time for {}x({} points) with precompute_factor={} is: {:?}",
        num_instance,
        1 << instance_size,
        precompute_factor,
        total_duration
    );
    Ok(())
}

pub fn serialize_precomputed_points(
    dir: &str,
    points: &[Point],
    append: bool,
) -> Result<(), preprocess::HarnessError> {
    // Check if dir exists
    std::fs::create_dir_all(dir).expect("Should create directory if it doesn't exist");
    let precomputed_points_path = format!("{}{}", dir, "/precomputed_points");

    let file = if append {
        File::options()
            .append(true)
            .create(true)
            .open(&precomputed_points_path)?
    } else {
        File::create(&precomputed_points_path)?
    };
    points.serialize_compressed(&file).unwrap();
    Ok(())
}

pub fn deserialize_precomputed_points(
    dir: &str,
) -> Result<Vec<Vec<Point>>, preprocess::HarnessError> {
    let mut precomputed_points = Vec::new();
    let precomputed_points_path = format!("{}{}", dir, "/precomputed_points");
    let file = File::open(precomputed_points_path)?;

    loop {
        let points = Vec::<Point>::deserialize_compressed_unchecked(&file);

        let points = match points {
            Ok(x) => x,
            _ => {
                break;
            }
        };
        precomputed_points.push(points);
    }

    Ok(precomputed_points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTANCE_SIZE: u32 = 16;
    const NUM_INSTANCE: u32 = 5;
    const UTILSPATH: &str = "mopro-core/src/middleware/gpu_explorations/utils/vectors";

    #[test]
    fn test_precompute_msm_points() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );
        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let instance_size = 1 << INSTANCE_SIZE;
        let instances = preprocess::FileInputIterator::open(&dir).unwrap();

        let precompute_factor = 2;
        let window_size = 4;

        for instance in instances {
            let points = instance.0;
            let precomputed_points = precompute_msm_points(&points, precompute_factor, window_size);

            match precomputed_points {
                Ok(precomputed_points) => {
                    assert_eq!(
                        precomputed_points.len(),
                        instance_size * precompute_factor as usize
                    );
                    let precomputed_points_slice = &precomputed_points[..instance_size as usize];
                    assert_eq!(points, precomputed_points_slice)
                }
                Err(_) => panic!("Function precompute_msm_points returned an error"),
            }
        }
    }

    #[test]
    fn test_precompute_points_from_instances() {
        let dir = format!(
            "{}/{}/{}x{}",
            preprocess::get_root_path(),
            UTILSPATH,
            INSTANCE_SIZE,
            NUM_INSTANCE
        );

        // Check if the vectors have been generated
        match preprocess::FileInputIterator::open(&dir) {
            Ok(_) => {
                println!("Vectors already generated");
            }
            Err(_) => {
                preprocess::gen_vectors(INSTANCE_SIZE, NUM_INSTANCE, &dir);
            }
        }

        let instance_size = 1 << INSTANCE_SIZE;
        let num_instance = NUM_INSTANCE;
        let instances = preprocess::FileInputIterator::open(&dir).unwrap();

        let precompute_factor = 2;
        let window_size = 4;
        let instances_vec: Vec<_> = instances.collect();

        let _ = precompute_points_from_instances(
            instances_vec.clone().into_iter(),
            instance_size,
            num_instance,
            precompute_factor,
            window_size,
            &dir,
        );

        // test for deserialization
        let precomputed_points = deserialize_precomputed_points(&dir).unwrap();
        assert_eq!(precomputed_points.len(), num_instance as usize);
        for (i, points) in precomputed_points.iter().enumerate() {
            assert_eq!(
                points.len(),
                instance_size as usize * precompute_factor as usize
            );
            let _points = &instances_vec[i].0;
            // check the original points are correct (in the first 0~instance_size of each extended points array)
            assert_eq!(&points[..instance_size as usize], _points);
        }
    }
}
