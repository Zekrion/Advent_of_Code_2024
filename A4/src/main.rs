use std::time::Instant;
use std::io;
use std::fs;

fn count_xmas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (0, 1),  // Right
        (1, 0),  // Down
        (1, 1),  // Diagonal down-right
        (1, -1), // Diagonal down-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {

                // Check for out-of-bounds
                if x as isize + 3 * dx < 0 || y as isize + 3 * dy < 0
                    || x as isize + 3 * dx >= rows as isize
                    || y as isize + 3 * dy >= cols as isize
                {
                    continue;
                }

                // Check for "XMAS"
                if grid[x][y] == 'X'
                    && grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == 'M'
                    && grid[(x as isize + 2 * dx) as usize][(y as isize + 2 * dy) as usize] == 'A'
                    && grid[(x as isize + 3 * dx) as usize][(y as isize + 3 * dy) as usize] == 'S'
                {
                    count += 1;
                }

                // Check for "SAMX"
                if grid[x][y] == 'S'
                    && grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == 'A'
                    && grid[(x as isize + 2 * dx) as usize][(y as isize + 2 * dy) as usize] == 'M'
                    && grid[(x as isize + 3 * dx) as usize][(y as isize + 3 * dy) as usize] == 'X'
                {
                    count += 1;
                }
            }
        }
    }

    return count
}

fn count_mas(grid: &[Vec<char>]) -> usize {
    let directions = [
        (1, 1),  // Diagonal down-right
        (-1, -1), // Diagonal up-left
    ];

    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {

                // Check for out-of-bounds
                if (x as isize) < 1 || (y as isize) < 1
                    || (x as isize)  >= (rows as isize - 1)
                    || (y as isize)  >= (cols as isize - 1)
                {
                    continue;
                }

                // Check for "X-MAS"
                if grid[x][y] == 'A'
                    && grid[(x as isize + dx) as usize][(y as isize + dy) as usize] == 'S'
                    && grid[(x as isize + -dx) as usize][(y as isize + -dy) as usize] == 'M'
                    && 
                    (
                        (grid[(x as isize + dx) as usize][(y as isize + -dy) as usize] == 'S'
                        && grid[(x as isize + -dx) as usize][(y as isize + dy) as usize] == 'M')
                        || 
                        (grid[(x as isize + dx) as usize][(y as isize + -dy) as usize] == 'M'
                        && grid[(x as isize + -dx) as usize][(y as isize + dy) as usize] == 'S')
                    )
                {
                    count += 1;
                }
            }
        }
    }

    return count
}

fn read_grid_from_file(file_path: &str) -> io::Result<Vec<Vec<char>>> {
    let content = fs::read_to_string(file_path)?;
    let grid = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    Ok(grid)
}

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "input.txt";

    // Read the grid
    let grid = read_grid_from_file(file_path)?;

    // eval performance
    let start = Instant::now();
    let count_xmas = count_xmas(&grid);
    let time_xmas = start.elapsed();

    println!("Found XMAS: {} occurrences, took {:?}", count_xmas, time_xmas);

    // eval performance
    let start = Instant::now();
    let count_mas = count_mas(&grid);
    let time_mas = start.elapsed();

    println!("Found X-MAS: {} occurrences, took {:?}", count_mas, time_mas);
    

    Ok(())
}