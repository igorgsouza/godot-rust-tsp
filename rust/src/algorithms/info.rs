use crate::math::matrix::*;
use crate::math::point::*;

use super::genetic::Gene;

pub struct Info {
    points: Vec<Point>,
    distance_matrix: Matrix,
}

impl Info {
    pub fn new(points: Vec<Point>) -> Self {
        let mut distance_matrix = Matrix::zeroes(points.len());

        for i in 0..points.len() {
            for j in i..points.len() {
                if i == j {
                    continue;
                }
                distance_matrix[i][j] = points[i].calculate_distance(&points[j]).unwrap();
            }
        }

        Self {
            points,
            distance_matrix: distance_matrix.transpose() + distance_matrix,
        }
    }

    pub fn get_distance_matrix(&self) -> &Matrix {
        &self.distance_matrix
    }

    pub fn evaluate_matrix(&self, path: &Matrix) -> f64 {
        path.dot(&self.distance_matrix).total()
    }

    pub fn evaluate_vector(&self, path: &Vec<usize>) -> f64 {
        let mut result = path
            .windows(2)
            .map(|i| {
                self.points[i[0]]
                    .calculate_distance(&self.points[i[1]])
                    .unwrap()
            })
            .sum();
        result += self.points[0]
            .calculate_distance(self.points.last().unwrap())
            .unwrap();
        result
    }

    pub fn best_of_matrix<'a>(&self, path_matrix: &'a Vec<Matrix>) -> (&'a Matrix, f64) {
        let mut best_path: &Matrix = &path_matrix[0];
        let mut best_evaluation: f64 = self.evaluate_matrix(best_path);

        for path in path_matrix.iter() {
            let evaluation: f64 = self.evaluate_matrix(&path);
            if evaluation < best_evaluation {
                best_path = path;
                best_evaluation = evaluation;
            }
        }
        (best_path, best_evaluation)
    }

    pub fn best_of_gene<'a>(&self, gene_vector: &'a Vec<Gene>) -> &'a Gene {
        let mut best_path = &gene_vector[0];
        let mut best_evaluation: f64 = best_path.eval;

        for path in gene_vector.iter() {
            let evaluation: f64 = path.eval;
            if evaluation < best_evaluation {
                best_path = path;
                best_evaluation = evaluation;
            }
        }
        best_path
    }
}
