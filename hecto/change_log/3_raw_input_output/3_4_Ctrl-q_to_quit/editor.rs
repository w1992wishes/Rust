// 我们现在需要更多地使用 Crossterm，特别是处理“事件”的能力。
// Crossterm 将终端中的任何活动（例如按键，但不仅限于此）通过 Event 公开给我们。
// 目前，我们只对按键事件感兴趣（例如，有人将某些内容粘贴到终端时触发的事件）。
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

// 我们在这里定义了一个新的空结构体，并且它是公共的。
// 公共意味着我们可以从其他地方访问它——例如 main.rs 文件。
// 这意味着我们可以将纯内部的辅助内容与我们希望其他人了解的内容分开。很棒！
pub struct Editor {
    should_quit: bool,
}

impl Editor {
    /**
    * pub 表示这是一个公共函数。
       fn 表示这是一个函数。
       default 是这个函数的名称。通常用于创建我们实现者认为是 Editor 的默认版本的东西。
       () 不仅表明这个函数不接受任何参数，还表明它可以“独立”调用（在其他编程语言中称为静态调用），不需要在这个结构体的特定实例上调用。在 Rust 中，这被称为关联函数。
       -> Self 表示这个函数返回一个 Self 的实例，即这个函数实现的结构体类型，即 Editor。
    */
    pub fn default() -> Self {
        // 3.4  we have to ensure all fields are initialised.
        Editor { should_quit: false }
    }
    /**
     * 有参数 &self。self 引用的是当前结构体实例（在这里是 Editor）。& 意味着我们不需要传递整个结构体，只需传递它的引用即可
     * 
     * 3.4 run 函数突然改变了它所调用的 Editor 实例。Rust 在处理变化时非常谨慎，在这里，需要在函数定义中明确指出打算变异我们正在处理的引用。
     */
    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            // 3.4 以上这些代码行使用了解构操作。这意味着：将 KeyEvent 拆分为其所有四个组件，并将这些组件分配给同名的变量。
            if let Key(KeyEvent {
                code, modifiers, kind, state
            }) = read()?
            {
                // 事件不再存在，我们无法记录它。因此，我们需要显式记录通过上面解构事件得到的组件。
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind: {kind:?} State: {state:?} \r");
                match code {
                    Char('d') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(())
    }
}
