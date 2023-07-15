# 作业报告

## 玄镇 计科01 2020010762

## 1. 程序结构

`src`部分的源码结构如下：

```shell
.
├── args.rs（解析命令行参数）
├── builtin_words.rs（词库）
├── config.rs（解析配置文件）
├── json.rs（保存、加载游戏状态）
├── main.rs
├── run.rs（游戏运行的具体实现）
├── sync（安全地使用全局变量）
│   ├── mod.rs
│   └── up.rs
├── tui.rs（TUI）
└── utils.rs（一些辅助函数）
```

我们使用`sync`中定义的`UPSafeCell`类来使用全局变量，并在`args.rs`中定义了若干个表示参数的全局变量，例如:

```rust
lazy_static! {
	pub static ref IS_TTY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
}
```

表示了当前是否为tty模式，同时我们定义了函数:

```rust
pub fn is_tty() -> bool { *IS_TTY.exclusive_access() }
```

用来在程序的其他地方快速地判断参数。

在程序启动后，我们首先进行参数解析（`args.rs/args_parse`），在参数解析过程中同时会判断程序的参数是否合法。然后开始运行程序（`run.rs/run`），该函数主要调用了`run_one_time`，表示游戏进行一轮。

在`run_one_time`中，我们首先获取游戏的答案，然后接受用户的猜测输入，并判断是否合法。如果合法，判断相应各个字母的颜色，为此我们定义了一个枚举类进行了抽象（`utils.rs/Status`）。具体的判断方法我们是对5个字母逐个判断。当一轮游戏结束后进行结果的统计和输出，然后询问是否继续。

为了统计全局赢和输的次数和其他游戏信息，我们在`run.rs`中也定义了全局变量，并且在每一轮结束后进行更新。

### 2. 主要功能

游戏实现了交互模式和测试模式，除了基础要求中的功能，还增加了TUI（参数为`-T/--tui`）和统计所有可能词（参数为`-p/--possible`）和推荐候选词（参数为`-R/--recommend`）

交互模式：

<img src="/Users/xuanzhen/Library/Application Support/typora-user-images/截屏2023-07-09 18.52.49.png" alt="截屏2023-07-09 18.52.49" style="zoom:50%;" /> 

交互模式：随机答案，利用可能词和推荐词来猜测<img src="/Users/xuanzhen/Library/Application Support/typora-user-images/image-20230709185459084.png" alt="image-20230709185459084" style="zoom:50%;" />

TUI：

<img src="/Users/xuanzhen/Library/Application Support/typora-user-images/image-20230709185736733.png" alt="image-20230709185736733" style="zoom:50%;" /> 

### 提高要求实现方式

+ TUI：在实现TUI的过程中，我主要参考了https://blog.logrocket.com/rust-and-tui-building-a-command-line-interface-in-rust/来学习如何绘制TUI. 我们使用了 [Crossterm](https://github.com/crossterm-rs/crossterm)作为后端，并且每200ms来监听输入。guess history和键盘字母，我们使用向量来时时保存状态，然后根据该状态进行渲染，因此在逻辑部分只需注重更新状态即可。对于全局游戏，我们利用状态机的思想，当每次有键盘输入时，更新guess history和键盘字母，更新ouput和input，并判断下一个状态。

+ 可用词：在实现判断所有可能词时，逻辑可以参考在困难模式下输入是否合法（`utils/difficult_valid`）但是二者还有一定差别，如果上一次的输入为AABAA，对应的颜色为YRGRR，那在困难模式下输入AABCC在困难模式下会被认为是合法的，但我们进行可以更加严格约束。具体的算法是：

    我们先统计输入的每个字母的词频，然后按三轮扫描，第一轮判断G，必须位置严格一样，并将词频-1；第二轮判断Y，必须保证位置不同，且词频至少剩1；第三轮判断R，词频只能为0.

+ 候选词：我们实现了信息熵的算法。具体而言是：

    只考虑候选词列表，对每个单词`a`，对每种可能的情况`r`，在候选词中共有概率`p`的满足，则单词`a`的信息熵为:$\sum_{r}-p\log_2(p)$

    在具体计算的时候，我发现如果先列出所有可能的情况（共有$4^5$种）然后在统计，这样算起来很慢。实际上对于每个答案和每个可能的输入，游戏给出的状态是唯一的，因此只需遍历一遍可能词然后计算结果，将结果作为数组下标来更新即可，这样避免了$4^5$次循环。

