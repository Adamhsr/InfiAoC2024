use rayon::prelude::*;

enum Ins {
    PushN(i32),
    PushX,
    PushY,
    PushZ,
    Add,
    Jmpos(usize),
    Ret,
}

fn parse_instruction(ins: &str) -> Ins {
    if let Some(num) = ins.strip_prefix("push ") {
        if num == "X" {
            Ins::PushX
        } else if num == "Y" {
            Ins::PushY
        } else if num == "Z" {
            Ins::PushZ
        } else {
            Ins::PushN(num.parse().expect("Failed to parse number"))
        }
    } else if ins == "add" {
        Ins::Add
    } else if let Some(num) = ins.strip_prefix("jmpos ") {
        Ins::Jmpos(num.parse().expect("Failed to parse number"))
    } else {
        Ins::Ret
    }
}

fn get_neighbors(x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
    let mut res = Vec::with_capacity(6);
    if x > 0 {
        res.push((x - 1, y, z));
    }
    if y > 0 {
        res.push((x, y - 1, z));
    }
    if z > 0 {
        res.push((x, y, z - 1));
    }
    if x < 29 {
        res.push((x + 1, y, z));
    }
    if y < 29 {
        res.push((x, y + 1, z));
    }
    if z < 29 {
        res.push((x, y, z + 1));
    }
    res
}

fn main() {
    // these will be output at end of program
    let mut blocks = 0;
    let mut clouds = 0;

    // parse instructions to cut down on file reading
    let instructions: Vec<Ins> = std::fs::read_to_string("input.txt")
    .expect("Could not the file")
    .split("\n")
    .collect::<Vec<_>>()
    .par_iter()
    .map(|ins| parse_instruction(ins))
    .collect();

    // run `XM-4S virtual stack machine` in paralell on y-z planes
    let mut cloud_array: Vec<Vec<Vec<bool>>> = (0..30)
    .into_par_iter()
    .map(|x| {
        let mut plane = vec![vec![false; 30]; 30];
        for y in 0..30 {
            for z in 0..30 {
                // keep track of program counter, stack, and stack-pointer
                let mut pc = 0;
                let mut stack: [i32; 16] = [0; 16];
                let mut sp = 0;
                while pc < instructions.len() {
                    // match current instruction, and continue accordingly
                    match &instructions[pc] {
                        Ins::PushX => {
                            stack[sp] = x;
                            sp += 1;
                        },
                        Ins::PushY => {
                            stack[sp] = y;
                            sp += 1;
                        },
                        Ins::PushZ => {
                            stack[sp] = z;
                            sp += 1;
                        },
                        Ins::PushN(n) => {
                            stack[sp] = *n;
                            sp += 1;
                        },
                        Ins::Add => {
                            sp -= 1;
                            stack[sp - 1] += stack[sp];
                        }
                        Ins::Jmpos(p) => {
                            if stack[sp - 1] >= 0 {
                                pc += p;
                            }
                        }
                        Ins::Ret => break,
                    }
                    pc += 1;
                }
                // if ending value of stack > 0, (x, y, z) is in a cloud
                // (on inspection, all values returned are either 1 or 0)
                
                // we don't add to the clouds variable here since this
                // is being computed in paralell
                if stack[sp - 1] == 1 {
                    plane[y as usize][z as usize] = true;
                }
            }
        }
        plane
    })
    .collect();

    // stack for 
    let mut checking: Vec<(usize, usize, usize)> = Vec::with_capacity(512);

    // step through every point in cube
    for cx in 0..30 {
        for cy in 0..30 {
            for cz in 0..30 {
                if cloud_array[cx][cy][cz] {
                    // when we hit a part of a cloud, add it to checking stack, and start checking neighbors
                    checking.push((cx, cy, cz));
                    clouds += 1; // will only be called when we hit an undiscovered cloud
                    while let Some((x, y, z)) = checking.pop() {
                        // loop through all adjascent blocks
                        for (nx, ny, nz) in get_neighbors(x, y, z) {
                            if cloud_array[nx][ny][nz] {
                                // if we hit another part of the cloud, mark it as found, and add it to checking
                                cloud_array[nx][ny][nz] = false;
                                blocks += 1; // called for each block hit
                                checking.push((nx, ny, nz));
                            }
                        }   
                    }
                }
            }
        }
    }

    // print output
    println!("There are {blocks} blocks of snow, making up {clouds} clouds.");
}