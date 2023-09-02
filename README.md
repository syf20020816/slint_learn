# Slint Learn

- author：syf20020816@outlook.com
- updateDate：20230902
- github：

# 如何学习本文档

## 学习顺序

本文档的学习顺序基本上就从上至下的，按照由前到后的顺序依次进行学习知道你遇到这个🚩标志，这个标志将引导你的学习顺序进行改变！

## 标志

- 🚩：说明学习顺序将发送改变（可能）或提示
- 👎：说明不建议使用

## 说明

本文档和官方文档是有一定的区别的，并不是翻译官方文档，官方文档中的内容可能会和本文档内容有一定的出入（名词解释、名词称呼、标记等），或许你可以在下表中找到对应。

| 官方 | 本文更名 |
| ---- | -------- |
| Builtin Elements | 普通组件 |
| color | Color.color |
| brush | Color.brush |
| physical-length | Length.phx |
| length | Length.size |
| relative-font-size | Length.rem |
| Builtin Elements | 普通组件 |

# Slint With VSCode

我建议大家使用VSCode进行Slint开发，VSCode提供的插件对Slint十分友好，插件如下：

![image-20230902121630598](E:\Rust\learn\slint_learn\README\imgs\image-20230902121630598.png)

![image-20230902121638124](E:\Rust\learn\slint_learn\README\imgs\image-20230902121638124.png)

# Slint With Rust

## 依赖

```bash
cargo add slint
```

## 👎定义宏

用于定义一个组件，这样就可以再rs文件中进行书写

```
slint::slint!{}
```

# Slint与Rust分离

实际上更推荐更好的方式应该是slint文件于rs文件的分离

![image-20230901032611576](E:\Rust\learn\slint_learn\README\imgs\image-20230901032611576.png)

## 1.添加编译依赖（slint-build）

```toml
[package]
name = "slint_learn"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = "1.1.1"

//添加编译依赖
[build-dependencies]
slint-build = "1.1.1"
```

## 2.编写slint文件

```slint
export component MainWindow inherits Window {
  Text{
    text: "Hello Slint";
  }
}
```

## 3.编写build.rs

```rust
fn main() {
  slint_build::compile("src/components/hello.slint").unwrap();
}
```

## 4.编写main.rs

```rust
//引入模块
slint::include_modules!();

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}
```

# 普通组件

组件需要使用`componment`进行声明使用`export`进行导出

## 生命周期

每一个组件都有一个init初始化生命周期，表示组件被初始化（渲染）时激活

```
init => {//init...}
```

## 主窗体Window

窗体需要继承（inherits）Window

### example

```slint
export component MainWindow inherits Window {
  default-font-family: "Helvetica,Verdana,Arial,sans-serif";
  default-font-size: 16px;
  default-font-weight: 700;
  background: @linear-gradient(90deg,#ddd 0%,#ddc5c5 50%,#ed9797 100%);
  always-on-top: true;
  no-frame: false;
  icon: @image-url("../../imgs/rust.png");
  title: "Window!";
  height: 720px;
  width: 1200px;
}
```

![image-20230901044105838](E:\Rust\learn\slint_learn\README\imgs\image-20230901044105838.png)

## 文本Text

### example

```
export component MainWindow inherits Window {
  height: 720px;
  width: 1080px;
  title: "Text!";
  Text {
    text: "I am a Text component";
    height: 200px;
    width: 100px;
    //文字换行
    wrap: word-wrap;
    color: #fff;
    font-size: 20px;
    padding: 8px;
    letter-spacing: 2px;
    //横向对齐
    horizontal-alignment:center;
    //纵向对齐
    vertical-alignment: center;
    overflow: elide;
  }  
}
```

![image-20230901045927835](E:\Rust\learn\slint_learn\README\imgs\image-20230901045927835.png)

## 按钮Button

### example

```slint
import { Button } from "std-widgets.slint";
export component MainWindow inherits Window {
  height: 720px;
  width: 1200px;
  title: "Button!";
  Button { 
    height: 66px;
    width: 310px;
    icon: @image-url("../../imgs/rust.png");
    text: "I am a Button";
    clicked => {
      self.text = "Clicked!";
      self.width = 360px;
    }
  }
}
```

