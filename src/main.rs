use std::env;
use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use json;

fn main() -> std::io::Result<()> {
    let his: String = env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or("none".to_string());
    if his.contains("none") {
        return Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Not found"))
    }

    let shypr_path = format!("/tmp/hypr/{his}/.socket.sock");
    let mut value = String::new();
    let mut stream = UnixStream::connect(shypr_path.clone())?;
    stream.write(b"-j/monitors")?;


    stream.read_to_string(&mut value)?;
    stream.shutdown(std::net::Shutdown::Both)?;

    let obj = json::parse(&value.to_string()).unwrap();

    for i in 0..obj.len() {
        match obj[i]["focused"].as_bool() {
            Some(v) => {
                if !v {
                    value = obj[i]["name"].to_string()
                }
            },
            None => {}
        }
    }

    let value = format!("/dispatch focusmonitor {}",value);
    let mut stream = UnixStream::connect(shypr_path.clone())?;
    stream.write(value.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Both)?;

    Ok(())
}
