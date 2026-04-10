#[cfg(test)]
mod integration {
    use tungsten_core::nodes::{
        gates::{and_gate::AndGate, xor_gate::XOrGate},
        traits::gate::GateNode,
    };
    use tungsten_types::position::Position;

    #[test]
    fn half_binary_adder() {
        // A, B, S, C
        let truth_table: [(bool, bool, bool, bool); 4] = [
            (false, false, false, false),
            (false, true, true, false),
            (true, false, true, false),
            (true, true, false, true),
        ];

        let mut xor = XOrGate::new(Position::ZERO);
        let mut and = AndGate::new(Position::ZERO);

        for (a, b, s, c) in truth_table {
            xor.io.get_input_mut("A").unwrap().set_value(a);
            xor.io.get_input_mut("B").unwrap().set_value(b);

            xor.process().expect("Process should have succeeded");

            and.io.get_input_mut("A").unwrap().set_value(a);
            and.io.get_input_mut("B").unwrap().set_value(b);
            and.process().expect("Process should have succeeded");

            assert_eq!(xor.io.get_output("C").unwrap().value, s);
            assert_eq!(and.io.get_output("C").unwrap().value, c);
        }
    }
}
