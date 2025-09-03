
use std::{
	io::Cursor,
	path::PathBuf,
	process::ExitCode,
	sync::Arc,
	thread::spawn,
};
use image::{
	RgbImage,
	ImageReader,
	imageops::{
		resize,
		FilterType,
	}
};
use turbojpeg::{
	decompress_image,
};

const MATCH_SIZE: u32 = 512;

fn load_image(path: &PathBuf) -> RgbImage {
	let image_data = std::fs::read(path).unwrap();
	match decompress_image(&image_data) {
		Ok(img) => img,
		Err(_) => ImageReader::new(Cursor::new(&image_data)).with_guessed_format().unwrap().decode().unwrap().to_rgb8(),
	}
}

fn reduce(c: u8) -> u8 {
	c & 0xF8
}

fn full_range(half: f32) -> f32 {
	half * 2.0 - 1.0
}

fn match_image(img1: &PathBuf, img2: &PathBuf) -> f32 {
	let img1 = Arc::new(img1.clone());
	let img2 = Arc::new(img2.clone());
	let job1 = spawn(move || {
		let img1 = load_image(&img1);
		Arc::new(resize(&img1, MATCH_SIZE, MATCH_SIZE, FilterType::Lanczos3))
	});
	let job2 = spawn(move || {
		let img2 = load_image(&img2);
		Arc::new(resize(&img2, MATCH_SIZE, MATCH_SIZE, FilterType::Lanczos3))
	});
	let img1 = job1.join().unwrap();
	let img2 = job2.join().unwrap();
	let mut sr: f32 = 0.0;
	let mut sg: f32 = 0.0;
	let mut sb: f32 = 0.0;
	for (x, y, p1) in img1.enumerate_pixels() {
		let p2 = img2.get_pixel(x, y);
		sr += (full_range(reduce_color(p1.0[0]) as f32 / 255.0) * full_range(reduce_color(p2.0[0]) as f32 / 255.0)).abs() as f32;
		sg += (full_range(reduce_color(p1.0[1]) as f32 / 255.0) * full_range(reduce_color(p2.0[1]) as f32 / 255.0)).abs() as f32;
		sb += (full_range(reduce_color(p1.0[2]) as f32 / 255.0) * full_range(reduce_color(p2.0[2]) as f32 / 255.0)).abs() as f32;
	}
	((sr + sg + sb) / (MATCH_SIZE * MATCH_SIZE * 3) as f32).max(0.0)
}

fn usage() {
	println!("Usage: imgmatch <image1> <image2>");
}

fn main() -> ExitCode {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 3 {usage(); return ExitCode::from(1);}
	
	let error = match_image(&PathBuf::from(&args[1]), &PathBuf::from(&args[2]));
	println!("{error}");
	ExitCode::from(0)
}
