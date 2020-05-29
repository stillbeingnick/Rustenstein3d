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
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut frame_time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        let frame_start = SystemTime::now();

        buffer = vec![0; D_WIDTH * D_HEIGHT];

        let rot_speed: f32 = frame_time*-0.5;
        let move_speed: f32 = frame_time*0.10;


        if window.is_key_down(Key::W) {
            if ((player.y + player.dir.y*move_speed) as usize) < map.len()
                && (player.y as usize) < map.len(){
            if map[(player.y + player.dir.y*move_speed) as usize].get(player.x as usize) != None {player.y += player.dir.y*move_speed};
            if map[player.y as usize].get((player.x + player.dir.x*move_speed) as usize) != None {player.x += player.dir.x*move_speed};
            }
        }
        if window.is_key_down(Key::S) && player.y > 0.0 {
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


        for x in 0..D_WIDTH{
            
            while !hit {

            }
                
            for i in 0..line_height as usize {
                if line_height as usize > 0 {
                    let pixel_to_draw = (start_pixel as usize+i)*(D_WIDTH) + x ;
                    if pixel_to_draw > 0 && pixel_to_draw < (D_HEIGHT*D_WIDTH){
                        // println!("{}", pixel_to_draw);
                        buffer[pixel_to_draw as usize] = 255;
                    }
                }
            }
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, D_WIDTH, D_HEIGHT)
            .unwrap();
        let frame_end = SystemTime::now();
        frame_time = (frame_end.duration_since(frame_start).unwrap().as_millis())as f32/100.0;
    }

}
