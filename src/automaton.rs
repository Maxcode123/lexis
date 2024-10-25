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

mod nfa {
    use std::collections::HashSet;

    const EPSILON: &str = "ε";

    pub struct Automaton {
        pub regex_str: String,
        pub start_state: State,
        transition_matrix: TransitionMatrix,
    }

    impl Automaton {
        pub fn from_regex(regex_str: &str) -> Automaton {
            if regex_str.len() == 1 {
                return Automaton::from_char(regex_str);
            }

            let (first_char, rest) = regex_str.split_at(1);

            Automaton::from_char(first_char).concatenate(rest)
        }

        pub fn from_char(character: &str) -> Automaton {
            let start = State::new(0, false);
            let end = State::new(1, true);

            let new = Automaton {
                regex_str: character.to_string(),
                start_state: start,
                transition_matrix: TransitionMatrix::new(),
            };

            new.add_transition(&new.start_state, &end, character);

            return new;
        }

        pub fn concatenate(&self, regex_str: &str) -> Automaton {
            let other = Automaton::from_regex(regex_str);
            let mut new = self.append(other);

            new.append_final();
            new.insert_start();
            new.regex_str = self.regex_str.as_str().to_string() + regex_str;

            new
        }

        pub fn union(&self, regex_str: &str) -> Automaton {
            let other = Automaton::from_regex(regex_str);
            let mut new = self.add(other);

            new.append_final();
            new.insert_start();
            new.regex_str =
                self.regex_str.as_str().to_string() + "|" + regex_str;

            new
        }

        pub fn kleene_closure(&self) -> Automaton {
            let mut new = Automaton::from_regex(self.regex_str.as_str());

            for end_state in new.end_states() {
                new.add_transition(&new.start_state, end_state, EPSILON);
            }

            new.append_final();
            new.insert_start();

            for end_state in new.end_states() {
                new.add_transition(&new.start_state, end_state, EPSILON);
            }

            new.regex_str = self.regex_str + "*";

            new
        }

        pub fn add_transition(
            &mut self,
            from_state: &State,
            to_state: &State,
            symbol: &str,
        ) {
            self.transition_matrix
                .add_transition(from_state, to_state, symbol);
        }

        pub fn transitions(&self) -> Iter<Transition> {}

        fn append(&self, other: Automaton) -> Automaton {}

        fn add(&self, other: Automaton) -> Automaton {}

        fn append_final(&mut self) {}

        fn insert_start(&mut self) {}

        fn end_states(&self) -> Iter<&State> {}
    }

    pub struct State {
        number: usize,
        name: String,
        is_final: bool,
    }

    impl State {
        pub fn new(number: usize, is_final: bool) -> State {
            State {
                number,
                name: "s".to_string() + number.to_string().as_str(),
                is_final,
            }
        }
    }

    pub struct Transition<'a> {
        from_state: &'a State,
        to_state: &'a State,
        symbol: String,
    }

    impl<'a> Transition<'a> {
        pub fn new(
            from_state: &'a State,
            to_state: &'a State,
            symbol: &str,
        ) -> Transition<'a> {
            Transition {
                from_state,
                to_state,
                symbol: symbol.to_string(),
            }
        }

        pub fn to_str(&self) -> String {
            format!(
                "({}->{},{})",
                &self.from_state.name, &self.to_state.name, &self.symbol
            )
        }

        pub fn from_str(transition_str: &str) -> Transition {}
    }

    type TransitionHash = String;

    pub struct TransitionMatrix {
        matrix: HashSet<TransitionHash>,
    }

    impl TransitionMatrix {
        pub fn new() -> TransitionMatrix {
            TransitionMatrix {
                matrix: HashSet::new(),
            }
        }

        pub fn is_valid(
            &self,
            from_state: &State,
            to_state: &State,
            symbol: &str,
        ) -> bool {
            self.matrix.contains(
                Transition::new(from_state, to_state, symbol).to_str().as_str(),
            )
        }

        pub fn add_transition(
            &mut self,
            from_state: &State,
            to_state: &State,
            symbol: &str,
        ) {
            self.matrix.insert(
                Transition::new(from_state, to_state, symbol)
                    .to_str()
                    .to_string(),
            );
        }
    }
}
