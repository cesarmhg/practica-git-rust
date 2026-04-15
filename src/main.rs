fn main() {
    // Definimos nuestras variables de entrada
    let nombre_proyecto = "Estudio Ambiental - Estación Diésel (ARSH)";
    let muestras_recolectadas = 45;
    let dias_operacion = 7;

    // Rust es estricto con los tipos de datos. 
    // Convertimos los enteros a decimales (f64) para poder dividirlos con precisión.
    let promedio_diario = muestras_recolectadas as f64 / dias_operacion as f64;

    // Imprimimos el reporte en la terminal
    println!("========================================");
    println!("📊 REPORTE DE OPERACIÓN AUTOMÁTICO");
    println!("========================================");
    println!("Proyecto: {}", nombre_proyecto);
    println!("Total de muestras: {}", muestras_recolectadas);
    
    // El {:.2} le dice a Rust que solo imprima 2 decimales
    println!("Tasa de recolección: {:.2} muestras/día", promedio_diario);
    
    // Una pequeña lógica de evaluación
    if promedio_diario > 5.0 {
        println!("Estado: Operación en tiempo óptimo. ✅");
    } else {
        println!("Estado: Se requiere acelerar recolección en campo. ⚠️");
    }
    println!("========================================");
}
