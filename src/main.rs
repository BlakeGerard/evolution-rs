
mod actor;

fn main() {

    // y = 1.0x + 1.0
    let data : Vec<(f32, f32)> = vec![(0.0, 1.0), (1.0, 2.0), (2.0, 3.0)];

    let actors = actor::initial_population(5);
    for actor in &actors {
	for pair in &data {
	    let response = actor.response(pair.0);
	    println!("Response: {} vs. {}", response, pair.1);
	}
    }
}
