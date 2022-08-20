#include <iostream>
#include "Lista.h"

enum {ALTA_INGRESO=1,BAJA_INGRESO,LISTADO_INGRESOS,BUSQUEDA_INGRESO,LISTADO_INGRESOS_INVERSAMENTE,
    BORRAR_INGRESOS_DUPLICADOS,SALIR};

#ifdef _WIN32
#define CLEAR "cls"
#elif defined(unix)||defined(__unix__)||defined(__unix)||defined(__APPLE__)||defined(__MACH__)
#define CLEAR "clear"
#else
#error "SO no soportado para limpiar pantalla"
#endif

using namespace std;

Lista listaIngresos;

int elegirOpcionMenu();
void registrarIngreso();
void eliminarIngreso();
void mostrarIngresos();
void mostrarIngresosInversamente();
void listarIngresos();
void buscarIngreso();
void borrarIngresosDuplicados();

int main()
{
    bool continuaPrograma=true;
    do{
        switch(elegirOpcionMenu()){
        case ALTA_INGRESO:
            registrarIngreso();
            break;
        case BAJA_INGRESO:
            eliminarIngreso();
            break;
        case LISTADO_INGRESOS:
            mostrarIngresos();
            break;
        case BUSQUEDA_INGRESO:
            buscarIngreso();
            break;
        case LISTADO_INGRESOS_INVERSAMENTE:
            mostrarIngresosInversamente();
            break;
        case BORRAR_INGRESOS_DUPLICADOS:
            borrarIngresosDuplicados();
            break;
        case SALIR:
            continuaPrograma=false;
            cout << "bytes!" << endl;
            break;
        default:
            cout << "Opcion no valida" << endl;
        }
        if (continuaPrograma){
            cout << "Presiona entrar para continuar...";
            cin.get();
        }
    }
    while(continuaPrograma);
    return 0;
}

int elegirOpcionMenu(){
    int opcionMenu;
    system(CLEAR);
    cout << "Control de Ingresos!" << endl << endl;
    cout << "Elige una de las siguiente opciones:" << endl;
    cout << "1. Alta de ingreso" << endl;
    cout << "2. Baja de ingreso" << endl;
    cout << "3. Listado de ingresos" << endl;
    cout << "4. Busqueda de ingreso" << endl;
    cout << "5. Listado de ingresos inversamente" << endl;
    cout << "6. Borrar ingresos duplicados" << endl;
    cout << "7. Salir" << endl;
    cout << "Opcion: ";
    cin >> opcionMenu;
    cin.get();
    return opcionMenu;
}

void registrarIngreso(){
    Ingreso ingreso;
    system(CLEAR);
    cout << "Alta de Ingreso" << endl << endl;
    if (!listaIngresos.estaLlena()){
        cout << "Dame el monto del ingreso a guardar: ";
        cin >> ingreso.monto;
        cout << "Dime el mes en que se obtuvo: ";
        cin >> ingreso.mes;
        cin.ignore();
        listaIngresos.inserta(ingreso,listaIngresos.fin());
    }
    else{
        cout << "No hay mas espacio para captura de ingresos" << endl;
    }
}

void eliminarIngreso(){
    int numeroElementoRemover,i;
    posicion p,q;
    system(CLEAR);
    cout << "Baja de Ingreso" << endl << endl;
    if (!listaIngresos.estaVacia()){
        listarIngresos();
        cout << "Dime cual ingreso deseas remover (1 a " << listaIngresos.dameCuenta() << "): ";
        cin >> numeroElementoRemover;
        cin.get();
        for(i=1,p=listaIngresos.primero(),q=listaIngresos.fin();i<numeroElementoRemover && p!=q;
            i++,p=listaIngresos.siguiente(p));
        if (p!=q && numeroElementoRemover>0){
            listaIngresos.suprime(p);
            cout << "Registro de ingreso dado de baja" << endl;
        }
        else{
            cout << numeroElementoRemover << " esta fuera del rango solicitado" << endl;
        }
    }
    else{
        cout << "No hay ingresos que eliminar" << endl;
    }
}

void imprimeIngreso(int numero,Ingreso& ingreso){
    cout << numero << ". " << ingreso.monto << " en " << ingreso.mes << endl;
}

void listarIngresos(){
    posicion p,q;
    Ingreso ingreso;
    int i;
    for(i=1,p=listaIngresos.primero(),q=listaIngresos.fin();p!=q;p=listaIngresos.siguiente(p),i++){
        ingreso=listaIngresos.recupera(p);
        imprimeIngreso(i,ingreso);
    }
}

void mostrarIngresos(){
    system(CLEAR);
    cout << "Listado de Ingresos" << endl << endl;
    if (!listaIngresos.estaVacia()){
        listarIngresos();
    }
    else{
        cout << "No hay ingresos que mostrar" << endl;
    }
}

void mostrarIngresosInversamente(){
    posicion p,q;
    Ingreso ingreso;
    int i;
    system(CLEAR);
    cout << "Listado de Ingresos Inversamente" << endl << endl;
    if (!listaIngresos.estaVacia()){
        p=listaIngresos.fin();
        q=listaIngresos.primero();
        i=listaIngresos.dameCuenta();
        do{
            p=listaIngresos.anterior(p);
            ingreso=listaIngresos.recupera(p);
            imprimeIngreso(i,ingreso);
            i--;
        }
        while(p!=q);
    }
    else{
        cout << "No hay ingresos que mostrar" << endl;
    }
}

void buscarIngreso(){
    posicion p;
    Ingreso ingreso;
    system(CLEAR);
    cout << "Busqueda de un Ingreso" << endl << endl;
    if (!listaIngresos.estaVacia()){
        cout << "Dame que monto del ingreso buscar: ";
        cin >> ingreso.monto;
        cout << "Dime en que mes fue: ";
        cin >> ingreso.mes;
        cin.get();
        p=listaIngresos.localiza(ingreso);
        if (p!=listaIngresos.fin()){
            cout << "El ingreso si se encuentra" << endl;
        }
        else{
            cout << "El ingreso buscado no se encuentra en la lista de ingresos" << endl;
        }
    }
    else{
        cout << "No hay ingresos capturados para buscar" << endl;
    }
}

bool Lista::mismo(tipo_elemento x,tipo_elemento y){
    //ahora "x" e "y" son el mismo solo si tambien es del mismo mes
    return x.monto>=y.monto && x.monto<=y.monto && x.mes==y.mes;
}

void purgaLista(Lista& lista){
    posicion p;
    //indispensable usar la funcion Lista::mismo() para determinar si dos registros son iguales
}

void borrarIngresosDuplicados(){
    system(CLEAR);
    cout << "Borrar Ingresos Duplicados" << endl << endl;
    cout << "BORRANDO...";
    purgaLista(listaIngresos);
    cout << "Hecho =)" << endl;
}
