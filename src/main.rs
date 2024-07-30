fn main ()  {
    loop{
        let x = Box::new(5);
        if *x == 5 {
            let y = 45;
            let z = 10;
            let x = 45;
            let t = y+z+x;
            if t==100 {
            println!("The value is ok {t}")
        }
        println!("All test case has been passed ");
            }
    }
}