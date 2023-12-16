use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (mut navigation, network) = process_input(input);

    let step_count = count_steps(&mut navigation, &network);

    step_count.to_string()
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn from_str(s: &str) -> Node {
        Node {
            left: s[1..4].to_string(),
            right: s[6..9].to_string(),
        }
    }
}

struct Network {
    start: String,
    nodes: HashMap<String, Node>,
}

impl Network {
    fn from_lines(lines: &Vec<&str>) -> Self {
        // let start = lines[0].split_once(" = ").unwrap().0.to_string();

        let nodes: HashMap<String, Node> = HashMap::from_iter(
            lines
                .iter()
                .map(|&line| line.split_once(" = ").unwrap())
                .map(|(source_text, destination_text)| {
                    (source_text.to_string(), Node::from_str(destination_text))
                }),
        );

        Self {
            start: "AAA".to_string(),
            nodes,
        }
    }
}

struct Navigation {
    steps: String,
    index: usize,
}

impl Navigation {
    fn next(&mut self) -> char {
        let direction = self.steps.chars().nth(self.index).unwrap();
        self.index = (self.index + 1) % self.steps.len();
        direction
    }

    fn from_line(line: &str) -> Self {
        Self {
            steps: line.to_string(),
            index: 0,
        }
    }
}

fn process_input(input: &str) -> (Navigation, Network) {
    let mut lines = input.lines();

    let navigation = Navigation::from_line(lines.next().unwrap());

    lines.next();

    let network = Network::from_lines(&lines.collect());

    (navigation, network)
}

fn count_steps(navigation: &mut Navigation, network: &Network) -> u64 {
    let mut position = &network.start;
    let mut step_count = 0;

    while position != &"ZZZ" {
        step_count += 1;
        if navigation.next() == 'L' {
            position = &network.nodes[position].left;
        } else {
            position = &network.nodes[position].right;
        }
    }

    step_count
}
