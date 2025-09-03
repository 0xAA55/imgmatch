# A Rust program to compare two images

## 语言｜Language

[简体中文](Readme-CN.md) | Chinglish

## Usage:

```
imgmatch <image1> <image2>
```

The program will try to resize the two images to the same size, then run pixel color difference summing, and print out a value.

* If the value is greater than `0.01`, the two images could be considered as different images.
* If the value is less than or equal to `0.01`, the two images could be considered as identical images.
* The program itself doesn't judge the images; you should use your images to test if a good threshold is `0.01`.

## The algorithm

See the pseudo code here:
```
for y in 0..MATCH_SIZE { // This line runs in parallel
	for x in 0..MATCH_SIZE {
		let p1 = img1.get_pixel(x, y);
		let p2 = img2.get_pixel(x, y);
		sr += ((p1.r as f32 / 255.0) - (p2.r as f32 / 255.0)).abs() as f32;
		sg += ((p1.g as f32 / 255.0) - (p2.g as f32 / 255.0)).abs() as f32;
		sb += ((p1.b as f32 / 255.0) - (p2.b as f32 / 255.0)).abs() as f32;
	}
}
let error = ((sr + sg + sb) / (MATCH_SIZE * MATCH_SIZE * 3) as f32).max(0.0);
println!("{error:.3}");
```

## Supported formats:

* PNG
* JPEG, loads 4x faster than normal
* GIF
* WEBP
* PNM
* TIFF
* TGA
* DDS
* BMP
* ICO
* HDR
* OpenEXR
* Farbfeld
* AVIF
* QOI
* PCX

## Deployment

First is to install Rust.

* On Non-NUMA systems, just run `cargo build --release` in the project directory to get the binary program, and use the program as this document says.
* On NUMA systems, compile with the feature `numa` enabled by running  `cargo build --release --features numa` to get the binary program that's NUMA-specific.

## Profiling

* Compile with the feature `profiling`, then the time consumption of each step, including image loading, resizing, and the image difference summing, will be displayed via `stderr`.

## Test results for a reference

	test1.png - test1.png = 0.000
	test1.png - test2.JPG = 0.004
	test1.png - test3.jpg = 0.401
	test1.png - test4.png = 0.720
	test1.png - test5.png = 0.402
	test2.JPG - test1.png = 0.004
	test2.JPG - test2.JPG = 0.000
	test2.JPG - test3.jpg = 0.401
	test2.JPG - test4.png = 0.720
	test2.JPG - test5.png = 0.402
	test3.jpg - test1.png = 0.401
	test3.jpg - test2.JPG = 0.401
	test3.jpg - test3.jpg = 0.000
	test3.jpg - test4.png = 0.464
	test3.jpg - test5.png = 0.007
	test4.png - test1.png = 0.720
	test4.png - test2.JPG = 0.720
	test4.png - test3.jpg = 0.464
	test4.png - test4.png = 0.000
	test4.png - test5.png = 0.463
	test5.png - test1.png = 0.402
	test5.png - test2.JPG = 0.402
	test5.png - test3.jpg = 0.007
	test5.png - test4.png = 0.463
	test5.png - test5.png = 0.000
