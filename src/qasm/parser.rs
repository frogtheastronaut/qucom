use crate::qasm::generator::QasmInstruction;

/// parse QASM 3 string into executable instructions
pub struct QasmParser;

impl QasmParser {
    /// parse a QASM 3 program string and return instructions
    pub fn parse(qasm_string: &str) -> Result<Vec<QasmInstruction>, String> {
        let mut instructions = Vec::new();
        let lines: Vec<&str> = qasm_string.lines().collect();
        let mut i = 0;
        
        while i < lines.len() {
            let trimmed = lines[i].trim();
            
            // skip empty lines and comments
            if trimmed.is_empty() || trimmed.starts_with("//") {
                i += 1;
                continue;
            }
            
            // skip version and include statements
            if trimmed.starts_with("OPENQASM") || trimmed.starts_with("include") {
                i += 1;
                continue;
            }
            
            // skip register declarations
            if trimmed.starts_with("qubit") || trimmed.starts_with("bit") 
                || trimmed.starts_with("qreg") || trimmed.starts_with("creg") {
                i += 1;
                continue;
            }
            
            // handle control flow blocks
            if trimmed.starts_with("if") {
                let (instr, lines_consumed) = Self::parse_if_block(&lines[i..])?;
                instructions.push(instr);
                i += lines_consumed;
                continue;
            }
            
            if trimmed.starts_with("while") {
                let (instr, lines_consumed) = Self::parse_while_block(&lines[i..])?;
                instructions.push(instr);
                i += lines_consumed;
                continue;
            }
            
            if trimmed.starts_with("for") {
                let (instr, lines_consumed) = Self::parse_for_block(&lines[i..])?;
                instructions.push(instr);
                i += lines_consumed;
                continue;
            }
            
            // parse gate instruction
            if let Some(instr) = Self::parse_instruction(trimmed) {
                instructions.push(instr);
            } else if !trimmed.is_empty() {
                return Err(format!("failed to parse line {}: {}", i + 1, trimmed));
            }
            
            i += 1;
        }
        
        Ok(instructions)
    }
    
    fn parse_instruction(line: &str) -> Option<QasmInstruction> {
        let line = line.trim_end_matches(';').trim();
        
        // split by whitespace but handle parentheses
        let gate_end = line.find('(').or_else(|| line.find(' ')).unwrap_or(line.len());
        let gate = &line[..gate_end];
        let args = line[gate_end..].trim();
        
        match gate {
            "h" => {
                let qubit = Self::extract_qubit_index(args)?;
                Some(QasmInstruction::H(qubit))
            }
            "x" => {
                let qubit = Self::extract_qubit_index(args)?;
                Some(QasmInstruction::X(qubit))
            }
            "y" => {
                let qubit = Self::extract_qubit_index(args)?;
                Some(QasmInstruction::Y(qubit))
            }
            "z" => {
                let qubit = Self::extract_qubit_index(args)?;
                Some(QasmInstruction::Z(qubit))
            }
            "s" | "sdg" => {
                let qubit = Self::extract_qubit_index(args)?;
                let is_dagger = gate == "sdg";
                Some(QasmInstruction::S(qubit, is_dagger))
            }
            "t" | "tdg" => {
                let qubit = Self::extract_qubit_index(args)?;
                let is_dagger = gate == "tdg";
                Some(QasmInstruction::T(qubit, is_dagger))
            }
            "cx" | "cnot" => {
                let indices = Self::extract_two_qubit_indices(args)?;
                Some(QasmInstruction::CX(indices.0, indices.1))
            }
            "cz" => {
                let indices = Self::extract_two_qubit_indices(args)?;
                Some(QasmInstruction::CZ(indices.0, indices.1))
            }
            "ccx" | "toffoli" => {
                let indices = Self::extract_three_qubit_indices(args)?;
                Some(QasmInstruction::CCX(vec![indices.0, indices.1, indices.2]))
            }
            "swap" => {
                let indices = Self::extract_two_qubit_indices(args)?;
                Some(QasmInstruction::Swap(indices.0, indices.1))
            }
            "rx" => {
                let (angle, qubit) = Self::extract_parameterized_gate(args)?;
                Some(QasmInstruction::Rx(angle, qubit))
            }
            "ry" => {
                let (angle, qubit) = Self::extract_parameterized_gate(args)?;
                Some(QasmInstruction::Ry(angle, qubit))
            }
            "rz" => {
                let (angle, qubit) = Self::extract_parameterized_gate(args)?;
                Some(QasmInstruction::Rz(angle, qubit))
            }
            "p" | "phase" => {
                let (angle, qubit) = Self::extract_parameterized_gate(args)?;
                Some(QasmInstruction::Phase(angle, qubit))
            }
            "u" | "u3" => {
                let (params, qubit) = Self::extract_u_gate(args)?;
                Some(QasmInstruction::U(params.0, params.1, params.2, qubit))
            }
            "reset" => {
                let args = args.trim();
                if args.is_empty() || args == "q" || args == "q;" {
                    Some(QasmInstruction::ResetAll)
                } else {
                    let qubit = Self::extract_qubit_index(args)?;
                    Some(QasmInstruction::Reset(qubit))
                }
            }
            "barrier" => {
                let args = args.trim();
                if args.is_empty() || args == "q" || args == "q;" {
                    Some(QasmInstruction::BarrierAll)
                } else {
                    let qubits = Self::extract_qubit_list(args)?;
                    Some(QasmInstruction::Barrier(qubits))
                }
            }
            "delay" => {
                let (duration, unit, qubit) = Self::extract_delay(args)?;
                Some(QasmInstruction::Delay(duration, unit, qubit))
            }
            "measure" => {
                Self::parse_measurement_from_args(line)
            }
            _ => None,
        }
    }
    
