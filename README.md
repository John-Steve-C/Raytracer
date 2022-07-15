# PPCA Raytracer 2022

作者：Leonard_Chen

## 当前进度

book2，Image13

![](https://s3.bmp.ovh/imgs/2022/07/15/c04af36edbe09680.jpg)

## 项目内容

利用 `Rust`实现一个 ray_tracer，追踪光线以绘制一张独特的图片。

[系列教程](https://raytracing.github.io/)

## 实现架构（leonard）

注意：由于 Rust 语言的特性，对于每一个子文件夹，都需要一个 `mod.rs` 来实现模块的 mod 功能。

### basic_component

基本的类，包括三维向量 vec3、光线 ray、相机 camera

对于相机，我们考虑了很多，从最基本的位置、方向、旋转角度，到观察的广角、光圈、景深的实现

### utility.rs

一些功能函数，比如弧度转化，随机数生成

### hittable

所有能和光线发生碰撞的物体，比如球体 sphere，和某些坐标轴垂直的长方形 aarect

同时，在 mod.rs 中实现了 HitRecord、HittableList 类，以及 Hittable 这一特性

### material

所有可能的材质，比如

- 理想散射 lambertian（朗伯体）

    是理想的漫射面，它所有方向的反射能量都相等，即在一个固定的照明分布下，从所有的视场方向上观测都具有相同亮度的表面，朗伯表面不吸收任何入射光）
- 金属 metal，通过加上随机向量来控制模糊度
- 电介质 dielectric（水/玻璃/钻石）

    同时发生 **反射** 和 **折射**

    如果内部没有其他固体杂质，那么可以认为全都是折射，观察到的图像是上下颠倒的（现实中少见）
- 主动发光的物体 diffuse_light，他们具有反射和发光两种性质

### optimiaztion

利用了 BVH（Bounding Volume Hierarchies）来加快渲染速度，实现的模型为 AABB（Axis-Aligned Bounding Boxes）

简单来说，就是把所有的球用长方体包起来，让光线与球相撞 `->` 光线和长方体相撞。

然后二分这个长方体，$O(N)$ `->` $O(\log N)$

### texture

为材料添加纹理，代码上体现为：

```rust
#[derive(Clone)]
pub struct Lambertian<T> 
where
	T : Texture
{
    pub albedo: T,
}
```

即只有具有 `Texture` 这一 `trait` 的变量类型，才能为其中的 `albedo` 赋值

目前实现了

- solid：纯色
- checker：棋盘状的纹理
- perlin：利用 perlin算法（自然噪声发生的伪随机算法）计算出的白噪声图形（噪点？），随后加入了平滑优化/频率控制/防止网格化，最后得到大理石纹理
- image：实现贴图功能

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