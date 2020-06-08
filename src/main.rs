use std::f32::consts::PI;
use minifb::{Key, Window, WindowOptions, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 1024;
const D_HEIGHT: usize = 768;
const MOVEMENT_SPEED: f32 = 2.0;

fn main() {

    let mut buffer: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];
    
    let max_depth = 16.0;

    let map: Vec<Vec<&str>> = vec![
    vec!["#","#","#","#","#","#","#","#","#","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".","#","#","#","#","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".","#",".",".",".","#"],
    vec!["#",".",".","#",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#","#","#","#","#","#","#","#","#","#"],
    ];
    

    let mut fov: f32 = (PI/4.0).to_degrees();

    #[derive(Debug, Copy, Clone)]
    struct FloatVec {
        x: f32,
        y: f32
    }

    #[derive(Debug)]
    struct Ray {
        dir_x: f32,
        dir_y: f32
    }

    #[derive(Debug, Copy, Clone)]
    struct Player {
        x: f32,
        y: f32,
        angle: f32
    }

    let mut player = Player{
        x: 5.0,
        y: 5.0,
        angle: 0.0
    };
    let mut window = Window::new(
        "Test - ESC to exit",
        D_WIDTH,
        D_HEIGHT,
        WindowOptions{
            resize: true,
            scale: Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    
    let mut frame_time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {   

        let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];
        
        let player_mov = FloatVec {
            x: player.angle.sin()*MOVEMENT_SPEED*frame_time,
            y: player.angle.cos()*MOVEMENT_SPEED*frame_time
        };
        
        if window.is_key_down(Key::W) { 
           if map[(player.y + player_mov.y)as usize][(player.x + player_mov.x) as usize] != "#" { 
                player.x += player_mov.x;
                player.y += player_mov.y;
           }
        }

        if window.is_key_down(Key::S) { 
            if map[(player.y - player_mov.y)as usize][(player.x - player_mov.x) as usize] != "#" { 
                 player.x -= player_mov.x;
                 player.y -= player_mov.y;
            }
        }
         
        if window.is_key_down(Key::Right){
            player.angle += MOVEMENT_SPEED*frame_time;
        }

        if window.is_key_down(Key::Left){
            player.angle -= MOVEMENT_SPEED*frame_time;
        }

        for x in 0..D_WIDTH{ 
            
            let mut step_size = 0.0;
            
            let current_rad = (player.angle-fov.to_radians()/2.0) + (x as f32/D_WIDTH as f32)*fov.to_radians();
            
            let ray = Ray{
                dir_x: current_rad.sin(),
                dir_y: current_rad.cos()
            };

            let mut hit = false;
            let mut current_point = FloatVec {
                x: player.x,
                y: player.y
            };
            
            while !hit {
                step_size += 0.1;
               
                current_point.x = player.x + ray.dir_x*step_size;
                current_point.y = player.y + ray.dir_y*step_size;
                if map[current_point.y as usize][current_point.x as usize] == "#" {hit = true}
                if current_point.x > max_depth || current_point.y > max_depth{
                    break;
                }

            }

            if hit == true {

                let mut distance = step_size;

                if distance <= 0.0 {
                    distance = 0.0;
                }else if distance >= max_depth {
                    distance = max_depth;
                }
                
                let line_height = (D_HEIGHT as f32/distance)as i32;
                let mut wall_end = (D_HEIGHT as i32/2) + (line_height as i32 /2) as i32;
                let mut wall_start = wall_end - line_height as i32;


                if wall_start <= 0 {
                    wall_start = 0;
                }

                if wall_end > D_HEIGHT as i32 {
                    wall_end = D_HEIGHT as i32;
                }else if wall_end < 0 {
                    wall_end = 0;
                }

                let color;
                if distance <= max_depth/4.0 {
                    color = 255;
                }else if distance < max_depth/3.0 {
                    color = 153;
                }else if distance < max_depth/2.0 {
                    color = 102
                }else if distance < max_depth {
                    color = 51;
                }else{
                    color = 0;
                }
               
                for y in 0..D_HEIGHT {
                    let pixel_to_draw: usize;
                    if wall_start <= y as i32 && wall_end >= y as i32 {
                        pixel_to_draw = y*D_WIDTH + x;
                        buffer[pixel_to_draw] = color;        

                    }
                }

                
            }

         
        }

        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        window.set_title(&format!("{}",1.0/frame_time));
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/1000.0;
    }

}
