use core::num;

fn main() {
    println!("Hello, world!");
    let data = Solution;
    // data.convert("hello".to_string(), 2);
    data.convert3("PAYPALISHIRING".to_string(), 4);
}

struct Solution;

impl Solution {
    pub fn convert(self, s: String, num_rows: i32) {
        let lenght = (s.len() - 1) as i32;
        let data = s.into_bytes();
        let mut m = 0;
        let mut final_data: Vec<u8> = vec![];
        for i in data.clone() {
            if m < num_rows {
                let mut n = m;
                println!("{m}");
                loop {
                    if n <= lenght {
                        final_data.push(data[n as usize]);
                        n += num_rows + num_rows - 1;
                        println!("{:?}", String::from_utf8(final_data.clone()));
                    } else {
                        break;
                    }
                }
                m += 1;
            } else {
                break;
            }
        }
        println!("{:?}", String::from_utf8(final_data));
    }

    fn zigzag(self, s: String, num_rows: i32) {
        let lenght = (s.len() - 1) as i32;
        let data = s.into_bytes();
        let mut m = 0;
        let mut final_data: Vec<u8> = vec![];
        let mut decition = true;

        for i in data.clone() {
            let mut n = m;
            if m < num_rows {
                loop {
                    if n <= lenght {
                        if m == 0 || m == num_rows - 1 {
                            final_data.push(data[n as usize]);
                            println!("{n}");
                            n += (num_rows * 2) - 2;
                            println!("{:?}", String::from_utf8(final_data.clone()));
                        } else {
                            // m = 1 
                            let o= n;
                            if decition{
                                final_data.push(data[n as usize]);
                                n += (num_rows * 2) - 2 - m - 1 ;
                                println!("{n}");
                                println!("treu {:?}", String::from_utf8(final_data.clone()));
                                decition = false
                            }
                            else {
                                // n=o;
                                // n=m;
                                final_data.push(data[n as usize]);
                                println!("{n}");
                                n += (num_rows * 2) - 2;
                                println!("fales {:?}", String::from_utf8(final_data.clone()));
                                decition = true;
                                // n=o;
                            }
                        }
                    } else {
                        break;
                    }
                }
                m += 1;
            } else {
                break;
            }
        }
    }
    fn zigzag2(self, s: String, num_rows: i32) {
        let mut vecString = s.into_bytes();
        let mut string: Vec<u8> = vec![];
        for r in 1..num_rows{
            for i in r..((num_rows -1)*2)  {
                if r==0 || r== num_rows - 1 {
                    string.push(vecString[i as usize]);
                }
            }
        }
        println!("fales {:?}", String::from_utf8(string.clone()));
    }

 pub fn convert2(self,s: String, num_rows: i32) -> String {
  let mut floors: Vec<String> = vec![String::from(""); num_rows as usize];

  if num_rows < 2 {
   return s.into();
  }

  let mut floor = 0;
  let mut down: bool = true;

  for c in s.chars() {
      floors[floor as usize].push(c);
      floor += if down { 1 } else { -1 };
      down = down == (floor > 0 && floor < num_rows - 1);
      println!("{:?}",floors);
  }
println!("{:?}",floors);
  floors.concat()
 }

  fn convert3(self,s: String, num_rows: i32)-> String{
    if num_rows < 2{
        return s.into();
    }
    let mut string: Vec<String> = vec![String::from("");num_rows as usize];

    let mut decition = 0;
    let mut arrow = true;

    for char in s.chars(){
        string[decition as usize].push(char);
        decition += if arrow{1} else { - 1};
        arrow = arrow==(decition>0&&decition<num_rows-1);
    }
    string.concat()
  }
}

//PAYPALISHIRING
