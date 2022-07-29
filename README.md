# PPCA Raytracer 2022

作者：Leonard_Chen

## 项目内容

利用 `Rust`实现一个 ray_tracer，追踪光线以绘制一张独特的图片。

[系列教程](https://raytracing.github.io/)


## 核心操作

1. 计算从相机出发，到达一个像素点的光线（`cam.get_ray`）
2. 判断光线与舞台中 **物体** 发生的碰撞，进而产生反射/散射

    如果没有碰撞，就返回背景的颜色；

    如果碰到的材料是金属，就会发生镜面反射；

    否则计算物体的发光 + 散射 + PDF 函数（用更少的采样次数，来获得更真实的图像）
3. 计算光线最后的 RGB 颜色，得到图片

## 实现架构（leonard）

注意：由于 Rust 语言的特性，对于每一个子文件夹，都需要一个 `mod.rs` 来实现模块的 mod 功能。

### basic_component

基本的类，包括三维向量 vec3、光线 ray、相机 camera、标准正交基 onb

对于相机，我们考虑了很多，从最基本的位置、方向、旋转角度，到观察的广角、光圈、景深的实现

### utility.rs

一些功能函数，比如弧度转化，随机数生成`random_double`（真随机，使用的是 `rand::Rng`, 函数为 `rand::thread_rng()`）

> 每次运行程序都画出 **相同** 的图。所以在画图时，如果遇到**位置**随机生成的物体，就要用伪随机（函数为 `StdRng::seed_from_u64`，从给定的种子生成）

### hittable

所有与碰撞相关的类

#### instance

对物体进行基本的变换，比如翻转 flipface，旋转 rotate，平移 translate，雾化 constant_mediun（等密度介质，光线经过时可能发生偏移/直接穿过，进入得越深，越可能发生反射）

#### objects

所有能和光线发生碰撞的物体，

- 球体 sphere
- 和坐标轴垂直的长方形 aarect
- 由 aarect 围成的长方体 cube
- 三角形 triangle

同时，在 mod.rs 中实现了 HitRecord、HittableList 类，以及 Hittable 这一特性

#### objloader.rs

用来导入 OBJ 文件。

将需要导入的 OBJ 文件放入 import_obj/someobj 文件夹中。

如果需要实现金属化贴图，需要手动在 loader 中特判文件名称，以及当前坐标的颜色，同时在 OBJTriangle 中存储目标材质。

> 事实上，按照这种写法，可以把特定颜色的 ImageTexture 都替换为某一种材质，只需要正确地进行 RGB 判断即可

同理，也实现了一个 STLloader

### material

所有可能的材质，比如

- 理想散射 lambertian（朗伯体）

    是理想的漫射面，它所有方向的反射能量都相等，即在一个固定的照明分布下，从所有的视场方向上观测都具有相同亮度的表面，朗伯表面不吸收任何入射光）
- 金属 metal，通过加上随机向量来控制模糊度 fuzz。模糊度越高，反光效果越模糊。
- 电介质 dielectric（水/玻璃/钻石）

    同时发生 **反射** 和 **折射**

    如果内部没有其他固体杂质，那么可以认为全都是折射，观察到的图像是上下颠倒的（默认）
- 主动发光的物体 diffuse_light，他们具有反射和发光两种性质，可以充当光源

> 实际上，本模型并没有对 **亮度** 这一参数做出有效的控制。

> 目前的方案是，`let light = DiffuseLight::new_from_color(Vec3::new(7., 7., 7.));` 。color 三元组的值越大，亮度就越高

- 各向同性 isotropic，用来实现雾化效果

### optimiaztion

1. BVH（Bounding Volume Hierarchies）

目的：加快渲染速度，实现的模型为 AABB（Axis-Aligned Bounding Boxes）

简单来说，就是把所有的球用长方体包起来，让光线与球相撞 `->` 光线和长方体相撞。

然后二分这个长方体，可以优化复杂度：$O(N)$ `->` $O(\log N)$

对于 obj_loader，往往有上万个三角形，所以 BVH 是必需的。

> 可以采用更好的算法来加速渲染，比如八叉树、包围球等等

2. PDF (Probability Density Function)

目的：为了强调某些光线，突出明暗，更快地消除噪点，模拟更真实的场景

$P(x\in[a,b])=\int_a^b p(t) dt$ 。其中 $P(x)$ 为概率分布函数，$p(x)$ 为概率密度函数。

算法：[蒙特卡洛积分法](https://zhuanlan.zhihu.com/p/333314002)，用 **随机抽样** 来 **无限逼近** 计算结果

$$
F_n(x)=\frac {1}{n} \Sigma_{k=1}^n \frac {f(x_k)}{pdf(x_k)}
$$

事实上，任何合理的 pdf 都能大约估计出积分的值。但是 pdf 越近似被积函数 f ，其收敛得越快。

具体说明：重要性采样（Importance Sampling）

选择不均匀（不为常数）的 pdf，就可以加速算式收敛（计算积分）的速度

同时更多地接收某一部分发出的光线（一般是光源），防止不需要的部分发出过多的 “noise”/小黑点（减少其权重），干扰成像。

- CosinePDF：每个物体反射产生的光线
- HittablePDF：更多地接收来自（光源）的光线
- MixturePDF：把两者线性混合，目前采用的方案是两者各半

### texture

为一个 material 添加纹理

包括以下几类：

- solid：纯色
- checker：棋盘状的纹理
- perlin：利用 perlin算法（自然噪声发生的伪随机算法）计算出的白噪声图形（噪点？），随后加入了平滑优化/频率控制/防止网格化，最后得到**大理石**纹理
- image：实现贴图功能，包括 objects 文件夹中的物体，以及对导入的 OBJ 进行贴图

### main.rs

主程序，包括了多线程部分。

我采用的方法是，把图片沿水平方向分割为 16 条（笔记本恰有16个 CPU），然后同时渲染这些长条。

- `ray_color` 函数：计算当前光线的颜色，`world` 表示舞台中的所有物体，`lights` 表示需要着重考虑从哪些位置发出的光线。

## 主要工作

- [x] 配置 `Rust` 环境
- [x] 学习 [`Rust`语法](https://m.runoob.com/rust/rust-basic-syntax.html)（**现代严谨**）

---

- [x] 学习并实现 book_1
- [x] ... book_2
- [x] ... book_3

---

- [x] 多线程优化，目前使用的是 16 进程

    - book2_image21
      - time：41 min `->` 5 min 43 s，
      - CPU 利用率：11% `->` 80%

- [x] track 1 : 把 Arc 修改为 引用
- [x] track 2 : 把 引用 修改为 泛型（除了 BvhNode、HittableList）
- [ ] track 3 : ​实现 code generation，即利用过程宏进行静态预编译

    过程宏分为三类：
    - 派生宏 #[derive]
    - 属性宏 #[attribute]
    - 函数式宏 #[proc_macro]
- [x] track 4 : 将 PDF 中的 引用 修改为 泛型
- [ ] track 5 : more code generation，从 json/yaml 中调包读取
- [x] track 6 : 增加对 transform (instance) 的 PDF 支持
- [ ] track 7 : 利用 benchmark 测试性能前后区别
- [x] track 8 : 实现 obj 导入，以及对应贴图功能。同时支持 stl 文件导入

---

- [x] 补充 Rotate 类，能够沿任意坐标轴旋转
- [x] 补全 PDF 函数，包括 Rect, Cube, Triangle 
- [x] 增加了 Zoom 类，用来缩放物体，调整其大小
- [x] 绘制一张作品
- [x] 实现金属材质贴图（任意材质）
- [x] 学习 BRDF（双向反射分布函数），模拟真实的物理模型

    事实上，标准光照模型（1975，裴祥风）认为一个 rt 需要考虑四种光（详见 ray_color 函数）

    - 自发光 emissive
    - 镜面反射（高光反射） specular
    - 漫反射 diffuse
    - 环境光 ambient，描述所有的间接光照（本 rt 并未采用）

    而如果基于真实的物理模型，就需要考虑反射等式，利用辐射率 radiance 来考虑光的亮度和颜色

## 作品展示

book1，Image21

![](https://s3.bmp.ovh/imgs/2022/07/10/462f85955635e531.jpg)

book2，Image22

![](https://s3.bmp.ovh/imgs/2022/07/17/a4957402f5ca6dc6.jpg)

book3，Image9

![](https://s3.bmp.ovh/imgs/2022/07/27/65dc6b082f963278.jpg)

派大星

![](https://s3.bmp.ovh/imgs/2022/07/27/6e73f1febe9e0fbb.jpg)

最终作品

![](https://s3.bmp.ovh/imgs/2022/07/27/d021ed13672e7b6a.jpg)