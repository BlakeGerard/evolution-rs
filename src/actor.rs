use std::fmt;

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Actor {
	m : f32,
	b : f32,
	fitness : f32,
	status : ActorStatus,
}

impl Actor {
    pub fn new() -> Self {
	Actor {
	    m : rand::random::<f32>(),
	    b : rand::random::<f32>(),
	    fitness : 0.0,
	    status : ActorStatus::Clear,
	}
    }

    pub fn response(&self, x : f32) -> f32 {
	self.m * x + self.b
    }
}

impl Default for Actor {
    fn default() -> Self {
	Actor::new()
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

pub fn initial_population(size : usize) -> Vec<Actor> {
    std::iter::repeat_with(Actor::default).take(size).collect()
}