![image-20230901055536612](E:\Rust\learn\slint_learn\README\imgs\image-20230901055536612.png)

### functions

| 事件名  | 说明         |
| ------- | ------------ |
| clicked | 按钮点击事件 |

```
  Button { 
    height: 66px;
    width: 310px;
    text: "I am a Button";
    clicked => {
      self.text = "Clicked!";
      self.width = 360px;
    }
  }
```

## 矩形盒子元素 Rectangle

Rectangle只是一个不显示任何内容的空项。通过设置颜色或配置边框，可以在屏幕上绘制矩形。当不是布局的一部分时，其宽度和高度默认为父元素的100%。

### example

```
export component MainWindow inherits Window {
  height: 720px;
  width: 1200px;
  Rectangle {
    background: red;
    border-color: #ddd;
    border-radius: 4px;
    border-width: 2px;
    height: 60px;
    width: 120px;
    //like overflow clip表示超出容器是否显示
    clip: true;
    Rectangle {
      background: blue;
      border-color: #ddd;
      border-radius: 4px;
      border-width: 2px;
      height: 20px;
      width: 30px;
      x: 0px;
      y: 10px;
    }
    Rectangle {
      background: blue;
      border-color: #ddd;
      border-radius: 4px;
      border-width: 2px;
      height: 202px;
      width: 300px;
      x: 50px;
      y: 10px;
    }
  }
  Rectangle {
    background: blue;
    border-color: #ddd;
    border-radius: 4px;
    border-width: 2px;
    height: 202px;
    width: 300px;
    x: 50px;
    y: 10px;
  }
}
```

![image-20230901082823831](E:\Rust\learn\slint_learn\README\imgs\image-20230901082823831.png)

## 输入框TextInput

这是一种低级输入框，它将直接使用父级容器的宽高，无法自己设置

### example

```
export component MainWindow inherits Window {
  height: 720px;
  width: 1200px;
  title: "Text Input!";
  TextInput {
    color: burlywood;
    font-family: "Verdana";
    font-size: 18px;
    font-weight: 400;
    horizontal-alignment: left;
    input-type: text;
    read-only: false;
    selection-background-color: blue;
    selection-foreground-color: red;
    single-line: false;
    wrap: word-wrap;
    text: "default text";
    text-cursor-width:8px;
  }
}
```

![image-20230901082809428](E:\Rust\learn\slint_learn\README\imgs\image-20230901082809428.png)

## 图片Image

### example

```
export component MainWindow inherits Window {
  height: 720px;
  width: 1200px;
  title: "Image!";
  Image {
    source: @image-url("../../imgs/rust.png");
    image-fit:fill;
    image-rendering: smooth;
    //设置旋转中心
    rotation-origin-x: 23px;
    rotation-origin-y: 56px;
    rotation-angle: 30deg;
    source-clip-height: 200;
    source-clip-x: 100;
    height: 300px;
  }
}
```

![image-20230901082746288](E:\Rust\learn\slint_learn\README\imgs\image-20230901082746288.png)

## 滚动窗口 Flickable

Flickable不是产生一个可滚动的窗口，而是给一个元素增加可滚动的容器。因为他是对于父容器而言，子容器可滚动，而不是使得父容器可滚动

### example

```
export component MainWindow inherits Dialog {
  height: 200px;
  width: 300px;
  title: "Flickable!";
  Flickable {
      interactive: true;
      viewport-height: 300px;
      viewport-width: 400px;
      viewport-x: 0px;
      viewport-y: 0px;
     Rectangle {
      height: 200px;
      width: 300px;
      background: #792121;
     }
  }
}
```

![image-20230901091507127](E:\Rust\learn\slint_learn\README\imgs\image-20230901091507127.png)

## 网格布局 GridLayout

使用网格布局下的元素会被添加

- col：所在列
- row：所在行
- colspan：列占比
- rowspan：行占比

这4个属性，通常使用这四个属性进行控制

也可以使用`Row{}`进行行声明将在同一行的元素全部放在一个Row下

