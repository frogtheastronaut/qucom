use qucom_rs::qasm::QasmParser;
use qucom_rs::qasm::generator::QasmInstruction;

#[test]
fn parse_single_gates() {
    let qasm = "h q[0];\nx q[1];\ny q[2];\nz q[0];";
    let instructions = QasmParser::parse(qasm).unwrap();
    
    assert_eq!(instructions.len(), 4);
    assert!(matches!(instructions[0], QasmInstruction::H(0)));
    assert!(matches!(instructions[1], QasmInstruction::X(1)));
}

#[test]
fn parse_two_qubit_gates() {
    let qasm = "cx q[0], q[1];\ncz q[1], q[2];";
    let instructions = QasmParser::parse(qasm).unwrap();
    
    assert_eq!(instructions.len(), 2);
    assert!(matches!(instructions[0], QasmInstruction::CX(0, 1)));
    assert!(matches!(instructions[1], QasmInstruction::CZ(1, 2)));
}

#[test]
fn parse_parameterized_gates() {
    use std::f64::consts::PI;
    
    let qasm = "rx(pi/2) q[0];\nry(1.57) q[1];";
    let instructions = QasmParser::parse(qasm).unwrap();
    
    assert_eq!(instructions.len(), 2);
    if let QasmInstruction::Rx(angle, _) = instructions[0] {
        assert!((angle - PI / 2.0).abs() < 1e-10);
    }
}
