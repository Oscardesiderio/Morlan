use reqrnpdno::{extractora,parameters::Parametros,utilidades};

fn main () {

    // Aquí se define la ruta en donde se guardarán los datos. 
    // En este caso es un directorio llamado "30-nov-2021" que aún no existe pero que crearemos en el directorio "salida". 
    // Para crearlo usamos la función de ayuda "crear_directorio()" para la cual el primer parámetro es la ruta objetivo y el segundo parámetro es el nombre del directorio a crear. 
    let ruta_salida = utilidades::crear_directorio("./salida/", "30-nov-2021").unwrap();

    // Aquí creamos la estructura de los parámetros necesarios para realizar la petición. 
    // Esta estructura es la que se necesita modificar si quieres aplicar algún filtro. 
    // Si no quieres filtrar no es necesario modificar la estructura.
    let parametros = Parametros::new();

    // Para este caso no modificaremos la estructura con los parametros. 

    // Por último, utilizamos la función de alto nivel "extraer_todo_iterando" para obtener nuestros datos.
    extractora::extraer_todo_iterando(&parametros, &ruta_salida).unwrap();

}