> ❗值得注意的是：个人认为Slint的网格布局有一定的问题，期待在后续版本中修复 （列占比和所在列需要强指定，弱指定会导致推测错误）
>
> 问题如下：
>
> 这里的第二行的白色Rectangle列占比应该是2但是显示的却是1，经过叠放检查得出白色盒子的另一半被绿色覆盖，所以弱指定无法推测出绿色盒子实际上应该在第3列，需要手动指定`col:2`
>
> ![image-20230901132355723](E:\Rust\learn\slint_learn\README\imgs\image-20230901132355723.png)

### example

```
export component MainWindow inherits Dialog {
  height: 200px;
  width: 300px;
  title: "GridLayout!";
  GridLayout {
    spacing: 10px;
    padding: 4px;
    //使用Row进行行声明
    Row{
      Rectangle { background: red; }
      Rectangle { background: yellow;}
      Rectangle { background: pink; }
    }
    Row{
      Rectangle { background: white; colspan: 2; }
      Rectangle { background: green; colspan: 1; col: 2;} 
    }
    Rectangle { background: violet; colspan: 3;row:3;}
    Rectangle { background: orange; colspan: 3;row:2;}
  }
}
```

![image-20230901133014226](E:\Rust\learn\slint_learn\README\imgs\image-20230901133014226.png)

## 横纵布局 HorizontalLayout | VerticalLayout

通过alignment属性对元素排列方式进行控制。横纵布局常常组合使用相互配合

### HorizontalLayout

横向布局即元素全部排列在同一行

```
export component MainWindow inherits Window {
  height: 330px;
  width: 400px;
  title: "HorizontalLayout!";
  HorizontalLayout {
    spacing: 5px;
    padding: 6px;
    height: 100px;
    width: 400px;
    x: 0px;
    y: 0px;
    alignment: center;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
  HorizontalLayout {
    spacing: 5px;
    padding: 6px;
    height: 100px;
    width: parent.width;
    x: 0px;
    y: 110px;
    alignment: space-around;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
  HorizontalLayout {
    spacing: 5px;
    padding: 6px;
    height: 100px;
    width: parent.width;
    x: 0px;
    y: 220px;
    alignment: end;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
}
```

![image-20230901135602285](E:\Rust\learn\slint_learn\README\imgs\image-20230901135602285.png)

### VerticalLayout

```
export component MainWindow inherits Window {
  height: 200px;
  width: 480px;
  title: "HorizontalLayout!";
  VerticalLayout {
    spacing: 5px;
    padding: 6px;
    height: root.height;
    width: 150px;
    x: 0px;
    y: 0px;
    alignment: center;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
  VerticalLayout {
    spacing: 5px;
    padding: 6px;
    height: root.height;
    width: 150px;
    x: 160px;
    y: 0px;
    alignment: space-around;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
  VerticalLayout {
    spacing: 5px;
    padding: 6px;
    height: root.height;
    width: 150px;
    x: 320px;
    y: 0px;
    alignment: end;
    Rectangle {background: red;height: 30px;width: 50px;}
    Rectangle {background: green; height: 30px;width: 100px;}
    Rectangle {background: blue; height: 80px;width: 50px;}
  }
}
```

![image-20230901140211538](E:\Rust\learn\slint_learn\README\imgs\image-20230901140211538.png)

## 画板 Path

通过描边的方式绘制形状，我称之为画板

- 使用Slint的路径命令进行绘制
- 使用SVG的路径命令进行绘制

### SVG路径命令和Slint路径命令

**如果指令字母是大写的，例如M, 则表示坐标位置是绝对位置；如果指令字母小写的，例如m, 则表示坐标位置是相对位置。**

使用`commands`属性进行声明（下面以函数形式书写帮助理解）：

```
commands:"M ..."
```

