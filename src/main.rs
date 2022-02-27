use lunatic::{net, process, Mailbox};
//use lunatic::net::{local_addr};
use std::io::{BufRead, BufReader, Write};

use rsbqn::fmt::{fmt_err};
use rsbqn::gen::code::c;
use rsbqn::schema::{A,Env,Stack,V,Ve,Vn,new_string};
use rsbqn::vm::{call,formatter,prog,run,runtime};
use bacon_rajan_cc::Cc;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:10080").unwrap();
    //println!("Listening on addr: {}", listener.local_addr().unwrap());
    while let Ok((tcp_stream, _peer)) = listener.accept() {
        // Pass the TCP stream as a context to the new process. We can't use a closures that
        // capture parent variables because no memory is shared between processes.
        process::spawn_with(tcp_stream, handle).unwrap();
    }
}

fn handle(mut tcp_stream: net::TcpStream, _: Mailbox<()>) {
    let mut buf_reader = BufReader::new(tcp_stream.clone());
    // rsbqn stuff
    let mut stack = Stack::new();
    let root = Env::new_root();
    let runtime = runtime(Some(&root),&mut stack).expect("couldnt load runtime");
    let compiler = run(Some(&root),&mut stack,c(&runtime)).expect("couldnt load compiler");
    let src = new_string("{𝕩+1}");
    let mut names = V::A(Cc::new(A::new(vec![],vec![0])));
    let mut redef = V::A(Cc::new(A::new(vec![],vec![0])));
    let program =
        match prog(&mut stack,&compiler,src,&runtime,&root,&names,&redef,0.0) {
            Ok((prog,_newnames,_newredef)) => {
                //names = V::A(Cc::new(newnames));
                //redef = V::A(Cc::new(newredef));
                //info!("names = {}",&names);
                //info!("redef = {}",&redef);
                prog
            },
            Err(e) => match e {
                Ve::S(s) => panic!("{}",s),
                Ve::V(v) => match v {
                    V::A(a) => {
                        match a.r.len() {
                            2 => panic!("{}",fmt_err(&a.r[1].as_a().unwrap().r)),
                            _ => panic!("{}",fmt_err(&a.r)),
                        }
                    },
                    _ => panic!("cant error on type"),
                },
            },
        };
    let exec =
        match run(Some(&root),&mut stack,program) {
            Ok(exec) => {
                exec
            },
            Err(e) => match e {
                Ve::S(s) => panic!("{}",s),
                Ve::V(v) => match v {
                    V::A(a) => panic!("{}",fmt_err(&a.r)),
                    _ => panic!("cant error on type"),
                },
            },
        };
    let mut state = V::Scalar(0.0);
    loop {
        let mut buffer = String::new();
        let read = buf_reader.read_line(&mut buffer).unwrap();
        let rtn = call(&mut stack,1,Vn(Some(&exec)),Vn(Some(&state)),Vn(None));
        match rtn {
            Ok(v) => {
                state = v.into_v().unwrap();
                println!("{:?}",&state);
            },
            Err(e) => panic!("some error"),
        }
        if buffer.contains("exit") || read == 0 {
            return;
        }
        println!("msg: {:?}",&buffer.as_bytes());
        tcp_stream.write(buffer.as_bytes()).unwrap();
    }
}
