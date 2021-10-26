//
//@author 001-Bayek-烟台 Team1 第七期
//@dev 使用Rust std标准库的功能实现一个tcp server
//

// 引用类库 io、net 用来完成TCP监听读取，
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
// 引入 thread 类库用来多线程处理
use std::thread;
// 引入 str 库，用来转换输入的 buf 到 str 类型。
use std::str;
// 引入 time 库，来模拟链接接入时间
use std::time;

fn main() { 
    // 创建一个Tcp监听，使其监听bind函数中的ip:port，代码执行后（MacOS）可以通过 lsof -i:8866 进行验证端口启动
    let listener = TcpListener::bind("127.0.0.1:1105").unwrap();
    // 调用 incoming() 方法接收客户端的链接信息，如果有新的信息进来就会返回一个Result枚举，OK(T:TcpStream)
    for stream in listener.incoming() {
        // 如果有客户端链接比如通过： telnet 127.0.0.1 1105 
        println!("监测到有新的链接进入");
        // match模式匹配
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                // 如果链接成功，开启一个新的线程（考虑到TCP客户请求可能有多个，因此用多线程）
                thread::spawn(move|| {
                    // 将客户端处理信息解耦到 handle_client 函数中，并移交 stream 变量所有权
                    handle_client(stream);
                });
            }
            // 当Result 枚举匹配错误时
            Err(e) => { 
                // 直接通过panic!宏输出错误信息，并终止程序运行。
                panic!("Oh, Something wrong here: {:?}", e) 
            }
        }
    }
    // 关闭Tcp监听链接
    drop(listener);
}

// 线程调用的处理函数
fn handle_client(mut stream: TcpStream) {
    // 调用sleep函数模拟实现客户链接处理过程，间隔1s
    thread::sleep(time::Duration::from_secs(1));
    println!("客户链接处理中...");

    // 定义一个存储用的数组，因为需要后续进行填充值所以声明为可变的 `mut`
    let mut buf = [0; 512];
    // 建立一个循环，来反复读取客户的输入信息，默认服务端是不关闭的
    loop {
        // 通过read方法,从流里面读内容，读到buf中
        let bytes_read = stream.read(&mut buf).expect("Failed to read!");

        // 把"Read from telnet: "转换成字节后，写到stream流中
        stream.write(b"Read from telnet: ").unwrap();

        // 输出调试信息
        println!("Content Details: byte size: {}", bytes_read);
        // 如果输入流的字符长度是0（读到结尾了）直接退出循环。
        if bytes_read == 0 {
            // 退出loop，实际上这里退出后整个方法也就退出了。
            break;
        }

        // 为了后面对比方便，将byte[] 转换为str 类型。
        let s = match str::from_utf8(&buf[..bytes_read]) {
            // 如果转换成功返回字符串值。
            Ok(v) => v,
            // 遇到转换错误输出错误信息，终止程序运行。
            Err(_e) => {
                // 输出调试信息。
                stream.write(b"Need utf-8 sequence.").unwrap();
                // 继续监听，虽然本次输入的字符流格式不是utf8 字符，但是不影响下次输入所以不需要 panic!
                continue;
            },
        };

        // 如果输入的前4个字符串是 `exit`则程序终止，为了防止越界所以需要先判断 s.len() >= 4
        if s.len() >= 4 && s[0..4] == "exit".to_string() {
            // 输出终止前的消息。
            stream.write(b"Exit the program\n").unwrap();
            // 直接跳出 loop 循环，整个Tcp链接终止
            break;
        }
        // 如果程序没有终止，返回输入的消息，也就是输入什么返回什么，unwrap() 表示不处理错误，遇到错误直接出错退出程序。
        stream.write(&buf[..bytes_read]).unwrap();
    }
}