fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;


    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f32 / (image_width -1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0 as f32;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            println!("{ir} {ig} {ib}");
        }
    }
}