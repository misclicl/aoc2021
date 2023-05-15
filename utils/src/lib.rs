use std::fmt::Display;

pub fn print_matrix<T>(matrix: &Vec<Vec<T>>)
where
    T: Display,
{
    for row in matrix {
        for cell in row {
            print!("{cell} ");
        }

        println!();
    }
    println!();
}
