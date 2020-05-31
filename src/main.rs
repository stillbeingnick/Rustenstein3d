use std::f32::consts::PI;
use minifb::{Key, Window, WindowOptions, ScaleMode, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 640;
const D_HEIGHT: usize = 640;

fn main() {

    let mut buffer: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];
    

    let map: Vec<Vec<&str>> = vec![
    vec!["#","#","#","#","#","#","#","#","#","#"],
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
    

    let fov: f32 = 90.0;

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
        dir: FloatVec
    }
    let mut player = Player{
        x: 5.0,
        y: 5.0,
        dir: FloatVec{
            x: 1.0,
            y: 0.0
        }
    };
    let mut plane = FloatVec {
        x: 0.0,
        y: 0.66
    };
    let mut window = Window::new(
        "Test - ESC to exit",
        D_WIDTH,
        D_HEIGHT,
        WindowOptions{
            resize: true,
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut left_fov = FloatVec {
        x : player.x - 1.0,
        y : player.y - 1.0
    };
    let mut right_fov = FloatVec {
        x : player.x + 1.0,
        y : player.y - 1.0,
    };
    let fov_plane_length = ((right_fov.x - left_fov.x).powf(2.0) + (right_fov.y-left_fov.y).powf(2.0)).sqrt();

    println!("{}",fov_plane_length);


    let mut frame_time = 0.0;

    let start_rad = 1.0;
    let mut current_rad = start_rad;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // let frame_start = SystemTime::now();
        
        

        buffer = vec![0; D_WIDTH * D_HEIGHT];
        

        // let rot_speed: f32 = frame_time*-0.5;
        // let move_speed: f32 = frame_time*0.10;


        if window.is_key_down(Key::W) {
           player.x += player.dir.y*0.1;
           player.y += player.dir.y*0.1;
        }
        if window.is_key_down(Key::S) {
            player.x -= player.dir.x*0.1;
            player.y -= player.dir.y*0.1;
         }
         if window.is_key_down(Key::Right){
            // player.dir.x += 0.1;
            // player.dir.y += 0.1;
            
            let origin_left = FloatVec {
                x: left_fov.x - player.x,
                y: left_fov.y - player.y
            };
            let rad:f32 = 0.1; 
            if current_rad > 6.28319 + start_rad {
                current_rad = start_rad;
            }else{
                current_rad += rad;
            }
            
            let sin = rad.sin();
            let cos = rad.cos();
            // let new_x = (origin_left.x*cos - origin_left.y*sin)+player.x;
            // let new_y = (origin_left.x*sin + origin_left.y*cos)+player.y;
            let old_player = player;
            player.dir.x = ((old_player.dir.x - 0.0)*cos + (old_player.dir.y - 0.0)*sin)+0.0;
            player.dir.y = ((old_player.dir.x - 0.0)*sin - (old_player.dir.y - 0.0)*cos)+0.0;
            left_fov = FloatVec {
                x : ((left_fov.x - player.x)*cos - (left_fov.y - player.y)*sin)+player.x,
                y : ((left_fov.x - player.x)*sin + (left_fov.y - player.y)*cos)+player.y
            };
            right_fov = FloatVec {
                x : ((right_fov.x - player.x)*cos - (right_fov.y - player.y)*sin)+player.x,
                y : ((right_fov.x - player.x)*sin + (right_fov.y - player.y)*cos)+player.y
            };
            // println!("{:?}", left_fov);
        }
        if window.is_key_down(Key::Left){
            
            
            let rad:f32 = -0.1;
            if current_rad < -6.28319 + start_rad {
                current_rad = start_rad;
            }else{
                current_rad += rad;
            } 
            let sin = rad.sin();
            let cos = rad.cos();
            let old_player = player;
            player.dir.x = ((old_player.dir.x - 0.0)*cos - (old_player.dir.y - 0.0)*sin)+0.0;
            player.dir.y = ((old_player.dir.x - 0.0)*sin + (old_player.dir.y - 0.0)*cos)+0.0;
            // let new_x = (origin_left.x*cos - origin_left.y*sin)+player.x;
            // let new_y = (origin_left.x*sin + origin_left.y*cos)+player.y;
            left_fov = FloatVec {
                x : ((left_fov.x - player.x)*cos - (left_fov.y - player.y)*sin)+player.x,
                y : ((left_fov.x - player.x)*sin + (left_fov.y - player.y)*cos)+player.y
            };
            right_fov = FloatVec {
                x : ((right_fov.x - player.x)*cos - (right_fov.y - player.y)*sin)+player.x,
                y : ((right_fov.x - player.x)*sin + (right_fov.y - player.y)*cos)+player.y
            };
            // println!("{:?}", left_fov);
        }

        let fov_step_rad = (fov/D_WIDTH as f32)*PI/180.0;
        
        let mut loop_rad = current_rad;
        for x in 0..D_WIDTH{ 
            
            let mut step_size = 0.0;

            loop_rad += fov_step_rad;

            let ray = Ray{
                dir_x: loop_rad.sin(),
                dir_y: loop_rad.cos()
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
                current_point.y = player.y +ray.dir_y*step_size;
                if map[current_point.y as usize][current_point.x as usize] == "#" {hit = true}
                
            }

            if hit == true {
                let distance = (current_point.x - player.x).abs() + (current_point.y - player.y).abs();
                let line_height = (D_HEIGHT as f32/(distance+1.0)) as i32;
                let start_pixel = -(line_height as i32)/2 + D_HEIGHT as i32/2;
                for j in 0..line_height as usize {
                    let pixel_to_draw = (start_pixel as usize + j) * D_WIDTH + x;
                    buffer[pixel_to_draw] = 255;
                }
                if line_height > 0 {
                    // println!("{} {}",start_pixel, distance)
                }
            }

         
        }

        
        
        println!("{:?}", player.dir);



            
        
            
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
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        // let frame_end = SystemTime::now();
        // frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/100.0;
    }

}
