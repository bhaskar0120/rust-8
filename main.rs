struct VM{
    mem :[u8;4096],
    reg :[u8;16],
    add:u16,
    pc:u16,
    sp:u16,
    scr:[u8;64*32]
}

fn main(){
    let mut vm = VM{
        mem : [0;4096],
        reg : [0;16],
        add : 0,
        pc  : 512,
        sp  : 511,
        scr : [0;64*32]
    };
    let s = std::fs::read("./test/IBM.ch8").unwrap();
    let mut counter = 0;
    for i in s.iter(){
        vm.mem[counter+512] = *i;
        counter+=1;
    }
    println!("{:?}",run(vm));

}

fn print_screen(scr:&[u8;64*32]){
    print!("\x1b[2J");
    for i in 0..32{
        for j in 0..64{
            if scr[i*64+j] == 1{
                print!("@");
            }
            else{
                print!(" ");
            }
        }
        print!("\n");
    }
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

        match nib[0]{
            0x0 => {
                match nib[3]{
                    0x0 => {
                        for i in machine.scr.iter_mut(){
                            *i = 0_u8;
                        }
                    },
                    0xe =>{
                        let new_add:u16 = ( (machine.mem[(machine.sp+1) as usize] as u16  )<< 8) 
                            | (machine.mem[(machine.sp+2) as usize]) as u16;
                        machine.pc = new_add;
                        machine.sp += 2;
                    }
                    _ =>{
                        panic!("Can't reach this");
                    }
                }
            },
            0x1 => {
                machine.pc = (nib[1] as u16)<<8|(nib[2] as u16)<<4|(nib[3] as u16);
            },
            0x2 => {
                machine.mem[machine.sp as usize] = (machine.pc&0x0F) as u8;
                machine.sp -= 1;
                machine.mem[machine.sp as usize] = (machine.pc&0xF0) as u8;
                machine.sp -= 1;
                machine.pc = (nib[1] as u16)<<8|(nib[2] as u16)<<4|(nib[3] as u16);
            },
            0x3 => {
                if machine.reg[nib[1] as usize] == ((nib[2]<<4) | nib[3]){
                    machine.pc+=2;
                }

            },
            0x4 => {
                if machine.reg[nib[1] as usize] != ((nib[2]<<4) | nib[3]){
                    machine.pc+=2;
                }
            },
            0x5 => {
                if machine.reg[nib[1] as usize] == machine.reg[nib[2] as usize]{
                    machine.pc+=2;
                }
            },
            0x6 => {
                machine.reg[nib[1] as usize] = (nib[2]<<4) | nib[3];
            },
            0x7 => {
                machine.reg[nib[1] as usize] = ( (machine.reg[nib[1] as usize] as usize + 
                                                  ((nib[2] << 4) | nib[3]) as usize)%255 ) as u8;
            },
            0x8 => {
                match nib[3] {
                    0x0 => {machine.reg[nib[1] as usize] = machine.reg[nib[2] as usize]},
                    0x1 => {machine.reg[nib[1] as usize] |= machine.reg[nib[2] as usize]},
                    0x2 => {machine.reg[nib[1] as usize] &= machine.reg[nib[2] as usize]},
                    0x3 => {machine.reg[nib[1] as usize] ^= machine.reg[nib[2] as usize]},
                    0x4 => {
                        if 255_u8 - machine.reg[nib[1] as usize] < machine.reg[nib[2] as usize] {
                            machine.reg[0xf] = 1;
                            machine.reg[nib[1] as usize] = machine.reg[nib[2] as usize] - (255_u8 - machine.reg[nib[1] as usize]);
                        }
                        else{
                            machine.reg[nib[1] as usize] += machine.reg[nib[2] as usize];
                        }
                    },
                    0x5 => {
                        if machine.reg[nib[2] as usize] > machine.reg[nib[1] as usize]{
                            machine.reg[0xf] = 0;
                            machine.reg[nib[1] as usize] = machine.reg[nib[2] as usize] - machine.reg[nib[1] as usize];
                        }
                        else{
                            machine.reg[0xf] = 1;
                            machine.reg[nib[1] as usize] -= machine.reg[nib[2] as usize];
                        }
                    },
                    0x6 => {
                        machine.reg[0xf] = machine.reg[nib[1] as usize]&1;
                        machine.reg[nib[1] as usize] >>= 1;
                    },
                    0xe => {
                        machine.reg[0xf] = machine.reg[nib[1] as usize]&0x80;
                        machine.reg[nib[1] as usize] <<= 1;
                    },
                    0x7 => {
                        if machine.reg[nib[2] as usize] > machine.reg[nib[1] as usize]{
                            machine.reg[0xf] = 1;
                            machine.reg[nib[1] as usize] = machine.reg[nib[2] as usize] - machine.reg[nib[1] as usize];
                        }
                        else{
                            machine.reg[0xf] = 0;
                            machine.reg[nib[1] as usize] -= machine.reg[nib[2] as usize];
                        }
                    },
                    _ =>{ panic!("Can't reach here"); }
                }
            },
            0x9 => {
                if machine.reg[nib[1] as usize] != machine.reg[nib[2] as usize]{
                    machine.pc+=2;
                }
            },
            0xa => {
                machine.add = (nib[1] as u16)<<8|(nib[2] as u16)<<4|(nib[3] as u16);
            },
            0xb => {
                machine.pc = machine.reg[0] as u16 + ((nib[1] as u16)<<8|(nib[2] as u16)<<4|(nib[3] as u16));

            },
            0xc => {
                machine.reg[nib[1] as usize] = rand::random::<u8>() & ((nib[2]<<4) | nib[3]);
            },
            0xd => {
                let X:usize = machine.reg[nib[1] as usize] as usize;
                let Y:usize = machine.reg[nib[2] as usize] as usize;
                let N:usize = nib[3] as usize;
                for i in 0..N{
                    for j in 0..8{
                        let x:usize = (X+j)%64;
                        let y:usize = (Y+i)%32;
                        let index = (y*64+x) as usize;
                        let pixel = (machine.mem[(machine.add+i as u16) as usize]>>(7-j))&1;
                        machine.scr[index] ^= pixel;
                    }
                }
                print_screen(&machine.scr);
            },
            0xe => {
                todo!();
                match nib[3]{
                    //TODO
                    0xe => {
                        if machine.reg[nib[1] as usize] == 0x9e{
                            machine.pc+=2;
                        }
                    },
                    0x1 => {
                        if machine.reg[nib[1] as usize] == 0xa1{
                            machine.pc+=2;
                        }
                    },
                    _ => {panic!("Can't reach here")}
                }
            },
            0xf => {
                match nib[2]{
                    0x07 => {
                        machine.reg[nib[1] as usize] = 0;
                    },
                    0x0a => {
                        todo!();
                    },
                    0x15 => {
                        todo!();
                    },
                    0x18 => {
                        todo!();
                        //TODO
                    },
                    0x1e => {
                        todo!();
                        //TODO
                    },
                    0x29 => {
                        todo!();
                        //TODO
                    },
                    0x33 => {
                        todo!();
                        //TODO
                    },
                    0x55 => {
                        todo!();
                        //TODO
                    },
                    0x65 => {
                        todo!();
                        //TODO
                    },
                    _ => {panic!("Can't reach here")}
                }
            },
            _ => {panic!("Can't reach here")},
        }
    };
    ExitCodes::OK
}


