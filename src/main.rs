
#![allow(dead_code)]
use std::{
	io::Cursor,
	path::PathBuf,
	process::ExitCode,
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
use rayon::prelude::*;

const MATCH_SIZE: u32 = 512;

fn load_image(path: &PathBuf) -> RgbImage {
	let image_data = std::fs::read(path).unwrap();
	match decompress_image(&image_data) {
		Ok(img) => img,
		Err(_) => ImageReader::new(Cursor::new(&image_data)).with_guessed_format().unwrap().decode().unwrap().to_rgb8(),
	}
}

fn match_image(img1: &PathBuf, img2: &PathBuf) -> f32 {
	let imgs: Vec<_> = [img1, img2].into_par_iter().map(move |path| {
		let img = load_image(&path);
		resize(&img, MATCH_SIZE, MATCH_SIZE, FilterType::Lanczos3)
	}).collect();
	let mut sr: f32 = 0.0;
	let mut sg: f32 = 0.0;
	let mut sb: f32 = 0.0;
	let rows: Vec<_> = imgs[0].enumerate_rows().zip(imgs[1].enumerate_rows()).map(|((_, row1), (_, row2))| {
		(row1, row2)
	}).collect();
	let rows: Vec<_> = rows.into_par_iter().map(move |(row1, row2)| {
		#[cfg(feature = "numa")]
		let (mut row1, mut row2) = (row1.collect::<Vec<_>>().into_iter(), row2.collect::<Vec<_>>().into_iter());
		#[cfg(not(feature = "numa"))]
		let (mut row1, mut row2) = (row1, row2);
		let mut rr: f32 = 0.0;
		let mut rg: f32 = 0.0;
		let mut rb: f32 = 0.0;
		for _ in 0..MATCH_SIZE {
			let (p1, p2) = (row1.next().unwrap().2.0, row2.next().unwrap().2.0);
			rr += ((p1[0] as f32 / 255.0) - (p2[0] as f32 / 255.0)).abs() as f32;
			rg += ((p1[1] as f32 / 255.0) - (p2[1] as f32 / 255.0)).abs() as f32;
			rb += ((p1[2] as f32 / 255.0) - (p2[2] as f32 / 255.0)).abs() as f32;
		}
		(rr, rg, rb)
	}).collect();
	for (rr, rg, rb) in rows.iter() {
		sr += rr;
		sg += rg;
		sb += rb;
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
	println!("{error:.3}");
	ExitCode::from(0)
}

#[test]
fn test() {
	let error = match_image(&PathBuf::from("test1.png"), &PathBuf::from("test2.JPG"));
	println!("{error:.3}");
}
