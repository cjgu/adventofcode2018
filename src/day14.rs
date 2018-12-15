fn main() {
    let input = 768071;
    let comp: Vec<usize> = vec![7, 6, 8, 0, 7, 1];
    let mut board = vec![3, 7];

    let mut elf1_pos = 0;
    let mut elf2_pos = 1;
    let mut step1 = false;

    loop {
        let sum = board[elf1_pos] + board[elf2_pos];

        let new_1 = sum / 10;
        let new_2 = sum % 10;
        let mut new_recepies = 1;

        if new_1 > 0 {
            board.push(new_1);
            new_recepies += 1
        }
        board.push(new_2);

        elf1_pos = (elf1_pos + board[elf1_pos] + 1) % board.len();
        elf2_pos = (elf2_pos + board[elf2_pos] + 1) % board.len();

        if !step1 && board.len() >= input + 10 {
            let last_10 = &board[input..input + 10];
            println!("Step 1: Last 10: {:?}", last_10);
            step1 = true;
        }

        if (board.len() as i32 - comp.len() as i32 - 1) >= 0 {
            if new_recepies == 2 {
                let test = &board[board.len() - 1 - comp.len()..board.len() - 1];
                if test == &comp[..] {
                    println!("Step 2: Found, after: {}", board.len() - comp.len() - 1);
                    break;
                }
            }
            if &board[board.len() - comp.len()..] == &comp[..] {
                println!("Step 2: Found, after: {}", board.len() - comp.len());
                break;
            }
        }
    }
}
