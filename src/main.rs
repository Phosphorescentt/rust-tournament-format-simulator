mod models;
mod simulation;

fn main() {
    let t1 = models::Team {
        name: "Team 1".into(),
        measured_skill: 500,
        measurement_error_factor: 50.0,
    };

    let t2 = models::Team {
        name: "Team 2".into(),
        measured_skill: 500,
        measurement_error_factor: 50.0,
    };

    let m = models::Map {
        teams: vec![t1, t2],
    };

    let results = m.run().unwrap();
    println!("{:?}", results);
}
