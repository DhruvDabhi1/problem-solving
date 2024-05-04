
fn main() {
    println!("Hello, world!");
    let data = Solution;
    data.longest_common_prefix5(vec!["car".to_string(), "cir".to_string()]);
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(self, strs: Vec<String>) {
        strs.iter().fold(strs[0].clone(), |acc, num| {
            for c in num.chars() {}
            num.to_string()
        });

        let mut pre_data = String::new();
        for (index, value) in strs.iter().enumerate() {
            // println!("{index},{value}");
            // println!("{pre_data}");
            // pre_data = value.to_string();
            // println!("{pre_data}");
            //   let demo_data = value.chars();
            //   demo_data.into_iter().all();
            //   println!("{:?}",demo_data)
        }
    }

    pub fn longest_common_prefix2(self, strs: Vec<String>) {
        let mut m = 0;
        strs.iter().fold(strs[0].clone(), |acc, num| {
            let mut longest_prifix: Vec<u8> = Vec::new();
            let string_bytes = num.clone().into_bytes();
            let string_bytes_acc = acc.into_bytes();
            println!("{:?} and {:?}", string_bytes, string_bytes_acc);
            for i in 0..string_bytes.len() {
                // println!("{i}");
                let mut copy_i = i;
                for j in 0..string_bytes_acc.clone().len() {
                    if string_bytes[copy_i] == string_bytes_acc[j] {
                        println!(
                            "{:?} & {:?}",
                            string_bytes[i] as char, string_bytes_acc[j] as char
                        );
                        longest_prifix.push(string_bytes[i]);
                    }
                    copy_i += 1;
                }
            }
            // println!("{:?}",string_bytes);
            if strs.len() != (m + 1) {
                println!("done");
                m += 1;
            }
            // strs[m].clone().to_string()
            println!("{:?}", String::from_utf8(longest_prifix.clone()).unwrap());
            String::from_utf8(longest_prifix).unwrap()
        });
    }

    fn longest_common_prefix3(self, strs: Vec<String>) {
        let mut pri:Vec<u8> = vec![];
        for i in 0..strs[0].clone().len() {
            for j in strs.clone().into_iter() {
                println!("{j}");
                let meta_data = j.clone().into_bytes();
                if i == j.clone().len() || meta_data[i] != strs[0].clone().into_bytes()[i] {
                    println!("{:?}",String::from_utf8(vec![strs[0].clone().into_bytes()[i]]));
                    pri.push(strs[0].clone().into_bytes()[i])
                }
            }
        }
        println!("{:?}",String::from_utf8(pri));
    }
fn longest_common_prefix4(self, strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }

let pre=strs.iter().fold(strs[0].clone(), |prefix, next_word| {
        prefix.chars().zip(next_word.chars())
            .take_while(|&(a, b)| a == b)
            .map(|(a, _)| a)
            .collect()
    });
    println!("{:?}",pre);
    pre

}

fn longest_common_prefix5(self,strs: Vec<String>) ->String{
     if strs.is_empty() {
        return "".to_string();
    }
    let mut common_pre=String::new();
    let pre= strs.iter().fold(strs[0].clone(), |current, last_word|{
         let mut pre:String = String::new();
         'a :for (i,j )in current.chars().zip(last_word.clone().chars()){
            if i==j {
                pre +=&i.to_string();           
            }
            if i!=j{
                break 'a;
            }
        }
        pre.clone()
    });
    println!("{pre}");
    pre
}
}
//monday
//monhay
