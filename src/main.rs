//
// zhttpto.rs
//
// Starting code for PS1
// Running on Rust 0.12
//
// Note that this code has serious security risks! You should not run it
// on any system with access to sensitive files.
//
// University of Virginia - cs4414 Spring 2014
// Weilin Xu and David Evans
// Version 0.3

#![feature(globs)]

use std::io::*;
use std::str;

fn main() {

    let addr = "127.0.0.1:4414";
    let mut acceptor = net::tcp::TcpListener::bind(addr).listen().unwrap();

    println!("Listening on [{}] ...", addr);

    for stream in acceptor.incoming() {
        // Spawn a task to handle the connection
        spawn(proc() {

            let mut stream = stream;

            match stream {
                Ok(ref mut s) => {
                    match s.peer_name() {
                        Ok(pn) => println!("Received connection from: [{}]", pn),
                        Err(_) => ()
                    }
                },
                Err(_) => ()
            }

            let mut buf = [0, ..500];
            let _ = stream.read(&mut buf);
            let request_str = str::from_utf8(&buf);
            println!("Received request :\n{}", request_str);

            let response =
                "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                 <doctype !html><html><head><title>Hello, Rust!</title>
                 <style>body { background-color: #111; color: #FFEEAA }
                        h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                        h2 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm green}
                 </style></head>
                 <body>
                 <h1>Greetings, Krusty!</h1>
                 </body></html>\r\n";
            let _ = stream.write(response.as_bytes());
            println!("Connection terminates.");
        });
    }

    drop(acceptor);
}
