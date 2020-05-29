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
    #[derive(Debug, Copy, Clone)]
    struct IntVec {
        x: i32,
        y: i32
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
        x: 2.0,
        y: 2.0,
        dir: FloatVec{
            x: -1.0,
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

    // let mut my_buff: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];

   



    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut frame_time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // println!("{:?}", player);

        let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];

        let rot_speed: f32 = frame_time*-0.5;
        let move_speed: f32 = frame_time*0.10;

        // println!("{}", move_speed);

        if window.is_key_down(Key::W) {
            if ((player.y + player.dir.y*move_speed) as usize) < map.len()
                && (player.y as usize) < map.len(){
            if map[(player.y + player.dir.y*move_speed) as usize].get(player.x as usize) != None {player.y += player.dir.y*move_speed};
            if map[player.y as usize].get((player.x + player.dir.x*move_speed) as usize) != None {player.x += player.dir.x*move_speed};
            }
        }
        if window.is_key_down(Key::S) && player.y > 0.0 {
            // println!("{}",map[(player.y + player.dir.y*move_speed) as usize][player.x as usize]);
            if map[(player.y - player.dir.y*move_speed) as usize][player.x as usize] != "#" {player.y -= player.dir.y*move_speed};
            if map[player.y as usize][(player.y - player.dir.y*move_speed) as usize] != "#" {player.x -= player.dir.x*move_speed};
        }
        if window.is_key_down(Key::A) {
            if map[player.y as usize][player.x as usize] != "#" {player.x += 0.1};
        }
        if window.is_key_down(Key::D) {
            if map[player.y as usize][player.x as usize] != "#" {player.x -= 0.1};
        }
        if window.is_key_down(Key::Right){
            let old_plane = plane;
            let old_player = player;
            player.dir.x = player.dir.x * rot_speed.cos() - player.dir.y * rot_speed.sin();
            player.dir.y = old_player.dir.x * rot_speed.sin() + player.dir.y * rot_speed.cos();
            plane.x = plane.x * rot_speed.cos() - plane.y * rot_speed.sin();
            plane.y = old_plane.x * rot_speed.sin() + plane.y * rot_speed.cos();
        }

        // println!("{:?}", player);

        for x in 0..D_WIDTH{
            let cam_x = (2.0 * x as f32) / (D_WIDTH as f32 - 1.0);
            let ray = Ray {
                dir_x : player.dir.x + plane.x * cam_x,
                dir_y : player.dir.y + plane.y * cam_x
            };
            // let mut current_point = FloatVec {
            //     x: player.x,
            //     y: player.y
            // };
            let mut side = false;
            let mut map_vec = IntVec {
                x: player.x as i32,
                y: player.y as i32
            };
            let mut step = IntVec {
                x : 0,
                y : 0
            };
            let mut delta_dist = FloatVec {
                x : 1.0/ray.dir_x,
                y : 1.0/ray.dir_y
            };
            let mut side_dist = FloatVec {
                x : 0.0,
                y : 0.0
            };
            if ray.dir_x > -0.0_f32{
                step.x = 1;
                side_dist.x = (map_vec.x as f32 + 1.0 - player.x) * delta_dist.x;
            }else{
                step.x = -1;
                side_dist.x = (player.x - map_vec.x as f32) * delta_dist.x;
            }
            if ray.dir_y > -0.0_f32 {
                step.x = 1;
                side_dist.y = (map_vec.y as f32 + 1.0 - player.y) * delta_dist.y;
            }else{
                step.y = -1;
                side_dist.y = (player.y - map_vec.y as f32) * delta_dist.y;
                

            }
            let mut hit = false;
            while !hit {

                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_vec.x += step.x;
                    side = true;
                }else{
                    side_dist.y += delta_dist.y;
                    map_vec.y += step.y;
                    side = false;
                }

                // println!("{:?}", map_vec);
            
                if map[map_vec.x as usize][map_vec.y as usize] == "#" { 
                    hit = true;
                    // println!("{:?}", map_vec);
                }
            }
                // let distance = side_dist.y.abs() + side_dist.x.abs();
                let distance;
                if !side {distance =(map_vec.x as f32 - player.x + (1-step.x) as f32 / 2.0) /ray.dir_x}
                else {distance = (map_vec.y as f32 - player.y + (1-step.y) as f32 /2.0) /ray.dir_y}
            
                let mut line_height: i32;
                let mut start_pixel;
                // if side == false { distance = side_dist.x}
                // else {distance = side_dist.y}
                if distance > 0_f32 {
                    line_height = (D_HEIGHT as f32/distance) as i32;
                    
                }else {
                    line_height = 1;
                    start_pixel = 0;
                }
                if line_height > -1 && line_height
                 < (D_HEIGHT as i32){
                    start_pixel = -line_height/2 + (D_HEIGHT as i32/2);
                }else{
                    start_pixel = 0;
                    line_height = 0;
                }
                
                
                // println!("{} {} {} {:?} {:?} {:?}", distance, line_height, start_pixel as usize, player, map_vec, ray);
                for i in 0..line_height as usize {
                    if line_height as usize > 0 {
                        let pixel_to_draw = (start_pixel as usize+i)*(D_WIDTH) + x ;
                        if pixel_to_draw > 0 && pixel_to_draw < (D_HEIGHT*D_WIDTH){
                            // println!("{}", pixel_to_draw);
                            buffer[pixel_to_draw as usize] = 255;
                        }
                    }
                }
                
                
            }
            // println!("{} {:?} {:?}", cam_x, ray, current_point);
        
        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/100.0;
    }

    



}
