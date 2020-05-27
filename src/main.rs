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

    #[derive(Debug)]
    struct PosVec {
        x: f32,
        y: f32
    }
    #[derive(Debug)]
    struct Ray {
        dir_x: f32,
        dir_y: f32
    }
    #[derive(Debug)]
    struct Player {
        x: f32,
        y: f32,
        dir: PosVec
    }

    let mut player = Player{
        x: 5.0,
        y: 5.0,
        dir: PosVec{
            x: -1.0,
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

    let mut odd_even = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        buffer = vec![0; D_WIDTH * D_HEIGHT];

        if window.is_key_down(Key::W) && player.y < map.len() as f32{
            if map[player.y as usize][player.x as usize] != "" {play};
        }
        if window.is_key_down(Key::S) && player.y > 0.0 {
            player.y -= 0.1;
        }
        if window.is_key_down(Key::A) {}

        // println!("{:?}", player);

        for x in 0..D_WIDTH{
            let cam_x = 2.0*x as f32/D_WIDTH as f32-1.0;
            let ray = Ray {
                dir_x : player.dir.x + plane.x * cam_x,
                dir_y : player.dir.y + plane.y * cam_x
            };
            let mut wall_hit = false;
            let mut current_point = PosVec {
                x: player.x,
                y: player.y
            };
            loop {
                current_point.x += ray.dir_x;
                current_point.y += 0.1;

                if current_point.x < map[0].len() as f32 
                && current_point.y < map.len() as f32 
                && current_point.x > -1.0
                && current_point.y > -1.0
                {
                    if map[current_point.x as usize][current_point.y as usize] == "#"{

                        let distance = (current_point.x-player.x + current_point.y-player.y).abs();
                        let line_height = D_HEIGHT as f32/distance;
                        let mut start_pixel = -line_height as i32/2 + D_HEIGHT as i32 /2;
                        for j in 0..line_height as u32 {
                            
                            if(start_pixel < 0){
                                start_pixel = 0;
                            }

                            let pixel_to_draw = ((start_pixel+j as i32) as usize * D_WIDTH) + x;
                            // println!("{}",pixel_to_draw);
                            if pixel_to_draw > 0 && pixel_to_draw < D_HEIGHT*D_WIDTH {
                                buffer[pixel_to_draw] = 255;
                            }
                            // println!("{}",j)
                        }

                        // wall_hit = true;
                        // buffer[(50*D_WIDTH) + x as usize] = 255;
                        // println!("{:?}", vec![current_point.x-player.x, current_point.y-player.y]);
                    }
                }else{
                    break;
                }
            }
        }
        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
    }


}
