// 我们现在需要更多地使用 Crossterm，特别是处理“事件”的能力。
// Crossterm 将终端中的任何活动（例如按键，但不仅限于此）通过 Event 公开给我们。
// 目前，我们只对按键事件感兴趣（例如，有人将某些内容粘贴到终端时触发的事件）。
use crossterm::event::{read, Event::Key, KeyCode::Char};  
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// 我们在这里定义了一个新的空结构体，并且它是公共的。
// 公共意味着我们可以从其他地方访问它——例如 main.rs 文件。
// 这意味着我们可以将纯内部的辅助内容与我们希望其他人了解的内容分开。很棒！
pub struct Editor {}

impl Editor {
    /**
    * pub 表示这是一个公共函数。
       fn 表示这是一个函数。
       default 是这个函数的名称。通常用于创建我们实现者认为是 Editor 的默认版本的东西。
       () 不仅表明这个函数不接受任何参数，还表明它可以“独立”调用（在其他编程语言中称为静态调用），不需要在这个结构体的特定实例上调用。在 Rust 中，这被称为关联函数。
       -> Self 表示这个函数返回一个 Self 的实例，即这个函数实现的结构体类型，即 Editor。
    */
    pub fn default() -> Self {
        Editor {}
    }
    /**
     * 有参数 &self。self 引用的是当前结构体实例（在这里是 Editor）。& 意味着我们不需要传递整个结构体，只需传递它的引用即可
     */
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        // 我们在这里用一个循环替换了 for..in，这个循环会持续运行，直到我们使用 break 退出循环。
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{:?} \r", event);
                    match (event.code) {
                        Char(c) => {
                            if c == 'q' {
                                break;
                            }
                        }
                        _ => (),
                    }
                }
                Err(err) => println!("Error: {}", err),
                _ => (),
            }
        }
        disable_raw_mode().unwrap();
    }
}
