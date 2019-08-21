use std::collections::HashSet;
use std::iter::FromIterator;

/// Deterministic Finite Automaton
/// For simplisity, we express states and alphabets as u64 integers,
/// For example, if states is 2 and alphabet is 4,
/// set of state and alphabet is [0, 1] and [0, 1, 2, 3].
/// transition function is expressed as two dimentional Vec of u64.
/// transition function for state 0 and alphabet 2 is store in transition_function[0][2].
pub struct Dfa {
    states: u64,
    alphabets: u64,
    transition_function: Vec<Vec<u64>>,
    initial_state: u64,
    accept_states: HashSet<u64>,
}

impl Dfa {
    pub fn new(
        states: u64,
        alphabets: u64,
        transition_function: Vec<Vec<u64>>,
        initial_state: u64,
        accept_states: Vec<u64>,
    ) -> Result<Self, ()> {
        // check transition_function's row and column is correct number.
        if transition_function.len() == states as usize
            && transition_function.iter().all(|v| v.len() == alphabets as usize) {

            Ok(Dfa {
                states,
                alphabets,
                transition_function,
                initial_state,
                accept_states: HashSet::from_iter(accept_states),
            })
        } else {
            Err(())
        }
    }

    fn transition(&self, current: u64, alphabet: u64) -> Result<u64, ()> {
        if current < self.states && alphabet < self.alphabets {
            let next = self.transition_function[current as usize][alphabet as usize];
            println!("transition from: {}, to: {}", current, next);
            Ok(next)
        } else {
            Err(())
        }
    }

    pub fn execute(&self, input: Vec<u64>) -> Result<bool, ()> {
        if !input.iter().all(|a| a < &self.alphabets) {
            return Err(())
        }

        let res = input
            .iter()
            .fold(Ok(self.initial_state), |state, alphabet| {
                self.transition(state.unwrap(), *alphabet)
            }).unwrap();
        Ok(self.accept_states.contains(&res))
    }

    pub fn is_language(&self, input: Vec<u64>) -> bool {
        match self.execute(input) {
            Ok(res) => res,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accepted1() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![1, 1, 1];
        assert_eq!(dfa.execute(input), Ok(true));
    }

    #[test]
    fn test_accepted2() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![1, 0, 0];
        assert_eq!(dfa.execute(input), Ok(true));
    }

    #[test]
    fn test_rejected1() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![1, 0, 1];
        assert_eq!(dfa.execute(input), Ok(false));
    }

    #[test]
    fn test_rejected2() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![0, 0, 0, 1];
        assert_eq!(dfa.execute(input), Ok(false));
    }

    #[test]
    fn test_language() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![1, 0, 0];
        assert!(dfa.is_language(input));
    }

    #[test]
    fn test_non_language() {
        let transition_function = vec![
            vec![1, 0],
            vec![0, 1],
        ];
        let dfa = Dfa::new(2, 2, transition_function, 0, vec![0]).unwrap();

        let input = vec![0, 0, 0, 1];
        assert!(!dfa.is_language(input));
    }
}
