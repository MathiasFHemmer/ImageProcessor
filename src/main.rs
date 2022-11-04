mod commands;
mod handlers;
mod image_history;
mod util;

use std::{ffi::OsString};
use std::sync::Mutex;
use image_history::{ImageRepository};
use terminal_size::{Width, Height, terminal_size};
use clap::Parser;

static DATABASE: Mutex<ImageRepository> = Mutex::new(ImageRepository{
    repository: Vec::new(),
    selected_history_id: 0,
    last_history_id: 0,
});

#[show_image::main]
fn main() {
    loop
    {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let argus = string_to_args(&input);
        let test = commands::commands::Commands::try_parse_from(argus);
        match test{
            Ok(cmd) => main_handler(cmd),
            Err(b) => {b.print().expect("what the fuck?"); ()},
        }

        if input == "exit".to_string(){ break;}
    }
}

fn main_handler(command: commands::commands::Commands){
    match command.operation{
        commands::commands::OperationType::Repository(args) => handlers::repository::repository::handle(args),
        commands::commands::OperationType::Image(args) => handlers::image::image::handle(args),
    }
}

fn string_to_args(string: &str) -> Vec<OsString> {
    // TODO: add handling of whitespace characters in quotes and character escaping
    let mut args = vec![OsString::from("pid")];

    for arg in string.split_whitespace() {
        args.push(arg.into());
    }
    args
}

fn print_header(){
    let size = terminal_size();
    
    if let Some((Width(w), Height(h))) = size {
        println!("{}", str::repeat("=", w as usize).to_string());
        println!("PID:      Processador de Imagens Digitais");
        println!("Autor:    Mathias Hemmer");
        println!("mathias.f.hemmer@gmail.com");
        println!("Versão:   0.1");
        println!("{}", str::repeat("=", w as usize).to_string());
        println!("");
        println!("Este programa serve como um editor de imagens atraves do processamento delas por comandos. Cada imagem alterada manten-se em um historico que pode ser acessado.");
        println!("Todas as imagnes residem no Repositorio, uma agrupamento de todos os historicos gerados.");
        println!("Cada nova imagem gera uma nova linha de tempo para ela, um novo historico");
        println!("Cada operacao na imagem gera uma nova versao dela, que é adicionada ao histórico");
        println!("Históricos podem ser ramificados, gerando um novo histórico a partir de uma imagem de um histórico original");
        println!("");
        println!("Podemos entender como:");
        println!("Repositorio:");
        println!("- Histórico 1: Img original -> Convolução na Img -> Extração do Azul");
        println!("- Histórico 2: Img original -> Escala de Cinza");
        println!("{}", str::repeat("=", w as usize).to_string());
        println!("");
        println!("Para inciar, digite o comando 'ajuda'");


    } else {
        println!("Unable to get terminal size");
    }

}




