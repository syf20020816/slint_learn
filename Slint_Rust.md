# Slint With Rust

## 基础知识

### `as_weak()`

as_weak() 是 Slint 提供的一个方法，用于将一个强引用(strong reference)转换为一个弱引用(weak reference)。

在 Rust 中，强引用指的是拥有所有权并增加引用计数的引用，而弱引用是一种不会增加引用计数的引用。

弱引用不会影响内存的生命周期，因此当所有的强引用都释放后，即便还存在弱引用，其引用的对象也可能已经被销毁。

使用 as_weak() 可以将强引用转换为弱引用，以避免强引用造成的循环引用或内存泄漏。通常情况下，通过调用 as_weak() 方法可以得到一个 `Weak<T>` 类型的引用，该类型可以使用 upgrade() 方法尝试获取原始对象的强引用。

### `clone_strong()`

clone_strong() 是 Slint 提供的另一个方法，用于克隆一个强引用。
在 Rust 中，克隆操作会创建一个新的引用，并增加引用计数。这意味着克隆后的引用仍然是强引用，它共享原始对象的所有权和引用计数。

通过调用 clone_strong() 方法，您可以创建原始强引用的一个副本，这样您就可以拥有一个独立的强引用，而不会与其他引用共享所有权。这对于需要拥有独立引用的场景非常有用，例如多线程操作或特定的 Rust 编程模型。

### `run(),show(),hide()`

- run : 显示组件 -> 事件循环 -> 隐藏组件
- show : 显示
- hide : 隐藏

### `fn global<T: Global<Self>>(&self) -> T`

全局单例的访问器

### 访问属性

- set_property_name : `fn set_<property_name>(&self, value: <PropertyType>)`
- get_property_name : `fn get_<property_name>(&self) -> <PropertyType>`

### 访问回调

### 调用回调

invoke_callback_name : `fn invoke_<callback_name>(&self)`

### 设置回调处理
on_callback_name : `fn on_<callback_name>(&self, callback: impl Fn(<CallbackArgs>) + 'static)`
