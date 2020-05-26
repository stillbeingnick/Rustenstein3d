use std::f32::consts::PI;
use minifb::{Key, Window, WindowOptions, ScaleMode, Scale};
use std::time::SystemTime;

const D_WIDTH: usize = 100;
const D_HEIGHT: usize = 100;

fn main() {

    let mut buffer: Vec<u32> = vec![0; D_WIDTH * D_HEIGHT];


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

    while window.is_open() && !window.is_key_down(Key::Escape) {
        //loop for the window
        let frame_time = SystemTime::from_inner(SystemTime::now());
        for xi in 0..D_WIDTH * D_HEIGHT{
            if frame_time % 2 == 0 {
                if xi % 2 == 0 {
                    buffer[xi] = 100;
                }else{
                    buffer[xi] = 255;
                }
            }else{
                if xi % 2 == 0 {
                    buffer[xi] = 255;
                }else{
                    buffer[xi] = 100;
                }
            }
        }


        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
    }

    

    let mut map: Vec<Vec<&str>> = vec![vec![]];
    map.push("##########".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("#........#".split("").collect());
    map.push("##########".split("").collect());
    
    // let mut fake_display = vec![
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    //     vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    // ];
    // let mut display_buffer: Vec<Vec<&str>> = vec![vec![]];
    // //create fake display buffer
    // for y in 0..d_height{
    //     let mut cols: Vec<&str> = vec![];
    //     for x in 0..d_width{
    //         cols.push("#");
    //     }
    //     display_buffer.push(cols);

    // }
    // //testing the display buffer
    // for yx in 0..display_buffer.len(){
    //     println!("{:?}", display_buffer[yx]);
    // }



    //fov entire cone of sight diveded in half to make the calcs easier given the right angle in the center
    //hard code fov at 90 right now

    //direction vector (1, tan(ray_num*ray_count/fov_rads))

    let fov: f32 = 90.0;

    // let ray_length: f32 = 100.0;

    let ray_count: f32 = 100.0;

    let fov_rads = fov*(PI/180.0);

    //how do we build a cone of rays?
    //translate the rays???
    // can we calc the start and end of the cone and fill in the end of the rays that way????
    #[derive(Debug)]
    struct PosVec {
        x: f32,
        y: f32
    }

    struct Player {
        x: f32,
        y: f32,
        dir: PosVec
    }
    #[derive(Debug)]
    struct Ray {
        dirX: f32,
        dirY: f32
    }

    let mut player = Player{
        x: 10.0,
        y: 10.0,
        dir: PosVec{
            x: 0.0,
            y: 1.0
        }
    };

    let mut rays = vec![];

    // fake_display[player.y as usize][player.x as usize] = 1;

    //init the rays
    for x in 0..ray_count as i32 {
        let rad_dev = ray_count/fov_rads;
        let current_rad;
        if x < (ray_count as i32)/2 {
            current_rad = rad_dev*(x as f32 + 1.0);
        } else {
            current_rad = rad_dev*((x as f32)-50.0 + 1.0);
        }
        let mut ray = Ray {
            dirX: 0.0,
            dirY: 0.1,

        };
        if x > (ray_count as i32)/2 {
            ray.dirX = -(ray.dirY*(current_rad.tan()*(PI/180.0)));
        }else{
            ray.dirX = ray.dirY*(current_rad.tan()*(PI/180.0));
        }
        rays.push(ray);
    }

    //testing the ray casting
    // for j in 0..10 {
    //     player.x += 1.0;
    
    //     for x in 0..rays.len() {
    //         let ray = &rays[x];
    //         let mut current_point = [player.x, player.y];
    //         for i in 0..500{
    //             current_point = [current_point[0] + ray.dirX, current_point[1] + ray.dirY];
    //             if current_point[0] <= 20.0 && current_point[1] <= 20.0 && current_point[0] > 0.0 && current_point[1] > 0.0 {
    //                 // println!("{:?}",current_point);
    //                 // fake_display[current_point[1] as usize][current_point[0] as usize] = 1;
    //             }else{
    //                 break;
    //             }

    //         }
    //     }
    //     for x in 0..fake_display.len() {
    //         // println!("{:?}", fake_display[x as usize])
    //     }
    // }


}