- `M(x:float,y:float)`： `MoveTo` 表示这是一个新的起点，将关闭上一个绘图。例子：`M 0 100`
- `L(x:float,y:float)`：`LineTo` 表示从上一个点到当前点，这将绘制一条直线段。例子：`L 100 100`
- `A(radius_x:float,radius_y:float,large_arc:bool,sweep:bool,x_rotation:float,x:float,y:float)`: `ArcTo`
  - radius_x : 内切椭圆横长半径
  - radius_y : 内切椭圆纵长半径
  - ![image-20230901143834518](E:\Rust\learn\slint_learn\README\imgs\image-20230901143834518.png)
  - large_arc：在封闭椭圆的两个弧中，此标志选择要渲染较大的弧。如果属性为false，则会呈现较短的弧度
  - sweep：绘制顺时针或逆时针方向（true为顺时针）
  - x_rotation：内切椭圆按照x轴旋转的度数
- `C(control_1_x:float,control_1_y:float,control_2_x:float,control_2_y:float,x:float,y:float)`:`CubicTo`,光滑贝塞尔曲线
  - control_1_x：一号控制点的横坐标，后面也一样，这里不全写了
- `Q(control_x:float,control_y:float,x:float,y:float)`：`QuadraticTo`二次贝塞尔曲线
- `Z()`：`Close`关闭当前子路径，从当前位置到起点进行连线

### example

```
export component MainWindow inherits Window {
  height: 200px;
  width: 480px;
  title: "Path!";
  Path {
    width: 100px;
    height: 100px;
    x: 50px;
    y: 50px;
    commands: "M 0 0 L 0 100 A 1 1 0 0 0 100 100 L 100 0 Z";
    stroke: red;
    stroke-width: 1px;
  }
  Path {
    width: 100px;
    height: 100px;
    stroke: blue;
    stroke-width: 1px;
    x: 250px;
    y: 50px;
    MoveTo {
      x: 0;
      y: 0;
    }
    LineTo{
      x: 0;
      y: 100;
    }
    ArcTo {
      radius-x: 1;
      radius-y: 1;
      x: 100;
      y: 100;
    }
    LineTo {
      x: 100;
      y: 0;
    }
    Close {
    }
  }
}
```

![image-20230901141811696](E:\Rust\learn\slint_learn\README\imgs\image-20230901141811696.png)

## 🚩Flag

**当你看到这里的时候，说明你已经把入门篇结束了，接下来为了你可以更好的理解高级组件，请移步到基础知识，学习完基础知识后进行高级组件学习！**

# 基础知识

当你看到这里的时候说明普通组件已经掌握，为了让你无障碍学习高级组件等后续知识请耐心学习基础知识，基础知识中有些名词经过我的修改并非和翻译出的名词名称一致，若发现一个你无法理解的名词请查询说明表。

## Slint文件编写顺序与结构

slint文件的编写顺序同js，是从上到下的，这意味着在下方块中的组件需要在上方块中进行定义才能使用（自定义组件），因此下面的代码是错误的！

```
import { Button } from "std-widgets.slint";
export component MainWindow inherits Window {
  MyButton{
    height: 50px;
    width: 50px;
  }
}

component MyButton inherits Button { 
  text: "My Button";
}
```

### 正确的代码

只需要将MyButton的声明移动到前面即可

```
import { Button } from "std-widgets.slint";

component MyButton inherits Button { 
  text: "My Button";
}

export component MainWindow inherits Window {
  MyButton{
    height: 50px;
    width: 50px;
  }
}

```

### Slint组件结构9

slint的组件结构为树形结构，每个slint文件都可以定义一个或多个组件

## 组件的访问与命名

### 组件的访问

知道组件的结构为树形结构后，显而易见的，我们可以通过树进行组件层级访问，slint显然考虑到了这点，因此在slint中按照以下方式进行组件的层级访问：

1. `root`：树根组件，也就是组件的最外层组件，是所有子组件的根
2. `self`：当前组件，通过self可以直接访问当前自己的所有属性以及方法
3. `parent`：当前组件的父组件

### 标识符（命名规范）

和多数其他语言规范相同，由`(a~z),(A~Z),(0~9),(_,-)`组成，不能以数字或`-`开头，对于slint，`_ 和 -`在非开头位置起到规范化名称相同，意思是：`my_button == my-button`

### 命名组件

通过使用`:=`对组件进行命名，以此获取组件的引用！

