use std::io::BufRead;
use super::aig::{AigEdge,and,not,var};
use nom::*;
use std::str::FromStr;
use std::num::ParseIntError;
use ascii::AsAsciiStr;


fn from_bytes(b: &[u8]) -> u32 {
    FromStr::from_str(b.as_ascii_str().expect("error parsing header").as_str()).expect("error parsing header")
}

named!(magic_head, tag!("aig"));
named!(uint32 <&[u8], u32>,
    preceded!(tag!(" "),
                map!(digit, from_bytes)
    )
);
named!(header_p<&[u8], (u32,u32,u32,u32,u32)>, 
    preceded!(magic_head,tuple!(uint32, uint32, uint32, uint32, uint32))
);


pub fn read_aig(r :&mut BufRead) -> Result<AigEdge, &'static str> {
    let mut header = String::new(); 
    if let Err(i) =  r.read_line(&mut header) {
        return Err("no header");
    }
        
    let parser= header_p(header.as_ref());


    Ok(var(String::from("a")))
}