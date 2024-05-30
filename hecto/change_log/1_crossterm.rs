use std::io::{self, Read};
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;


// from https://www.flenker.blog/hecto-chapter-2/

// Try it out, and you will notice that every character you type in is immediately printed out, 
// and as soon as you type q, the program ends.
fn main() {
    enable_raw_mode().unwrap();
    // 1. 从标准输入读取字符
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        // is_control() tells us if c is a control character , which is something we wouldn't want to print out (such as Backspace).
        if c.is_control() {
            println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
        } else {
            // {0:08b} 获取字符串后的第0个参数（即 b）。它获取该参数并使用其二进制表示（在大括号末尾的 b 表示的是二进制，这个 b 和参数也被称为 b 没有关系）。中间的 08 表示将二进制表示填充为 8 位，这样输出更易读。
            // {0:#03} 也获取 b，即第 0 个参数。这次，我们将它基本上原样输出，但我们将它填充为 3 位的 0。
            // 1:#?} 现在获取第一个参数（即 c）并输出其调试表示，这是面向开发人员的。如果在调试表示和我们先前使用的表示之间切换，看看会发生什么1
            println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
        }
        println!("{}", c);
        // 2.Press q to quit
        if c == 'q' {
            disable_raw_mode().unwrap();
            break;
        }
    }

    println!("Exiting...");
 }