use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..55] {   
        println!("{:?}", arg.split(':').nth(1).unwrap());
    }
}

// {top-tl:"red",top-tm:"blue",top-tr:"red",top-ml:"blue",top-mm:"white",top-mr:"red",top-bl:"orange",top-bm:"green",top-br:"yellow",left-tl:"blue",left-tm:"white",left-tr:"yellow",left-ml:"orange",left-mm:"green",left-mr:"white",left-bl:"white",left-bm:"blue",left-br:"green",front-tl:"green",front-tm:"yellow",front-tr:"orange",front-ml:"green",front-mm:"red",front-mr:"yellow",front-bl:"white",front-bm:"green",front-br:"red",right-tl:"blue",right-tm:"yellow",right-tr:"blue",right-ml:"blue",right-mm:"blue",right-mr:"orange",right-bl:"yellow",right-bm:"white",right-br:"white",back-tl:"white",back-tm:"red",back-tr:"yellow",back-ml:"green",back-mm:"orange",back-mr:"yellow",back-bl:"orange",back-bm:"white",back-br:"orange",bottom-tl:"red",bottom-tm:"red",bottom-tr:"green",bottom-ml:"orange",bottom-mm:"yellow",bottom-mr:"red",bottom-bl:"green",bottom-bm:"orange",bottom-br:"blue"}
