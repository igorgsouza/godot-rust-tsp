use std::time::Instant;

use crate::algorithms::*;
use crate::math::matrix::Matrix;
use crate::math::point::*;
use godot::classes::Object;
use godot::global::push_warning;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Object, init)]
struct EvolutionManager {
    base: Base<Object>,
}

#[godot_api]
impl EvolutionManager {
    #[signal]
    fn iteration_results(path: PackedInt64Array);

    #[func]
    fn ant_colony(&mut self, points: PackedVector2Array, config: Dictionary) -> Array<Dictionary> {
        let mut point_arr: Vec<Point> = Vec::new();
        for vector in points.as_slice() {
            point_arr.push(Point::from_vector2(&vector))
        }

        push_warning(&["starting".to_variant()]);
        let best_result: Vec<(Matrix, f64)> = ant_colony::execute(
            point_arr,
            ant_colony::AntColonyConfiguration {
                alpha: config.get_or_nil("alpha").try_to().unwrap_or_else(|_| 0.5),
                beta: config.get_or_nil("beta").try_to().unwrap_or_else(|_| 0.5),
                pheromone_power: config
                    .get_or_nil("pheromone_power")
                    .try_to()
                    .unwrap_or_else(|_| 1.0),
                ant_quantity: config
                    .get_or_nil("ant_quantity")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 10) as usize,
                max_iter: config
                    .get_or_nil("max_iter")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 100) as usize,
                path_size: points.len(),
                evaporation: config
                    .get_or_nil("evaporation")
                    .try_to()
                    .unwrap_or_else(|_| 0.5),
            },
        );
        push_warning(&["finished".to_variant()]);

        Array::from_iter(best_result.iter().map(
            |(path, eval)| dict! {"path":path.to_packed_int64_array(), "eval": eval.to_godot()},
        ))
    }

    #[func]
    fn genetic(&mut self, points: PackedVector2Array, config: Dictionary) -> Array<Dictionary> {
        let mut point_arr: Vec<Point> = Vec::new();
        for vector in points.as_slice() {
            point_arr.push(Point::from_vector2(&vector))
        }

        push_warning(&["starting".to_variant()]);
        let best_result: Vec<(Vec<usize>, f64)> = genetic::execute(
            point_arr,
            genetic::GeneticConfiguration {
                pool_size: config
                    .get_or_nil("pool_size")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 10) as usize,
                max_iter: config
                    .get_or_nil("max_iter")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 100) as usize,
                path_size: points.len(),
                mutate_chance: config
                    .get_or_nil("mutate_chance")
                    .try_to()
                    .unwrap_or_else(|_| 0.5),
            },
        );
        push_warning(&["finished".to_variant()]);

        Array::from_iter(
            best_result
                .iter()
                .map(|(path, eval)| dict! {"path": PackedInt64Array::from_iter(path.iter().map(|x| *x as i64)), "eval": eval.to_godot()} ),
        )
    }

    #[func]
    fn permutation(&mut self, points: PackedVector2Array, config: Dictionary) -> Array<Dictionary> {
        let mut point_arr: Vec<Point> = Vec::new();
        for vector in points.as_slice() {
            point_arr.push(Point::from_vector2(&vector))
        }

        godot_print!("funciona ai vai");
        push_warning(&["starting".to_variant()]);
        let best_result: Vec<(Matrix, f64)> = permutation::execute(
            point_arr,
            permutation::PermutationConfiguration {
                max_time: config
                    .get_or_nil("max_time")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 10000) as usize,
                path_size: points.len(),
                start: Instant::now(),
                max_iter: config
                    .get_or_nil("max_iter")
                    .try_to::<i64>()
                    .unwrap_or_else(|_| 100) as usize,
            },
        );
        push_warning(&["finished".to_variant()]);

        Array::from_iter(best_result.iter().map(
            |(path, eval)| dict! {"path":path.to_packed_int64_array(), "eval": eval.to_godot()},
        ))
    }
}

impl Point {
    pub fn from_vector2(vector: &Vector2) -> Point {
        Point::new(vector.x as i32, vector.y as i32)
    }
}

impl Matrix {
    pub fn to_packed_int64_array(&self) -> PackedInt64Array {
        let mut path_vector: Vec<usize> = Vec::new();
        let mut current_line = 0;
        while path_vector.len() < self.get_size() {
            let index: usize = self[current_line].iter().position(|&x| x > 0.0).unwrap();
            path_vector.push(index);
            current_line = index;
        }
        PackedInt64Array::from_iter(path_vector.iter().map(|x| *x as i64))
    }
}
