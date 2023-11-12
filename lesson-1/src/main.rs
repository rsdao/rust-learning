mod print_chars {
    pub fn print_lower_a_to_upper_z(){
        let upper_a = 'A' as u8;
        let upper_z = 'Z' as u8;
        let lower_a = 'a' as u8;
        let lower_z = 'z' as u8;
        for i in 0..255{
            if i > lower_a && i<=lower_z {
                let c = char::from_u32(i.into()).unwrap();
                print!("{}", c);
            } else if i >= 128 && i - 128 >= upper_a && i -128 < upper_z {
                let c = char::from_u32((i-128).into()).unwrap();
                print!("{}", c);
            }

        }
    }
    pub mod print_upper_a_to_lower_z {
        pub fn print_upper_a_to_lower_z() {
            let upper_a = 'A' as u8;
            let upper_z = 'Z' as u8;
            let lower_a = 'a' as u8;
            let lower_z = 'z' as u8;

            for i in 0..127{
                if i > upper_a && i<=upper_z || i>=lower_a && i < lower_z {
                    let c = char::from_u32(i.into()).unwrap();
                    print!("{}", c);
                }
            }
        }
    }
}

fn main() {
    println!("[+] 'a'~'Z'之间的所有字符: ");
    print_chars::print_lower_a_to_upper_z();
    println!("\n[+] 'A'~'z'之间的所有字符: ");
    print_chars::print_upper_a_to_lower_z::print_upper_a_to_lower_z();
    println!();
}