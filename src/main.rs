use std::f32::consts::PI;

fn main() {

    let mut fake_display = vec![
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

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
        // let round_x = (player.x + ray.dirX).round();
        // let round_y = (player.x + ray.dirY).round();

        // println!("x: {} y: {}",round_x, round_y);

        // if round_x >= 0.0 && round_y >= 0.0 && round_x <= 9.0 && round_y <= 9.0 {
        //     fake_display[round_x as usize][round_y as usize] = 1;
            
        // }

        // println!("{}", round_x as usize);
        // fake_display[(ray.dirX.round()) as usize][(ray.dirY.round()) as usize] = 1;
        rays.push(ray);
    }
    for j in 0..10 {
        player.x += 1.0;
    
        for x in 0..rays.len() {
            let ray = &rays[x];
            let mut current_point = [player.x, player.y];
            for i in 0..500{
                current_point = [current_point[0] + ray.dirX, current_point[1] + ray.dirY];
                if current_point[0] <= 20.0 && current_point[1] <= 20.0 && current_point[0] > 0.0 && current_point[1] > 0.0 {
                    // println!("{:?}",current_point);
                    fake_display[current_point[1] as usize][current_point[0] as usize] = 1;
                }else{
                    break;
                }

            }
        }
        for x in 0..fake_display.len() {
            println!("{:?}", fake_display[x as usize])
        }
    }


    // let l_fov = Ray{
    //     start: PosVec {
    //         x: player.x,
    //         y: player.y
    //     },
    //     end: PosVec { 
    //         x : player.x - ((fov*0.5)*(PI/180.0).tan())*ray_length, 
    //         y: player.y + ray_length}
    // };

    // let r_fov = Ray {
    //     start: PosVec {
    //         x: player.x,
    //         y: player.y 
    //     },
    //     end: PosVec {
    //         x: player.x + ((fov*0.5)*(PI/180.0).tan())*ray_length,
    //         y: player.y + ray_length
    //     }
    // };

    // let ray_difference = PosVec {
    //     x: l_fov.end.x - r_fov.end.x,
    //     y: l_fov.end.y - r_fov.end.y
    // }

    
    // for x in 0..ray_count as i32 {
    //     if ray_difference.x != 0.0 {
    //         let ray_x_factor = ray_difference.x/ray_count;
    //     }else{
    //         let ray_x_factor = 0.0;

    //     }
    //     if ray_difference.y != 0.0 {
    //         let ray_y_factor = ray_difference.y/ray_count;
    //     }else{
    //         let ray_y_factor = 0.0;

    //     }
    //     let new_ray = Ray {
    //         start: PosVec {
    //             x: player.x,
    //             y: player.y
    //         },
    //         end: PosVec {
    //             x: 0.0,
    //             y: 0.0
    //         }
    //     }
    //     if rays.len() > 1 {
    //         let previous_ray = &rays[x-1];
    //         if(ray_x_factor !=)

    //     }else{
            
    //     }
    // }


    // let middle_ray = Ray {
    //     start: PosVec {
    //         x: player.x,
    //         y: player.y
    //     },
    //     end: PosVec {
    //         x: player.x + 10.0,
    //         y: player.y + 10.0
    //     }
    // };

    // rays.push(middle_ray);

    fake_display[player.y as usize][player.x as usize] = 1;


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

    // for x in 0..fake_display.len() {
    //     println!("{:?}", fake_display[x as usize])
    // }

    // print!("{:?}{:?}", l_fov, r_fov);
    // print!("{:?}", rays);
    // print!("{}", fov_rads/ray_count)

    // loop{

    // }



}
