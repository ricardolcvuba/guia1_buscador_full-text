use std::collections::HashMap;
use std::fs;
use std::io::{self,BufReader, Read};
use std::path::Path;

pub enum Error{
    leer_arch_dir,
}

fn eliminar_signo_de_puntuacion(contenido : String) -> String{
    let mut contenido_limpio = String::new();

    contenido_limpio = contenido.chars()
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
        .collect();

    contenido_limpio
}

fn tokenizar(contenido : String) -> Vec<String> {
    let mut contenido_tokenizado = Vec::new();

    contenido_tokenizado = contenido.split(|c| c==' ' || c=='\n')
        .map(|s| s.to_string())
        .collect();

    contenido_tokenizado
}

fn obtener_stop_words() -> Result<Vec<String>, Error::leer_arch_dir> {
    let mut stop_words = Vec::new();

    let archivo = fs::File::open("src/stop_words.txt");

    let lector = BufReader::new(archivo);

    for linea in lector {
        stop_words.push(linea)
    }

    Ok(stop_words);
}

fn parsear_contenido(contenido:String) -> Vec<String>{
    let mut contenido_parseado = Vec::new();

    let minuscula = contenido.to_lowercase();
    let signo_puntuacion = eliminar_signo_de_puntuacion(minuscula);
    let

    contenido_parseado
}

fn leer_todos_los_arch(dir : String) -> Result<HashMap<String, Vec<i32, i32>>, Error::leer_arch_dir>{
    let hash = HashMap::new();

    for entrada_arch in fs::read_dir(dir)? {
        let entrada = entrada_arch?;
        let path_arch = entrada.path();

        if path_arch.is_file() && path_arch.extension().and_then(|s| s.to_str()) == Some("txt"){
            let contenido = leer_arch(&path_arch)?;
        }
    }

    Ok(hash)
}

fn leer_arch(path_arch : &Path) -> Result<String, Error::leer_arch_dir>{
    let mut arch = fs::File::open(path_arch)?;
    let mut contenido = String::new();

    arch.read_to_string(&mut contenido);

    Ok(contenido)
}

fn main() {
    println!("Hello, world!");
}
