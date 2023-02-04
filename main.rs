/*Game of Life

MIT License
Copyright (c) 2022 Darwin Geiselbrecht
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Rect};
use std::time::Duration;

extern crate rand;
//use rand::Rng;
//use std::[thread,time];


const X_MAX: usize= 1600;               // width of the world in pixels
const Y_MAX: usize = 1000;              // heigth of the world in pixels
const SIZE: usize = 6;                  // size of a cell

const NUM_X: usize = X_MAX / SIZE;       // width of the world in cells
const NUM_Y: usize = Y_MAX / SIZE;       // height of the world in cells


// generates a new copy of world with each cell containing either a zero or one randomly
fn randomize (mut world:  [[i8;NUM_Y]; NUM_X]) -> [[i8;NUM_Y]; NUM_X] {
 
    for  y in 0 .. NUM_Y  {
        for x in 0.. NUM_X {
            if rand::random() {
                world[x][y]  = 1;         
            } else {
                world[x][y] = 0;         
            }
        }
    } 
    world
}   

// generate the next generation based on the rules of the game of life
fn generation ( world:  [[i8;NUM_Y]; NUM_X]) -> [[i8;NUM_Y]; NUM_X] {

    let mut new_world = [[0i8;NUM_Y]; NUM_X];

    for  y in 0 .. NUM_Y  {
        for x in 0.. NUM_X {
            let mut count = 0;
            if x > 0 {
                count = count + world[x-1][y];
            }
            if x > 0 && y >0 {
                count = count + world[x-1][y-1];
            }
            if x > 0 &&  y < NUM_Y-1{
                count = count + world[x-1][y+1];
            }
            if x < NUM_X - 1 && y > 0{
                count = count + world[x+1][y-1];
            }
            if x < NUM_X - 1 {
                count = count + world[x+1][y];
            }
            if x < NUM_X - 1 && y < NUM_Y - 1{
                count = count + world [x+1][y+1];
            }
            if y > 0 {
                count = count + world[x][y-1];
            }
            if y < NUM_Y - 1 {
                count = count + world[x][y+1];
            }

            new_world[x][y] = 0;

            if (count <3) && world[x][y] == 1{
                new_world[x][y] = 0
            }
            if world[x][y] == 1 && (count ==2) || count == 3 {
                new_world[x][y] = 1;
            }
            if (world[x][y] == 0) && (count ==3) {
                new_world[x][y] = 1;
            }
        }    
    }    
    new_world
}


fn main() -> Result<(), String> {

    let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Game of Life   Press esc to exit, up to speed up, down to slow down,left to pause,right to resume",
     X_MAX.try_into().unwrap(), Y_MAX.try_into().unwrap())
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");
    

    let mut canvas = window.into_canvas().present_vsync().build()
        .expect("could not make a canvas");

        canvas.set_draw_color(Color::RGB(200,205,200));     // very pale green
        canvas.clear();

   
    let mut world = [[0i8;NUM_Y]; NUM_X];

    world = randomize(world);


    let mut event_pump = sdl_context.event_pump()?;
    let mut frames_per_second: u32 = 10;
    let mut paused: bool = false;
        'running: loop {

            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {  // look for exit
                        break 'running;
                    },
                    Event::KeyDown { keycode: Some(Keycode::Space), ..}  => {   // re-start the world on space
                        world = randomize(world);
                    },
                    Event::KeyDown { keycode: Some(Keycode::Left), ..}  => {    // pause on left arrow
                        paused = true;
                    }, 
                    Event::KeyDown { keycode: Some(Keycode::Right), ..}  => {   // un-pause on right arrow
                        paused = false;
                    }, 
                    Event::KeyDown { keycode: Some(Keycode::Up), ..}  => {      // run faster on up arrow
                        frames_per_second += 1;
                    },                                         
                    Event::KeyDown { keycode: Some(Keycode::Down), ..}  => {    // run slower on down arrow
                        frames_per_second -= 1;
                        if frames_per_second <= 1 {
                            frames_per_second = 1;
                        }
                    },                                          
                    _ => {}
                }
            }

            if !paused {
            world = generation(world);
            }
        
        // display the world
        let i_size: i32 = SIZE.try_into().unwrap();
        let mut y_pos: i32= 0;
               for  y in 0 .. NUM_Y {
                let mut x_pos = 0;
                for x in 0.. NUM_X {
                    if world[x][y] != 0 {

                        canvas.set_draw_color(Color::RGB(0,0,0));           
                    } else {
                         canvas.set_draw_color(Color::RGB(200,205,200));   // very pale green        
                    }
                // A draw a rectangle in the cell
                canvas.fill_rect(Rect::new(x_pos+1, y_pos+1, (SIZE-2).try_into().unwrap(), (SIZE-2).try_into().unwrap())).expect("Couldn't build rectangle");   
                x_pos =x_pos + i_size;
                }
    
        y_pos = y_pos + i_size;

        }
        canvas.present();

      

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / frames_per_second));
    } // end of game loop

    Ok(())
}