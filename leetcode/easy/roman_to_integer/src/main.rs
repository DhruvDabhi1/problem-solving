use std::{borrow::Borrow, vec};

fn main() {
    println!("Hello, world!");
    let solve = Solution;
    solve.roman_to_integer(/*"MCMXCIV"*/"XVIII".to_string());
    // solve.roman_to("VII".to_string());
}
struct Solution;

impl Solution{
    pub  fn roman_to_integer(&self,s: String) -> i32{
        #[derive(Debug,PartialEq, PartialOrd)]
      enum Roman{
          I = 1,
          V = 5,
          X = 10,
          L = 50,
          C = 100,
          D = 500,
          M =1000 
      }
    let mut c = 0;
    let mut value = Roman::I;
    let mut vec_value:Vec<Roman> = Vec::new();
    for i in s.as_bytes(){
        match i {
            //i
            73 => {vec_value.push(Roman::I)},
            //v
            86 => {vec_value.push(Roman::V)},
            //x
            88 => {vec_value.push(Roman::X)},
            //l
            76 => {vec_value.push(Roman::L)},
            //c
            67 => {vec_value.push(Roman::C)},
            //d
            68 => {vec_value.push(Roman::D)},
            //m
            77 => {vec_value.push(Roman::M)},
            _=> {c += 0}
        }
    }

    let mut m = 0;
        for i in vec_value.iter(){
            println!("{:?}",i);
            if i < vec_value.get(m+1).unwrap_or(&Roman::I){

                match i {
                Roman::I => {c -= 1},
                Roman::V => {c -= 5},
                Roman::X => {c -= 10},
                Roman::L => {c -= 50},
                Roman::C => {c -= 100},
                Roman::D => {c -= 500},
                Roman::M => {c -= 1000},
                _=> {c += 0}
            }
            m +=1;
            }else{
                match i {
                Roman::I => {c += 1},
                Roman::V => {c += 5},
                Roman::X => {c += 10},
                Roman::L => {c += 50},
                Roman::C => {c += 100},
                Roman::D => {c += 500},
                Roman::M => {c += 1000},
                _=> {c += 0}
            }
            m+=1;
        }
        }
        println!("{}",c);
        c
    }

    pub fn roman_to(&self,s:String){
      
        
        
    }
}