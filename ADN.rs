// se debe crear un programa que al ingresar una cadena de ADN, este debe detectar por separado
// las letras que la componen y entregar su version opuesta, pero con un orden fijo, es decir,
// si el usuario ingresa la cadena TGATCGT, el programa debe imprimir la cadena opuesta ACUAGCA

use std::io::stdin;
fn main() -> io::Result<()> {
    let mut ingreso_cadena = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut ingreso_cadena);
    return cadena
}
fn cambio_letras(cadena){
    let s:String = s.chars()
    .map(|x| match x { 
        'A' => 'T', 
        'T' => 'A', 
        'C' => 'G',
        'G' => 'C',
    }).collect();
    println!("{}", s);
}
