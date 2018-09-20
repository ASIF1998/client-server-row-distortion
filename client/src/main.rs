use std::time::Duration;
use std::thread;

// Этот модуль обеспечивает коммуникацию на основе сообщений по каналам.
// Стандартная библиотека синхронизации.
use std::sync::mpsc:: {
    self, // Импортируем саму библиотеку.
    TryRecvError // Тип ошибки.
};

// TCP-поток между локальным и удаленным сокетами.
use std::net::{
    TcpStream,
    Shutdown
};

use std::io:: {
    self, // Импортируем саму библиотеку.
    ErrorKind, // Код ошибки.
    Read,
    Write
};

#[inline]
fn sleep () {
    thread::sleep(::std::time::Duration::from_millis(100));
}

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;

fn main() {
    let mut client = TcpStream::connect(LOCAL).expect("Поток не удалось подключить");
    client.set_nonblocking(true).expect("Не удалось инициализировать неблокирование");

    loop {
        let mut buff = String::new();
        io::stdin().read_line(&mut buff).expect("Не удалось считать строку с консоли.");
        let mut buff = buff.trim().to_string();

        if buff.len() >= 1 {
            client.write_all(&mut buff.clone().into_bytes());

            if (buff == "exit".to_string()) {
                client.shutdown(Shutdown::Both);
                return;
            }

            sleep();
            let mut buff = vec![0u8; MSG_SIZE];
            client.read(&mut buff);
            // Считываем буффер до того момента, пока x не равер 0.
            let msg = buff.into_iter().take_while(|&x| x !=  0).collect::<Vec<u8>>();

            // Преобразуем набор байт в строку типа utf8.
            let msg = String::from_utf8(msg).expect("Недопустимое сообщение utf8");
            println!("Ответ от сервера: {:?}", msg);

        }
    }
}
