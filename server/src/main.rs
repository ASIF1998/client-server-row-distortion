use std::net::{TcpListener, Shutdown};
use std::thread;
use std::sync::mpsc;
use std::fmt::Write as FmtWrite;

// Стандартный ввод/вывод.
use std::io:: {
    // Список, определяющий общие категории ошибок ввода-вывода.
    ErrorKind,
    // Характеристика чтения позволяет считывать байты из источника.
    Read,
    // Характеристика записи позволяет читать байты из источника.
    Write
};

// Местный хост с портом.
const LOCAL: &str = "127.0.0.1:6000";

// Размер предолжения.
const MSG_SIZE: usize = 32;

// Переводим текущий поток в спящий режим на указанное время.
#[inline]
fn sleep () {
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn distortSentence (sentence: &mut [u8]) -> u64 {

    for i in 0..sentence.len() {
        sentence[i] = sentence[i] + 1;
    }

    ((1.0 / sentence.len() as f32) * 100.0) as u64
}

fn main() {
    let server = TcpListener::bind(LOCAL).expect("Прослушивателю не удалось выполнить привязку");

    // Неблокирующий режим позволяет серверу постоянно проверять наличие сообщений.
    server.set_nonblocking(true).expect("Yе получилось установить не блокирующий режим.");

    loop {
        // После установки будет возвращен соответствующий поток Tcp и адрес удаленного узла.
        if let Ok((mut sockket, addr)) = server.accept() {
            println!("Клиент {} подключился.", addr);

            // каждый клиент обрабатываем в отдельном потоке.
            thread::spawn(move || loop {
                let mut buff = vec![0u8; MSG_SIZE];

                // Прочитать точное количество байтов,
                // необходимых для заполнения buff.
                match sockket.read(&mut buff) {
                    Ok(_) => {
                        let mut msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();


                        if (msg == "exit".to_string().into_bytes()) {
                            sockket.shutdown(Shutdown::Both);
                            break
                        }

                        let dis = distortSentence(&mut msg);
                        let mut msg = String::from_utf8(msg.clone()).expect("Не получилось форматировать строку в utf8");
                        write!(&mut msg, "   {}%", dis);

                        sockket.write(&mut msg.clone().into_bytes()).expect("Не удалось записать в сокет.");
                    }

                    // Операция должна быть заблокирована для завершения, но операция
                    // блокировки не была запрошена.
                    Err(ref err) => if err.kind() == ErrorKind::WouldBlock {()},

                    Err(_) => {
                        println!("Закрытие соединения с:{}", addr);
                        break;
                    }
                }

                sleep();
            });
        }
    }
}
