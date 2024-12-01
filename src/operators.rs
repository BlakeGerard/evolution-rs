use crate::actor;
use rand::Rng;

fn RunTournament(src: &mut Vec<actor::Actor>, tournament_size: usize) -> usize {
    assert!(tournament_size >= 1);
    let num_actors = src.len();
    let mut winning_index = num_actors;
    for _ in 0..tournament_size {
        let index: usize = rand::thread_rng().gen_range(0..src.len() - 1);
        let actor = match src.get_mut(index) {
            Some(actor) => actor,
            None => panic!("oops"),
        };

        match actor.status() {
            actor::ActorStatus::Clear => actor.set_status(actor::ActorStatus::Tournament),
            actor::ActorStatus::Selected => {
                actor.set_status(actor::ActorStatus::TournamentSelected)
            }
            actor::ActorStatus::Tournament => continue,
            actor::ActorStatus::TournamentSelected => continue,
        }

        if winning_index == num_actors || actor.fitness() < src[winning_index].fitness() {
            winning_index = index;
        }
    }
    return winning_index;
}

pub fn TournamentSelection(
    src: &mut Vec<actor::Actor>,
    tournament_size: usize,
    quota: f32,
) -> Vec<actor::Actor> {
    let mut selected: Vec<actor::Actor> = Vec::new();

    // Compute the number of selected actors
    let num_actors = src.len();
    let mut num_selections: usize = ((num_actors as f32) * quota).ceil() as usize;
    num_selections = if num_selections % 2 == 0 {
        num_selections
    } else {
        num_selections + 1
    };
    assert!(num_selections <= num_actors);

    for actor in src.iter_mut() {
        actor.set_status(actor::ActorStatus::Clear);
    }

    let mut num_tournaments = 0;
    let mut times_winner_already_selected = 0;
    let mut num_selected = 0;
    while num_selected < num_selections {
        let winner_index = RunTournament(src, tournament_size);
        num_tournaments += 1;

        if *src[winner_index].status() == actor::ActorStatus::Tournament {
            println!("Made a selection at index {}", winner_index);
            src[winner_index].set_status(actor::ActorStatus::Selected);
            selected.push(src[winner_index].clone());
            num_selected += 1;
        } else {
            times_winner_already_selected += 1;
        }

        // TournamentSelectionUpdateStatuses
        for actor in src.iter_mut() {
            match actor.status() {
                actor::ActorStatus::Clear => continue,
                actor::ActorStatus::Selected => continue,
                actor::ActorStatus::Tournament => actor.set_status(actor::ActorStatus::Clear),
                actor::ActorStatus::TournamentSelected => {
                    actor.set_status(actor::ActorStatus::Selected)
                }
            }
        }
    }
    println!("Number of tournaments: {}", num_tournaments);
    println!(
        "Number of times winner already selected: {}",
        times_winner_already_selected
    );
    selected
}

fn MateActors(
    left_parent: &actor::Actor,
    right_parent: &actor::Actor,
    mutation_likelihood: u8,
) -> actor::Actor {
    let mut child = actor::Actor::new_clear();

    // Crossover
    child.set_m((left_parent.m() + right_parent.m()) / 2.0);
    child.set_b((left_parent.b() + right_parent.b()) / 2.0);

    // Mutation
    let roll: u8 = rand::thread_rng().gen_range(0..100);

    if roll < mutation_likelihood {
        let m_mutation = rand::thread_rng().gen_range(-2.0..2.0);
        child.set_m(child.m() + m_mutation);

        let b_mutation = rand::thread_rng().gen_range(-2.0..2.0);
        child.set_b(child.b() + b_mutation);
    }
    child
}

pub fn ReproductionFixedOffspringCount(
    src: &Vec<actor::Actor>,
    num_offspring: usize,
    mutation_likelihood: u8,
) -> Vec<actor::Actor> {
    let num_actors = src.len();
    // Only working with even actor counts or now
    assert!(num_actors % 2 == 0);
    let generation_len = num_actors + ((num_actors as f32 / 2.0).ceil() as usize) * num_offspring;

    let mut new_gen = Vec::new();
    let mut left = 0;
    let mut right = 1;
    while left < num_actors - 1 && right < num_actors {
        let left_parent = &src[left];
        let right_parent = &src[right];

        new_gen.push(left_parent.clone());
        new_gen.push(right_parent.clone());

        for _ in 0..num_offspring {
            let child = MateActors(&left_parent, &right_parent, mutation_likelihood);
            new_gen.push(child);
        }

        left += 2;
        right += 2;
    }
    new_gen
}
