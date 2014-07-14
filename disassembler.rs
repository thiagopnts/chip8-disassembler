
use std::io::fs::File;
use std::vec::Vec;
use std::iter::range_step;
use std::os;

fn disassemble(buffer: Vec<u8>) {
  let mut address: int = 0x200;
  for i in range_step(0, buffer.len(), 2) {
    println!("{:04x} {:02x} {:02x}", address, *buffer.get(i), *buffer.get(i + 1));
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

