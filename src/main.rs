use std::env;
use ant25lib::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut field = FieldSparseMatrix::new();

    println!("Calculating available cells...");
    mark_cells_avail_for_ant(&mut field, 1000, 1000);
    println!("Available cells count: {}.", count_cells_avail_for_ant(&field));

    if args.len() == 2 {
        println!("Writing PPM-image into {}...", &args[1]);
        write_ppm(&field, &args[1], 500, 500, 2499, 2499);
    }
}
