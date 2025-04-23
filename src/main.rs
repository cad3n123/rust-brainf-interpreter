fn main() {
    let mut bytes: Box<[u8]> = vec![0; u16::MAX as usize].into_boxed_slice();
    let mut pointer: u16 = 0;
    let mut stack = Vec::with_capacity(10);

    let input_code = "++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.".as_bytes();
    let total_chars = input_code.len();
    let mut i = 0;
    while i < total_chars {
        let c = input_code[i] as char;
        match c {
            '+' => {
                bytes[pointer as usize] += 1;
            }
            '-' => {
                bytes[pointer as usize] -= 1;
            }
            '>' => {
                pointer += 1;
            }
            '<' => {
                pointer -= 1;
            }
            '.' => {
                print!("{}", bytes[pointer as usize] as char);
            }
            '[' => {
                stack.push(i);
            }
            ']' => {
                if bytes[pointer as usize] == 0 {
                    stack.pop();
                } else {
                    i = *stack.last().unwrap();
                }
            }
            _ => {}
        }
        i += 1;
    }
}
