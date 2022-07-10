# PPCA Raytracer 2022

作者：Leonard_Chen

## 当前进度

book1，Image21

![](https://s3.bmp.ovh/imgs/2022/07/10/462f85955635e531.jpg)

## 项目内容

利用 `Rust`实现一个 ray_tracer，追踪光线以绘制一张独特的图片。

[系列教程](https://raytracing.github.io/)

## 实现架构（leonard）

注意：由于 Rust 语言的特性，对于每一个子文件夹，都需要一个 `mod.rs` 来实现模块的 mod 功能。

### basic_component

基本的类，包括三维向量 vec3、光线 ray、相机 camera

### utility.rs

一些功能函数，比如弧度转化，随机数生成

### hittable

所有能和光线发生碰撞的物体，比如球体 sphere

同时，在 mod.rs 中实现了 HitRecord、HittableList 类，以及 Hittable 这一特性

### material

所有可能的材质，比如

- 理想散射 lambertian（朗伯体）

    是理想的漫射面，它所有方向的反射能量都相等，即在一个固定的照明分布下，从所有的视场方向上观测都具有相同亮度的表面，朗伯表面不吸收任何入射光）
- 金属 metal
- 电介质 dielectric

### 核心操作：

1. 计算从眼睛（原点）出发，到达像素的光线路径
2. 判断光线的交点/反射
3. 计算/设置交点的 RGB 颜色，得到图片

## 主要工作

- [x] 配置 `Rust` 环境
- [x] 学习 [`Rust`语法](https://m.runoob.com/rust/rust-basic-syntax.html)（**现代严谨**）

---

- [x] 学习并实现 book_1
- [ ] ... book_2
- [ ] ... book_3

---

- [ ] 多线程优化
- [ ] 其他改进？