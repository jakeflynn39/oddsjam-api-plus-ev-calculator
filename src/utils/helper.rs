use crate::{BookInfo, BetOptions};
use crate::utils::constants::{PINN_WEIGHT, CIRCA_WEIGHT, PUB_WEIGHT};

pub fn american_to_percent(american: &f64) -> f64 {
    if *american > 0.0 {
        100.0 / (100.0 + *american)
    } else if *american < 0.0 {
        -*american / (-*american + 100.0)
    } else {
        0.0
    }
}

pub fn percent_to_american(percent: &f64) -> f64 {
    if *percent < 0.5 {
        100.0 / *percent - 100.0
    } else if *percent > 0.5 {
        100.0 / (*percent - 1.0) + 100.0
    } else {
        100.0
    }
}

pub fn get_bet_name(bet_name: &str) -> String {
    let words: Vec<&str> = bet_name.split_whitespace().collect();

    // 49ers Over 24.5 becomes 49ers, to be used in the bet name
    let name = words[..words.len() - 2].join(" ");
    name
}

pub fn get_consensus_odds(all_books: &Vec<BookInfo>) -> f64 {
    let mut num_pinn: f64 = 0.0;
    let mut num_circa: f64 = 0.0;
    let mut num_pub: f64 = 0.0;

    let mut sum_pinn: f64 = 0.0;
    let mut sum_circa: f64 = 0.0;
    let mut sum_pub: f64 = 0.0;

    for book in all_books {
        match book.name.as_str() {
            "Pinnacle" => {
                num_pinn += 1.0;
                sum_pinn += book.price
            }

            "Circa" => {
                num_circa += 1.0;
                sum_circa += book.price
            }

            _ => {
                num_pub += 1.0;
                sum_pub += book.price
            }
        }
    }
        // Calculate the average odds for each book
        let avg_pinn: f64 = if num_pinn != 0.0 { sum_pinn / num_pinn } else { 0.0 };
        let avg_circa: f64 = if num_circa != 0.0 { sum_circa / num_circa } else { 0.0 };
        let avg_pub: f64 = if num_pub != 0.0 { sum_pub / num_pub } else { 0.0 };

        let weights: [f64; 3] = [avg_pinn * PINN_WEIGHT, avg_circa * CIRCA_WEIGHT, avg_pub * PUB_WEIGHT];

        // Create a mask to ignore books that don't have odds for this bet
        let mask = weights
            .iter()
            .map(|&avg| if avg != 0.0 { 1.0 } else { 0.0 })
            .collect::<Vec<f64>>();

        let denominator: f64 = [PINN_WEIGHT, CIRCA_WEIGHT, PUB_WEIGHT]
            .iter()
            .zip(mask.iter())
            .map(|(weight, mask_val)| weight * mask_val)
            .sum::<f64>();

        let consensus_per: f64 = weights
            .iter()
            .sum::<f64>()
            / denominator;

    consensus_per
}

pub fn calc_ev(ev_bet: &BetOptions) -> f64 {
    (ev_bet.fair_odds.unwrap() - ev_bet.best_price.price) / ev_bet.best_price.price * 100.0
}