use super::aig::AigEdge;
use ascii::AsciiStr;
use nom::*;
use nom::number::streaming::be_u8;
use nom::character::streaming::digit1;
use nom::character::is_digit;
use std::io::BufRead;
use std::str::FromStr;

//fn from_bytes(b: &[u8]) -> u32 {
//   b.as_ascii_str().map_err(?.as_str()FromStr::from_str().expect("error parsing header")
//}

named!(magic_head, tag!("aig"));
named!(asciiuint <&[u8], u32>,map_res!(map!(map_res!(digit1, AsciiStr::from_ascii), |s| s.as_str()), FromStr::from_str));
named!(uint32<&[u8], u32>, preceded!(tag!(" "), asciiuint));
named!(header_p<&[u8], (u32,u32,u32,u32,u32)>,
    preceded!(magic_head,tuple!(uint32, uint32, uint32, uint32, uint32))
);

fn encode_int(encoded: &[u8]) -> IResult<&[u8], u32> {
    let mut ret: u32 = 0;
    let mut index = 0;
    let mut encoded_i = encoded;
    loop {
        match be_u8(encoded_i) {
            Ok((encoded1, byte)) => {
                encoded_i = encoded1;
                let mut converted_byte: u32 = 0;

                if byte < 0x80 {
                    converted_byte += (byte as u32);
                    converted_byte <<= index;
                    ret += converted_byte;
                    break;
                } else {
                    converted_byte += (byte & 0x7F) as u32;
                    converted_byte << index;
                    index += 7;
                }
            }
            Err(e) => return Err(e)
        }
    }

    Ok((encoded_i, ret))
}

named!(output_list<&[u8], Vec<u32>>, separated_list!(tag!("\n"), uint32));

named!(gate_inputs<&[u8], (u32,u32)>, tuple!(encode_int, encode_int));

pub fn read_aig(r: &mut BufRead) -> Result<AigEdge, &'static str> {
    let mut header = String::new();
    if let Err(i) = r.read_line(&mut header) {
        return Err("no header");
    }

    let parsedHeader = header_p(header.as_ref());

    println!("{:?}", parsedHeader);

    //Ok(var(String::from("a")))
    Err("unimpl")
}
