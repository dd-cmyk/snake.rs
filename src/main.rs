use std::io::{stdout, Read, Stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use rand::seq::SliceRandom;
const HEIGHT: u16 = 30;
const WIDTH: u16 = 30;
const BG_CHAR: char = '#';
const SNAKE_CHAR: char = 'O';
const CHERRY_CHAR: char = 'K';
const CHERRY_TIME: u32 = 10;
#[derive(Clone, PartialEq, Copy)]
struct Block {
    pos_x: u16,
    pos_y: u16,
}
#[derive(Clone, Copy)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}
enum SnakeError {
    OutOfBounds,
    EatItself,
    IntoItself,
}

// The coordinated are supposed to be (1, 1) - based, yet i start from 0 and it is the beginning,
// why is that?
fn draw(stdout: &mut RawTerminal<Stdout>, snake: &Vec<Block>, cherry: &Vec<Block>) {
    let points: u16 = (snake.len() - 1) as u16 * 100;
    for x in 1..WIDTH {
        for y in 1..HEIGHT {
            write!(stdout, "{}{}", termion::cursor::Goto(x + 1, y + 1), BG_CHAR).unwrap();
        }
    }
    for block in cherry.iter() {
        write!(stdout, "{}{}", termion::cursor::Goto(block.pos_x + 1, block.pos_y + 1), CHERRY_CHAR).unwrap();
    }
    for block in snake.iter() {
        write!(stdout, "{}{}", termion::cursor::Goto(block.pos_x + 1, block.pos_y + 1), SNAKE_CHAR).unwrap();
    }
    write!(stdout, "{}snake length: {}", termion::cursor::Goto(WIDTH + 5, 1), snake.len()).unwrap();
    write!(stdout, "{}points: {}", termion::cursor::Goto(WIDTH + 5, 2), points).unwrap();

    stdout.flush().unwrap();
}
fn add_cherry(snake: &mut Vec<Block>, cherry: &mut Vec<Block>) {
   // TODO: logic for addin cherry, incredibly inefficient but I dont want to choose a random
   // number multiple times, because that might result in lag caused by the fact that there
   // are not very many fields left
   let mut possible: Vec<Block> = vec![];
   for i in 1..HEIGHT {
       for j in 1..WIDTH {
           possible.push( Block { pos_x: i, pos_y: j } );
       }
   }
   for block in snake.iter() {
   possible.retain(|&x| &x != block);
   }
   cherry.push(*possible.choose(&mut rand::thread_rng()).unwrap());
}
fn move_snake(snake: &mut Vec<Block>, dir: &Move, cherry: &mut Vec<Block>) -> Result<(), SnakeError> {
    // TODO: make ramming into self not make a move  
    let mut new_x: u16 = snake[0].pos_x;
    let mut new_y: u16 = snake[0].pos_y;
    match dir {
        Move::Left => new_x -= 1, 
        Move::Right => new_x += 1, 
        Move::Up => new_y -= 1,
        Move::Down => new_y += 1,
    }
    if new_x == snake[1].pos_x && new_y == snake[1].pos_y {
       return Err(SnakeError::IntoItself); 
    }
    if !(1..HEIGHT).contains(&new_x) || !(1..WIDTH).contains(&new_y){
        return Err(SnakeError::OutOfBounds);
        }
    for block in snake.iter() { // I suppose #[Derive(PartialEq)] for struct Block would be easier
        if block.pos_x == new_x && block.pos_y == new_y {
        return Err(SnakeError::EatItself);
        }
        
    }

    let a = vec!(Block { pos_x: new_x, pos_y: new_y });
    let mut on_cherry: bool = false;
    for i in 0..cherry.len() {
        if cherry[i].pos_x == snake[0].pos_x && cherry[i].pos_y == snake[0].pos_y {
            on_cherry = true;
            cherry.remove(i);
            break;
        }
    }
    if on_cherry == false {
        snake.pop();
    }
    snake.splice(0..0, a.iter().cloned());
    Ok(())
}
fn main() {
    let mut stdin = termion::async_stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut snake: Vec<Block> = vec![Block { pos_x: 1, pos_y: 1}];
    let mut cherry: Vec<Block> = vec![Block { pos_x: 5, pos_y: 10}];
    //for i in 0..10 { // Test snake, and cherry
    //    snake.push( Block { pos_x: i + 2, pos_y: 1} );
    //    cherry.push( Block { pos_x: i + 2, pos_y: 20 });
    //} 
    snake.push( Block { pos_x: 1, pos_y: 1 } );
    //TODO: Snake of len 1 crashes the game
    let mut counter: u32 = 0;

    draw(&mut stdout, &snake, &cherry);
    let mut next_move = Move::Down; // I chose Move::Down arbitraily
    loop {
        let mut input = [0]; // Why are we making input a array of one element?
                             // This array is a buffer of type [i32; 1] (probably)
        let old_move = next_move;

        if stdin.read(&mut input).is_ok() {
            match input[0] {
                b'q' => break,
                b'w' => next_move = Move::Up,
                b'a' => next_move = Move::Left,
                b's' => next_move = Move::Down, 
                b'd' => next_move = Move::Right, 
                _ => {}, 
            }
        }
        match move_snake(&mut snake, &next_move, &mut cherry) {
            Ok(_) => {},
            Err(err) => {
                match err {
                    SnakeError::OutOfBounds => {break;}, 
                    SnakeError::IntoItself => {next_move = old_move}, 
                    SnakeError::EatItself => {break;},
                }
            }

        }
        draw(&mut stdout, &snake, &cherry);
        std::thread::sleep(std::time::Duration::from_millis(40));
        // Add a cherry randomly every CHERRY_TIME cycles
        counter += 1;
        if counter == CHERRY_TIME {
            add_cherry(&mut snake, &mut cherry);
            counter = 0;
        }

        write!(stdout, "{}", termion::clear::All).expect("io error!");
        stdout.flush().unwrap();
    }
    println!("You Lost!");

}
