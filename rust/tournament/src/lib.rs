use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Display, Formatter};
use std::str::FromStr;

enum MatchResult {
    Won,
    Drawn,
    Lost,
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Won => MatchResult::Lost,
            MatchResult::Drawn => MatchResult::Drawn,
            MatchResult::Lost => MatchResult::Won,
        }
    }
}

impl FromStr for MatchResult {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "win" => Ok(MatchResult::Won),
            "draw" => Ok(MatchResult::Drawn),
            "loss" => Ok(MatchResult::Lost),
            _ => Err(s.to_string()),
        }
    }
}

struct Match {
    team_a: String,
    team_b: String,
    result: MatchResult,
}

impl FromStr for Match {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<String> = s.split(';').map(|it| it.trim().to_string()).collect();
        let (team_a, team_b, result) = (
            parts.get(0).ok_or("Cannot Parse Team A")?.clone(),
            parts.get(1).ok_or("Cannot Parse Team B")?.clone(),
            parts
                .get(2)
                .ok_or("Cannot Parse Result")?
                .parse::<MatchResult>()?,
        );
        Ok(Self {
            team_a,
            team_b,
            result,
        })
    }
}

#[derive(Default, Clone, Eq, PartialEq)]
struct Team {
    name: String,
    won_count: u32,
    drawn_count: u32,
    lost_count: u32,
}

impl Team {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Self::default()
        }
    }

    fn record(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Won => self.won_count += 1,
            MatchResult::Drawn => self.drawn_count += 1,
            MatchResult::Lost => self.lost_count += 1,
        }
    }

    fn total_count(&self) -> u32 {
        self.won_count + self.drawn_count + self.lost_count
    }

    fn points(&self) -> u32 {
        3 * self.won_count + self.drawn_count
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        self.points()
            .cmp(&other.points())
            .then_with(|| self.name.cmp(&other.name).reverse())
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Team {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name,
            self.total_count(),
            self.won_count,
            self.drawn_count,
            self.lost_count,
            self.points()
        )
    }
}

struct ScoreBoard {
    teams: HashMap<String, Team>,
}

impl ScoreBoard {
    fn new() -> Self {
        Self {
            teams: HashMap::new(),
        }
    }

    fn record(&mut self, m: &Match) {
        self.teams
            .entry(m.team_a.clone())
            .or_insert(Team::new(&m.team_a))
            .record(&m.result);
        self.teams
            .entry(m.team_b.clone())
            .or_insert(Team::new(&m.team_b))
            .record(&m.result.reverse());
    }
}

impl<'a> IntoIterator for &'a ScoreBoard {
    type Item = &'a Team;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut team_list: Vec<_> = self.teams.values().collect();
        team_list.sort();
        team_list.reverse();
        team_list.into_iter()
    }
}

impl Display for ScoreBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut ret = "Team                           | MP |  W |  D |  L |  P".to_string();
        for team in self.into_iter() {
            ret = format!("{}\n{}", ret, team.to_string(),)
        }
        write!(f, "{}", ret)
    }
}

pub fn tally(match_results: &str) -> String {
    let mut score_board = ScoreBoard::new();
    match_results
        .split('\n')
        .map(|r| r.parse::<Match>())
        .filter_map(Result::ok)
        .for_each(|m| score_board.record(&m));
    score_board.to_string()
}
