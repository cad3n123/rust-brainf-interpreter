use std::io::{self, Write};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    enable_raw_mode().expect("Failed to enter raw mode");

    let mut bytes: Box<[u8]> = vec![0; u16::MAX as usize + 1].into_boxed_slice();
    let mut pointer: u16 = 0;
    let mut stack = Vec::with_capacity(10);

    let input_code = b"-[>+<-------]>-.>-[>++<-----]>-.>>+[++[++>]<<+]>+..>>+[+>+[<]>->]<.";
    let total_chars = input_code.len();
    let mut i = 0;
    while i < total_chars {
        let c = input_code[i] as char;
        match c {
            '+' => {
                bytes[pointer as usize] = bytes[pointer as usize].wrapping_add(1);
            }
            '-' => {
                bytes[pointer as usize] = bytes[pointer as usize].wrapping_sub(1);
            }
            '>' => {
                pointer = pointer.wrapping_add(1);
            }
            '<' => {
                pointer = pointer.wrapping_sub(1);
            }
            '.' => {
                print!("{}", bytes[pointer as usize] as char);
                io::stdout().flush().expect("Error flushing stdout");
            }
            ',' => loop {
                if let Ok(Event::Key(event)) = event::read() {
                    if event.kind == KeyEventKind::Press {
                        if event.modifiers.contains(KeyModifiers::CONTROL)
                            && event.code == KeyCode::Char('c')
                        {
                            disable_raw_mode().ok(); // cleanup
                            std::process::exit(-1);
                        }

                        match event.code {
                            KeyCode::Enter => {
                                bytes[pointer as usize] = b'\n';
                                break;
                            }
                            KeyCode::Backspace => {
                                bytes[pointer as usize] = 8;
                                break;
                            }
                            KeyCode::Char(c) => {
                                bytes[pointer as usize] = c as u8;
                                break;
                            }
                            _ => {}
                        }
                    }
                }
            },
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

    disable_raw_mode().expect("Failed to exit raw mode");
}
