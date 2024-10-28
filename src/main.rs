fn instructions() -> () {
    println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("::Bienvenido al simulador de combate pokemon profesional, aquí podra probar los daños que llegaria a hacer un pokemon profesional a otro           ::");
    println!("::Lo que debe hacer joven entrenador es: Elegir un pokemon de primera a septima generacion, el simulador te mostrara sus estadisisticas            ::");
    println!("::Despues tendras que elegir una habilidad que puede usar ese pokemon, despues te mostrara las estadisticas de un pokemon de categoria profesional ::");
    println!("::Acto seguido eligiras un pokemon para que reciba el ataque, el cual va a mostrar su estadistica de vida en modo profesional, tu pokemon          ::");
    println!("::Atacara al otro mostrando el daño que harian teoricamente 2 pokemones profesionales en una lucha real                                            ::");
    println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("::Reglas profecionales: Pokeones a nivel 50, IV=31 en cada estadistica, Ev=250 Total en estadisticas, dependiendo la temporada                     ::");
    println!("::Se permiten el uso de pokemones legendarios, pero si miticos y singularares.Disfrute el simulador                                                ::");
    println!(":::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
}


fn main() {
    instructions();
    let path_pokemon: &str = "./data/pokemon_data.csv";
    println!("This is the path of the pokemon data: {}", path_pokemon);
}
