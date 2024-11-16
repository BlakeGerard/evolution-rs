use std::fmt;

enum ActorStatus {
    Clear = 0,
    Tournament = 1,
    Selected = 2,
    TournamentSelected = 3,
}

impl fmt::Display for ActorStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ActorStatus::Clear => write!(f, "Clear"),
            ActorStatus::Tournament => write!(f, "Tournament"),
            ActorStatus::Selected => write!(f, "Selected"),
            ActorStatus::TournamentSelected => write!(f, "TournamentSelected"),
        }
    }
}

pub struct Actor {
    m: f32,
    b: f32,
    fitness: f32,
    status: ActorStatus,
}

impl Actor {
    pub fn new_random() -> Self {
        Actor {
            m: rand::random::<f32>(),
            b: rand::random::<f32>(),
            fitness: 0.0,
            status: ActorStatus::Clear,
        }
    }

    pub fn new_clear() -> Self {
        Actor {
            m: 0.0,
            b: 0.0,
            fitness: 0.0,
            status: ActorStatus::Clear,
        }
    }

    pub fn evaluate(&mut self, data: &Vec<(f32, f32)>) {
        self.fitness = 0.0;
        for sample in data {
            let response = self.m * sample.0 + self.b;
            // Squared error loss
            let error = sample.1 - response;
            let sqerror = error * error;
            self.fitness += sqerror;
        }
    }
}

impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actor {{\n")?;
        write!(f, "  m : {}\n", self.m)?;
        write!(f, "  b : {}\n", self.b)?;
        write!(f, "  fitness : {}\n", self.fitness)?;
        write!(f, "  status : {}\n", self.status)?;
        write!(f, "}}\n")
    }
}

pub fn random_population(size: usize) -> Vec<Actor> {
    std::iter::repeat_with(Actor::new_random)
        .take(size)
        .collect()
}

pub fn clear_population(size: usize) -> Vec<Actor> {
    std::iter::repeat_with(Actor::new_clear)
        .take(size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn random_population_is_randomly_initialized() {
        let pop = random_population(2);
        for actor in pop {
            let m_in_range = actor.m >= 0.0 && actor.m <= 1.0;
            let b_in_range = actor.b >= 0.0 && actor.b <= 1.0;
            assert_eq!(m_in_range && b_in_range, true);
            assert_eq!(actor.fitness, 0.0);
            assert!(matches!(actor.status, ActorStatus::Clear));
        }
    }

    #[test]
    fn clear_population_is_zero_initialized() {
        let pop = clear_population(2);
        for actor in pop {
            assert_eq!(actor.m, 0.0);
            assert_eq!(actor.b, 0.0);
            assert_eq!(actor.fitness, 0.0);
            assert!(matches!(actor.status, ActorStatus::Clear));
        }
    }

    #[test]
    fn actor_evaluate_squared_error_loss_correct() {
        let data: Vec<(f32, f32)> = vec![(0.0, 0.0), (1.0, 2.0), (2.0, 4.0)];
        let mut a = Actor {
            m: 2.0,
            b: 0.0,
            fitness: 0.0,
            status: ActorStatus::Clear,
        };

        let expected = 0.0;
        a.evaluate(&data);

        assert_eq!(a.fitness, expected);
    }
}
