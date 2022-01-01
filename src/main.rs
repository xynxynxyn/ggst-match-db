#[macro_use]
extern crate lazy_static;
use std::collections::{HashMap, HashSet};

use ggst_api::*;
use tokio::time;
use bbt::*;

lazy_static! {
    static ref RATER: bbt::Rater = bbt::Rater::new(1.8);
}

struct Leaderboard {
    ratings: HashMap<Player, Rating>,
    matches: HashSet<Match>,
}

impl Leaderboard {
    fn new() -> Self {
        Leaderboard { ratings: HashMap::new(), matches: HashSet::new() }
    }
    fn get_rating(&mut self, id: Player) -> Rating {
        self.ratings.entry(id).or_insert(bbt::Rating::new(3000.0, 900.0)).to_owned()
    }

    fn update_rating(&mut self, id: &Player, new_rating: Rating) {
        *self.ratings.get_mut(id).unwrap() = new_rating;
    }

    fn print_top_n(&self, n: usize) {
        let mut players = self.ratings.iter().collect::<Vec<_>>();
        players.sort_by(|a, b| a.1.mu().partial_cmp(&b.1.mu()).unwrap());
        println!();
        println!("### TOP {} ###", n);
        for (i, (p, r)) in players.iter().rev().take(n).enumerate() {
            println!("#{:>4} {:<4.0}+-{:>3.0} {}", i + 1, r.mu(), r.sigma(), p);
        }
        println!();
    }
}

async fn update_database(db: &mut Leaderboard) -> error::Result<()> {
    println!("updating database... {} players, {} total matches", db.ratings.len(), db.matches.len());
    let replays = ggst_api::get_replays(&Context::default(), 30, Floor::Celestial, Floor::Celestial).await?;
    for r in replays.filter(|m| m.timestamp() < &chrono::Utc::now()) {
        if db.matches.contains(&r) { continue };
        // Insert replay into database and update rating for the player based on the character
        let winner_rating = db.get_rating(r.winner().clone());
        let loser_rating = db.get_rating(r.loser().clone());
        let (new_winner_rating, new_loser_rating) = RATER.duel(winner_rating, loser_rating, Outcome::Win);
        db.update_rating(r.winner(), new_winner_rating);
        db.update_rating(r.loser(), new_loser_rating);
        db.matches.insert(r);
    }
    Ok(())
}

/// Only tracking celestial matches
#[tokio::main]
async fn main() {
    // We update the database every 1 minutes
    let mut interval = time::interval(time::Duration::from_secs(60));
    let mut db = Leaderboard::new();
    // Open database
    loop {
        for _ in 0..60 {
        interval.tick().await;
        if let Err(e) = update_database(&mut db).await {
            eprintln!("{}", e);
        };

        // Print top 100 every hour
        db.print_top_n(100);
    };
}}
