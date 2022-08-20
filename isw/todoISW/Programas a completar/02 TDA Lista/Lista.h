#ifndef LISTA_H_INCLUDED
#define LISTA_H_INCLUDED
#include "Tipos_y_constantes.h"

typedef int posicion;

class Lista{
    tipo_elemento elementos[MAX_ELEMENTOS];
    int ultimo;
public:
    Lista();
    posicion primero();
    posicion fin();
    posicion siguiente(posicion p);
    posicion anterior(posicion p);
    posicion localiza(tipo_elemento x);
    void inserta(tipo_elemento x,posicion p);
    void suprime(posicion p);
    bool estaLlena();
    bool estaVacia();
    int dameCuenta();
    tipo_elemento recupera(posicion p);
    bool mismo(tipo_elemento x,tipo_elemento y);
};


Lista::Lista(){
    ultimo=-1;
}

posicion Lista::primero(){
    if (estaVacia()){
        return ultimo+1;
    }
    else{
        return 0;
    }
}

posicion Lista::fin(){
    return ultimo+1;
}

posicion Lista::siguiente(posicion p){
    if (p>=0 && p<=ultimo){
        return p+1;
    }
    else{
        return ELEMENTO_NO_EXISTE;
    }
}

posicion Lista::anterior(posicion p){
    if (p>0 && p<=ultimo+1){
        return p-1;
    }
    else{
        return ELEMENTO_NO_EXISTE;
    }
}

posicion Lista::localiza(tipo_elemento x){
    posicion p,q;
    for(p=primero(),q=fin();p!=q;p=siguiente(p)){
        if (mismo(x,elementos[p])){
            break;
        }
    }
    return p;
}

void Lista::inserta(tipo_elemento x,posicion p){
    int i;
    if (p>=0 && p<=ultimo+1){
        for(i=ultimo+1;i>p;i--){
            elementos[i]=elementos[i-1];
        }
        elementos[p]=x;
        ultimo++;
    }
}
void Lista::suprime(posicion p){
    int i;
    if (p>=0 && p<=ultimo){
        for(i=p;i<ultimo;i++){
            elementos[i]=elementos[i+1];
        }
        ultimo--;
    }
    //else no suprimo posicion no valida
}
bool Lista::estaLlena(){
    return ultimo==MAX_ELEMENTOS-1;
}
bool Lista::estaVacia(){
    return ultimo==-1;
}
int Lista::dameCuenta(){
    return ultimo+1;
}
tipo_elemento Lista::recupera(posicion p){
    tipo_elemento elementoErroneo;
    if (p>=0 && p<=ultimo){
        return elementos[p];
    }
    else{
        return elementoErroneo;
    }
}


#endif // LISTA_H_INCLUDED
