struct VM{
    mem :[u8;4096],
    reg :[u8;16],
    add:u16,
    pc:u16,
    scr:[u8;64*32]
}

fn main(){
    let mut vm = VM{
        mem : [0;4096],
        reg : [0;16],
        add : 0,
        pc  : 0,
        scr : [0;64*32]
    };
    println!("Hello world {}" , vm.mem[200]);
}


enum exit_code{
    OK,
}

fn run(machine:VM) -> exit_code{
    exit_code::OK
}


