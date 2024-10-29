use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};

fn main (){
    loop {
        // usually print! will store the content in a cache, 
        // so sometimes we can not immediately see "<" 
        // by using flush we can ensure "<" will show before read_line
        print!("> ");
        stdout().flush();
        // read input
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        // seperate input into several commands by "|"
        let mut commands = input.trim().split(" | ").peekable();
        // initialize previous_command as None
        let mut previous_command = None; 
        // use Some(subcommand) because commands.next() will return Option
        while let Some (subcommand) = commands.next() {
            // for each subcommand in commands: it can be seperated into action + args
            // split input by whitespace so it can handle command like "ls -a"
            let mut parts = subcommand.trim().split_whitespace();
            let action = parts.next().unwrap();
            let args = parts;
            
            match action {
                // manage shell build-in
                "cd" => {
                    // get the next args as new_dir, if no args, use "/"
                    let new_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new (new_dir);
                    // set current dir as root(new_dir)
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    // cd does not have any output, after cd the previous_command (cd) should be None
                    previous_command = None;
                },
                "exit" => return,
                _ => {
                    let stdin = previous_command
                        .map_or(
                            // input comes from terminal
                            Stdio::inherit(),
                            // set the output of type Child, use output.stdout.unwrap() to get the object of ChildStdout and change it into object of Stdio
                            |output: Child| Stdio::from (output.stdout.unwrap())
                        );
                    let stdout = if commands.peek().is_some() {
                        // creates a pipe to send the output through a pipe to input
                        Stdio::piped() 
                    } else {
                        // outputs directly to the terminal
                        Stdio::inherit() 
                    };

                    let output = Command:: new (action)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();// allows parallel processing

                    match output {
                        Ok (output) => {previous_command = Some (output);}, // why not use wait??
                        Err(e) => {
                            previous_command = None;
                            eprintln! ("{}", e);
                        }
                    }
                }
            }
        }
        if let Some (mut final_command) = previous_command {
            final_command.wait();
        }
    }
   

    
}