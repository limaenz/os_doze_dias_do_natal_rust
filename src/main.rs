fn main() {
    let dias_natal = ["primeiro", "segundo", "terceiro", "quarto", "quinto",
        "sexto", "sétimo", "oitavo", "nono", "décimo", "décimo primeiro", "décimo segundo"];

    let mut contador: i32 = 1;

    for elemento in dias_natal.iter() {
        println!("No {} dia de Natal", elemento);
        println!("meu verdadeiro amor envio a mim");

        if contador == 1 {
            println!("A perdiz em uma árvore de pêra");
            println!("");
            contador = contador + 1;
            continue;            
        } 
 
        if contador == 12 {
            println!("Doze bateristas tambores");
        }

        if contador >= 11 {
            println!("onze gaiteiros que conduzem");
        }

        if contador >= 10 {
            println!("dez senhores a-salto");
        }

        if contador >= 9 {
            println!("nove senhoras que dançam");
        }

        if contador >= 8 {
            println!("Oito empregadas domésticas uma ordenha");
        }

        if contador >= 7 {
            println!("Sete cisnes uma natação");
        }

        if contador >= 6 {
            println!("Seis gansos-de postura");
        }

        if contador >= 5 {
            println!("Cinco anéis de ouro");
        }

        if contador >= 4 {
            println!("Quatro pássaros chamando");
        }

        if contador >= 3 {
            println!("Três galinhas francesas");
        }

        if contador >= 4 {
            println!("Quatro pássaros chamando");
        }

        println!("Duas rolas");
        println!("E uma perdiz em uma pereira");
            
        //Imprimir letra especifica
        //Imprimir o que já está na lista
        //Armazenar a letra especifica na lista
        println!("");
        contador = contador + 1;
    }
}
