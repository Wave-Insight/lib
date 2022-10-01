use serde_derive::{Deserialize, Serialize};
use std::fmt::{self, Display};
use bitvec::macros::internal::funty::Fundamental;
use anyhow::Result;
use crate::log::InvalidData;
// use crate::log::;
/// A four-valued logic scalar value.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Value {
    /// Logic low (prefixed with `V` to make a valid Rust identifier)
    V0,

    /// Logic high (prefixed with `V` to make a valid Rust identifier)
    V1,

    /// An uninitialized or unknown value
    X,

    /// The "high-impedance" value
    Z,
}
use Value::*;
impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        write!(
            f,
            "{}",
            match *self {
                V0 => "0",
                V1 => "1",
                X => "X",
                Z => "Z",
            }
        )
    }
}

impl Value {
    fn parse(v: &u8) -> Result<Value,InvalidData> {
        match v {
            b'0' => Ok(V0),
            b'1' => Ok(V1),
            b'x' | b'X' => Ok(X),
            b'z' | b'Z' => Ok(Z),
            _ => Err(InvalidData::new(format!("Invalid VCD value: {:?}",v.as_char()))),
        }
    }
    fn decode(v: &u8) -> [Self; 4] {
        VALUE_LUT[v.as_usize()]
    }
    fn to_u8(&self) -> u8 {
        match self {
            V0 => 0,
            V1 => 1,
            X  => 2,
            Z  => 3,
        }
    }
    fn encode(values: [Self; 4]) -> u8 {
        values[0].to_u8()+values[1].to_u8()*4+values[2].to_u8()*16+values[3].to_u8()*64
    }
}
const VALUE_LUT:[[Value;4];256] = [
    //0
    [V0, V0, V0, V0],
    [V1, V0, V0, V0],
    [ X, V0, V0, V0],
    [ Z, V0, V0, V0],
    //4
    [V0, V1, V0, V0],
    [V1, V1, V0, V0],
    [ X, V1, V0, V0],
    [ Z, V1, V0, V0],
    //8
    [V0,  X, V0, V0],
    [V1,  X, V0, V0],
    [ X,  X, V0, V0],
    [ Z,  X, V0, V0],
    //12
    [V0,  Z, V0, V0],
    [V1,  Z, V0, V0],
    [ X,  Z, V0, V0],
    [ Z,  Z, V0, V0],
    //16
    [V0, V0, V1, V0],
    [V1, V0, V1, V0],
    [ X, V0, V1, V0],
    [ Z, V0, V1, V0],
    //20
    [V0, V1, V1, V0],
    [V1, V1, V1, V0],
    [ X, V1, V1, V0],
    [ Z, V1, V1, V0],
    //24
    [V0,  X, V1, V0],
    [V1,  X, V1, V0],
    [ X,  X, V1, V0],
    [ Z,  X, V1, V0],
    //28
    [V0,  Z, V1, V0],
    [V1,  Z, V1, V0],
    [ X,  Z, V1, V0],
    [ Z,  Z, V1, V0],
    //32
    [V0, V0,  X, V0],
    [V1, V0,  X, V0],
    [ X, V0,  X, V0],
    [ Z, V0,  X, V0],
    //36
    [V0, V1,  X, V0],
    [V1, V1,  X, V0],
    [ X, V1,  X, V0],
    [ Z, V1,  X, V0],
    //40
    [V0,  X,  X, V0],
    [V1,  X,  X, V0],
    [ X,  X,  X, V0],
    [ Z,  X,  X, V0],
    //44
    [V0,  Z,  X, V0],
    [V1,  Z,  X, V0],
    [ X,  Z,  X, V0],
    [ Z,  Z,  X, V0],
    //48
    [V0, V0,  Z, V0],
    [V1, V0,  Z, V0],
    [ X, V0,  Z, V0],
    [ Z, V0,  Z, V0],
    //52
    [V0, V1,  Z, V0],
    [V1, V1,  Z, V0],
    [ X, V1,  Z, V0],
    [ Z, V1,  Z, V0],
    //56
    [V0,  X,  Z, V0],
    [V1,  X,  Z, V0],
    [ X,  X,  Z, V0],
    [ Z,  X,  Z, V0],
    //60
    [V0,  Z,  Z, V0],
    [V1,  Z,  Z, V0],
    [ X,  Z,  Z, V0],
    [ Z,  Z,  Z, V0],
    //64
    [V0, V0, V0, V1],
    [V1, V0, V0, V1],
    [ X, V0, V0, V1],
    [ Z, V0, V0, V1],
    //68
    [V0, V1, V0, V1],
    [V1, V1, V0, V1],
    [ X, V1, V0, V1],
    [ Z, V1, V0, V1],
    //72
    [V0,  X, V0, V1],
    [V1,  X, V0, V1],
    [ X,  X, V0, V1],
    [ Z,  X, V0, V1],
    //76
    [V0,  Z, V0, V1],
    [V1,  Z, V0, V1],
    [ X,  Z, V0, V1],
    [ Z,  Z, V0, V1],
    //80
    [V0, V0, V1, V1],
    [V1, V0, V1, V1],
    [ X, V0, V1, V1],
    [ Z, V0, V1, V1],
    //84
    [V0, V1, V1, V1],
    [V1, V1, V1, V1],
    [ X, V1, V1, V1],
    [ Z, V1, V1, V1],
    //88
    [V0,  X, V1, V1],
    [V1,  X, V1, V1],
    [ X,  X, V1, V1],
    [ Z,  X, V1, V1],
    //92
    [V0,  Z, V1, V1],
    [V1,  Z, V1, V1],
    [ X,  Z, V1, V1],
    [ Z,  Z, V1, V1],
    //96
    [V0, V0,  X, V1],
    [V1, V0,  X, V1],
    [ X, V0,  X, V1],
    [ Z, V0,  X, V1],
    //100
    [V0, V1,  X, V1],
    [V1, V1,  X, V1],
    [ X, V1,  X, V1],
    [ Z, V1,  X, V1],
    //104
    [V0,  X,  X, V1],
    [V1,  X,  X, V1],
    [ X,  X,  X, V1],
    [ Z,  X,  X, V1],
    //108
    [V0,  Z,  X, V1],
    [V1,  Z,  X, V1],
    [ X,  Z,  X, V1],
    [ Z,  Z,  X, V1],
    //112
    [V0, V0,  Z, V1],
    [V1, V0,  Z, V1],
    [ X, V0,  Z, V1],
    [ Z, V0,  Z, V1],
    //116
    [V0, V1,  Z, V1],
    [V1, V1,  Z, V1],
    [ X, V1,  Z, V1],
    [ Z, V1,  Z, V1],
    //120
    [V0,  X,  Z, V1],
    [V1,  X,  Z, V1],
    [ X,  X,  Z, V1],
    [ Z,  X,  Z, V1],
    //124
    [V0,  Z,  Z, V1],
    [V1,  Z,  Z, V1],
    [ X,  Z,  Z, V1],
    [ Z,  Z,  Z, V1],
    //128
    [V0, V0, V0,  X],
    [V1, V0, V0,  X],
    [ X, V0, V0,  X],
    [ Z, V0, V0,  X],
    //132
    [V0, V1, V0,  X],
    [V1, V1, V0,  X],
    [ X, V1, V0,  X],
    [ Z, V1, V0,  X],
    //136
    [V0,  X, V0,  X],
    [V1,  X, V0,  X],
    [ X,  X, V0,  X],
    [ Z,  X, V0,  X],
    //140
    [V0,  Z, V0,  X],
    [V1,  Z, V0,  X],
    [ X,  Z, V0,  X],
    [ Z,  Z, V0,  X],
    //144
    [V0, V0, V1,  X],
    [V1, V0, V1,  X],
    [ X, V0, V1,  X],
    [ Z, V0, V1,  X],
    //148
    [V0, V1, V1,  X],
    [V1, V1, V1,  X],
    [ X, V1, V1,  X],
    [ Z, V1, V1,  X],
    //152
    [V0,  X, V1,  X],
    [V1,  X, V1,  X],
    [ X,  X, V1,  X],
    [ Z,  X, V1,  X],
    //156
    [V0,  Z, V1,  X],
    [V1,  Z, V1,  X],
    [ X,  Z, V1,  X],
    [ Z,  Z, V1,  X],
    //160
    [V0, V0,  X,  X],
    [V1, V0,  X,  X],
    [ X, V0,  X,  X],
    [ Z, V0,  X,  X],
    //164
    [V0, V1,  X,  X],
    [V1, V1,  X,  X],
    [ X, V1,  X,  X],
    [ Z, V1,  X,  X],
    //168
    [V0,  X,  X,  X],
    [V1,  X,  X,  X],
    [ X,  X,  X,  X],
    [ Z,  X,  X,  X],
    //172
    [V0,  Z,  X,  X],
    [V1,  Z,  X,  X],
    [ X,  Z,  X,  X],
    [ Z,  Z,  X,  X],
    //176
    [V0, V0,  Z,  X],
    [V1, V0,  Z,  X],
    [ X, V0,  Z,  X],
    [ Z, V0,  Z,  X],
    //180
    [V0, V1,  Z,  X],
    [V1, V1,  Z,  X],
    [ X, V1,  Z,  X],
    [ Z, V1,  Z,  X],
    //184
    [V0,  X,  Z,  X],
    [V1,  X,  Z,  X],
    [ X,  X,  Z,  X],
    [ Z,  X,  Z,  X],
    //188
    [V0,  Z,  Z,  X],
    [V1,  Z,  Z,  X],
    [ X,  Z,  Z,  X],
    [ Z,  Z,  Z,  X],
    //192
    [V0, V0, V0,  Z],
    [V1, V0, V0,  Z],
    [ X, V0, V0,  Z],
    [ Z, V0, V0,  Z],
    //196
    [V0, V1, V0,  Z],
    [V1, V1, V0,  Z],
    [ X, V1, V0,  Z],
    [ Z, V1, V0,  Z],
    //200
    [V0,  X, V0,  Z],
    [V1,  X, V0,  Z],
    [ X,  X, V0,  Z],
    [ Z,  X, V0,  Z],
    //204
    [V0,  Z, V0,  Z],
    [V1,  Z, V0,  Z],
    [ X,  Z, V0,  Z],
    [ Z,  Z, V0,  Z],
    //208
    [V0, V0, V1,  Z],
    [V1, V0, V1,  Z],
    [ X, V0, V1,  Z],
    [ Z, V0, V1,  Z],
    //212
    [V0, V1, V1,  Z],
    [V1, V1, V1,  Z],
    [ X, V1, V1,  Z],
    [ Z, V1, V1,  Z],
    //216
    [V0,  X, V1,  Z],
    [V1,  X, V1,  Z],
    [ X,  X, V1,  Z],
    [ Z,  X, V1,  Z],
    //220
    [V0,  Z, V1,  Z],
    [V1,  Z, V1,  Z],
    [ X,  Z, V1,  Z],
    [ Z,  Z, V1,  Z],
    //224
    [V0, V0,  X,  Z],
    [V1, V0,  X,  Z],
    [ X, V0,  X,  Z],
    [ Z, V0,  X,  Z],
    //228
    [V0, V1,  X,  Z],
    [V1, V1,  X,  Z],
    [ X, V1,  X,  Z],
    [ Z, V1,  X,  Z],
    //232
    [V0,  X,  X,  Z],
    [V1,  X,  X,  Z],
    [ X,  X,  X,  Z],
    [ Z,  X,  X,  Z],
    //236
    [V0,  Z,  X,  Z],
    [V1,  Z,  X,  Z],
    [ X,  Z,  X,  Z],
    [ Z,  Z,  X,  Z],
    //240
    [V0, V0,  Z,  Z],
    [V1, V0,  Z,  Z],
    [ X, V0,  Z,  Z],
    [ Z, V0,  Z,  Z],
    //244
    [V0, V1,  Z,  Z],
    [V1, V1,  Z,  Z],
    [ X, V1,  Z,  Z],
    [ Z, V1,  Z,  Z],
    //248
    [V0,  X,  Z,  Z],
    [V1,  X,  Z,  Z],
    [ X,  X,  Z,  Z],
    [ Z,  X,  Z,  Z],
    //252
    [V0,  Z,  Z,  Z],
    [V1,  Z,  Z,  Z],
    [ X,  Z,  Z,  Z],
    [ Z,  Z,  Z,  Z],
];

