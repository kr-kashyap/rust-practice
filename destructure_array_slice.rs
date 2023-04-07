fn main() {
    let tuple = [1,2,3,4,5,6];
    match tuple {
        [0,..] => println!("1st element is 0. Rest doesn't matter."),
        [first,second,middle@..,second_last,last] => println!(
            "{}, {}, {:?}, {}, {}"
            ,first,second,middle,second_last,last
        ),
    }
}