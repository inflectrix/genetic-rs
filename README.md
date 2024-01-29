# genetic-rs
![Crates.io Total Downloads](https://img.shields.io/crates/d/genetic-rs)
![GitHub deployments](https://img.shields.io/github/deployments/HyperCodec/genetic-rs/testing)

A small crate for quickstarting genetic algorithm projects

### How to Use
First off, this crate comes with the `builtin` and `genrand` features by default. If you want to add the builtin crossover reproduction extension, you can do so by adding the `crossover` feature. If you want it to be parallelized, you can add the `rayon` feature.

Once you have eveything imported as you wish, you can define your entity and impl the required traits:

```rust
#[derive(Clone, Debug)] // clone is currently a required derive for pruning nextgens.
struct MyEntity {
    field1: f32,
}

// required in all of the builtin functions as requirements of `DivsionReproduction` and `CrossoverReproduction`
impl RandomlyMutable for MyEntity {
    fn mutate(&mut self, rate: f32, rng: &mut impl rand::Rng) {
        self.field1 += rng.gen::<f32>() * rate;
    }
}

// required for `division_pruning_nextgen`.
impl DivsionReproduction for MyEntity {
    fn spawn_child(&self, rng: &mut impl rand::Rng) -> Self {
        let mut child = self.clone();
        child.mutate(0.25, rng); // use a constant mutation rate when spawning children in pruning algorithms.
        child
    }
}

// required for the builtin pruning algorithms.
impl Prunable for MyEntity {
    fn despawn(self) {
        // unneccessary to implement this function, but it can be useful for debugging and cleaning up entities.
        println!("{:?} died", self);
    }
}

// helper trait that allows us to use `Vec::gen_random` for the initial population.
impl GenerateRandom for MyEntity {
    fn gen_random(rng: &mut impl rand::Rng) -> Self {
        Self { field1: rng.gen() }
    }
}
```

Once you have a struct, you must create your fitness function:
```rust
fn my_fitness_fn(ent: &MyEntity) -> f32 {
    // this just means that the algorithm will try to create as big a number as possible due to fitness being directly taken from the field.
    // in a more complex genetic algorithm, you will want to utilize `ent` to test them and generate a reward.
    ent.field1
}
```


Once you have your reward function, you can create a `GeneticSim` object to manage and control the evolutionary steps:

```rust
fn main() {
    let mut rng = rand::thread_rng();
    let mut sim = GeneticSim::new(
        // you must provide a random starting population. 
        // size will be preserved in builtin nextgen fns, but it is not required to keep a constant size if you were to build your own nextgen function.
        // in this case, you do not need to specify a type for `Vec::gen_random` because of the input of `my_fitness_fn`.
        Vec::gen_random(&mut rng, 100),
        my_fitness_fn,
        division_pruning_nextgen,
    );
 
    // perform evolution (100 gens)
    for _ in 0..100 {
        sim.next_generation(); // in a genetic algorithm with state, such as a physics simulation, you'd want to do things with `sim.entities` in between these calls
    }
 
    dbg!(sim.entities);
}
```

That is the minimal code for a working pruning-based genetic algorithm. You can [read the docs](https://docs.rs/genetic-rs) or [check the examples](/examples/) for more complicated systems.

### License
This project falls under the `MIT` license.