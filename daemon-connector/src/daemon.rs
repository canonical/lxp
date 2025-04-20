use std::{io::{Read, Write}, os::unix::net::{UnixListener, UnixStream}, path::PathBuf};

use anyhow::{anyhow, Context};

use crate::{message::Message, serve_target::ServeTarget};

const MESSAGE_SIZE: usize = 1024;

pub struct LinuxPoolDaemon {
    stream: UnixStream,

    is_serving: bool,
    target: ServeTarget,
}

impl LinuxPoolDaemon {
    pub fn serve(target: ServeTarget) -> anyhow::Result<Self> {
        match dirs::home_dir() {
            Some(home_directory) => {
                let socket_path: &PathBuf = match &target {
                    ServeTarget::Root => &home_directory.join(PathBuf::from("lxp.socket")),
                    ServeTarget::Client(file) => &home_directory.join(format!("lxp.{}.socket", file)),
                };

                if std::fs::metadata(socket_path).is_ok() {
                    std::fs::remove_file(socket_path)
                        .with_context(|| { format!("could not delete previous socket at {:?}", socket_path) })?;
                }

                let socket_listener = UnixListener::bind(socket_path).context("Could not create the unix socket");
                let (stream, _) = socket_listener?
                    .accept()
                    .context("Failed to accept client connection")?;

                Ok(
                    LinuxPoolDaemon {
                        stream,
                        target,
                        is_serving: true,
                    }
                )
            },
            None => Err(anyhow!("Could not find /home/$USER/ directory"))
        }
    }

    pub fn connect(target: ServeTarget) -> anyhow::Result<Self> {
        match dirs::home_dir() {
            Some(home_directory) => {
                let socket_path: &PathBuf = match &target {
                    ServeTarget::Root => &home_directory.join(PathBuf::from("lxp.socket")),
                    ServeTarget::Client(file) => &home_directory.join(format!("lxp.{}.socket", file)),
                };

                let stream: UnixStream = UnixStream::connect(socket_path)
                    .context("Could not connect to daemon, make sure it is installed and running")?;

                Ok(
                    LinuxPoolDaemon {
                        stream,
                        target,
                        is_serving: false,
                    }
                )
            },
            None => Err(anyhow!("Could not find /home/$USER/ directory"))
        }
    }

    pub fn send_message(&mut self, message: &Message) -> anyhow::Result<()> {
        self.write(message)
    }

    pub fn send_request(&mut self, message: &Message) -> anyhow::Result<Message> {
        self.write(message)?;
        self.read()
    }

    pub fn listen_for_message(&mut self) -> anyhow::Result<Message> {
        self.read()
    }

    fn write(&mut self, message: &Message) -> anyhow::Result<()> {
        let message: Vec<u8> = bincode::serialize(&message)?;
        let length: usize = message.len();

        if length > MESSAGE_SIZE - 4 {
            anyhow::bail!("Message too large to write ({} bytes > {} bytes)", length, MESSAGE_SIZE);
        }

        let mut buffer: [u8; MESSAGE_SIZE] = [0u8; MESSAGE_SIZE];
        buffer[..4].copy_from_slice(&(length as u32).to_be_bytes());
        buffer[4..4 + length].copy_from_slice(&message);

        self.stream.write_all(&buffer)?;
        self.stream.flush()?;

        Ok(())
    }

    fn read(&mut self) -> anyhow::Result<Message> {
        let mut buffer: [u8; MESSAGE_SIZE] = [0u8; MESSAGE_SIZE];
        self.stream.read_exact(&mut buffer)?;

        let length = u32::from_be_bytes([buffer[0], buffer[1], buffer[2], buffer[3]]);
        if length > (MESSAGE_SIZE - 4) as u32 {
            anyhow::bail!("Message too large to read ({} bytes > {} bytes)", length, MESSAGE_SIZE);
        }

        let message = bincode::deserialize(&buffer[4..4 + length as usize])?;
        Ok(message)
    }
}

impl Drop for LinuxPoolDaemon {
    fn drop(&mut self) {
        if !(self.target.is_root() || self.is_serving) {
            self.write(&Message::End).ok();
        }

        if self.is_serving {
            if let Some(home_directory) = dirs::home_dir() {
                let socket_path: &PathBuf = match &self.target {
                    ServeTarget::Root => &home_directory.join(PathBuf::from("lxp.socket")),
                    ServeTarget::Client(file) => &home_directory.join(format!("lxp.{}.socket", file)),
                };
    
                if std::fs::metadata(socket_path).is_ok() {
                    std::fs::remove_file(socket_path).ok();
                }
            }
        }
    }
}
