use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader, Read};
use std::path::Path;

#[derive(Debug)]
pub enum Errores {
    IoError(io::Error),
    LeerArchDir,
    leerInputUs
}

// Implementa From<io::Error> para que `?` pueda convertir automáticamente los errores
impl From<io::Error> for Errores {
    fn from(error: io::Error) -> Self {
        Errores::IoError(error)
    }
}

struct Documento{
    nombre : String,
    id_doc: i32,
}
impl Documento{
    pub fn new(nombre:String, id_doc:i32) -> Documento{
        Documento{
            nombre,
            id_doc
        }
    }
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

fn leer_todos_los_arch(dir : String, documentos:&mut Vec<Documento>) -> Result<HashMap<String, Vec<EstadisticaPalabra>>, Errores>{
    let mut hash = HashMap::new();

    let entradas_de_arch = fs::read_dir(dir);

    let mut id_doc = 0;

    //Para cada archivo que sea extencion .txt se va a leer el contenido del documento
    for entrada_arch in entradas_de_arch? {
        let entrada = entrada_arch?; //Devuelve la entrada del inodo leido
        let path_arch = entrada.path(); //Obtiene el path de la entrada del archivo

        //Verifica si la entrada leida es un archivo y es una extencion .txt
        if path_arch.is_file() && path_arch.extension().and_then(|s| s.to_str()) == Some("txt"){
            let nombre_archivo = entrada.file_name().to_str().unwrap().to_string(); //Obtener el nombre del documento
            documentos.push(Documento::new(nombre_archivo, id_doc as i32));//Colocarlo en el vector de documentos del corpus
            let contenido = leer_arch(&path_arch)?; //Baja todo el contenido del acrhivo
            let contenidos_parseado = parsear_contenido(contenido)?; //Hace el preprocesamiento y la vectorizacion al contenido leido

            for palabra in contenidos_parseado.iter(){
                if !hash.contains_key(palabra){//Si no esta contenida la palabra (la clave) en el hashmap entonces colocar la palabra (la clave) como una nueva entrada al hashmap
                    let estadistica_palabra = EstadisticaPalabra::new(id_doc as i32, 1); //Inicializo las estadisticas de la palabra nueva en el documento presente
                    let mut vector_estadisticas_palabras : Vec<EstadisticaPalabra> = Vec::new();//Creo un nuevo vector de las estadisticas de la palabras para los documentos que aparezca
                    vector_estadisticas_palabras.push(estadistica_palabra);//Agrego las estadisticas de la palabra del documento en un vector
                    hash.insert(palabra.to_string(), vector_estadisticas_palabras);//Agrego la palabra con sus estadisticas en cada documento (hasta ahora es solo un doc) en el hash
                }
                else {
                    let estadisticas_de_los_docs = hash.get_mut(palabra).unwrap();
                    let mut encontrado = false;
                    for estadistica_en_el_doc in estadisticas_de_los_docs.iter_mut() {
                        if estadistica_en_el_doc.get_id_doc() == id_doc{
                            estadistica_en_el_doc.sumar_cant();
                            encontrado = true;
                            break
                        }
                    }
                    if !encontrado {
                        let estadistica_palabra = EstadisticaPalabra::new(id_doc, 1);
                        estadisticas_de_los_docs.push(estadistica_palabra)
                    }
                }
            }
        }
        id_doc+=1
    }
    Ok(hash)
}


fn leer_arch(path_arch : &Path) -> Result<String, Errores>{
    let mut arch = fs::File::open(path_arch)?;
    let mut contenido = String::new();

    arch.read_to_string(&mut contenido)?;

    Ok(contenido)
}

/*##########################----->CALCULO DE SCORE<--------########################*/
/*
El último paso es implementar la búsqueda. Para ello, se debe solicitar al usuario una frase a buscar y aplicar la
tokenización de la misma y la eliminación de las stop words. Se debe buscar los documentos que contengan los términos de búsqueda ingresados.

Luego se debe determinar la relevancia de cada documento resultado de la búsqueda. Para esto, se debe determinar
el puntaje del documento. Esto se puede computar a partir de sumar las frecuencias de cada uno de los términos encontrados.

Para mejorar el cálculo de puntaje del documento, calcularemos la frecuencia inversa de documentos para un término
(denominado tf-idf) dividiendo la cantidad de documentos (N) en el índice por la cantidad de documentos que contienen
el término, y tomaremos el logaritmo.
*/

fn input_usuario() -> Result<String, Errores>{
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => { Ok(input) }
        Err(_) => { Err(Errores::leerInputUs) }
    }
}

#[test]
pub fn test_parsear_contenido(){
    let resultado = parsear_contenido("El hola como como y\n".to_string()).unwrap();
    let esperado = vec!["hola".to_string(), "como".to_string(), "como".to_string()];
    assert_eq!(resultado, esperado)
}

#[test]
pub fn test_leer_todos_los_arch(){
    let mut vector : Vec<Documento> = Vec::new();
    let resultado = leer_todos_los_arch("src/corpus".to_string(), &mut vector).unwrap();
    assert!(resultado.contains_key(&"futbol".to_string()))
}

fn main() {
    println!("Hello, world!");
}
