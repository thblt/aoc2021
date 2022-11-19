use lib::*;

fn a() -> (u32,u32) {
    let mut lines = read_lines("../inputs/4.txt").unwrap();
    let mut first: Option<u32> = None;
    let mut last: u32 = 0;

    // Read numbers
    let numbers: Vec<u32> = lines.next()
        .expect("no line?")
        .expect("why?")
        .split(',')
        .map(|n| n.parse::<u32>().expect("no parse?"))
        .collect();

    // Read grids
    let mut grids: Vec<Grid> = Vec::new();

    let mut raw: String = String::new();

    for l in lines.skip(1) {
        if l.as_ref().unwrap().is_empty() {
            grids.push(grid_parse(raw));
            raw = String::new();
        } else {
            raw.push_str(l.unwrap().as_str());
            raw.push(' ');
        }
    }

    println!("Got {} grids", grids.len());

    for num in numbers {
        println!("Drawn {}", num);
        grids.retain(|grid| !grid_winning(&grid));

        for mut g in &mut grids {
            grid_mark(&mut *g, num);
            if let Some(score) = grid_maybe_score(&g) {
                grid_print(&g);
                if first.is_none() {
                    first = Some(score*num)
                }
                last = score*num
            }
        }
    }
    (first.unwrap(), last)
}

#[derive(Copy, Clone, Debug)]
struct Cell {
    number: u32,
    marked: bool
}

type Grid = [Cell; 25];

fn grid_maybe_score(g: &Grid) -> Option<u32> {
    if grid_winning(g) {
        Some(grid_score(g))
    } else {
        None
    }
}

/// Compute a grid score
fn grid_score(g: &Grid) -> u32 {
    fn fold(sum: u32, cell: &Cell) -> u32 {
        if cell.marked {
            sum
        } else {
            sum + cell.number
        }
    }
    g.iter().fold(0, fold)
}

/// Determine if a grid is winning
fn grid_winning(g: &Grid) -> bool {
    // Check for colums
    for i in 0..4 {
        if g[i].marked && g[i+5].marked && g[i+10].marked && g[i+15].marked && g[i+20].marked {
            return true
        }
    }
    // Check for rows
    for i in [0,5,10,15,20] {
        if g[i].marked && g[i+1].marked && g[i+2].marked && g[i+3].marked && g[i+4].marked {
            return true
        }
    }
    false
}

fn grid_print(g: &Grid) {
    for i in 0..25 {
        if i % 5 == 0 {
            println!();
        }

        if g[i].marked {
            print!("[{:02}] ", g[i].number)
        }else {
            print!(" {:02}  ", g[i].number)
        }
    }
    print!("\n");
}

/// Mark a grid
fn grid_mark(g: &mut Grid, n: u32) {
    for i in 0..g.len() {
        if g[i].number == n {
            g[i].marked = true;
        }
    }
}

/// Parse a grid
fn grid_parse(raw: String) -> Grid {
    let mut ret: Grid = [Cell {number: 0, marked: false}; 25];
    let mut i: usize = 0;

    for number in raw.split(' ').filter(|l| !l.is_empty()).map(|number| number.parse::<u32>().expect("no parse")) {
        println!("Got {}", number);
        ret[i] = Cell { number, marked: false };
        i+=1;
        if i == 25 {break};
    }
    println!("Parsed {}, Got:", raw);
    grid_print(&ret);

    ret
}

fn main() {
    let result = a();
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}
