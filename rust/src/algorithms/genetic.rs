use crate::math::point::*;
use rand::prelude::*;

use super::info::Info;

pub struct GeneticConfiguration {
    pub mutate_chance: f64,
    pub path_size: usize,
    pub max_iter: usize,
    pub pool_size: usize,
}

#[derive(Clone)]
pub struct Gene {
    data: Vec<usize>,
    path_size: usize,
    mutation_chance: f64,
    pub eval: f64,
}

impl Gene {
    fn new(config: &GeneticConfiguration) -> Self {
        Gene {
            path_size: config.path_size,
            data: {
                let mut vec: Vec<usize> = (0..config.path_size).collect();
                vec.shuffle(&mut thread_rng());
                vec
            },
            mutation_chance: config.mutate_chance,
            eval: 0.0,
        }
    }
    pub fn mutate(&mut self) -> () {
        let mut rng: ThreadRng = thread_rng();
        let random_number: f64 = rng.gen();

        if random_number < self.mutation_chance {
            return;
        }

        let i: usize = rng.gen_range(0..(self.data.len()));
        let mut j: usize = rng.gen_range(0..(self.data.len()) - 1);

        if i == j {
            j = self.data.len() - 1;
        }
        self.data.swap(i, j)
    }
    pub fn crossover(&self, other: &Gene) -> (Gene, Gene) {
        let crossover_point: usize = self.path_size / 2;

        let mut a = self.clone();
        let mut b = other.clone();
        a.data = vec![self.path_size; crossover_point];
        b.data = vec![self.path_size; crossover_point];

        for i in 0..crossover_point {
            a.data[i] = self.data[i];
            b.data[i] = other.data[i];
        }
        for i in 0..self.data.len() {
            if !a.data.contains(&other.data[i]) {
                a.data.push(other.data[i])
            }
            if !b.data.contains(&self.data[i]) {
                b.data.push(self.data[i])
            }
        }
        (a, b)
    }
    fn eval(&mut self, info: &Info) -> () {
        self.eval = info.evaluate_vector(&self.data)
    }
}

pub fn execute(points: Vec<Point>, config: GeneticConfiguration) -> Vec<(Vec<usize>, f64)> {
    let info: Info = Info::new(points);

    let mut output: Vec<(Vec<usize>, f64)> = Vec::new();
    let mut gene_pool: Vec<Gene> = Vec::new();
    for _ in 0..config.pool_size {
        gene_pool.push(Gene::new(&config))
    }
    for _ in 0..config.max_iter {
        for gene in &mut gene_pool {
            gene.eval(&info)
        }

        let best = info.best_of_gene(&gene_pool);

        output.push((best.data.clone(), best.eval));

        let total: f64 = gene_pool.iter().map(|gene| gene.eval).sum();

        let mut new_gene_pool: Vec<Gene> = Vec::new();
        for gene in &gene_pool {
            let n: usize = ((config.pool_size as f64) * gene.eval / total).round() as usize;
            for _ in 0..n {
                new_gene_pool.push({
                    let mut clone_gene = gene.clone();
                    clone_gene.eval = 0.0;
                    clone_gene
                })
            }
        }
        while new_gene_pool.len() < config.pool_size {
            new_gene_pool.push({
                let mut clone_gene = best.clone();
                clone_gene.eval = 0.0;
                clone_gene
            })
        }

        let mut indexes: Vec<usize> = (0..config.pool_size).collect();
        indexes.shuffle(&mut thread_rng());
        while indexes.len() > 1 {
            let i = *indexes.last().unwrap();
            indexes.pop();
            let j = *indexes.last().unwrap();
            indexes.pop();
            let (a, b) = new_gene_pool[i].crossover(&new_gene_pool[j]);
            new_gene_pool[i] = a;
            new_gene_pool[j] = b;
        }

        for gene in &mut new_gene_pool {
            gene.mutate()
        }

        gene_pool = new_gene_pool
    }
    output
}
