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
        x: 5.0,
        y: 5.0,
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

        if window.is_key_down(Key::W) && player.y > 0.0 && player.y < (D_WIDTH*D_HEIGHT) as f32{
            player.y += 0.1;
        }else if window.is_key_down(Key::S) && player.y > 0.0 && player.y < (D_WIDTH*D_HEIGHT) as f32{
            player.y -= 0.1;
        }

        // let player_buff = ((player.y*D_WIDTH as f32) + player.x) as u32;
        // buffer[player_buff as usize] = 100;
        for x in 0..rays.len() {
            let ray = &rays[x];
            let mut current_point = [player.y, player.x];
            

            for i in 0..100{
                current_point = [current_point[0] + ray.dirY, current_point[1] + ray.dirX];

                // let buffer_point = (((current_point[0]*D_WIDTH as f32) + current_point[1]).round()) as u32;
                // if buffer_point < (D_HEIGHT*D_WIDTH) as u32{
                //     buffer[buffer_point as usize] = 255255255;
                // }
                // println!("{:?}", map[0]);
                if (current_point[1] as usize) < map[0].len()
                    && (current_point[0] as usize) < map.len() {

                        // print!("{}", map[current_point[0] as usize][current_point[1] as usize]);

                }
                else{
                    break;
                }
                if map[current_point[0] as usize][current_point[1] as usize] == "#"{
                        // println!{"HIT!"};
                        buffer[50*D_HEIGHT + (current_point[1]*50.0)as usize] = 255;
                }
                
                

            }
        }
        

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
    }


}
