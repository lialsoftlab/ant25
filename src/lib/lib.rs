use std::collections::HashMap;
use std::convert::TryInto;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

///
/// Get the sum of digits in a number.
///
/// Extract single digits from a number and returns the sum of its values.
/// 
/// ```
/// use ant25lib::get_digits_sum;
/// 
/// assert_eq!(get_digits_sum(0), 0);
/// assert_eq!(get_digits_sum(123), 6);
/// if usize::BITS == 32 { assert_eq!(get_digits_sum(999_999_999), 81) };
/// if usize::BITS == 64 { assert_eq!(get_digits_sum(9_999_999_999_999_999_999), 171) };
/// ```
/// 
pub fn get_digits_sum(n: usize) -> u16 {
    let mut n_rem = n;
    let mut acc = 0;

    while n_rem > 0 {
        acc += n_rem % 10;
        n_rem = n_rem / 10;
    }

    return acc.try_into().unwrap();
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldCellState {
    Clear,
    Obstacle,
    Avail,
}

pub type FieldSparseMatrix = HashMap<usize, HashMap<usize, FieldCellState>>;

///
/// Set the state of the specified cell in a field matrix.
/// 
/// Sets the absolute state of the specified in the `v` parameter cell in 
/// `x` and `y` coordinates in the sparse matrix `field` constructed from HashMaps.
/// 
/// ```
/// use ant25lib::*;
/// use std::collections::HashMap;
/// 
/// let mut field = FieldSparseMatrix::new();
/// set_cell_state(&mut field, 10, 10, FieldCellState::Obstacle);
/// set_cell_state(&mut field, 101, 101, FieldCellState::Clear);
/// set_cell_state(&mut field, 1234, 4321, FieldCellState::Avail);
/// ```
/// 
pub fn set_cell_state(field: &mut FieldSparseMatrix, x: usize, y: usize, v: FieldCellState) {
    match field.get_mut(&y) {
        Some(row) => row.insert(x, v),
        None => {
            field.insert(y, HashMap::new());
            field.get_mut(&y).unwrap().insert(x, v)
        }
    };
}

///
/// Get the specified cell state from a field matrix.
/// 
/// Gets the specified in `x` and `y` parameters cell state from the `field` sparse matrix.
/// 
/// ```
/// use ant25lib::*;
/// use std::collections::HashMap;
/// 
/// let mut field = FieldSparseMatrix::new();
/// assert_eq!(get_cell_state(&mut field, 1000, 1000), FieldCellState::Clear);
/// assert_eq!(get_cell_state(&mut field, 999, 999),   FieldCellState::Obstacle);
/// ```
///
pub fn get_cell_state(field: &FieldSparseMatrix, x: usize, y: usize) -> FieldCellState {
    match field.get(&y) {
        Some(row) =>
            match row.get(&x) {
                Some(&cell_state) => cell_state,
                None => calc_field_cell_state(x, y),
            },
        None => calc_field_cell_state(x, y),
    }
}

///
/// Mark cells in a field sparse matrix with Avail state when ants can to pass from the starting point.
/// 
/// Marks cells in the `field` sparse matrix with Avail state, when ants can to pass straight from the 
/// specified starting point in `x` and `y` into that cells.
/// 
/// WARNING: May require extended stack capacity for process/thread to process wide field areas 
/// since it's recursive by nature.

/// ```
/// use ant25lib::*;
/// use std::collections::HashMap;
/// 
/// let mut field = FieldSparseMatrix::new();
/// mark_cells_avail_for_ant(&mut field, 742, 703);
/// ```
/// 
pub fn mark_cells_avail_for_ant(field: &mut FieldSparseMatrix, x: usize, y: usize) {
    if get_cell_state(field, x, y) != FieldCellState::Clear { return; }

    set_cell_state(field, x, y, FieldCellState::Avail);

    if y <= usize::MAX { mark_cells_avail_for_ant(field, x, y + 1) };
    if y >  usize::MIN { mark_cells_avail_for_ant(field, x, y - 1) };
    if x >  usize::MIN { mark_cells_avail_for_ant(field, x - 1, y) };
    if x <= usize::MAX { mark_cells_avail_for_ant(field, x + 1, y) };
}

//
// Calculate available cells count to pass an ant in field sparse matrix.
//
pub fn count_cells_avail_for_ant(field: &FieldSparseMatrix) -> usize {
    field.iter().map(|(_, row)| 
        row.iter().map(|(_,&cell)| if cell == FieldCellState::Avail { 1 } else { 0 })
        .reduce(|acc, x| acc + x).unwrap_or_default()
    ).reduce(|acc, x| acc + x).unwrap_or_default()
}

//
// Write an image in PPM format to file from a field sparse matrix.
//
pub fn write_ppm(field: &FieldSparseMatrix, filename: &str, x_start: usize, y_start: usize, x_end: usize, y_end:usize) {
    let path = Path::new(&filename);

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };


    file.write("P6\n".as_bytes()).unwrap();
    file.write(format!("{} {}\n", x_end - x_start + 1, y_end - y_start + 1).as_bytes()).unwrap();
    file.write("255\n".as_bytes()).unwrap();

    for y in y_start..y_end+1 {
        for x in x_start..x_end+1 {
            file.write(&map_state_to_color(get_cell_state(&field, x, y))).unwrap(); 
        }
    };
}

