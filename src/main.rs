mod actor;

const NUM_ITERS: usize = 100;
const NUM_ACTORS: usize = 5;

fn reproduction(src: &Vec<actor::Actor>, dst: &mut Vec<actor::Actor>) {}

fn selection(src: &Vec<actor::Actor>, dst: &mut Vec<actor::Actor>) {}

fn evaluation(actors: &mut Vec<actor::Actor>, data: &Vec<(f32, f32)>) {
    for actor in actors {
        actor.evaluate(&data);
    }
}

fn print_actors(actors: &Vec<actor::Actor>) {
    for actor in actors {
        println!("{}", actor);
    }
}

fn evolution(data: &Vec<(f32, f32)>) {
    let mut arena_A = actor::random_population(NUM_ACTORS);
    let mut arena_B = actor::clear_population(NUM_ACTORS);
    evaluation(&mut arena_A, data);
    print_actors(&arena_A);

    for _ in 0..NUM_ITERS {
        selection(&arena_A, &mut arena_B);
        reproduction(&arena_B, &mut arena_A);
        evaluation(&mut arena_A, data);
    }

    print_actors(&arena_A);
}

fn main() {
    // y = 2.0x + 0.0
    let data: Vec<(f32, f32)> = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 4.0), (3.0, 6.0), (4.0, 8.0)];

    let start = std::time::Instant::now();
    evolution(&data);
    let elapsed = start.elapsed();
    println!("Evolution time: {}\n", elapsed.as_micros());
}
