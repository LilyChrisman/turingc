use crate::production_map::*;
use sscanf::sscanf;

#[allow(nonstandard_style)]
mod ParseState {
    pub const Initial              : u8 = 0;
    pub const OpeningRule          : u8 = 1;
    pub const RuleState            : u8 = 2;
    pub const RuleCharacter        : u8 = 3;
    pub const ClosingRule          : u8 = 4;
    pub const Equal                : u8 = 5;
    pub const OpeningProduction    : u8 = 6;
    pub const TransitionState      : u8 = 7;
    pub const TransitionCharacter  : u8 = 8;
    pub const ProductionComma      : u8 = 9;
    pub const Direction            : u8 = 10;
    pub const ClosingProduction    : u8 = 11;

    pub const Comment              : u8 = 20;
}

pub fn parse_file(file: String, productions: &mut ProductionMap) {
    let (line_1, file) = file.split_once('\n').expect("Simply too short");
    let (line_2, file) = file.split_once('\n').expect("Simply too short");

    let initial_state = sscanf!(line_1, "Initial = {String}")
        .expect(r#"Expected initial state definition in form "Initial = StateName""#);
    let final_state = sscanf!(line_2, "Final = {String}")
        .expect(r#"Expected final state definition in form "Final = StateName""#);

    productions.initial_state = initial_state;
    productions.final_state = final_state;
    
    let mut state = ParseState::Initial;

    let mut state_name = String::new();
    let mut state_character = '\0';

    let mut transition_name = String::new();
    let mut transition_character = '\0';
    let mut transition_direction = Direction::Left;

    for c in file.chars() {
        if c.is_whitespace() {continue;}
        match state {
            ParseState::Initial if c == '#' => state = ParseState::Comment,
            ParseState::Comment if c == '#' => state = ParseState::Initial,
            ParseState::Comment => {},

            ParseState::Initial if c == 'δ' => state += 1,
            ParseState::OpeningRule if c == '(' => state += 1,
            ParseState::RuleState =>  {
                if c == ',' {
                    state += 1;
                } else {
                    state_name.push(c);
                }
            },
            ParseState::RuleCharacter => {
                state_character = c;
                state += 1;
            },
            ParseState::ClosingRule if c == ')' => state += 1,
            ParseState::Equal if c == '=' => state += 1,
            ParseState::OpeningProduction if c == '(' => state += 1,
            ParseState::TransitionState => {
                if c == ',' {
                    state += 1;
                } else {
                    transition_name.push(c);
                }
            },
            ParseState::TransitionCharacter => {
                transition_character = c;
                state += 1;
            },
            ParseState::ProductionComma if c == ',' => state += 1,
            ParseState::Direction if c == 'L' => {
                transition_direction = Direction::Left;
                state += 1;
            },
            ParseState::Direction if c == 'R' => {
                transition_direction = Direction::Right;
                state += 1;
            },
            ParseState::ClosingProduction if c == ')' => {
                let t = Transformation::new(transition_name, transition_character, transition_direction);

                if let Some(prod) = productions.map.get(&(state_name.clone(), state_character)) {
                    panic!("Duplicate productions {:?}\n{:?}", prod, t);
                }

                productions.map.insert(
                    (state_name, state_character),
                    t.clone()
                );
                state_name = String::new();
                transition_name = String::new();
                state = ParseState::Initial;
            },
            _ => panic!("Unexpected token '{}' at state {}", c, state),
        }
    }
}