///
/// Calculate predicate: is it the cell clear to pass for an ant or is it depricated for pass? 
/// 
fn calc_field_cell_state(x: usize, y: usize) -> FieldCellState {
    if get_digits_sum(x) + get_digits_sum(y) > 25  { 
        FieldCellState::Obstacle 
    } else { 
        FieldCellState::Clear 
    }
}

//
//
//
fn map_state_to_color(x: FieldCellState) -> [u8; 3] {
    let clear    = [0xFF, 0xFF, 0x00];
    let obstacle = [0x00, 0x00, 0xFF];
    let avail    = [0x00, 0xFF, 0x00];

    match x {
        FieldCellState::Clear => clear,
        FieldCellState::Obstacle => obstacle,
        FieldCellState::Avail => avail,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_cell_state_set_and_get_fn() {
        let mut field = FieldSparseMatrix::new();

        assert_eq!(get_cell_state(&field, 10,   10  ), FieldCellState::Clear);
        assert_eq!(get_cell_state(&field, 101,  101 ), FieldCellState::Clear);
        assert_eq!(get_cell_state(&field, 1234, 4321), FieldCellState::Clear);
        assert_eq!(get_cell_state(&field, 999,  999 ), FieldCellState::Obstacle);

        set_cell_state(&mut field, 10,   10,   FieldCellState::Obstacle);
        set_cell_state(&mut field, 101,  101,  FieldCellState::Clear);
        set_cell_state(&mut field, 1234, 4321, FieldCellState::Avail);
    
        assert_eq!(field[&10][&10],     FieldCellState::Obstacle);
        assert_eq!(field[&101][&101],   FieldCellState::Clear);
        assert_eq!(field[&4321][&1234], FieldCellState::Avail);

        assert_eq!(get_cell_state(&field, 10,   10  ), FieldCellState::Obstacle);
        assert_eq!(get_cell_state(&field, 101,  101 ), FieldCellState::Clear);
        assert_eq!(get_cell_state(&field, 1234, 4321), FieldCellState::Avail);

        assert_eq!(field.len(), 3);
        for (_y, row) in field {
            assert_eq!(row.len(), 1);
        }
    }

    #[test]
    fn test_mark_calc_cells_avail_for_ant() {
        let mut field = FieldSparseMatrix::new();

        let cage: [&str; 5] = [
            "XXXX**XXXX",
            "X********X",
            "X**X**X**X",
            "****X****X",
            "XXXXXXXXX ",
        ];
    
        for y in 0..5 {
            let row: Vec<char> = cage[y].chars().collect();
            for x in 0..10  {
                set_cell_state(&mut field, x, y, if row[x] == 'X' {FieldCellState::Obstacle} else {FieldCellState::Clear})
            }
        }
                      
        mark_cells_avail_for_ant(&mut field, 4, 2);
    
        let mut avail_cells_count = 0;

        for y in 0..5 {
            let row: Vec<char> = cage[y].chars().collect();
            for x in 0..10  {
                match field[&y][&x] {
                    FieldCellState::Clear    => assert_eq!(row[x], ' '),
                    FieldCellState::Obstacle => assert_eq!(row[x], 'X'),
                    FieldCellState::Avail    => { assert_eq!(row[x], '*'); avail_cells_count += 1; },
                }
            }
        }

        assert_eq!(count_cells_avail_for_ant(&field), avail_cells_count);
    }

}