/// Hello world example for Rust.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WavePlot {
    /// Hello world example for Rust.
    pub size: usize,
    /// Hello world example for Rust.
    pub len: usize,
    /// Hello world example for Rust.
    pub x_time: Vec<usize>,
    /// Hello world example for Rust.
    pub y_wave: Vec<Vec<u8>>,
}
impl WavePlot {
    #[must_use]
    #[inline]
    /// Hello world example for Rust.
    pub fn new(
        size: usize,
    ) -> Self {
        let len:usize;
        if size%4==0 {
            len = size/4;
        } else {
            len = size/4+1;
        };
        let wave_data= vec![170;len];
        Self{
            size,
            len,
            x_time: vec![0],
            y_wave: vec![wave_data],
        }
    }
    /// Hello world example for Rust.
    pub fn add_wave(&mut self, x_time: usize, y_wave_str: &str)->Result<(),InvalidData>{
        if y_wave_str.len() != self.size{
            return Err(InvalidData::new(format!(
                "Length of wave_str is not match to wave size, wave_str={}",y_wave_str
            )));
        }
        let mut wave_data= vec![170;self.len];
        let mut buf = [X;4];
        let mut idx_v:usize = 0;
        let mut idx_data:usize = 0;
        for v in y_wave_str.as_bytes(){
            let v = Value::parse(v)?;
            buf[idx_v] = v;
            if idx_v == 3{
                wave_data[idx_data] = Value::encode(buf);
                idx_data = idx_data + 1;
                idx_v = 0;
            }else {
                idx_v = idx_v + 1;
            }
        }
        wave_data[self.len-1] = Value::encode(buf);
        if x_time==0 {
            self.y_wave[0] = wave_data;
        }else{
            self.x_time.push(x_time);
            self.y_wave.push(wave_data);
        }
        Ok(())
    }
    /// Hello world example for Rust.
    pub fn decode_waves(&self)->Result<Vec<Vec<Value>>>{
        let mut waves:Vec<Vec<Value>>= vec![vec![];self.y_wave.len()];
        let mut buf:Vec<Value> = vec![V0;self.len*4];
        for (i, wave) in self.y_wave.iter().enumerate() {
            for (j, encode_date) in wave.iter().enumerate() {
                let values = Value::decode(encode_date);
                for (k, value) in values.iter().enumerate() {
                    buf[j*4+k] = value.to_owned();
                }
            }
            waves[i] = buf[..self.size].to_vec();
        }
        Ok(waves)
    }
}

/// Hello world example for Rust.
#[cfg(test)]
mod test {
    use super::WavePlot;
    use super::Value::*;
    #[test]
    fn tt() {
        let mut x = WavePlot::new(5);
        println!("{:?}",x);
        println!("{:?}",x.decode_waves());
        let e= x.add_wave(2, "1xz00");
        println!("{:?}",e);
        println!("{:?}",x);
        if let Ok(waves) = x.decode_waves(){
            let w = vec![vec![X,X,X,X,X],vec![V1,X,Z,V0,V0]];
            assert_eq!(waves,w)
        };
    }
}