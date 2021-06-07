use std::process::*;
use std::env;
use std::io::prelude::*;
use std::fs::*;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args[1] == "-h" || args[1] == "--help" {
        println!("Usage: ivgtk [options] testbench-file [+soruce-files]");
        println!("Options:");
        println!(" -h, --help           Print help message.");
        exit(0);
    }

    let top = args[1].replace(".v", "");
    
    //wrapper file template
    let wrapper = format!("module tb;\n{0} {0}();\ninitial begin\n$dumpfile(\"sim_output.vcd\");\n$dumpvars(0,tb);\nend\nendmodule\n", top);

    // create wrapper file
    let mut fc = File::create(&Path::new("wrap.v")).unwrap();
    let _ = fc.write_all(wrapper.as_bytes());

    // remove executable file name to arguments
    args.drain(..1);

    // Simulation execution
    let s = Command::new("iverilog")
            .arg("wrap.v")
            .args(args)
            .arg("-o")
            .arg("sim_output")
            .output();

    // Judging of Simulation success
    if let Result::Err(msg) = s {
        println!("Simulation error: {}.", msg);
        let n = msg.raw_os_error();
        if let Some(tt) = n {
            exit(tt)
        }
    }

    remove_file("wrap.v").unwrap();

    // wavefile generate command
    let _ = Command::new("vvp")
            .arg("sim_output")
            .output();
    
    // Show logic wave
    let s = Command::new("gtkwave")
            .arg("sim_output.vcd")
            .output();

    // Judging of gtkwave showing success
    if let Result::Err(msg) = s {
        println!("gtkwave error: {}.", msg);
        let n = msg.raw_os_error();
        if let Some(tt) = n {
            exit(tt)
        }
    }

    remove_file("sim_output").unwrap();
    remove_file("sim_output.vcd").unwrap();
}
