use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub enum Errores {
    LeerArchDir(io::Error),
}

// Implementa From<io::Error> para que `?` pueda convertir automáticamente los errores
impl From<io::Error> for Errores {
    fn from(error: io::Error) -> Self {
        Errores::LeerArchDir(error)
    }
}

struct Documento{
    nombre : String,
    id_doc : i32,
}

struct EstadisticaPalabra {
    id_doc:i32,
    cant:i32,
}

impl EstadisticaPalabra {
    pub fn new(id_doc:i32 ,cant:i32) -> EstadisticaPalabra {
        EstadisticaPalabra {
            id_doc,
            cant,
        }
    }

    pub fn sumar_cant(&mut self){
        self.cant += 1;
    }

    pub fn get_id_doc(&self) -> i32{
        self.id_doc
    }
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
        .filter(|s| !s.is_empty()) //Elemina los tokens ""
        .map(|s| s.to_string())
        .collect();

    contenido_tokenizado
}

fn obtener_stop_words() -> Result<Vec<String>, Errores> {
    let mut stop_words = Vec::new();

    let archivo = fs::File::open("src/stop_words.txt")?;
    let lector = BufReader::new(archivo);

    for linea in lector.lines() {
        stop_words.push(linea?)
    }

    Ok(stop_words)
}

fn parsear_contenido(contenido:String) -> Result<Vec<String>, Errores>{
    let mut contenido_parseado = Vec::new();

    let minuscula = contenido.to_lowercase();
    let signo_puntuacion = eliminar_signo_de_puntuacion(minuscula);
    let mut tokenizado = tokenizar(signo_puntuacion);
    let stop_words = obtener_stop_words()?;

    tokenizado.retain(|t| !stop_words.contains(t)); //Elimina los stop_words del vector tokenizado
    contenido_parseado = tokenizado.clone();

    Ok(contenido_parseado)
}

fn agregar_al_hash(palabra:String, hash : &mut HashMap<String, Vec<EstadisticaPalabra>>) -> &mut HashMap<String, Vec<EstadisticaPalabra>> {

    if !hash.contains_key(&palabra){

    }

    hash
}

fn leer_todos_los_arch(dir : String) -> Result<HashMap<String, Vec<EstadisticaPalabra>>, Errores>{
    let mut hash = HashMap::new();

    let entradas_de_arch = fs::read_dir(dir)?;

    //Para cada archivo que sea extencion .txt se va a leer el contenido del documento
    for entrada_arch in entradas_de_arch {
        let entrada = entrada_arch?; //Devuelve la entrada del inodo leido
        let path_arch = entrada.path(); //Obtiene el path de la entrada del archivo

        //Verifica si la entrada leida es un archivo y es una extencion .txt
        if path_arch.is_file() && path_arch.extension().and_then(|s| s.to_str()) == Some("txt"){
            let contenido = leer_arch(&path_arch)?; //Baja todo el contenido del acrhivo
            let contenidos_parseado = parsear_contenido(contenido)?; //Hace el preprocesamiento y la vectorizacion al contenido leido

            for (id_doc, palabra) in contenidos_parseado.iter().enumerate(){
                if !hash.contains_key(palabra){
                    let estadistica_palabra = EstadisticaPalabra::new(id_doc as i32, 1);
                    let mut vector_estadisticas_palabras : Vec<EstadisticaPalabra> = Vec::new();
                    vector_estadisticas_palabras.push(estadistica_palabra);
                    hash.insert(palabra.to_string(), vector_estadisticas_palabras);
                }
                else if hash.contains_key(palabra) && hash.get_mut(palabra).unwrap().get_mut(id_doc).unwrap().get_id_doc() == id_doc as i32{
                    hash.get_mut(palabra).unwrap().get_mut(id_doc).unwrap().sumar_cant();
                }
            }

        }
    }

    Ok(hash)
}

fn leer_arch(path_arch : &Path) -> Result<String, Errores>{
    let mut arch = fs::File::open(path_arch)?;
    let mut contenido = String::new();

    arch.read_to_string(&mut contenido)?;

    Ok(contenido)
}

#[test]
pub fn test_parsear_contenido(){
    let resultado = parsear_contenido("El hola como como y\n".to_string()).unwrap();
    let esperado = vec!["hola".to_string(), "como".to_string(), "como".to_string()];
    assert_eq!(resultado, esperado)
}

fn main() {
    println!("Hello, world!");
}