```slint
export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  text1:=Text {
    text: "Hello" + num;
  }
}
```

## 注释

- `//` ： 单行注释
- `/* ..*/` ：多行注释

## Slint中的类型

> ❗注意：类型中我进行了些许的修改

| 类型        | 说明                                                         | 默认值      |
| ----------- | ------------------------------------------------------------ | ----------- |
| int         | 有符号整数                                                   | 0           |
| float       | 有符号32位浮点数（f32）                                      | 0           |
| bool        | 布尔值                                                       | false       |
| string      | 字符串                                                       | ""          |
| Color.color | RGB颜色，带有Alpha通道，每个通道的精度为8位，也可以是16进制色 | transparent |
| Color.brush | 特殊的颜色，可以从基础色进行渐变或更改，使用的更加广泛       | transparent |
| Length.phx  | 用于进行单位转换的量，长度 = 整数 * 1phx                     | 0phx        |
| Length.size | 常用长度单位，分为`px,pt,in,mm,cm`(`pt：1/72英寸`,`in(英寸):2.54cm`) | 0px         |
| Length.rem  | 跟组件字体大小单位                                           | 0rem        |
| duration    | 时间单位，用在动画上，分为：`ms,s`                           | 0ms         |
| angle       | 角度单位，多用于旋转，渐变。分为：`deg,rad,turn`（`1turn = 360deg = 2Πrad`） | 0deg        |
| easing      | 动画速率，分为：`ease,ease_in,ease_in_out,ease_out，linear)`就是常说的缓入缓出，线性 | linear      |
| image       | 图像，使用`@image-url()`                                     | 空图像      |
| percent     | 带有`%`的百分数                                              | 0%          |

### 颜色

普通颜色Color.color和特殊颜色Color.brush是有区别的，brush使用画笔填充元素或画出轮廓。而且brush多了一些方法：

- **`brighter(factor: float) -> brush`**

  返回从此颜色派生的新颜色，但其亮度增加了指定的系数。 例如，如果因子是0.5（或例如50%），则返回的颜色明亮50%。负面因素 降低亮度。

- **`darker(factor: float) -> brush`**

  返回从该颜色派生的新颜色，但其亮度已按指定因子降低。 例如，如果因子是.5（或例如50%），则返回的颜色是50%更暗。负面因素 增加亮度。

- **`mix(other: brush, factor: float) -> brush`**

  返回一个新颜色，它是此颜色和`other`，有比例 因子由\一个因子给出（该因子将被限制在`0.0` 和`1.0`）.

- **`transparentize(factor: float) -> brush`**

  返回一个新的颜色，其不透明度减少了`factor`. 透明度是通过将alpha通道乘以`(1 - factor)`.

- **`with_alpha(alpha: float) -> brush`**

  返回alpha值设置为`alpha` （介于0和1之间）

#### 线性渐变

```
//语法：
@linear-gradient(渐变角度, 颜色 占比, 颜色 占比, ...);

@linear-gradient(90deg, #3f87a6 0%, #ebf8e1 50%, #f69d3c 100%);
```

#### 径向渐变

```
//语法
@linear-gradient(circle, 颜色 占比, 颜色 占比, ...);

@radial-gradient(circle, #f00 0%, #0f0 50%, #00f 100%);
```

### 🚩Flag

在学习自定义类型前请先移步属性进行学习，这将有利于你对自定义属性的理解

## 自定义类型

### 结构体

通过自定义结构体就能声明复杂的类型，这通常来说并不能再称之为属性，而是内部数据！（按照作用）但在本文还是称为属性，但请严格进行辨别。

```slint
struct User {
  name:string,
  age:int,
}

export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  Text {
    property <User> user:{name:"I am Mat",age:16};
    text: user.name;
  }
}
```

### 匿名结构体

匿名结构体指的是直接在进行声明而不再外部设置名称的无法被复用的结构体

```
export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  Text {
    property <{name:string,age:int}> user:{name:"I am Mat",age:16};
    text: user.name;
  }
}
```

### 枚举

