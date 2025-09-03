# A Rust program to compare two images

## 语言｜Language

[简体中文](Readme-CN.md) | Chinglish

## Usage:

```
imgmatch <image1> <image2>
```

The program will try to resize the two images into a same size, then run pixel color difference summing, print out a value.

* If the value is greater than `0.01`, the two images could be considered as different images.
* If the value is less than or equal to `0.01`, the two images could be considered as identical images.
