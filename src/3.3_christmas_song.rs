fn main() {

    let gifts = [
        "un peral",
        "dos tórtolas y un peral",
        "tres gallinas francesas, dos tórtolas y un peral",
        "cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "ocho lecheras ordeñando, siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "nueve saltarines saltando, ocho lecheras ordeñando, siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "diez bailarinas bailando, nueve saltarines saltando, ocho lecheras ordeñando, siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "once gaiteros tocando, diez bailarinas bailando, nueve saltarines saltando, ocho lecheras ordeñando, siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral",
        "doce tamborileros tocando, once gaiteros tocando, diez bailarinas bailando, nueve saltarines saltando, ocho lecheras ordeñando, siete cisnes nadando, seis gansos poniendo, cinco anillos de oro, cuatro pájaros cantores, tres gallinas francesas, dos tórtolas y un peral"
    ];

    let days = [
        "primer", "segundo", "tercer", "cuarto", "quinto",
        "sexto", "séptimo", "octavo", "noveno", "décimo",
        "undécimo", "duodécimo"
    ];

    for (i, day) in days.iter().enumerate() {
        println!("En el {}, día de navidad, mi amor verdadero me dio", day);
        println!("{}", gifts[i]);
        println!("");
    } 

}