```
enum CompoentType{
  System,
  Define
}


export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  Text {
    property <CompoentType> type : CompoentType.System ;
    text: type == CompoentType.System?"Sys":"Define";
  }
}
```

![image-20230902203615470](E:\Rust\learn\slint_learn\README\imgs\image-20230902203615470.png)

### 数组

数组的声明非常简单`[数据类型]`即可，其访问也是使用`[索引]`进行访问

```
export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  property <[color]> colors:[#fff,#dc3b3b,#eee] ;
  background: colors[1];
}
```



## 属性

所有组件都有属性，属性是组件的重要组成部分，属性有默认的也有自定义的，属性有四种访问等级，对应其可见性。

### 属性可见性

- **`private`** ：只能从组件内部访问，它是默认的
- **`in`**：属性是输入。它可以由该组件的用户设置和修改， 例如通过绑定或通过回调中的分配。 组件可以提供默认绑定，但不能对其进行分配
- **`out`**：只能由组件设置的输出属性，可以被外部获取
- **`in-out`**：公开读写的属性

### 自定义属性

```
export component MainWindow inherits Window {
  in property <int> num1;
  in-out property <int> num2;
  out property <int> num3;
  // property <int> num4
  private property <int> num4;
}
```

### 属性赋值（属性的单向绑定）

通过直接在声明的属性后设置值即为属性默认值，同时也代表对属性进行了单向绑定

```
export component MainWindow inherits Window {
  in property <int> counter : 10;
}
```

### 属性的双向绑定

属性的双向绑定能够响应式的修改属性，通过使用`<=>`起到双向绑定的效果

#### private属性访问

通过结合双向绑定和组件命名private属性也是可以被访问的

```
import { Button } from "std-widgets.slint";
export component MainWindow inherits Window {
  height: 300px;
  width: 300px;
  property <int> root-num <=> text1.num;
  title: root-num;
  text1:=Text {
    x: 0px;
    y: 0px;
    property <int> num : 10;
    text: "Hello" + num;
  }
  Button { 
    text: "click here!";
    clicked => {
      parent.root-num +=2;
    }
  }
}
```

![image-20230902201146649](E:\Rust\learn\slint_learn\README\imgs\image-20230902201146649.png)







# 高级组件

## Dialog 对话框

```
import { Button , StandardButton} from "std-widgets.slint";
export component MainWindow inherits Dialog {
  height: 720px;
  width: 1200px;
  title: "Dialog!";
  icon: @image-url("../../imgs/rust.png");
  //主元素
  Text {
    font-size: 30px;
    text: "This is a dialog";
  }
  StandardButton { 
    kind: ok;
  }
  StandardButton { 
    kind: cancel;
  }
  Button {
    width: 120px;
    text: "info";
    // 假扮成dialog-button元素
    dialog-button-role: action;
  }
}
```



# 属性速查

## 常用

### 高度 height

```
height: 200px;
```

### 宽度 width

```
width:200px;
```

### 位置 x和y

元素相对于其父元素的位置

```
x:20px;
```

### 叠放等级 z

元素在同一级元素中的堆叠次序，默认值为0

```
z:1111;
```

### 网格布局 col，row， colspan，rowspan

```
Rectangle { background: green; colspan: 1; col: 2;} 
```

### 拉伸 horizontal-stretch和vertical-stretch

```
horizontal-stretch: 2;
```

### 元素的最大大小 max-width和max-height

```
max-width:1000px;
```

### 元素的最小大小 min-width和min-height

```
min-width:120px;
```

### 元素的首选尺寸 preferred-width和preferred-height

```
preferred-height:100px;
```

### 是否显示 visible

可见性，默认true

```
visible:false;
```

### 透明度 opacity

默认值为1（0是完全透明的，1是完全不透明的）

```
opacity:0.5;
```

### 加速渲染 cache-rendering-hint 👎

默认false

### 阴影半径 drop-shadow-blur

阴影的模糊程度，默认值为0

```
 drop-shadow-blur：2;
```

### 阴影颜色 drop-shadow-color

### 阴影位置 drop-shadow-offset-x和drop-shadow-offset-y

阴影与元素框架的水平和垂直距离，若为负值，阴影位于元素的左边和上方

