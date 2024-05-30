mod editor;  // 这就是我们如何让当前模块知道我们的新模块——editor。它会查找一个名为 editor.rs 的文件或一个名为 editor/mod.rs 的文件。
use editor::Editor;

fn main() {
    let editor = Editor::default(); 
    editor.run();    
    println!("Exiting...");
}
