use crate::math::{matrix::Matrix, point::*};
use rand::prelude::*;

use super::info::Info;

pub struct AntColonyConfiguration {
    pub alpha: f64,
    pub beta: f64,
    pub pheromone_power: f64,
    pub path_size: usize,
    pub ant_quantity: usize,
    pub evaporation: f64,
    pub max_iter: usize,
}

pub struct Ant<'a> {
    weight_matrix: &'a Matrix,
    path_size: usize,
    pheromones: &'a Matrix,
    alpha: f64,
    beta: f64,
}

fn update_pheromones(
    pheromones: &Matrix,
    best: &(&Matrix, f64),
    paths: &Vec<Matrix>,
    config: &AntColonyConfiguration,
) -> Matrix {
    let mut path_weight: Matrix = Matrix::zeroes(config.path_size);
    let best_path: Matrix = best.0.mirror();

    for mat in paths {
        path_weight = path_weight + best_path.dot(&mat.mirror());
    }
    path_weight * config.pheromone_power / best.1 + (pheromones * (1.0 - config.evaporation))
}

impl<'a> Ant<'a> {
    fn new(
        pheromones: &'a Matrix,
        weight_matrix: &'a Matrix,
        config: &AntColonyConfiguration,
    ) -> Self {
        Ant {
            weight_matrix,
            path_size: config.path_size,
            pheromones,
            alpha: config.alpha,
            beta: config.beta,
        }
    }
    fn explore(&self) -> Matrix {
        let mut rng: ThreadRng = rand::thread_rng();
        let mut path: Matrix = Matrix::zeroes(self.path_size);
        let mut available_points: Vec<usize> = (0..self.path_size).collect();

        let random_index = rng.gen_range(0..available_points.len());
        let first_point: usize = available_points[random_index];
        let mut current_point = first_point;
        available_points.remove(random_index);

        while available_points.len() > 0 {
            let mut decision: f64 = rng.gen();
            let probability_vector =
                self.generate_probability_vector(current_point, &available_points);

            for index in 0..available_points.len() {
                decision -= probability_vector[index];
                if decision <= 0.0 {
                    let next_point: usize = available_points[index];
                    path[current_point][next_point] = 1.0;
                    current_point = next_point;
                    available_points.remove(index);
                    break;
                }
            }
        }
        path[current_point][first_point] = 1.0;
        path
    }
    fn generate_probability_vector(
        &self,
        current_index: usize,
        available_indexes: &Vec<usize>,
    ) -> Vec<f64> {
        let mut probability_vector = Vec::new();
        for index in available_indexes {
            probability_vector.push(
                self.pheromones[current_index][*index].powf(self.alpha)
                    * self.weight_matrix[current_index][*index].powf(self.beta),
            )
        }
        let sum: f64 = probability_vector.iter().sum();
        for prob in &mut probability_vector {
            *prob /= sum;
        }
        probability_vector
    }
}

pub fn execute(points: Vec<Point>, config: AntColonyConfiguration) -> Vec<(Matrix, f64)> {
    let info: Info = Info::new(points);
    let weight_matrix: Matrix = 1.0 / info.get_distance_matrix();
    let mut pheromones_matrix: Matrix = Matrix::random_gen(config.path_size).change_diagonal(0.0);

    let mut output: Vec<(Matrix, f64)> = Vec::new();

    for _ in 0..config.max_iter {
        let mut result_vector: Vec<Matrix> = Vec::new();
        for _ in 0..config.ant_quantity {
            let ant: Ant = Ant::new(&pheromones_matrix, &weight_matrix, &config);
            result_vector.push(ant.explore())
        }
        let best: (&Matrix, f64) = info.best_of_matrix(&result_vector);
        pheromones_matrix = update_pheromones(&pheromones_matrix, &best, &result_vector, &config);

        output.push((best.0.clone(), best.1))
    }

    output
}
