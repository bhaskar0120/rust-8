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
        pc  : 512,
        scr : [0;64*32]
    };
    let s = std::fs::read("./test/jason.ch8").unwrap();
    let mut counter = 0;
    for i in s.iter(){
        vm.mem[counter+512] = *i;
        counter+=1;
    }
    println!("{:?}",run(vm));

}


#[derive(Debug)]
enum ExitCodes{
    OK,
}

fn run(mut machine:VM) -> ExitCodes{
    loop{
        let nib:[u8;4] = [
        ( machine.mem[machine.pc as usize]&0xF0)>>4,
        machine.mem[machine.pc as usize]&0x0F,
        ( machine.mem[(machine.pc+1) as usize]&0xF0)>>4,
        machine.mem[(machine.pc+1) as usize]&0x0F,
        ];
        machine.pc+=2;

        println!("{:?}",&nib);
        break;
    };
    ExitCodes::OK
}


