use crate::math::{matrix::Matrix, point::Point};
use std::time::Instant;

use super::info::Info;

pub(crate) struct PermutationConfiguration {
    pub max_time: usize,
    pub start: Instant,
    pub path_size: usize,
    pub max_iter: usize,
}

pub fn eval_next(
    path: &Matrix,
    chosen: &Vec<usize>,
    index: usize,
    output: &mut Vec<(Matrix, f64)>,
    info: &Info,
    config: &PermutationConfiguration,
) -> () {
    let mut current_chosen: Vec<usize> = chosen.clone();
    current_chosen.push(index);
    if config.start.elapsed().as_millis() > config.max_time as u128 {
        return;
    }
    if output.len() > config.max_iter {
        return;
    }
    if current_chosen.len() == config.path_size {
        let mut current_path: Matrix = path.clone();
        current_path[index][0] = 1.0;
        output.push((current_path.clone(), info.evaluate_matrix(&current_path)))
    }
    for n in 0..config.path_size {
        if !current_chosen.contains(&n) {
            let mut current_path: Matrix = path.clone();
            current_path[index][n] = 1.0;
            eval_next(&current_path, &current_chosen, n, output, info, config)
        }
    }
}

pub fn execute(points: Vec<Point>, config: PermutationConfiguration) -> Vec<(Matrix, f64)> {
    let info: Info = Info::new(points);

    let mut output: Vec<(Matrix, f64)> = Vec::new();

    eval_next(
        &Matrix::zeroes(config.path_size),
        &Vec::new(),
        0,
        &mut output,
        &info,
        &config,
    );

    output
}
