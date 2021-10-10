use std::fmt::Debug;

fn main() {
    println!("Hello, world!");
}
// A DFA is a  5-tuple(Q,Sigma,Delta,q0,F)
// where
// 1. Q is a finite set called the states
// 2. Sigma(A) is a finite set called the alphabet.
// 3. Delta: Q x Sigma -> Q is the transition function.
// 4. q0 is an element of Q
// 5. F is a subset of Q.
#[derive(Clone)]
pub struct DeterministicFiniteAutomaton<
    Q: Clone + PartialEq + Debug,
    A: PartialEq + Debug + Clone,
    F: Fn(Q,&A) -> Option<Q> + Clone>{
    states: Vec<Q>,
    input_alphabet: Vec<A>,
    transition_function: F,
    start_state: Q,
    final_states: Vec<Q>,
}

pub struct Sequence<
    Q: Clone + PartialEq + Debug,
    A: PartialEq + Debug + Clone,
    F: Fn(Q,&A) -> Option<Q> + Clone>
{
    dfa: DeterministicFiniteAutomaton<Q,A,F>,
    sequence: Vec<(Q,String)>,
    accept: bool,
}

impl<Q, A, F> Sequence<Q, A, F>
    where
Q: Clone + PartialEq + Debug,
A: PartialEq + Debug + Clone,
F: Fn(Q,&A) -> Option<Q> + Clone {
    pub fn print_and_accept(&self) -> bool {
        if self.accept {
            println!("accept on");
            println!("{:?}",self.sequence);
            println!("with final states of {:?}",self.dfa.final_states);
            self.accept
        } else {
            println!("reject on");
            println!("{:?}",self.sequence);
            println!("with final states of {:?}",self.dfa.final_states);
            self.accept
        }
    }
}



impl<Q,A,F> DeterministicFiniteAutomaton<Q,A,F>
where
Q: Clone + PartialEq + Debug,
A: PartialEq + Debug + Clone,
F: Fn(Q,&A) -> Option<Q> + Clone
    {
        // Let M = (Q,Sigma,Delta,q0,F) be a finite automaton
        // let w = w1,w2,...,w_n be a member where each w_i is an element of Sigma
        // M accepts w if a sequence of states r0,r1,...rn in Q exist with three conditions
        // 1. r0 = q0
        // 2. F(r_i,w_i+1)= r_i+1 for i = 0, ..., n-1,
        // 3. r_n is an element of F
        pub fn is_accepted(&self, string: Vec<A>) -> Sequence<Q,A,F> {
            // let w = w1,w2,...,w_n be a member where each w_i is an element of Sigma
            for a in string.iter() {
                assert!(self.input_alphabet.contains(a));
            }
            // r0 = q0
            let mut state = self.start_state.clone();
            let mut sequence = Sequence{
                dfa: self.clone(),
                sequence: vec![(state.clone(),format!("{:?}", state.clone()))],
                accept: self.final_states.contains(&state)

            };
            // Our sequence is described as ( state, input that took us to that state)
            sequence.sequence.push((state.clone(), format!("")));
            for symbol in string {
                match (self.transition_function)(state.clone(), &symbol) {
                    // Since we have no control over the contents of F
                    // we check that the state it returns is an element of Q
                    Some(next_state) => {
                        assert!(self.states.contains(&next_state));
                        state = next_state;
                        sequence.accept = self.final_states.contains(&state);
                        sequence.sequence.push((state.clone(), format!("{:?}", &symbol))); },
                    None => {
                        sequence.accept = false;
                        return sequence
                    }
                }
            }
            // r_n is an element of F
            if self.final_states.contains(&state) {
                sequence.accept = true;
                return sequence
            } else {
                return sequence;
            }
        }
        // Checks that
        // q0 is an element of Q and
        // F is a subset of Q.
        pub fn new(    states: Vec<Q>,
                       input_alphabet: Vec<A>,
                       transition_function: F,
                       start_state: Q,
                       final_states: Vec<Q>,
        ) -> Self {
            assert!(states.contains(&start_state));
            final_states.iter().inspect(|q| assert!(states.contains(q)));
            Self{
                states,
                input_alphabet,
                transition_function,
                start_state,
                final_states,
            }
        }
}



