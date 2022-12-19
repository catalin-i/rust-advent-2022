


    pub fn _day1_p1() {
        let input = include_str!("../input/day1.txt");
        let mut max = 0;

        let elfs: Vec<&str> = input.split("\n\n").collect();
        for elf in elfs {
            let mut sum = 0_u32;
            let calory_strings: Vec<&str> = elf.split("\n").collect();

            for cs in calory_strings {
                let value = cs.trim().parse::<u32>().unwrap();
                sum += value;
            }
            if sum > max {
                max = sum;
            }
        }
        println!("Max is: {}", max);
    }

    pub fn day1_p2() {
        let input = include_str!("../input/day1.txt");
        let mut calory_totals: Vec<u32> = vec![];

        let elfs: Vec<&str> = input.split("\n\n").collect();
        for elf in elfs {
            let mut sum = 0_u32;
            let calory_strings: Vec<&str> = elf.split("\n").collect();

            for cs in calory_strings {
                let value = cs.trim().parse::<u32>().unwrap();
                sum += value;
            }
            calory_totals.push(sum);
        }
        calory_totals.sort_by(|a, b| b.cmp(a));
        let three_max = calory_totals[0] + calory_totals[1] + calory_totals[2];
        println!("Max 3 is: {}", three_max);
    }


