use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug)]
pub struct Nodo<T> {
    ciudad: T,
    hojas: Vec<Rc<RefCell<Nodo<T>>>>,
}

impl<String: std::fmt::Debug + for<'a> std::cmp::PartialEq<&'a str>> Nodo<String> {
    pub fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Nodo {
            ciudad: value,
            hojas: Vec::new(),
        }))
    }

    pub fn anadir_hoja(parent: Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        parent.borrow_mut().hojas.push(child);
    }

    pub fn profundidad_todos(
        nodo_origen: &Rc<RefCell<Self>>,
        depth: usize,
        visitados: &mut HashSet<usize>,
        ciudad_destino: &str,
    ) {
        let node_ptr = Rc::as_ptr(nodo_origen) as usize;
        if visitados.contains(&node_ptr) {
            return;
        }
        visitados.insert(node_ptr);

        if nodo_origen.borrow().ciudad.eq(&ciudad_destino) {
            println!(
                "{:indent$}{:?} <---**",
                "",
                nodo_origen.borrow().ciudad,
                indent = depth * 2
            );
        }

        println!(
            "{:indent$}{:?}",
            "",
            nodo_origen.borrow().ciudad,
            indent = depth * 2
        );

        for child in &nodo_origen.borrow().hojas {
            Nodo::profundidad_todos(child, depth + 1, visitados, ciudad_destino);
        }
    }

    pub fn busqueda_profundidad(
        nodo: &Rc<RefCell<Self>>,       // nodo actual en la función
        espacio: usize,                 // espacios para la indentación
        visitados: &mut HashSet<usize>, //hash set de las ciudades que ya fueron visitadas para evitar ciclos infinitos
        ciudad_destino: &str,           // ciudad que se quiere encontrar
    ) -> Option<Rc<RefCell<Self>>> {
        let nodo_ptr = Rc::as_ptr(nodo) as usize; // apuntador al nodo actual
        if visitados.contains(&nodo_ptr) {
            return None; // si está en el hashset regresa none
        }
        visitados.insert(nodo_ptr); // si no, continúa e ingresa el nodo actual a los ya visitados

        if nodo.borrow().ciudad.eq(&ciudad_destino) { // verificar si la ciudad en el nodo actual es la meta
            println!(
                "{:indent$}{:?} <--*", // formato del string
                "",
                nodo.borrow().ciudad, // nombre de la ciudad
                indent = espacio * 2  // espacios para indentar
            );
            return Some(nodo.clone()); // si encontrado regresa el nodo entero
        }

        println!( // imprimir ciudad aunque no tenga coincidencia
            "{:indent$}{:?}",
            "",
            nodo.borrow().ciudad,
            indent = espacio * 2
        );

        // iterar en todos los nodos hijo del nodo actual y aplicar función
        for child in &nodo.borrow().hojas {
            // asignar el valor del nodo si regresa un valor del mismo tipo (encuentra la ciudad)
            if let Some(nodo_destino) =
                Nodo::busqueda_profundidad(child, espacio + 1, visitados, ciudad_destino)
            {
                return Some(nodo_destino);
            }
        }

        None
    }

    pub fn busqueda_profundidad_lim(
        nodo: &Rc<RefCell<Self>>,       // nodo actual en la función
        espacio: usize,
        profundidad: i32,
        visitados: &mut HashSet<usize>, //hash set de las ciudades que ya fueron visitadas para evitar ciclos infinitos
        ciudad_destino: &str,           // ciudad que se quiere encontrar
        limite: i32
    ) -> Option<Rc<RefCell<Self>>> {
        let nodo_ptr = Rc::as_ptr(nodo) as usize; // apuntador al nodo actual
        if visitados.contains(&nodo_ptr) {
            return None; // si está en el hashset regresa none
        }
        visitados.insert(nodo_ptr); // si no, continúa e ingresa el nodo actual a los ya visitados

        if nodo.borrow().ciudad.eq(&ciudad_destino) {
            println!(
                "{:indent$}{:?} <--*", // formato del string
                "",
                nodo.borrow().ciudad, // nombre de la ciudad
                indent = espacio * 2  // espacios para indentar
            );
            return Some(nodo.clone()); // si encontrado regresa el nodo entero
        }

        println!( // imprimir ciudad aunque no tenga coincidencia
            "{:indent$}{:?}",
            "",
            nodo.borrow().ciudad,
            indent = espacio * 2
        );

        // iterar en todos los nodos hijo del nodo actual y aplicar función
        for child in &nodo.borrow().hojas {
            // asignar el valor del nodo si regresa un valor del mismo tipo (encuentra la ciudad)
            if let Some(nodo_destino) =
            Nodo::busqueda_profundidad_lim(child, espacio + 1, profundidad + 1, visitados, ciudad_destino, limite)
            {
                return Some(nodo_destino);
            }
            if profundidad >= limite { // comprobar limite de la busqueda, si es mayor o igual regresar
                return None;
            }
        }

        None
    }
}

