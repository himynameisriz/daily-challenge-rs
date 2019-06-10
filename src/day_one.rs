pub fn calc(input: &[i32], expect: i32) -> (&i32, &i32) {
    for (i, number) in input.iter().enumerate() {
        let find: i32 = expect - number;
        // println!("we are looking at {} and are looking for {}", number, find);
        if i == input.len()-1 { panic!("{}", "None found"); }
        for n in input.iter().skip(i+1) {
            // println!("we are comparing {} and {}", number, n);
            if n == &find { return (number, n); }
        }
    }
    panic!("{}", "None found");
}
