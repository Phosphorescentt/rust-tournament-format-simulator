use rand::prelude::*;
use rand_distr::{Normal, NormalError};

// trait Fixture {
//     fn run(self) -> Result<Vec<Team>, NormalError>;
// }

#[derive(Clone, Debug)]
pub struct Team {
    pub name: String,
    pub measured_skill: i32,
    pub measurement_error_factor: f32,
}

#[derive(Debug)]
pub struct Map {
    pub teams: Vec<Team>,
}

// impl Fixture for Map {
impl Map {
    pub fn run(self) -> Result<Vec<Team>, NormalError> {
        if self.teams.len() > 2 {
            panic!("Incorrect number of teams.")
        } else {
            let team1 = self.teams[0].clone();
            let team2 = self.teams[1].clone();

            let mut rng = thread_rng();
            let normal = Normal::new(0.0, 1.0)?;
            let delta = team1.measurement_error_factor * normal.sample(&mut rng);
            let team1_true_skill = (team1.measured_skill as f32) + delta;

            let delta = team2.measurement_error_factor * normal.sample(&mut rng);
            let team2_true_skill = (team2.measured_skill as f32) + delta;

            let skill_delta = team1_true_skill - team2_true_skill;
            let prob_team1_win = 1.0 - (1.0 / (1.0 + (10 as f32).powf(skill_delta / 400.0)));

            if rng.gen_range(0.0..1.0) < prob_team1_win {
                return Ok(vec![team1, team2]);
            } else {
                return Ok(vec![team2, team1]);
            }
        }
    }
}