#[cfg(test)]
mod tests {
    use super::*;

    // Example from page 34
    // Describes a DFA whose language is
    // A = { w | w contains at least one 1 and an even number of 0s follow the last 1}
    #[test]
    fn figure_one_six() {
        let states = vec![1,2,3];
        let final_states = vec!{2};
        let input_alphabet = vec![0,1];
        fn transition_function(q:i32,a:&i32) -> Option<i32> {
            match (q,a) {
                (1, 0) => {Some(1)},
                (1, 1) => {Some(2)},
                (2, 0) => {Some(3)},
                (2, 1) => {Some(2)},
                (3, 0) => {Some(2)},
                (3, 1) => {Some(2)},
                (_, _) => {None},
            }
        }
        let dfa = DeterministicFiniteAutomaton{
            states, input_alphabet, transition_function,
            start_state: 1,
            final_states,
        };
        assert!(dfa.is_accepted(vec![1,0,0]).print_and_accept());
        assert!(dfa.is_accepted(vec![0,0,0,1,0,0,0,0]).print_and_accept());
        assert!(dfa.is_accepted(vec![0,0,1,0,0,0,0,0,0]).print_and_accept());
        assert!(dfa.is_accepted(vec![1,1,0,1,0,1,0,1,0,0,0,0]).print_and_accept());
        assert_eq!(dfa.is_accepted(vec![1,0,0,0]).print_and_accept(),false);
        assert_eq!(dfa.is_accepted(vec![0,0]).print_and_accept(),false);
        assert_eq!(dfa.is_accepted(vec![]).print_and_accept(),false);
    }
    // A machine that accepts all strings that start with an a and end with an a or start with a b
    // and end with a b
    #[test]
    fn figure_one_eleven() {
        let states = vec!["s","q1","q2","r1","r2"];
        let final_states = vec!["q1","r1"];
        let input_alphabet = vec!['a','b'];
        let start_state = "s";
        fn transition_function(q:&'static str,a:&char) -> Option<&'static str> {
            match (q,a) {
                ("s",'a') => {Some("q1")},
                ("s",'b') => {Some("r1")},
                ("q1",'a') => {Some("q1")},
                ("q1",'b') => {Some("q2")},
                ("q2",'a') => {Some("q1")},
                ("q2",'b') => {Some("q2")},
                ("r1",'a') => {Some("r2")},
                ("r1",'b') => {Some("r1")},
                ("r2",'a') => {Some("r2")},
                ("r2",'b') => {Some("r1")},
                (_, _) => {None},
            }
        }

        let dfa = DeterministicFiniteAutomaton::new(
            states,
            input_alphabet,
            transition_function,
            start_state,
            final_states
        );
        assert!(dfa.is_accepted(vec!['a']).print_and_accept());
        assert!(dfa.is_accepted(vec!['b']).print_and_accept());
        assert!(dfa.is_accepted(vec!['a','b','a']).print_and_accept());
        assert!(dfa.is_accepted(vec!['b','a','b']).print_and_accept());
        assert!(dfa.is_accepted(vec!['a','a','a']).print_and_accept());
        assert!(dfa.is_accepted(vec!['b','b','b']).print_and_accept());
        assert!(dfa.is_accepted(vec!['a','b','a','b','a']).print_and_accept());
        assert!(dfa.is_accepted(vec!['b','a','b','b','b','b','a','b']).print_and_accept());
        assert_eq!(dfa.is_accepted(vec![]).print_and_accept(),false);
        assert_eq!(dfa.is_accepted(vec!['a','b']).print_and_accept(),false);
        assert_eq!(dfa.is_accepted(vec!['b','b','a']).print_and_accept(),false);
        assert_eq!(dfa.is_accepted(vec!['a','b','a','b']).print_and_accept(),false);
    }


}