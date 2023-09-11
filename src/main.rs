use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
mod utils {
    pub mod constants;
    pub mod helper;
}
use crate::utils::helper::{american_to_percent, percent_to_american, get_bet_name, get_consensus_odds, calc_ev};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};
fn main() {
    let mut odds: File = File::open("example_odds.json").expect("Failed to find the lines.");
    let mut json_string: String = String::new();
    odds.read_to_string(&mut json_string).expect("Failed to read the line");

    // Deserialize the JSON string into a OddsData struct
    let odds_data: OddsData = serde_json::from_str(&json_string).expect("Failed to parse JSON");

    // Get the list of events from the OddsData struct
    let event_list: Vec<EventData> = odds_data.data;

    // Sort the bets by type (over/under/spread)
    let sorted_bets: Vec<BetTypes> = sort_bets(&event_list);
    
    // Find the bets that are +EV
    let plus_ev_bets: Vec<&BetOptions> = find_ev_bets(&sorted_bets);

    // Print the +EV bets
    for ev_bet in plus_ev_bets {
        let ev: f64 = calc_ev(ev_bet);
        println!(
            "\n\n{:.2}% EV at {:?} for {} at {:.0} consensous of {:.0}",
            ev,
            ev_bet.best_price.books,
            ev_bet.bet_name,
            percent_to_american(&ev_bet.best_price.price),
            percent_to_american(&ev_bet.consensus_odds.unwrap()),
        );
    }
}

fn sort_bets(event_list: &Vec<EventData>) -> Vec<BetTypes> {
    let mut sorted_bets: Vec<BetTypes> = Vec::new();
    for event in event_list {
        let mut diff_bets: HashMap<String, BetTypes> = HashMap::new();

        for odd in &event.odds {
            // get the bet name, which changes depending on the type of bet
            let bet_name: String = match &odd.bet_points {
                Some(bet_points) => {
                    if odd.market_name.contains("Spread") || odd.market_name.contains("Handicap") {
                        format!(
                            "{} {}",
                            bet_points,
                            &odd.market_name
                        )
                    } else {
                        format!(
                            "{} {} {}",
                            get_bet_name(&odd.name),
                            bet_points,
                            &odd.market_name
                        )
                    }
            },
                None => format!(
                    "{} vs. {} {}", 
                    &event.home_team, 
                    &event.away_team, 
                    &odd.market_name
                ),
            };

            // convert the american odds to a percent so it is easier to compare
            let bet_price = american_to_percent(&odd.price);

            // should eventually make this recursive so i dont have to write 2 match cases for the same thing
            match diff_bets.entry(bet_name.clone()) {
                Occupied(mut entry) => {
                    // check to see if the type (over/under/spread) is in the hash
                    let bet_types = entry.get_mut();

                    // see if the bet we are at now is a better deal than the 
                    match bet_types.hash.entry(odd.name.clone()) {
                        Occupied(mut bet_types_entry) => {
                           let bet_options = bet_types_entry.get_mut();
                           if bet_options.best_price.price > bet_price {
                                bet_options.best_price = BestBooks {
                                    price: bet_price, 
                                    books: vec![odd.sports_book_name.clone()]
                                };
                           } else if bet_options.best_price.price == bet_price {
                                bet_options.best_price.books.push(odd.sports_book_name.clone())
                           }
                           bet_options.all_books.push(
                                BookInfo { 
                                    price: bet_price,
                                    name: odd.sports_book_name.clone()
                                }
                           )
                           
                        }

                        // if the type of bet is not in the hash, add it
                        Vacant(bet_types_entry) => {
                            bet_types_entry.insert(BetOptions {
                                best_price: BestBooks { 
                                    price: bet_price, 
                                    books: vec![odd.sports_book_name.clone()]
                                },
                                all_books: vec![BookInfo {
                                    price: bet_price,
                                    name: odd.sports_book_name.clone()
                                }],
                                consensus_odds: None,
                                fair_odds: None,
                                bet_name: format!("{} {}", &odd.name, &odd.market_name),
                            });
                        }
                    }
                }

                // if the bet name is not in the hash, add it
                Vacant(entry) => {
                    // Create a new BetTypes entry and initialize the inner hashmap (hash) with the BetOptions entry
                    let mut init_hash: HashMap<String, BetOptions> = HashMap::new();

                    init_hash.insert(odd.name.clone(), BetOptions {
                        best_price: BestBooks { 
                            price: bet_price, 
                            books: vec![odd.sports_book_name.clone()]
                        },
                        all_books: vec![BookInfo {
                            price: bet_price,
                            name: odd.sports_book_name.clone(),
                        }],
                        consensus_odds: None,
                        fair_odds: None,
                        bet_name: format!("{} {}", &odd.name, &odd.market_name),

                    });
                    entry.insert(BetTypes { hash: init_hash });
                }
            }
        }
        // re work maybe later?
        for (_, mut bet_types) in diff_bets {
            for bet_type in bet_types.hash.values_mut() {
                bet_type.consensus_odds = Some(get_consensus_odds(&bet_type.all_books));
            }
        
            let sum_of_odds: f64 = bet_types
                .hash
                .values()
                .into_iter()
                .map(|bet| bet.consensus_odds.unwrap())
                .sum::<f64>();

            if bet_types.hash.len() == 1 {
                continue;
            };
        
            for bet_type in bet_types.hash.values_mut() {
                bet_type.fair_odds = Some(bet_type.consensus_odds.unwrap() / sum_of_odds);
            }
            sorted_bets.push(bet_types);
        }
    }
    sorted_bets
}

fn find_ev_bets(sorted_bets: &Vec<BetTypes>) -> Vec<&BetOptions> {
    let mut plus_ev_bets: Vec<&BetOptions> = Vec::new();

    for bet_types in sorted_bets {
        for bet in bet_types.hash.values() {
            // need a better way to organize spread bets
            if bet.fair_odds.unwrap() > bet.consensus_odds.unwrap() {
                continue;
            }
            if bet.best_price.price < bet.fair_odds.unwrap() {
                plus_ev_bets.push(bet)
            }
        }
    }
    plus_ev_bets
}
#[derive(Debug, Deserialize, Serialize)]
struct OddsData {
    data: Vec<EventData>
}

#[derive(Debug, Deserialize, Serialize)]
struct EventData {
    id: String,
    sport: String,
    league: String,
    home_team: String,
    away_team: String,
    is_live: bool,
    is_popular: Option<bool>,
    tournament: Option<String>,
    status: Option<String>,
    odds: Vec<BetData>
}

#[derive(Debug, Deserialize, Serialize)]
struct BetData {
    id: String,
    sports_book_name: String,
    name: String,
    price: f64,
    checked_date: String,
    timestamp: f64,
    bet_points: Option<f64>,
    is_main: bool,
    is_live: bool,
    market_name: String,
    home_rotation_number: Option<i32>,
    away_rotation_number: Option<i32>,
    deep_link_url: Option<String>,
    player_id: Option<String>
}

#[derive(Debug)]
struct BetTypes {
    hash: HashMap<String, BetOptions>
}

#[derive(Debug)]
pub struct BetOptions {
    best_price: BestBooks,
    all_books: Vec<BookInfo>,
    consensus_odds: Option<f64>,
    fair_odds: Option<f64>,
    bet_name: String,
}

#[derive(Debug)]
struct BestBooks {
    price: f64,
    books: Vec<String>
}

#[derive(Debug)]
pub struct BookInfo {
    price: f64,
    name: String
}
