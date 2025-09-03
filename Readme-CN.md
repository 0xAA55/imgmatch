# 一个用于比较两幅图像的 Rust 程序

## 语言｜Language

简体中文 | [Chinglish](Readme.md)

## 用法：

```
imgmatch <image1> <image2>
```

该程序将尝试将两幅图像调整为相同大小，然后计算像素色差并打印出一个值。

* 如果该值大于 `0.01`，则两幅图像可被视为不同的图像。
* 如果该值小于或等于 `0.01`，则两幅图像可被视为相同的图像。
* 程序本身不会对图像进行判断而是仅打印差异值；请使用你自己的图像来测试 `0.01` 是否是一个合适的阈值。

## 算法

请参考伪代码：
```
for y in 0..MATCH_SIZE { // 此行并行运行
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

## 支持的格式：

* PNG
* JPEG，加载速度比正常速度快 4 倍
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

## 部署

首先是安装 Rust。

* 在非 NUMA 系统上，只需在项目目录中运行 `cargo build --release` 即可获取二进制程序，并按照本文档的说明使用该程序。
* 在 NUMA 系统上，运行 `cargo build --release --features numa` 启用 `numa` 特性进行编译，即可获取特定于 NUMA 的二进制程序。

## 性能分析

* 使用 `profiling` 功能进行编译，每个步骤（包括图像加载、调整大小以及图像差异求和）的耗时将通过 `stderr` 显示。

## 测试结果（参考用）

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
