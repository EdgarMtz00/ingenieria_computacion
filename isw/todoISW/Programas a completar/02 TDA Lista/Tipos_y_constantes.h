#ifndef TIPOS_Y_CONSTANTES_H_INCLUDED
#define TIPOS_Y_CONSTANTES_H_INCLUDED

#include <iostream>

using namespace std;

struct Ingreso{
    float monto;
    string mes;
};

typedef Ingreso tipo_elemento;
#define MAX_ELEMENTOS 3
#define ELEMENTO_NO_EXISTE -1

#endif // TIPOS_Y_CONSTANTES_H_INCLUDED