```
drop-shadow-offset-x：2px;
```

## 窗口属性Window Params

| 属性                | 说明（类型）                 | 示例                                                         |
| ------------------- | ---------------------------- | ------------------------------------------------------------ |
| default-font-family | 默认文字类型（String）       | default-font-family: "Helvetica,Verdana,Arial,sans-serif";   |
| default-font-size   | 默认文字大小(Size)           | default-font-size: 16px;                                     |
| default-font-weight | 默认文字粗细（Int）          | default-font-weight:700                                      |
| background          | 背景(Color)                  | background: @linear-gradient(90deg,#ddd 0%,#ddc5c5 50%,#ed9797 100%); |
| always-on-top       | 永远处于其他页面上层（Bool） | always-on-top: true;                                         |
| no-frame            | 无边框，默认false（Bool）    | no-frame: false;                                             |
| icon                | 窗口图标（Image）            | icon: @image-url("../../imgs/rust.png");                     |
| title               | 窗口标题（String）           | title: "Window!";                                            |

## 文字属性Text Params

| 属性                 | 说明（类型）                        | 示例                                                       |
| -------------------- | ----------------------------------- | ---------------------------------------------------------- |
| horizontal-alignment | 横向对齐（TextHorizontalAlignment） | default-font-family: "Helvetica,Verdana,Arial,sans-serif"; |
| vertical-alignment   | 纵向对齐（TextVerticalAlignment）   | default-font-size: 16px;                                   |
| wrap                 | 文字换行（TextWrap）                | default-font-weight:700                                    |
| overflow             | 文字超出策略（TextOverflow）        | overflow: elide;                                           |
| font-size            | 文字大小（Size）                    | font-size: 20px;                                           |
| color                | 文字颜色（Color）                   | color: #fff;                                               |
| font-weight          | 文字粗细(Int)                       | font-weight:700;                                           |
| letter-spacing       | 文字间隔大小（Size）                | letter-spacing:2px;                                        |
| text                 | 文字内容（String）                  | text: "I am a Text component";                             |

### TextOverflow

此枚举描述了如果文本太宽而无法适应Text宽度，文本的显示方式。

- clip：文本将被简单地剪切。
- elide：文本将被省略为…

### TextHorizontalAlignment

此枚举描述了文本沿Text元素水平轴对齐的不同类型的内容。

- left：文本将与包含框的左边缘对齐。
- center：文本将在包含框中水平居中。
- right：文本将排列在包含框的右侧。

###  TextVerticalAlignment

此枚举描述了文本沿Text元素垂直轴对齐的不同类型的内容。

- top：文本将与包含框的顶部对齐。
- center：文本将垂直居中于包含框中。
- bottom：文本将与包含框的底部对齐。

###  TextWrap

此枚举描述了文本太宽而无法适应Text宽度时如何包装。

- no-wrap：文本不会包装，而是会溢出。
- word-wrap：文本将以单词边界包装。

## 输入框属性Textnput Params

含有文字属性（Text Param）

| 属性                       | 说明（类型）                 | 示例                              |
| -------------------------- | ---------------------------- | --------------------------------- |
| input-type                 | 输入框类型（InputType）      | input-type: text;                 |
| read-only                  | 是否只读（Bool）             | read-only: false;                 |
| selection-background-color | 输入时文字的背景色（Color）  | selection-background-color: blue; |
| selection-foreground-color | 输入时文字的颜色（Color）    | selection-foreground-color: red;  |
| single-line                | 是否为单行，即不换行（Bool） | single-line: false;               |
| text-cursor-width          | 光标的宽度（Size）           | text-cursor-width:8px;            |

###  InputType

此枚举用于定义输入字段的类型。目前，这只能区分文本和密码输入，但将来可以扩展它，以定义应该显示哪种类型的虚拟键盘，例如。

- text：默认值。这将正常呈现所有字符
- password：这将呈现所有字符，其字符默认为`*`

## 图片属性 Image Params

| 属性                                  | 说明（类型）                   | 示例                                       |
| ------------------------------------- | ------------------------------ | ------------------------------------------ |
| colorize                              | 覆盖前景色（Color）            | colorize:Colors.aliceblue;                 |
| source                                | 图像源（URL）                  | source: @image-url("../../imgs/rust.png"); |
| image-fit                             | 图片填充类型（ImageFit）       | image-fit:fill;                            |
| image-rendering                       | 图片缩放方式（ImageRendering） | image-rendering: smooth;                   |
| rotation-origin-x，rotation-origin-y  | 设置旋转中心的位置（Size）     | rotation-origin-x: 23px;                   |
| rotation-angle                        | 旋转角度（Size.deg）           | rotation-angle: 30deg;                     |
| source-clip-height，source-clip-width | 裁剪高度\|宽度（Size.length）  | source-clip-height: 200;                   |
| source-clip-x，source-clip-y          | 裁剪位置（Size.length）        | source-clip-x: 100;                        |

### ImageFit

该枚举定义了源图像如何融入Image元素。

- fill：缩放和拉伸源图像，以适应Image元素的宽度和高度。
- contain：源图像被缩放以适应Image元素的尺寸，同时保留宽高比。
- cover：源图像被缩放以覆盖到Image元素的尺寸，同时保留宽高比。如果源图像的宽高比与元素的宽高比不匹配，那么图像将被裁剪以适合。

### ImageRendering

此枚举指定了源图像的缩放方式。

- smooth：使用线性插值算法对图像进行缩放。
- pixelated：使用最近邻算法缩放图像。

## 滚动窗口 Flickable Params

| 属性                            | 说明（类型）                            | 示例                    |
| ------------------------------- | --------------------------------------- | ----------------------- |
| interactive                     | 输入框类型（InputType）                 | interactive: true;      |
| viewport-height，viewport-width | 滚动窗口大小（Size.length）             | viewport-height: 300px; |
| viewport-x，viewport-y          | 子元素相对滚动窗口的位置（Size.length） | viewport-x: 0px;        |

## 网格布局 GridLayOut

| 属性                             | 说明（类型）              | 示例           |
| -------------------------------- | ------------------------- | -------------- |
| spacing                          | 元素间距（Size.length）   | spacing: 10px; |
| padding（left,right,top,bottom） | 布局内边距（Size.length） | padding: 4px;  |

## 横纵布局 HorizontalLayout | VerticalLayout

| 属性                             | 说明（类型）                        | 示例           |
| -------------------------------- | ----------------------------------- | -------------- |
| spacing                          | 元素间距（Size.length）             | spacing: 10px; |
| padding（left,right,top,bottom） | 布局内边距（Size.length）           | padding: 4px;  |
| alignment                        | 元素排列对齐方式（LayoutAlignment） | alignment: end |

### LayoutAlignment

表示HorizontalBox、VerticalBox、HorizontalLayout或VerticalLayout的对齐属性的枚举。

- stretch：使用布局中所有元素的最小大小，根据元素拉伸属性在所有元素之间分配剩余空间。
- center：使用所有元素的首选大小，在第一个元素之前和最后一个元素之后均匀分布剩余空间。
- start：使用所有元素的首选大小，将剩余空间放在最后一个元素之后。
- end：对所有元素使用首选大小，将剩余空间放在第一个元素之前。
- space-between：对所有元素使用首选大小，在元素之间均匀地分配剩余空间。
- space-around：使用所有元素的首选大小，在第一个元素之前、最后一个元素之后和元素之间均匀分布剩余空间。

# 可访问性 

**我认为这是一种特性并不算属性**

- accessible-role：元素角色（大多数元素默认为none，但文本元素为text）
- accessible-checkable：是否可以选中元素
- accessible-checked：是否选中了元素——对应复选框、单选按钮和其他小部件的“已选中”状态
- accessible-description：当前元素的描述
- accessible-has-focus：当当前元素当前具有焦点时，设置为true。
- accessible-label：交互式元素的标签（大多数元素默认为空，或文本元素的text属性值）
- accessible-value-maximum：最大值
- accessible-value-minimum：最小值
- accessible-value-step：当前值可以改变的最小增量
- accessible-value：当前值。
