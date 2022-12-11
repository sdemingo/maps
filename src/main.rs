use rand::{thread_rng,Rng};
use rand::seq::SliceRandom;
use sdl2::event::Event;
//use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point,Rect};
use std::time::Duration;

const TILE_SZ: u32 = 16;
const COLS: u32 = 32;
const ROWS: u32 = 32;

const WIDTH: u32 = COLS * TILE_SZ;
const HEIGHT: u32 = ROWS * TILE_SZ;



// Tile types
const WALL: Color = Color::RGB(85,85,85);
const FLOOR: Color = Color::RGB(195,195,195);






fn build_maze(map: &mut Vec<Vec<Tile>>, start: (usize,usize)){

    // marco como usado
    append_to_maze(map,start);

    let mut locations: Vec<(usize,usize)> = vec![];
    locations.push(start);

    loop{
        let current = locations.pop();
        if current.is_none(){
            break;
        }

        let next = make_connection(map, current.unwrap());
        match next {
            Some(n) => {
                locations.push(n);
            },
            None => drop(locations.pop()),
        }
    }

}


// añade un tile al laberinto
fn append_to_maze(map: &mut Vec<Vec<Tile>>, pos:(usize,usize)){
    map[pos.0][pos.1].color=FLOOR;
}


// Calcula el siguiente tile a añadir al laberinto
fn make_connection(map: &mut Vec<Vec<Tile>>, location: (usize,usize)) -> Option<(usize,usize)>{
    let mut rng = thread_rng();

    let x = location.0;
    let y = location.1;

    let mut neighbour: [(i32,i32);4] = [(1,0),(0,1),(0,-1),(-1,0)];
    neighbour.shuffle(&mut rng);

    for n in &neighbour{
        let nx = x as i32 + n.0;
        let ny = y as i32 + n.1;

        if nx >= 0 && nx < COLS as i32 
            && ny >= 0 && ny < ROWS as i32
            && map[nx as usize][ny as usize].color != FLOOR  // is in maze
        {
            let next = (nx as usize,ny as usize);
            append_to_maze(map, next);
            return Some(next);
        }
    }

    return None;
}



#[derive(Clone, Copy)]
struct Tile{
    row:u32,
    col:u32,
    position: Point,
    color:Color,
}


fn build_map() -> Vec<Vec<Tile>>{

    let mut map: Vec<Vec<Tile>> = vec![
        vec![Tile{
            row:0,
            col:0,
            position: Point::new(0,0),
            color: Color::RGB(0,0,0),
        }; ROWS as usize]; COLS as usize];


    for (i, row) in map.iter_mut().enumerate(){
        for (j, col) in row.iter_mut().enumerate(){
            col.row=i as u32;
            col.col=j as u32;
            col.position=Point::new(i as i32 * TILE_SZ as i32, j as i32 * TILE_SZ as i32);
            col.color=WALL;
        }
    }

    return map;
}



pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo",WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Build the map
    let mut map = build_map();
    build_maze(&mut map,(0,0));

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    ..
                } => break 'running,
                _ => {}
            }
        }


        // Draw the background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        
        // Draw the map
        
        for row in map.iter(){
            for tile in row.iter(){
                canvas.set_draw_color(tile.color);
                canvas.fill_rect(Rect::new(tile.position.x, tile.position.y, TILE_SZ,TILE_SZ));
            }
        }

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

