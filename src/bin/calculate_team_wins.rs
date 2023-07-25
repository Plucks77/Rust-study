#[derive(Debug, Clone)]
struct Team {
    name: String,
    points: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    matches_played: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut teams: Vec<Team> = vec![];
    match_results
        .lines()
        .map(|line| line.split(";"))
        .map(|mut words| {
            let team_a = words.next().unwrap();
            let team_b = words.next().unwrap();
            let result = words.next().unwrap();
            (team_a, team_b, result)
        })
        .for_each(|(team_a, team_b, result)| {
            let mut team_a = find_or_create_team(&mut teams, team_a);
            let mut team_b = find_or_create_team(&mut teams, team_b);

            match result {
                "win" => {
                    team_a.wins += 1;
                    team_a.points += 3;
                    team_b.losses += 1;
                    team_a.matches_played += 1;
                    team_b.matches_played += 1;
                    teams.push(team_a);
                    teams.push(team_b);
                }
                "loss" => {
                    team_b.wins += 1;
                    team_b.points += 3;
                    team_a.losses += 1;
                    team_b.matches_played += 1;
                    team_a.matches_played += 1;
                    teams.push(team_b);
                    teams.push(team_a);
                }
                "draw" => {
                    team_a.draws += 1;
                    team_a.points += 1;
                    team_b.points += 1;
                    team_b.draws += 1;
                    team_a.matches_played += 1;
                    team_b.matches_played += 1;
                    teams.push(team_a);
                    teams.push(team_b);
                }
                _ => println!("Unknown result: {}", result),
            };
        });
    return calculate_leaderboard(&mut teams);
}

fn find_or_create_team(teams: &mut Vec<Team>, name: &str) -> Team {
    let team = teams
        .iter_mut()
        .enumerate()
        .find(|(_index, team)| team.name == name);
    match team {
        Some((index, _team)) => teams.remove(index),
        None => Team {
            name: name.to_string(),
            points: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            matches_played: 0,
        },
    }
}

fn calculate_leaderboard(teams: &mut Vec<Team>) -> String {
    teams.sort_by(|a, b| b.points.cmp(&a.points));
    let mut leaderboard = String::new();
    leaderboard += &format!(
        "{:30} | {} |  {} |  {} |  {} |  {}\n",
        "Team", "MP", "W", "D", "L", "P"
    );
    for team in teams {
        leaderboard += &format!(
            "{:30} |  {} |  {} |  {} |  {} |  {}\n",
            team.name, team.matches_played, team.wins, team.draws, team.losses, team.points
        );
    }
    leaderboard.pop();
    return leaderboard;
}

fn main() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;draw\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    // let input = "Allegoric Alaskans;Blithering Badgers;win
    // Devastating Donkeys;Courageous Californians;draw
    // Devastating Donkeys;Allegoric Alaskans;win
    // Courageous Californians;Blithering Badgers;loss
    // Blithering Badgers;Devastating Donkeys;loss
    // Allegoric Alaskans;Courageous Californians;win";
    let result = tally(input.as_str());
    println!("{result}");
}
#[test]
fn just_the_header_if_no_input() {
    let input = "";
    let expected = "Team                           | MP |  W |  D |  L |  P";
    assert_eq!(tally(input), expected);
}
#[test]

fn a_win_is_three_points_a_loss_is_zero_points() {
    let input = "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tally(input), expected);
}
#[test]

fn a_win_can_also_be_expressed_as_a_loss() {
    let input = "Blithering Badgers;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  1 |  1 |  0 |  0 |  3\n"
        + "Blithering Badgers             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tally(input), expected);
}
#[test]

fn a_different_team_can_win() {
    let input = "Blithering Badgers;Allegoric Alaskans;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Blithering Badgers             |  1 |  1 |  0 |  0 |  3\n"
        + "Allegoric Alaskans             |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tally(input), expected);
}
#[test]

fn there_can_be_more_than_one_match() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";
    assert_eq!(tally(&input), expected);
}
#[test]

fn a_draw_is_one_point_each() {
    let input = "Allegoric Alaskans;Blithering Badgers;draw\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  1 |  0 |  4\n"
        + "Blithering Badgers             |  2 |  0 |  1 |  1 |  1";
    assert_eq!(tally(&input), expected);
}
#[test]

fn there_can_be_more_than_one_winner() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  1 |  0 |  1 |  3\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3";
    assert_eq!(tally(&input), expected);
}
#[test]

fn there_can_be_more_than_two_teams() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Blithering Badgers;Courageous Californians;win\n"
        + "Courageous Californians;Allegoric Alaskans;loss";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  0 |  1 |  3\n"
        + "Courageous Californians        |  2 |  0 |  0 |  2 |  0";
    assert_eq!(tally(&input), expected);
}
#[test]

fn typical_input() {
    let input = "Allegoric Alaskans;Blithering Badgers;win\n".to_string()
        + "Devastating Donkeys;Courageous Californians;draw\n"
        + "Devastating Donkeys;Allegoric Alaskans;win\n"
        + "Courageous Californians;Blithering Badgers;loss\n"
        + "Blithering Badgers;Devastating Donkeys;loss\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Devastating Donkeys            |  3 |  2 |  1 |  0 |  7\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  3 |  1 |  0 |  2 |  3\n"
        + "Courageous Californians        |  3 |  0 |  1 |  2 |  1";
    assert_eq!(tally(&input), expected);
}
#[test]

fn incomplete_competition_not_all_pairs_have_played() {
    let input = "Allegoric Alaskans;Blithering Badgers;loss\n".to_string()
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;draw\n"
        + "Allegoric Alaskans;Courageous Californians;win";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  0 |  1 |  6\n"
        + "Blithering Badgers             |  2 |  1 |  1 |  0 |  4\n"
        + "Courageous Californians        |  2 |  0 |  1 |  1 |  1\n"
        + "Devastating Donkeys            |  1 |  0 |  0 |  1 |  0";
    assert_eq!(tally(&input), expected);
}
#[test]

fn ties_broken_alphabetically() {
    let input = "Courageous Californians;Devastating Donkeys;win\n".to_string()
        + "Allegoric Alaskans;Blithering Badgers;win\n"
        + "Devastating Donkeys;Allegoric Alaskans;loss\n"
        + "Courageous Californians;Blithering Badgers;win\n"
        + "Blithering Badgers;Devastating Donkeys;draw\n"
        + "Allegoric Alaskans;Courageous Californians;draw";
    let expected = "".to_string()
        + "Team                           | MP |  W |  D |  L |  P\n"
        + "Allegoric Alaskans             |  3 |  2 |  1 |  0 |  7\n"
        + "Courageous Californians        |  3 |  2 |  1 |  0 |  7\n"
        + "Blithering Badgers             |  3 |  0 |  1 |  2 |  1\n"
        + "Devastating Donkeys            |  3 |  0 |  1 |  2 |  1";
    assert_eq!(tally(&input), expected);
}
