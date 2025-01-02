fn process_data(file_path: &str) -> Vec<[String; 4]> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");
    let mut lines = file_content.lines();

    lines.by_ref().find(|line| line.is_empty());

    let gates: Vec<[String; 4]> = lines
        .map(|line| {
            line.replace("-> ", "")
                .split(' ')
                .map(|part| part.to_owned())
                .collect::<Vec<String>>()
                .try_into()
                .expect("Should contain four String values")
        })
        .collect();

    gates
}

fn find_swapped_wires(gates: &[[String; 4]]) -> String {
    let gate_exists = |gates: &[[String; 4]], output_wire: &str, next_operation: &str| -> bool {
        gates
            .iter()
            .any(|[input_wire_1, operation, input_wire_2, _]| {
                next_operation == operation
                    && (output_wire == input_wire_1 || output_wire == input_wire_2)
            })
    };

    let z_max = gates
        .iter()
        .map(|[_, _, _, output_wire]| output_wire)
        .filter(|output_wire| output_wire.starts_with('z'))
        .max()
        .expect("Should contain more than one z wire");

    let mut wrong_wires: Vec<&str> = Vec::new();

    for [input_wire_1, operation, _, output_wire] in gates {
        match operation.as_str() {
            "AND" => {
                if input_wire_1 != "x00"
                    && input_wire_1 != "y00"
                    && !gate_exists(gates, output_wire, "OR")
                {
                    wrong_wires.push(output_wire);
                }
            }
            "OR" => {
                if output_wire.starts_with('z') && output_wire != z_max {
                    wrong_wires.push(output_wire);
                }
            }
            "XOR" => {
                if input_wire_1.starts_with('x') || input_wire_1.starts_with('y') {
                    if input_wire_1 != "x00"
                        && input_wire_1 != "y00"
                        && !gate_exists(gates, output_wire, "XOR")
                    {
                        wrong_wires.push(output_wire);
                    }
                } else if !output_wire.starts_with('z') {
                    wrong_wires.push(output_wire);
                }
            }
            _ => unreachable!(),
        }
    }

    wrong_wires.sort_unstable();
    wrong_wires.join(",")
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let gates: Vec<[String; 4]> = process_data("./input/24.txt");
    let result: String = find_swapped_wires(&gates);
    Ok(format!("Day 24 Crossed Wires (Part 2): {}.", result))
}
