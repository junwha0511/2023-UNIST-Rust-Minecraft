pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let height = minefield.len();
    if height == 0 { return result; }
    let width = minefield[0].len();
    for i in 0..height {
        result.push(
            minefield[i].chars().enumerate().map(
                |(j, x)| {
                    if x == '*' { x }
                    else {
                        let mut count = 0;
                        let uppermost = i == 0;
                        let lowermost = i == (height-1);
                        let leftmost = j == 0;
                        let rightmost = j == (width-1);
                        if !uppermost {
                            if minefield[i-1].chars().nth(j).unwrap() == '*' {count += 1;}
                            if !leftmost && minefield[i-1].chars().nth(j-1).unwrap() == '*' {count += 1;}
                            if !rightmost && minefield[i-1].chars().nth(j+1).unwrap() == '*' {count += 1;}
                        }
                        if !lowermost {
                            if minefield[i+1].chars().nth(j).unwrap() == '*' {count += 1;}
                            if !leftmost && minefield[i+1].chars().nth(j-1).unwrap() == '*' {count += 1;}
                            if !rightmost && minefield[i+1].chars().nth(j+1).unwrap() == '*' {count += 1;}
                        }
                        if !leftmost && minefield[i].chars().nth(j-1).unwrap() == '*' {   
                            count += 1;
                        }
                        if !rightmost && minefield[i].chars().nth(j+1).unwrap() == '*' {      
                            count += 1;
                        }
        
                        if count == 0 {
                            ' '
                        } else {
                            char::from_digit(count as u32, 10).unwrap()
                        }
                    }
                }
            ).collect::<String>()
        );
        // let mut row = String::from("");
        // for j in 0..width {
        //     if minefield[i].chars().nth(j).unwrap() == '*' {
        //         row.push('*');
        //         continue;
        //     }
        //     let mut count = 0;
        //     let uppermost = i == 0;
        //     let lowermost = i == (height-1);
        //     let leftmost = j == 0;
        //     let rightmost = j == (width-1);
        //     if !uppermost {
        //         if minefield[i-1].chars().nth(j).unwrap() == '*' {count += 1;}
        //         if !leftmost && minefield[i-1].chars().nth(j-1).unwrap() == '*' {count += 1;}
        //         if !rightmost && minefield[i-1].chars().nth(j+1).unwrap() == '*' {count += 1;}
        //     }
        //     if !lowermost {
        //         if minefield[i+1].chars().nth(j).unwrap() == '*' {count += 1;}
        //         if !leftmost && minefield[i+1].chars().nth(j-1).unwrap() == '*' {count += 1;}
        //         if !rightmost && minefield[i+1].chars().nth(j+1).unwrap() == '*' {count += 1;}
        //     }
        //     if !leftmost && minefield[i].chars().nth(j-1).unwrap() == '*' {   
        //         count += 1;
        //     }
        //     if !rightmost && minefield[i].chars().nth(j+1).unwrap() == '*' {      
        //         count += 1;
        //     }

        //     if count == 0 {
        //         row.push(' ');
        //     } else {
        //         row.push(char::from_digit(count as u32, 10).unwrap());
        //     }
        // }
        // result.push(row);
    }
    
    return result;
    // unimplemented!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{:#?}\n", minefield);
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
