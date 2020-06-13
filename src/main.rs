use std::f64::consts::PI;
use minifb::{Key, Window, WindowOptions, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 640;
const D_HEIGHT: usize = 320;
const MOVEMENT_SPEED: f64 = 2.0;

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
    

    let mut fov: f64 = 66.0_f64.to_radians();

    #[derive(Debug, Copy, Clone)]
    struct FloatVec {
        x: f64,
        y: f64
    }

    #[derive(Debug)]
    struct Ray {
        dir_x: f64,
        dir_y: f64
    }

    #[derive(Debug, Copy, Clone)]
    struct Player {
        x: f64,
        y: f64,
        angle: f64
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

    let mut focal_length: f64 = 0.8;


    while window.is_open() && !window.is_key_down(Key::Escape) {  
        
        

        let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];
        
        let player_mov = FloatVec {
            x: player.angle.sin()*MOVEMENT_SPEED*frame_time,
            y: player.angle.cos()*MOVEMENT_SPEED*frame_time
        };

        if window.is_key_down(Key::Q) { 
            focal_length+=0.01*frame_time;

         }
         if window.is_key_down(Key::E) { 
           focal_length-=0.01*frame_time;
         }
        
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
            player.angle -= MOVEMENT_SPEED*frame_time;
           
        }

        if window.is_key_down(Key::Left){
            player.angle += MOVEMENT_SPEED*frame_time;
        }


        for x in 0..D_WIDTH{ 
            
            let mut step_size = 0.0;
            
            // let current_rad = (player.angle-fov/2.0) + (x as f64/D_WIDTH as f64)*fov;
            
            let cam_x = x as f64 / D_WIDTH as f64 - 0.5;
            
            let current_rad = player.angle-cam_x.atan2(focal_length);

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
                let x_dist: f64;
                let y_dist: f64;

                let x_dist_tan: f64;
                let y_dist_tan: f64;


                if current_point.x.fract() == 0.0 {
                    x_dist = 1.0;
                }else if ray.dir_x > 0.0_f64 {
                    x_dist = current_point.x.ceil() - current_point.x;
                }else{
                    x_dist = current_point.x - current_point.x.floor();
                }

                if current_point.y.fract() == 0.0 {
                    y_dist = 1.0;
                }else if ray.dir_y > 0.0_f64 {
                    y_dist = current_point.y.ceil() - current_point.y ;
                }else{
                    y_dist = current_point.y - current_point.y.floor();
                }

                if x_dist < y_dist {
                    current_point.x += x_dist;
                    current_point.y += x_dist*current_rad.tan();
                    // step_size += (x_dist).powf(2.0)+(y_dist*current_rad.tan()).powf(2.0);

                }else{
                    current_point.x += y_dist*current_rad.tan();
                    current_point.y += y_dist;
                    // step_size += (y_dist*current_rad.tan()).powf(2.0)+(y_dist).powf(2.0);
                }
                
                // x_dist_tan = x_dist*current_rad.tan();
                // y_dist_tan = y_dist/current_rad.tan();

                //hyptonese
                // step_size += 0.01;

                let old_point = current_point;
               
                // current_point.x = player.x + ray.dir_x*step_size;
                // current_point.y = player.y + ray.dir_y*step_size;
                if current_point.x > map[1].len() as f64 || current_point.y > map.len() as f64 
                || current_point.y < 0.0_f64 || current_point.x < 0.0_f64{
                    step_size = max_depth;
                    break;
                }
                if map[current_point.y as usize][current_point.x as usize] == "#" {
                    hit = true;
                    step_size = ((current_point.x-player.x).powf(2.0)+(current_point.y-player.y).powf(2.0)).sqrt();

                }
                
                println!("{} {}", current_point.x, current_point.y);


            }

            if hit == true {

                // let mut distance = (step_size)*(current_rad-player.angle).cos();
                let mut distance = (step_size);

                let line_height = D_HEIGHT as f64/distance;
                let mut wall_end = (D_HEIGHT as i32/2) + (line_height as i32 /2) as i32;
                let mut wall_start = wall_end - line_height as i32;

                // println!("{} {} {} {}", distance, line_height, wall_start, wall_end);

                if wall_start <= 0 {
                    wall_start = 0;
                }

                if wall_end > D_HEIGHT as i32 {
                    wall_end = D_HEIGHT as i32;
                }else if wall_end < 0 {
                    wall_end = 0;
                }

                let color;
                if step_size <= max_depth/4.0 {
                    color = 255;
                }else if step_size < max_depth/3.0 {
                    color = 153;
                }else if step_size < max_depth/2.0 {
                    color = 102
                }else if step_size < max_depth {
                    color = 51;
                }else{
                    color = 0;
                }
               
                for y in 0..D_HEIGHT {
                    let pixel_to_draw: usize = y*D_WIDTH + x;
                    if wall_start <= y as i32 && wall_end >= y as i32 {
                        buffer[pixel_to_draw] = color;        

                    }else if (y as i32) < wall_start {
                        buffer[pixel_to_draw] = from_u8_rgb(47, 79, 79); 
                    }else if (y as i32) > wall_end{
                        buffer[pixel_to_draw] = from_u8_rgb(47, 79, 79); 
                    }
                }

                
            }

         
        }

        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        window.set_title(&format!("{}",1.0/frame_time));
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f64/1000.0;
    }

}
fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
