
use std::io::fs::File;
use std::vec::Vec;
use std::iter::range_step;
use std::os;

fn disassemble(buffer: Vec<u8>) {
  let mut address: int = 0x200;
  for i in range_step(0, buffer.len(), 2) {
    let first_part = *buffer.get(i);
    let second_part = *buffer.get(i + 1);
    let instruction = match first_part >> 4 {
      0x0 => match second_part {
        0xE0 => "CLS".to_string(),
        0xEE => "RTS".to_string(),
        _    => "unkwown".to_string(),
      },
      0x1   => format!("JUMP ${:01x}{:02x}", first_part & 0xF, second_part),
      0x2   => format!("CALL ${:01x}{:02x}", first_part & 0xF, second_part),
      0x3   => format!("SKIP.EQ V{:01X},#${:02x}", first_part & 0xF, second_part),
      0x4   => format!("SKIP.NE V{:01X},#${:02x}", first_part & 0xF, second_part),
      0x5   => format!("SKIP.EQ V{:01X},V{:01X}", first_part & 0xF, second_part >> 4),
      0x6   => format!("MVI V{:01X},#${:02x}", first_part & 0xF, second_part),
      0x7   => format!("ADI V{:01X},#${:02x}", first_part & 0xF, second_part),
      0x8   => match first_part >> 4 {
        0 => format!("MOV. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        1 => format!("OR. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        2 => format!("AND. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        3 => format!("XOR. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        4 => format!("ADD. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        5 => format!("SUB. V{:01X},V{:01X},V{:01X}", first_part & 0x0F, first_part & 0x0F, second_part >> 4),
        6 => format!("SHR. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        7 => format!("SUB. V{:01X},V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4, second_part >> 4),
        0xE => format!("SHL. V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
        _   => "unknown".to_string(),
      },
      0x9 => format!("SKIP.NE V{:01X},V{:01X}", first_part & 0x0F, second_part >> 4),
      0xA => format!("MVI I,#${:01x}{:02x}", first_part & 0x0F, second_part),
      0xB => format!("JUMP ${:01x}{:02x}(V0)", first_part & 0x0F, second_part),
      0xC => format!("RNDMSK V{:01X},#${:02x}", first_part & 0x0F, second_part),
      _     => "not implemented".to_string(),
    };
    println!("{:04x} {:02x} {:02x} {}", address, first_part, second_part, instruction);
    address += 2;
  }
}

fn main() {
  let path = Path::new(os::args().get(1).to_string());
  let mut file = match File::open(&path) {
    Ok(f) => f,
    Err(e) => fail!("error opening file: {}", e),
  };

  let buf = match file.read_to_end() {
    Ok(content) => content,
    Err(e) => fail!("error: {}", e),
  };

  disassemble(buf);
}

