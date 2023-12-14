fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let boat_race = process_input(&input);

    boat_race.record_opportunities().to_string()
}

struct BoatRace {
    time: u64,
    distance: u64,
}

impl BoatRace {
    fn record_opportunities(&self) -> u64 {
        let mut opportunities = 0;
        let mut travel_time;
        let mut travel_distance;

        for charge_time in 1..self.time {
            travel_time = self.time - charge_time;
            travel_distance = charge_time * travel_time;
            if travel_distance > self.distance {
                opportunities += 1;
            }
        }

        opportunities
    }
}

fn process_input(input: &str) -> BoatRace {
    let mut lines = input.lines();

    let (_time_label, time_text) = lines.next().unwrap().split_once(':').unwrap();
    let (_distance_label, distance_text) = lines.next().unwrap().split_once(':').unwrap();

    let time = time_text.replace(' ', "").parse::<u64>().unwrap();
    let distance = distance_text.replace(' ', "").parse::<u64>().unwrap();

    BoatRace { time, distance }
}
