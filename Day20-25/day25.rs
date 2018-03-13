const START_STATE: u8 = 'a' as u8;
const NUM_STEPS: u32 = 12994925;

#[derive(Debug, Clone)]
struct Transition {
    new_val: usize,
    movement: i32,
    next_state: u8
}

impl Transition {
    fn new(new_val: usize, movement: i32, next_state: char) -> Transition {
        Transition{
            new_val: new_val,
            movement: movement,
            next_state: next_state as u8,
        }
    }
}

fn make_tm() -> Vec<Vec<Transition>> {
    vec![
        vec![
            Transition::new(1, 1, 'b'),
            Transition::new(0, -1, 'f')
        ],
        vec![
            Transition::new(0, 1, 'c'),
            Transition::new(0, 1, 'd')
        ],
        vec![
            Transition::new(1, -1, 'd'),
            Transition::new(1, 1, 'e')
        ],
        vec![
            Transition::new(0, -1, 'e'),
            Transition::new(0, -1, 'd')
        ],
        vec![
            Transition::new(0, 1, 'a'),
            Transition::new(1, 1, 'c')
        ],
        vec![
            Transition::new(1, -1, 'a'),
            Transition::new(1, 1, 'a')
        ]
    ]
}

fn run_tm(steps: u32) -> u32 {
    let mut tape: [usize; 15000] = [0; 15000];
    let mut loc = (tape.len() / 2) as usize;
    let mut state = START_STATE;
    let mut num_ones = 0;
    let tm = make_tm();

    for _ in 0..steps {
        let val = tape[loc];
        let t = &tm[(state - START_STATE) as usize][val];

        if t.new_val == 1 && val == 0 {
            num_ones += 1;
        } else if t.new_val == 0 && val == 1 {
            num_ones -= 1;
        }

        tape[loc] = t.new_val;
        loc = ((loc as i32) + t.movement) as usize;
        state = t.next_state;
    }

    num_ones
}

pub fn main() {
    println!("Part 1: {}", run_tm(NUM_STEPS));
    println!("Part 2: ⭐");
}