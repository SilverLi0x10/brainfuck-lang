# Brainfuck Interpreter with REPL

[English Version](#english-version) | [中文版本](#中文版本)

---

## 中文版本

一个轻量级的 **Brainfuck 解释器**，现已支持 **REPL (Read–Eval–Print Loop)** 模式。
你可以在交互式环境中直接输入 Brainfuck 代码并立即查看运行结果，方便调试和学习。

### ✨ 特性

-   🧠 **完整的 Brainfuck 语言支持**
-   💻 **REPL 模式**：即时输入、即时执行
-   📜 **脚本执行**：支持运行 `.bf` 文件
-   ⚡ **轻量快速**：零依赖，启动即用
-   🛠️ **可扩展**：代码结构清晰，便于二次开发

### 🚀 安装

```bash
git clone https://github.com/yourname/brainfuck-interpreter.git
cd brainfuck-interpreter
make   # 或 cargo build / go build / python setup.py install 等
```

### 🖥️ 使用方法

**运行 REPL**

```bash
./bf
```

```
>>> +++++ [ > +++++ <- ] > .
输出: A
```

**运行脚本**

```bash
./bf examples/hello.bf
```

### 📂 示例

-   `examples/hello.bf` → 打印 "Hello World!"
-   `examples/fib.bf` → 输出斐波那契数列
-   `examples/calc.bf` → 简单计算器

### 🛣️ Roadmap

-   [ ] 增加调试模式（单步执行、内存可视化）
-   [ ] 增加输入历史和命令补全
-   [x] 发布跨平台二进制包

### 📜 License

MIT License. 欢迎自由使用与修改。

---

## English Version

[中文版本](#中文版本) | [English Version](#english-version)

A lightweight **Brainfuck interpreter** with **REPL (Read–Eval–Print Loop)** support.
You can enter Brainfuck code interactively and see results immediately — perfect for debugging and learning.

### ✨ Features

-   🧠 **Full Brainfuck language support**
-   💻 **REPL mode**: instant input & execution
-   📜 **Script execution**: run `.bf` files
-   ⚡ **Lightweight & fast**: zero dependencies
-   🛠️ **Extensible**: clean code structure for easy hacking

### 🚀 Installation

```bash
git clone https://github.com/yourname/brainfuck-interpreter.git
cd brainfuck-interpreter
make   # or cargo build / go build / python setup.py install
```

### 🖥️ Usage

**Run REPL**

```bash
./bf
```

```
>>> +++++ [ > +++++ <- ] > .
Output: A
```

**Run script**

```bash
./bf examples/hello.bf
```

### 📂 Examples

-   `examples/hello.bf` → prints "Hello World!"
-   `examples/fib.bf` → outputs Fibonacci sequence
-   `examples/calc.bf` → simple calculator

### 🛣️ Roadmap

-   [ ] Add debugging mode (step execution, memory visualization)
-   [ ] Add input history & command completion
-   [x] Release cross-platform binaries

### 📜 License

MIT License. Free to use and modify.
