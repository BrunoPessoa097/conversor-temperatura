use core::f32;
use std::io;

/// Converte a temperatura informada para fahrenheit.
fn fahrenheit(t:f32) -> f32{
    (t * 1.8)+32.0
}

/// Converte a temperatura informada para celsius.
fn celsius(t:f32) -> f32{
    (t-32.00)/1.8
}

fn main() {
    loop {
        println!("converta o em celsius para fahrenheit");

        // input opção usuário.
        println!("Escolha o tipo da conversão:");
        println!("Digite 1-Celsius");
        println!("Digite 2-Fahrenheit");
        println!("Digite 3-finalizar o programa");

        let mut esc = String::new();

        // Lendo as informações informadas dos usuários.
        io::stdin()
                .read_line(&mut esc)
                .expect("Escolha errada");

        // Converte a informação de string para numérico.
        let _esc:f32 = match esc.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                -1.00
            }
        };

        // Input temperatura usuário.
        println!("Digite a temperatura: ");
        let mut temperatura = String::new();

        // Lendo as informações do usuário.
        io::stdin()
            .read_line(&mut temperatura)
            .expect("Temperatura errada");

        // Converte a informação de string para numérico.
        let temperatura:f32 = match temperatura.trim().parse() {
            Ok(num) => num,
            Err(_e) => {
                -1.00
            }
        };

        // Selecionando a opção do usuário.
        if _esc == 1.0{
            let c:f32 = celsius(temperatura);
            println!("temperatura em celsius: {c}");
            break;
        }else if _esc ==2.0{
            let f:f32 = fahrenheit(temperatura);
            println!("temperatura em fahrenheit: {f}");
            break;
        }else{
            break;
        };
    }
}
