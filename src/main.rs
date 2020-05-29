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
        x: 50.0,
        y: 50.0,
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
        x : player.dir.x - 50.0,
        y : player.dir.y + 50.0
    };
    let mut right_fov = FloatVec {
        x : player.dir.x + 50.0,
        y : player.dir.y + 50.0,
    };
    let fov_plane_length = ((right_fov.x - left_fov.x) + (right_fov.y-left_fov.y)).sqrt();

    println!("{}",fov_plane_length);


    let mut frame_time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        // let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];

        // let rot_speed: f32 = frame_time*-0.5;
        // let move_speed: f32 = frame_time*0.10;


        if window.is_key_down(Key::W) {
           player.x += 0.0;
           player.y += 1.0;
        }
         if window.is_key_down(Key::Right){
            // player.dir.x += 0.1;
            // player.dir.y += 0.1;

            let old_left = left_fov;

            left_fov = FloatVec {
                x : (&old_left.x*(0.1_f32.cos()) - &old_left.y *(0.1_f32.sin())),
                y : (&old_left.y*(0.1_f32.cos()) + &old_left.x *(0.1_f32.sin())),
            };

            println!("{:?}", left_fov);
            // right_fov = FloatVec {
            //     x : (&right_fov.x.cos() - &right_fov.y * 1.0_f32.sin()).abs(),
            //     y : (&right_fov.y.cos() - &right_fov.x * 1.0_f32.sin()).abs(),
            // };
        }


        // for x in 0..D_WIDTH{
        //     let cam_x = ((fov_plane_length/D_WIDTH as f32)*x as f32) - fov_plane_length/2.0;
        //     let m = (right_fov.y-left_fov.y)/(right_fov.x-left_fov.y);
        //     let ray = FloatVec{
        //         x : left_fov.x + cam_x,
        //         y : left_fov.y
        //     };
        //     let mut current_point = FloatVec {
        //         x: player.x,
        //         y: player.y
        //     };
        //     for i in 0..10{
        //         current_point.x += ray.x;
        //         current_point.y += ray.y;
                // buffer[((current_point.y*D_WIDTH as f32)+current_point.x).abs() as usize] = 255;
        //     }
            // println!("{}", cam_x);

            
        buffer[(((left_fov.y+player.y)*D_WIDTH as f32)+(left_fov.x+player.x)).abs() as usize] = 255;
        buffer[(((right_fov.y+player.y)*D_WIDTH as f32)+right_fov.x+player.x).abs() as usize] = 255;
        buffer[((player.y*D_WIDTH as f32)+player.x).abs() as usize] = 255;

            
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
