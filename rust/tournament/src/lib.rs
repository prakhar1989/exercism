use std::cmp::Ordering;
use std::collections::HashMap;

const HEADER: &'static str = "Team                           | MP |  W |  D |  L |  P";

enum GameResult {
    WIN,
    DRAW,
    LOSS,
}

struct Stats {
    team: String,
    wins: u32,
    draws: u32,
    losses: u32,
}

impl Stats {
    pub fn new(team: &str) -> Self {
        Stats {
            team: team.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    pub fn add_win(&self) -> Self {
        Stats {
            team: self.team.to_string(),
            wins: self.wins + 1,
            draws: self.draws,
            losses: self.losses,
        }
    }
    pub fn add_loss(&self) -> Self {
        Stats {
            team: self.team.to_string(),
            losses: self.losses + 1,
            draws: self.draws,
            wins: self.wins,
        }
    }
    pub fn add_draw(&self) -> Self {
        Stats {
            team: self.team.to_string(),
            draws: self.draws + 1,
            losses: self.losses,
            wins: self.wins,
        }
    }

    pub fn matches_played(&self) -> u32 {
        self.draws + self.losses + self.wins
    }

    pub fn total_points(&self) -> u32 {
        self.draws + self.wins * 3
    }
}

impl std::fmt::Display for Stats {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.team,
            self.matches_played(),
            self.wins,
            self.draws,
            self.losses,
            self.total_points(),
        )
    }
}

struct Score {
    team1: String,
    team2: String,
    result: GameResult,
}

pub fn tally(match_results: &str) -> String {
    let scores: Result<Vec<Score>, _> = match_results
        .split("\n")
        .map(|s| parse_input_line(s))
        .collect();

    let leaderboard = cumulative(&scores.unwrap_or_else(|_| Vec::new()));

    let mut sorted: Vec<&Stats> = leaderboard.values().collect::<Vec<_>>();
    sorted.sort_by(|a, b| match b.total_points().cmp(&a.total_points()) {
        Ordering::Equal => a.team.cmp(&b.team),
        i => i,
    });

    vec![HEADER.to_string()]
        .into_iter()
        .chain(sorted.iter().map(|stat| stat.to_string()))
        .collect::<Vec<String>>()
        .join("\n")
}

fn cumulative(scores: &[Score]) -> HashMap<String, Stats> {
    let mut h = HashMap::new();

    for score in scores {
        let t1 = &score.team1;
        let t2 = &score.team2;
        let t1_stats = h.remove(t1).unwrap_or_else(|| Stats::new(t1));
        let t2_stats = h.remove(t2).unwrap_or_else(|| Stats::new(t2));

        let (t1_stats, t2_stats) = match score.result {
            GameResult::WIN => (t1_stats.add_win(), t2_stats.add_loss()),
            GameResult::LOSS => (t1_stats.add_loss(), t2_stats.add_win()),
            GameResult::DRAW => (t1_stats.add_draw(), t2_stats.add_draw()),
        };
        h.insert(t1.to_string(), t1_stats);
        h.insert(t2.to_string(), t2_stats);
    }

    h
}

impl Score {
    pub fn new(t1: &str, t2: &str, result: GameResult) -> Self {
        Score {
            team1: t1.to_string(),
            team2: t2.to_string(),
            result,
        }
    }
}

/// Parse an input line
fn parse_input_line(input: &str) -> Result<Score, ()> {
    let parts: Vec<_> = input.split(";").collect();
    if parts.len() == 3 {
        let status = match parts[2] {
            "win" => GameResult::WIN,
            "loss" => GameResult::LOSS,
            "draw" => GameResult::DRAW,
            _ => unreachable!(),
        };
        return Ok(Score::new(parts[0], parts[1], status));
    }
    Err(())
}
