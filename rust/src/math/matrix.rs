use std::{
    ops::{Add, Div, Index, IndexMut, Mul},
    usize,
};

use rand::{rngs::ThreadRng, Rng};

#[derive(Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    size: usize,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        let size = data.len();
        Matrix { data, size }
    }

    pub fn zeroes(size: usize) -> Self {
        let data = vec![vec![0.0; size]; size];
        Matrix { data, size }
    }

    pub fn ones(size: usize) -> Self {
        let data = vec![vec![1.0; size]; size];
        Matrix { data, size }
    }

    pub fn random_gen(size: usize) -> Self {
        let mut rng: ThreadRng = rand::thread_rng(); // You can use any seed value here
        let data: Vec<Vec<f64>> = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| rng.gen()) // Generate random number between 0 and 1
                    .collect()
            })
            .collect();
        Matrix { data, size }
    }

    pub fn transpose(&self) -> Self {
        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[j][i] = self.data[i][j];
            }
        }
        Matrix::new(result)
    }

    pub fn change_diagonal(mut self, value: f64) -> Self {
        for i in 0..self.size {
            self.data[i][i] = value;
        }
        self
    }

    pub fn mirror(&self) -> Self {
        self.transpose() + self
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn dot(&self, other: &Matrix) -> Self {
        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] += self.data[i][j] * other.data[i][j];
            }
        }
        Matrix::new(result)
    }

    pub fn total(self) -> f64 {
        self.data.iter().map(|v| v.iter().sum::<f64>()).sum()
    }

    pub fn get_data(self) -> Vec<Vec<f64>> {
        self.data
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Matrix) -> Self::Output {
        if self.size != other.size {
            panic!("Matrices must be the same size to add");
        }

        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix::new(result)
    }
}

impl Add<&Matrix> for Matrix {
    type Output = Self;

    fn add(self, other: &Matrix) -> Self::Output {
        if self.size != other.size {
            panic!("Matrices must be the same size to add");
        }

        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Matrix::new(result)
    }
}

impl Mul<f64> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Matrix {
        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] * other;
            }
        }
        Matrix::new(result)
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Matrix {
        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] * other;
            }
        }
        Matrix::new(result)
    }
}

impl Div<f64> for Matrix {
    type Output = Matrix;

    fn div(self, other: f64) -> Matrix {
        let mut result = vec![vec![0.0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] / other;
            }
        }
        Matrix::new(result)
    }
}

impl Div<&Matrix> for f64 {
    type Output = Matrix;

    fn div(self, other: &Matrix) -> Matrix {
        let mut result = vec![vec![0.0; other.size]; other.size];
        for i in 0..other.size {
            for j in 0..other.size {
                result[i][j] = self / other.data[i][j];
            }
        }
        Matrix::new(result)
    }
}

impl Index<usize> for Matrix {
    type Output = Vec<f64>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
