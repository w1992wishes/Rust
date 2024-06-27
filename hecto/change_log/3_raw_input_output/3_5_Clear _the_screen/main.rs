#![warn(clippy::all, clippy::pedantic)]
mod editor; // 这就是我们如何让当前模块知道我们的新模块——editor。它会查找一个名为 editor.rs 的文件或一个名为 editor/mod.rs 的文件。
use editor::Editor;

fn main() {
    // 3.4 在之前使用 let editor... 时，我们告诉 Rust 我们想要一个对 Editor 的只读引用。然而，editor.run() 现在修改了 Editor。
    // 我们可以将其变成我们打算修改的明确引用，或者通过消除变量 editor 并直接在 default() 的输出上调用 run() 来回避这个问题。
    Editor::default().run();
    println!("Exiting...");
}
