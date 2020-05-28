use std::f32::consts::PI;
use minifb::{Key, Window, WindowOptions, ScaleMode, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 100;
const D_HEIGHT: usize = 100;

fn main() {

    let mut buffer: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];
    

    let map: Vec<Vec<&str>> = vec![
    vec!["#","#","#","#","#","#","#","#","#","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#",".",".",".",".",".",".",".",".","#"],
    vec!["#","#","#","#","#","#","#","#","#","#"],
    ];
    

    let fov: f32 = 90.0;

    #[derive(Debug, Copy, Clone)]
    struct PosVec {
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
        dir: PosVec
    }

    let mut player = Player{
        x: 5.0,
        y: 5.0,
        dir: PosVec{
            x: 1.0,
            y: 0.0
        }
    };

    let mut plane = PosVec {
        x: 0.0,
        y: 0.66
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

    // let mut my_buff: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];

   



    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut frame_time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];

        let rot_speed: f32 = -1.0*frame_time;
        let move_speed: f32 = 0.10*frame_time;

        if window.is_key_down(Key::W) {
            if map[(player.y + player.dir.y*move_speed) as usize][player.x as usize] != "" {player.y += player.dir.y*move_speed};
            if map[player.y as usize][(player.x + player.dir.x*move_speed) as usize] != "" {player.x += player.dir.x*move_speed};
        }
        if window.is_key_down(Key::S) && player.y > 0.0 {
            if map[(player.y - player.dir.y*move_speed) as usize][player.x as usize] != "" {player.y -= player.dir.y*move_speed};
            if map[player.y as usize][(player.y - player.dir.y*move_speed) as usize] != "" {player.x -= player.dir.x*move_speed};
        }
        if window.is_key_down(Key::A) {
            if map[player.y as usize][player.x as usize] != "" {player.x += 0.1};
        }
        if window.is_key_down(Key::D) {
            if map[player.y as usize][player.x as usize] != "" {player.x -= 0.1};
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
            let cam_x = 2.0*x as f32/ (D_WIDTH as f32)-1.0;
            let ray = Ray {
                dir_x : player.dir.x + plane.x * cam_x,
                dir_y : player.dir.y + plane.y * cam_x
            };
            let mut wall_hit = false;
            // let mut current_point = PosVec {
            //     x: player.x,
            //     y: player.y
            // };
            let mut side = false;
            let mut map_vec = PosVec {
                x: player.x,
                y: player.y
            };
            let mut step = PosVec {
                x : 0.0,
                y : 0.0
            };
            let mut delta_dist = PosVec {
                x : 1.0/ray.dir_x,
                y : 1.0/ray.dir_y
            };
            let mut side_dist = PosVec {
                x : 0.0,
                y : 0.0
            };
            if ray.dir_x < 0.0 {
                step.x = -1.0;
                side_dist.x = (player.x - map_vec.x.round()) * delta_dist.x;
            }else{
                step.x = 1.0;
                side_dist.x = (map_vec.x.round() + 1.0 - player.x) * delta_dist.x;
            }
            if ray.dir_y < 0.0 {
                step.y = -1.0;
                side_dist.y = (player.y - map_vec.y.round()) * delta_dist.y;

            }else{
                step.x = 1.0;
                side_dist.y = (map_vec.y.round() + 1.0 - player.y) * delta_dist.y;

            }
            let mut hit = false;
            loop {

                if side_dist.x < side_dist.y {
                    side_dist.x += delta_dist.x;
                    map_vec.x += step.x;
                    side = true;
                }else{
                    side_dist.y += delta_dist.y;
                    map_vec.y += step.y;
                    side = false;
                }
                println!("{}", x);
                if map[map_vec.y as usize][map_vec.x as usize] == "#" {
                    let distance = (map_vec.x - player.x).abs() + (map_vec.y - player.y).abs();
                    let mut line_height = D_HEIGHT as f32/(distance as f32);
                    if line_height < 0.0{
                        line_height = 0.0;
                    }else if line_height > D_HEIGHT as f32 {
                        line_height = D_HEIGHT as f32 - 1.0
                    }
                    let start_pixel = -line_height/2.0 + D_HEIGHT as f32 /2.0;
                    println!("{} {}", distance, line_height as usize);
                    for i in 0..line_height as usize {
                        if line_height > 0.0 {
                            let pixel_to_draw = (start_pixel+i as f32)*D_WIDTH as f32 + x as f32;
                            if pixel_to_draw > -0.0 && pixel_to_draw < (D_HEIGHT*D_WIDTH) as f32{
                                buffer[pixel_to_draw as usize] = 255255255;
                            }
                        }
                    }
                    break;
                    // hit = true;
                }




                // current_point.x += 0.010/ray.dir_x;
                // current_point.y += 0.010/ray.dir_y;

                // let distance = (current_point.x-player.x).abs() + (current_point.y-player.y).abs();
                // if current_point.x > map[0].len() as f32 || current_point.y > map.len() as f32 
                // || current_point.x < -0.0000 || current_point.y < -0.0000 {
                //     break;
                // }
                // if map[current_point.y as usize][current_point.x as usize] == "#" {
                //     let mut line_height = (1.0/distance)*D_HEIGHT as f32;
                //     if line_height < 0.0{
                //         line_height = 0.0;
                //     }
                //     let start_pixel = -line_height/2.0 + D_HEIGHT as f32 /2.0;
                //     println!("{} {}", line_height, x);
                //     for i in 0..line_height as usize {
                //         if line_height > 0.0 {
                //             let pixel_to_draw = (start_pixel+i as f32)*D_WIDTH as f32 + x as f32;
                //             if pixel_to_draw > -0.0 && pixel_to_draw < (D_HEIGHT*D_WIDTH) as f32{
                //                 buffer[pixel_to_draw as usize] = 255;
                //             }
                //         }
                //     }
                //     hit = true;
                
                
            }
            // println!("{} {:?} {:?}", cam_x, ray, current_point);
        }
        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/100.0;
    }

    



}
