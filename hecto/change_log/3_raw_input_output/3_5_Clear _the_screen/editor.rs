// 我们现在需要更多地使用 Crossterm，特别是处理“事件”的能力。
// Crossterm 将终端中的任何活动（例如按键，但不仅限于此）通过 Event 公开给我们。
// 目前，我们只对按键事件感兴趣（例如，有人将某些内容粘贴到终端时触发的事件）。
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use std::io::stdout;
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
        Self::initialize().unwrap();
        let result: Result<(), std::io::Error> = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        // 这就是我们使用 crossterm 清屏的方法。execute! 表示我们希望立即写出内容，而不是等到缓冲区填满后再写出。
        let mut stdout: std::io::Stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            let event: Event = read()?;
            // 处理事件的逻辑现在移动到了一个单独的函数中。我们没有直接传递事件，而是只传递了它的引用，如 & 所示。
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('t') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            print!("Goodbye.\r\n");
        }
        Ok(())
    }
}
