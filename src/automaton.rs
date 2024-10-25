use std::collections::HashMap;

type StateIndex = usize;
type Symbol = String;

#[derive(Copy, Clone)]
pub struct State {
    pub number: StateIndex,
    is_final: bool,
    is_error: bool,
}

impl State {
    pub fn new(number: StateIndex, is_final: bool, is_error: bool) -> State {
        State {
            number,
            is_final,
            is_error,
        }
    }
}

pub struct TransitionMatrix {
    matrix: Vec<HashMap<Symbol, State>>,
    start_state: State,
}

impl TransitionMatrix {
    pub fn new() -> TransitionMatrix {
        TransitionMatrix {
            matrix: Vec::new(),
            start_state: State::new(0, false, false),
        }
    }

    pub fn start_state(&self) -> &State {
        &self.start_state
    }

    pub fn transition(&self, state: &State, symbol: &str) -> Option<&State> {
        if state.number >= self.matrix.len() {
            return None;
        }

        match self.matrix.get(state.number).unwrap().get(symbol) {
            None => None,
            Some(state) => Some(state),
        }
    }

    pub fn add(&mut self, from_state: State, to_state: State, symbol: &str) {
        if from_state.number >= self.matrix.len() {
            self.matrix.resize(from_state.number + 1, HashMap::new())
        }
        self.matrix
            .get_mut(from_state.number)
            .unwrap()
            .insert(symbol.to_string(), to_state);
    }
}

pub struct Automaton {
    transition_matrix: TransitionMatrix,
}

impl Automaton {
    pub fn new() -> Automaton {
        Automaton {
            transition_matrix: TransitionMatrix::new(),
        }
    }

    pub fn add_transition(
        &mut self,
        from_state: State,
        to_state: State,
        symbol: &str,
    ) {
        self.transition_matrix.add(from_state, to_state, symbol);
    }

    pub fn consume(&self, sequence: &str) -> bool {
        let mut current_state = self.transition_matrix.start_state();

        for symbol in sequence.chars() {
            match self.transition(current_state, symbol) {
                None => {
                    return false;
                }
                Some(state) => {
                    current_state = state;
                }
            }
        }

        return current_state.is_final & !current_state.is_error;
    }

    fn transition(&self, state: &State, symbol: char) -> Option<&State> {
        self.transition_matrix
            .transition(state, symbol.to_string().as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_automaton() -> Automaton {
        let start = State::new(0, false, false);
        let first = State::new(1, false, false);
        let second = State::new(2, true, false);

        let mut automaton = Automaton::new();
        automaton.add_transition(start, first, "a");
        automaton.add_transition(first, second, "b");

        return automaton;
    }

    #[test]
    fn test_consume_ab() {
        let automaton = create_automaton();
        assert_eq!(automaton.consume("ab"), true);
    }

    #[test]
    fn test_consume_abc() {
        let automaton = create_automaton();
        assert_eq!(automaton.consume("abc"), false);
    }

    #[test]
    fn test_consume_cab() {
        let automaton = create_automaton();
        assert_eq!(automaton.consume("cab"), false);
    }
}
