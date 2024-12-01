mod actor;
mod operators;

const NUM_ITERS: usize = 1;
const NUM_ACTORS: usize = 5;
const TOURNAMENT_SIZE: usize = 2;
const SELECTION_QUOTA: f32 = 0.5;
const REPRODUCTION_FIXED_OFFSPRING_COUNT: usize = 2;
const MUTATION_LIKELIHOOD: u8 = 50;

fn print_actors(actors: &Vec<actor::Actor>) {
    for actor in actors {
        println!("{}", actor);
    }
}

fn reproduction(src: &Vec<actor::Actor>) -> Vec<actor::Actor> {
    operators::ReproductionFixedOffspringCount(
        src,
        REPRODUCTION_FIXED_OFFSPRING_COUNT,
        MUTATION_LIKELIHOOD,
    )
}

fn selection(src: &mut Vec<actor::Actor>) -> Vec<actor::Actor> {
    operators::TournamentSelection(src, TOURNAMENT_SIZE, SELECTION_QUOTA)
}

fn evaluation(actors: &mut Vec<actor::Actor>, data: &Vec<(f32, f32)>) {
    for actor in actors {
        actor.evaluate(&data);
    }
}

fn evolution(data: &Vec<(f32, f32)>) {
    let mut arena_A = actor::random_population(NUM_ACTORS);
    evaluation(&mut arena_A, data);
    print_actors(&arena_A);

    for _ in 0..NUM_ITERS {
        let selected = selection(&mut arena_A);
        let mut next_gen = reproduction(&selected);
        evaluation(&mut next_gen, data);
        print_actors(&next_gen);
    }
}

fn main() {
    // y = 2.0x + 0.0
    let data: Vec<(f32, f32)> = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0)];

    let start = std::time::Instant::now();
    evolution(&data);
    let elapsed = start.elapsed();
    println!("Evolution time: {}\n", elapsed.as_micros());
}
