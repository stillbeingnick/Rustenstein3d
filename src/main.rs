use std::f32::consts::PI;
use minifb::{Key, Window, WindowOptions, ScaleMode, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 120;
const D_HEIGHT: usize = 80;
const movement_speed: f32 = 2.0;

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
    

    let mut fov: f32 = (PI/2.0).to_degrees();

    #[derive(Debug, Copy, Clone)]
    struct FloatVec {
        x: f32,
        y: f32
    }
    // #[derive(Debug)]
    // struct Wall {
    //     start: i32,
    //     end: i32,
    //     distance: f32,
    //     color: u32
    // }
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
            scale: Scale::X8,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    // window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    let mut frame_time = 0.0;

    let mut correction: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let rad:f32 = 0.1;

        let mut avg_wall_start = 0;

        
        let frame_start = SystemTime::now();

        // for b in 0..D_HEIGHT/2 {
        //     for g in 0..D_WIDTH{
        //         buffer[(b*D_WIDTH)+g] = (D_HEIGHT as u32/2)-b as u32;
        //         buffer[((D_HEIGHT-b-1)*D_WIDTH)+g] = (D_HEIGHT as u32/2)-b as u32;

        //     }
        // }
        
        

        buffer = vec![0; D_WIDTH * D_HEIGHT];
        

        // let rot_speed: f32 = frame_time*-0.5;
        // let move_speed: f32 = frame_time*0.10;

        let player_mov = FloatVec {
            x: player.angle.sin()*movement_speed*frame_time,
            y: player.angle.cos()*movement_speed*frame_time
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

        player.angle += movement_speed*frame_time;


        let sin = rad.sin();
        let cos = rad.cos();

        let old_player = player;

        // player.dir.x = ((old_player.dir.x - 0.0)*cos + (old_player.dir.y - 0.0)*sin)+0.0;
        // player.dir.y = ((old_player.dir.x - 0.0)*sin - (old_player.dir.y - 0.0)*cos)+0.0;

        }
        if window.is_key_down(Key::Left){
            player.angle -= movement_speed*frame_time;
            
            let sin = rad.sin();
            let cos = rad.cos();
            let old_player = player;

            // player.dir.x = ((old_player.dir.x - 0.0)*cos - (old_player.dir.y - 0.0)*sin)+0.0;
            // player.dir.y = ((old_player.dir.x - 0.0)*sin + (old_player.dir.y - 0.0)*cos)+0.0;
        }

        let fov_step_rad = (fov/D_WIDTH as f32)*PI/180.0;
        let mut current_rad = 0.1;
        for x in 0..D_WIDTH{ 
            
            let mut step_size = 0.0;
            let current_rad = (player.angle-fov.to_radians()/2.0) + (x as f32/D_WIDTH as f32)*fov.to_radians();
            let ray = Ray{
                dir_x: current_rad.sin(),
                dir_y: current_rad.cos()
            };
            let mut hit = false;
            // println!("{}"  , current_rad);
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

                // println!("{:?}", opp_ray);                
            }

            if hit == true {

                //let distance = ((player.x - current_point.x).powf(2.0) + (player.y+current_point.y).powf(2.0)).sqrt();
                // let mut distance = step_size;
                let mut distance;
                let mut distance_x = (current_point.x - player.x + (0.1 + step_size) / 2.0) / ray.dir_x;
                let mut distance_y = (current_point.y - player.y + (0.1 + step_size) / 2.0) / ray.dir_y;

                
                if distance_x < distance_y && distance_x > 0.0{
                    distance = distance_x;
                }else if distance_y > 0.0{
                    distance = distance_y
                }else{
                    distance = 0.0;
                }

                println!("{}" , distance);
                

                if distance <= 0.0 {
                    distance = 0.0;
                }else if distance >= max_depth {
                    distance = max_depth;
                }
                
                // let mut line_height = (D_HEIGHT as f32/distance)as i32;
                let mut wall_start = ((D_HEIGHT as f32/2.0) - (D_HEIGHT as f32/distance)) as i32;
                let mut wall_end = (D_HEIGHT as i32/2) + (D_HEIGHT as f32/distance) as i32;

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
                // if x % 3 == 0 {
                //     if x > 0 {
                //         avg_wall_start += wall_start;
                //         avg_wall_start = avg_wall_start/2 as i32;
                //     }
                // }
                // if avg_wall_start - wall_start > 19 || avg_wall_start - wall_start < -19 {
                //     avg_wall_start = wall_start;
                // }

                // if line_height > 0 {
                //     // println!("{} {}",line_height, distance);
                // }
                // println!("{} {}", avg_wall_start-wall_start, wall_end);
                for y in 0..D_HEIGHT {
                    let pixel_to_draw: usize;
                    if wall_start <= y as i32 && wall_end >= y as i32 {
                        pixel_to_draw = y*D_WIDTH + x;
                        buffer[pixel_to_draw] = color;        

                    }
                }

                
            }

         
        }

        
        
        // println!("{:?}", player.dir);




            
        
            
        //     while !hit {

        //     }
                
        //     for i in 0..line_height as usize {
        //         if line_height as usize > 0 {
        //             let pixel_to_draw = (start_pixel as usize+i)*(D_WIDTH) + x ;
        //             if pixel_to_draw > 0 && pixel_to_draw < (D_HEIGHT*D_WIDTH){
        //                 // println!("{}", pixel_to_draw);
        //                 buffer[pixel_to_draw as usize] = 255;
        //             }
        //         }
        // }
        // We unwrap here as we want this code to exit if it fails. 
        // Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        window.set_title(&format!("{}",1.0/frame_time));
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/1000.0;
        // println!("fps: {}", frame_time);
    }

}
