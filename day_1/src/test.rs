#[cfg(test)]
mod tests {

    extern crate test;


    use crate::spin;
    use test::Bencher;

    #[test]
    fn add_over_once_non_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "R60");
        assert_eq!(dial, 10);
        assert_eq!(over, 1);
    }
    #[test]
    fn add_over_once_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "R50");
        assert_eq!(dial, 0);
        assert_eq!(over, 1);
    }
    #[test]
    fn sub_over_once_non_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "L60");
        assert_eq!(dial, 100 - 10);
        assert_eq!(over, 1);
    }
    #[test]
    fn sub_over_once_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "L50");
        assert_eq!(dial, 0);
        assert_eq!(over, 1);
    }
    #[test]
    fn hit_zero_directly() {
        let mut dial = 50;
        let over = spin(&mut dial, "R50");
        assert_eq!(dial, 0);
        assert_eq!(over, 1);
    }
    #[test]
    fn add_over_multiple_times_non_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "R270");
        assert_eq!(dial, 20);
        assert_eq!(over, 3);
    }
    #[test]
    fn add_over_multiple_timer_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "R250");
        assert_eq!(dial, 0);
        assert_eq!(over, 3);
    }
    #[test]
    fn sub_over_multiple_times_non_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "L270");
        assert_eq!(dial, 100 - 20);
        assert_eq!(over, 3);
    }
    #[test]
    fn sub_over_multiple_times_zero() {
        let mut dial = 50;
        let over = spin(&mut dial, "L250");
        assert_eq!(dial, 0);
        assert_eq!(over, 3);
    }
    #[bench]
    fn bench_result(b: &mut Bencher) {
        b.iter(|| {
            test::black_box({
                let mut zero_counter = 0;
                let mut skip_count = 0;
                let read_input = std::fs::read_to_string("input.txt");
                if let Ok(input) = read_input {
                    let mut dial = 50;
                    let instructions = input.split("\n").collect::<Vec<&str>>();
                    instructions.iter().for_each(|&instruction| {
                        let skip = spin(&mut dial, instruction);
                        if dial == 0 {
                            zero_counter += 1;
                        }
                        skip_count += skip;
                    });
                    println!("Counter = {}", zero_counter);
                    println!("Skip Count = {}", skip_count);
                } else {
                    eprintln!("{}", read_input.unwrap_err());
                }
            });
        });
    }
}
