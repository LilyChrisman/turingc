use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::production_map::*;

#[derive(Clone)]
struct TapeNode {
    content: char,
    left: Option<Rc<RefCell<TapeNode>>>,
    right: Option<Rc<RefCell<TapeNode>>>,
}
impl Default for TapeNode {
    fn default() -> Self {
        Self {
            content: '☐',
            left: None,
            right: None
        }
    }
}
impl TapeNode {
    fn left(this: &Rc<RefCell<TapeNode>>) -> Rc<RefCell<TapeNode>> {
        if let Some(p) = &this.clone().borrow().left {
            p.clone()
        } else {
            let left = Rc::new(RefCell::new(TapeNode::default()));
            left.borrow_mut().right = Some(this.clone());
            this.borrow_mut().left = Some(left);
            TapeNode::left(this)
        }
    }
    fn right(this: &Rc<RefCell<TapeNode>>) -> Rc<RefCell<TapeNode>> {
        if let Some(p) = &this.clone().borrow().right {
            p.clone()
        } else {
            let right = Rc::new(RefCell::new(TapeNode::default()));
            right.borrow_mut().left = Some(this.clone());
            this.borrow_mut().right = Some(right);
            TapeNode::right(this)
        }
    }
}

struct InfiniteTape {
    current: Rc<RefCell<TapeNode>>,
    initial: Rc<RefCell<TapeNode>>,
}
impl InfiniteTape {
    pub fn shift(&mut self, d: Direction) {
        match d {
            Direction::Left => {
                let was_none = self.current.borrow().left.is_none();
                let left = TapeNode::left(&self.current);
                if was_none {
                    self.initial = left.clone();
                }
                self.current = left;
            },
            Direction::Right => {
                let right = TapeNode::right(&self.current);
                self.current = right;
            },
        };
    }

    pub fn set_current(&self, c: char) {
        self.current.borrow_mut().content = c;
    }
}
impl FromIterator<char> for InfiniteTape {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut curr = Rc::new(RefCell::new(TapeNode::default()));
        let initial = curr.clone();
        for mut c in iter {
            if c == '\n' {c = '☐'}
            curr.borrow_mut().content = c;
            let right = TapeNode::right(&curr);
            curr = right;
        }

        Self { current: initial.clone(), initial }
    }
}
impl Display for InfiniteTape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut curr = self.initial.clone();
        s.push(curr.borrow().content);
        while let Some(r) = curr.clone().borrow().right.clone() {
            s.push(r.borrow().content);
            curr = r;
        }

        f.write_str(&s)
    }
}

pub fn verify(input: &str, productions: &ProductionMap) {
    let mut state = productions.initial_state.clone();
    let mut tape = InfiniteTape::from_iter(input.chars());
    println!("Initial Tape:\n{}", tape);
    
    loop {
        let c = tape.current.borrow().content;

        match productions.map.get(&(state.clone(), c)) {
            Some(transformation) => {
                println!(
                    "δ({}, {}) -> ({}, {}, {})",
                    &state,
                    c,
                    &transformation.new_state,
                    transformation.write,
                    transformation.direction
                );
                state = transformation.new_state.clone();
                tape.set_current(transformation.write);
                tape.shift(transformation.direction);
            },
            None if c == '☐' => {
                break;
            }
            None => {
                eprintln!(r#"'{}' has no production from state "{}""#, c, state);
                break;
            }
        }
    }

    println!("Final Tape:\n{}", tape);

    if state == productions.final_state {
        println!("{} ∈ L", input.trim());
    } else {
        println!("{} ∉ L", input.trim());
    }
}