    fn extract_qubit_index(s: &str) -> Option<usize> {
        // handle q[0] or $0 format
        if s.contains('[') {
            let start = s.find('[')? + 1;
            let end = s.find(']')?;
            s[start..end].parse().ok()
        } else if s.starts_with('$') {
            s[1..].parse().ok()
        } else {
            s.parse().ok()
        }
    }
    
    fn extract_two_qubit_indices(s: &str) -> Option<(usize, usize)> {
        // handle "q[0],q[1]" or "q[0], q[1]" or "$0,$1" format
        let s = s.trim();
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return None;
        }
        let q1 = Self::extract_qubit_index(parts[0])?;
        let q2 = Self::extract_qubit_index(parts[1])?;
        Some((q1, q2))
    }
    
    fn extract_three_qubit_indices(s: &str) -> Option<(usize, usize, usize)> {
        // handle "q[0],q[1],q[2]" or "q[0], q[1], q[2]" or "$0,$1,$2" format
        let s = s.trim();
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        if parts.len() != 3 {
            return None;
        }
        let q1 = Self::extract_qubit_index(parts[0])?;
        let q2 = Self::extract_qubit_index(parts[1])?;
        let q3 = Self::extract_qubit_index(parts[2])?;
        Some((q1, q2, q3))
    }
    
    fn extract_parameterized_gate(s: &str) -> Option<(f64, usize)> {
        // handle "rx(pi/2) q[0]" or "rx(1.57) $0" format
        if !s.contains('(') {
            return None;
        }
        
        let param_start = s.find('(')? + 1;
        let param_end = s.find(')')?;
        let param_str = &s[param_start..param_end];
        let angle = Self::parse_angle(param_str)?;
        
        let qubit_part = s[param_end + 1..].trim();
        let qubit = Self::extract_qubit_index(qubit_part)?;
        
        Some((angle, qubit))
    }
    
    fn extract_u_gate(s: &str) -> Option<((f64, f64, f64), usize)> {
        // handle "u(theta,phi,lambda) q[0]" format
        if !s.contains('(') {
            return None;
        }
        
        let param_start = s.find('(')? + 1;
        let param_end = s.find(')')?;
        let params_str = &s[param_start..param_end];
        let params: Vec<&str> = params_str.split(',').collect();
        
        if params.len() != 3 {
            return None;
        }
        
        let theta = Self::parse_angle(params[0].trim())?;
        let phi = Self::parse_angle(params[1].trim())?;
        let lambda = Self::parse_angle(params[2].trim())?;
        
        let qubit_part = s[param_end + 1..].trim();
        let qubit = Self::extract_qubit_index(qubit_part)?;
        
        Some(((theta, phi, lambda), qubit))
    }
    
    fn parse_angle(s: &str) -> Option<f64> {
        let s = s.trim();
        
        // handle pi, tau, euler constants
        if s.contains("pi") || s.contains("π") {
            let pi = std::f64::consts::PI;
            let s = s.replace("π", "pi");
            
            if s == "pi" {
                return Some(pi);
            } else if s.starts_with("pi/") {
                let denom: f64 = s[3..].parse().ok()?;
                return Some(pi / denom);
            } else if s.starts_with("pi*") {
                let mult: f64 = s[3..].parse().ok()?;
                return Some(pi * mult);
            } else if s.ends_with("*pi") {
                let mult: f64 = s[..s.len() - 3].parse().ok()?;
                return Some(mult * pi);
            } else if s.contains("*pi/") {
                let parts: Vec<&str> = s.split("*pi/").collect();
                if parts.len() == 2 {
                    let mult: f64 = parts[0].parse().ok()?;
                    let denom: f64 = parts[1].parse().ok()?;
                    return Some(mult * pi / denom);
                }
            }
        }
        
        if s.contains("tau") || s.contains("τ") {
            let tau = 2.0 * std::f64::consts::PI;
            let s = s.replace("τ", "tau");
            
            if s == "tau" {
                return Some(tau);
            } else if s.starts_with("tau/") {
                let denom: f64 = s[4..].parse().ok()?;
                return Some(tau / denom);
            } else if s.starts_with("tau*") {
                let mult: f64 = s[4..].parse().ok()?;
                return Some(tau * mult);
            }
        }
        
        if s.contains("euler") || s.contains("ℇ") {
            let euler = std::f64::consts::E;
            let s = s.replace("ℇ", "euler");
            
            if s == "euler" {
                return Some(euler);
            }
        }
        
        // handle plain numbers
        s.parse().ok()
    }
    
    fn extract_qubit_list(s: &str) -> Option<Vec<usize>> {
        let s = s.trim();
        if s.is_empty() {
            return Some(Vec::new());
        }
        
        let parts: Vec<&str> = s.split(',').map(|p| p.trim()).collect();
        let mut qubits = Vec::new();
        
        for part in parts {
            qubits.push(Self::extract_qubit_index(part)?);
        }
        
        Some(qubits)
    }
    
    fn extract_delay(s: &str) -> Option<(f64, String, usize)> {
        // handle "delay[100ns] q[0]" or "delay[1.5us] q[0]"
        if !s.contains('[') {
            return None;
        }
        
        let bracket_start = s.find('[')?;
        let bracket_end = s.find(']')?;
        let duration_str = &s[bracket_start + 1..bracket_end];
        
        // extract duration and unit
        let (duration, unit) = Self::parse_duration(duration_str)?;
        
        let qubit_part = s[bracket_end + 1..].trim();
        let qubit = Self::extract_qubit_index(qubit_part)?;
        
        Some((duration, unit, qubit))
    }
    
    fn parse_duration(s: &str) -> Option<(f64, String)> {
        let s = s.trim();
        
        // try to find where the number ends and unit begins
        let mut num_end = 0;
        for (i, c) in s.chars().enumerate() {
            if c.is_ascii_digit() || c == '.' || c == '-' {
                num_end = i + 1;
            } else {
                break;
            }
        }
        
        if num_end == 0 {
            return None;
        }
        
        let duration: f64 = s[..num_end].parse().ok()?;
        let unit = s[num_end..].trim().to_string();
        
        Some((duration, unit))
    }
    
    fn parse_if_block(lines: &[&str]) -> Result<(QasmInstruction, usize), String> {
        let first_line = lines[0].trim();
        
        // extract condition
        let if_start = first_line.find('(').ok_or("missing (")?;
        let if_end = first_line.find(')').ok_or("missing )")?;
        let condition = &first_line[if_start + 1..if_end];
        
        let parts: Vec<&str> = condition.split("==").map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return Err("invalid condition".to_string());
        }
        
        let bit = Self::extract_qubit_index(parts[0]).ok_or("invalid bit index")?;
        let value: usize = parts[1].parse().map_err(|_| "invalid value")?;
        
        // check if single-line if statement
        if !first_line.contains('{') {
            let instr_str = first_line[if_end + 1..].trim();
            if let Some(instr) = Self::parse_instruction(instr_str) {
                return Ok((QasmInstruction::If(bit, value, vec![instr]), 1));
            }
        }
        
        // multi-line if block
        let mut if_body = Vec::new();
        let mut else_body = Vec::new();
        let mut i = 1;
        let mut in_else = false;
        let mut brace_count = 1;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    i += 1;
                    break;
                }
            }
            
            if brace_count == 1 && line.starts_with("else") {
                in_else = true;
                i += 1;
                continue;
            }
            
            if !line.is_empty() && !line.starts_with("//") {
                if let Some(instr) = Self::parse_instruction(line) {
                    if in_else {
                        else_body.push(instr);
                    } else {
                        if_body.push(instr);
                    }
                }
            }
            
            i += 1;
        }
        
        if else_body.is_empty() {
            Ok((QasmInstruction::If(bit, value, if_body), i))
        } else {
            Ok((QasmInstruction::IfElse(bit, value, if_body, else_body), i))
        }
    }
    
    fn parse_while_block(lines: &[&str]) -> Result<(QasmInstruction, usize), String> {
        let first_line = lines[0].trim();
        
        let while_start = first_line.find('(').ok_or("missing (")?;
        let while_end = first_line.find(')').ok_or("missing )")?;
        let condition = &first_line[while_start + 1..while_end];
        
        let parts: Vec<&str> = condition.split("==").map(|p| p.trim()).collect();
        if parts.len() != 2 {
            return Err("invalid condition".to_string());
        }
        
        let bit = Self::extract_qubit_index(parts[0]).ok_or("invalid bit index")?;
        let value: usize = parts[1].parse().map_err(|_| "invalid value")?;
        
        let mut body = Vec::new();
        let mut i = 1;
        let mut brace_count = 1;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    i += 1;
                    break;
                }
            }
            
            if !line.is_empty() && !line.starts_with("//") {
                if let Some(instr) = Self::parse_instruction(line) {
                    body.push(instr);
                }
            }
            
            i += 1;
        }
        
        Ok((QasmInstruction::While(bit, value, body), i))
    }
    
    fn parse_for_block(lines: &[&str]) -> Result<(QasmInstruction, usize), String> {
        let first_line = lines[0].trim();
        
        // parse "for i in [0:10]" or similar
        let in_pos = first_line.find(" in ").ok_or("missing 'in'")?;
        let var_part = &first_line[3..in_pos].trim();
        
        let bracket_start = first_line.find('[').ok_or("missing [")?;
        let bracket_end = first_line.find(']').ok_or("missing ]")?;
        let range_part = &first_line[bracket_start + 1..bracket_end];
        
        let range_parts: Vec<&str> = range_part.split(':').collect();
        if range_parts.len() != 2 {
            return Err("invalid range".to_string());
        }
        
        let start: usize = range_parts[0].trim().parse().map_err(|_| "invalid start")?;
        let end: usize = range_parts[1].trim().parse().map_err(|_| "invalid end")?;
        
        let mut body = Vec::new();
        let mut i = 1;
        let mut brace_count = 1;
        
        while i < lines.len() {
            let line = lines[i].trim();
            
            if line.contains('{') {
                brace_count += line.matches('{').count();
            }
            if line.contains('}') {
                brace_count -= line.matches('}').count();
                if brace_count == 0 {
                    i += 1;
                    break;
                }
            }
            
            if !line.is_empty() && !line.starts_with("//") {
                if let Some(instr) = Self::parse_instruction(line) {
                    body.push(instr);
                }
            }
            
            i += 1;
        }
        
        Ok((QasmInstruction::For(var_part.to_string(), start, end, body), i))
    }
    
    fn parse_measurement_from_args(line: &str) -> Option<QasmInstruction> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return None;
        }
        
        // check for "measure q -> c" format (measure all)
        if parts.len() >= 4 && parts[2] == "->" {
            return Some(QasmInstruction::MeasureAll);
        }
        
        // handle "measure q[0] -> c[0]" format
        let qubit_str = parts.get(1)?;
        let qubit = Self::extract_qubit_index(qubit_str)?;
        
        let classical_str = parts.get(3)?;
        let classical = Self::extract_qubit_index(classical_str)?;
        
        Some(QasmInstruction::Measure(qubit, classical))
    }
}