fn main() {
    //ciudades
    let oradea = Nodo::new("oradea");
    let zerind = Nodo::new("zerind");
    let arad = Nodo::new("arad");
    let sibiu = Nodo::new("sibiu");
    let timisoara = Nodo::new("timisoara");
    let lugoj = Nodo::new("lugoj");
    let mehadia = Nodo::new("mehadia");
    let dobreta = Nodo::new("dobreta");
    let craiova = Nodo::new("craiova");
    let rimnicu_vilcea = Nodo::new("rimnicu_vilcea");
    let pitesti = Nodo::new("pitesti");
    let bucarest = Nodo::new("bucarest");
    let giurgiu = Nodo::new("giurgiu");
    let fagaras = Nodo::new("fagaras");
    let urziceni = Nodo::new("urziceni");
    let vaslui = Nodo::new("vaslui");
    let iasi = Nodo::new("iasi");
    let neamt = Nodo::new("neamt");
    let hirsova = Nodo::new("hirsova");
    let eforie = Nodo::new("eforie");

    //rutas ciudad-ciudad
    Nodo::anadir_hoja(oradea.clone(), zerind.clone());
    Nodo::anadir_hoja(oradea.clone(), sibiu.clone());

    Nodo::anadir_hoja(zerind.clone(), oradea.clone());
    Nodo::anadir_hoja(zerind.clone(), arad.clone());

    Nodo::anadir_hoja(arad.clone(), zerind.clone());
    Nodo::anadir_hoja(arad.clone(), sibiu.clone());
    Nodo::anadir_hoja(arad.clone(), timisoara.clone());

    Nodo::anadir_hoja(sibiu.clone(), oradea.clone());
    Nodo::anadir_hoja(sibiu.clone(), arad.clone());
    Nodo::anadir_hoja(sibiu.clone(), fagaras.clone());
    Nodo::anadir_hoja(sibiu.clone(), rimnicu_vilcea.clone());

    Nodo::anadir_hoja(timisoara.clone(), arad.clone());
    Nodo::anadir_hoja(timisoara.clone(), lugoj.clone());

    Nodo::anadir_hoja(lugoj.clone(), mehadia.clone());
    Nodo::anadir_hoja(lugoj.clone(), timisoara.clone());

    Nodo::anadir_hoja(mehadia.clone(), lugoj.clone());
    Nodo::anadir_hoja(mehadia.clone(), dobreta.clone());

    Nodo::anadir_hoja(dobreta.clone(), mehadia.clone());
    Nodo::anadir_hoja(dobreta.clone(), craiova.clone());

    Nodo::anadir_hoja(craiova.clone(), dobreta.clone());
    Nodo::anadir_hoja(craiova.clone(), rimnicu_vilcea.clone());
    Nodo::anadir_hoja(craiova.clone(), pitesti.clone());

    Nodo::anadir_hoja(rimnicu_vilcea.clone(), sibiu.clone());
    Nodo::anadir_hoja(rimnicu_vilcea.clone(), pitesti.clone());
    Nodo::anadir_hoja(rimnicu_vilcea.clone(), craiova.clone());

    Nodo::anadir_hoja(fagaras.clone(), sibiu.clone());
    Nodo::anadir_hoja(fagaras.clone(), bucarest.clone());

    Nodo::anadir_hoja(pitesti.clone(), rimnicu_vilcea.clone());
    Nodo::anadir_hoja(pitesti.clone(), bucarest.clone());
    Nodo::anadir_hoja(pitesti.clone(), craiova.clone());

    Nodo::anadir_hoja(bucarest.clone(), fagaras.clone());
    Nodo::anadir_hoja(bucarest.clone(), pitesti.clone());
    Nodo::anadir_hoja(bucarest.clone(), giurgiu.clone());
    Nodo::anadir_hoja(bucarest.clone(), urziceni.clone());

    Nodo::anadir_hoja(giurgiu.clone(), bucarest.clone());

    Nodo::anadir_hoja(urziceni.clone(), bucarest.clone());
    Nodo::anadir_hoja(urziceni.clone(), hirsova.clone());
    Nodo::anadir_hoja(urziceni.clone(), vaslui.clone());

    Nodo::anadir_hoja(hirsova.clone(), urziceni.clone());
    Nodo::anadir_hoja(hirsova.clone(), eforie.clone());

    Nodo::anadir_hoja(eforie.clone(), hirsova.clone());

    Nodo::anadir_hoja(vaslui.clone(), urziceni.clone());
    Nodo::anadir_hoja(vaslui.clone(), iasi.clone());

    Nodo::anadir_hoja(iasi.clone(), neamt.clone());
    Nodo::anadir_hoja(iasi.clone(), vaslui.clone());

    Nodo::anadir_hoja(neamt.clone(), iasi.clone());

    let mut visitados = HashSet::new();
    let res = Nodo::busqueda_profundidad(&oradea, 0, &mut visitados, "iasi");
    if res.is_some() {
        println!("Encontrado");
    } else {
        println!("No encontrado");
    }

    // let reslim = Nodo::busqueda_profundidad_lim(&oradea, 0, 0, &mut visitados, "iasi", 8);
    // if reslim.is_some() {
    //     println!("Encontrado");
    // } else {
    //     println!("No encontrado");
    // }
}
