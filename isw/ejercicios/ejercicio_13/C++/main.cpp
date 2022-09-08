#include <iostream>

using namespace std;

const int CANTIDAD_NUMEROS = 10;

int main()
{
    int vectorIngresado[CANTIDAD_NUMEROS];
    int vectorResultante[CANTIDAD_NUMEROS];
    int multiplicador;

    for (int i = 0; i < CANTIDAD_NUMEROS; i++){
        cout << "Ingrese el elemento #" << i + 1 <<": ";
        cin >> vectorIngresado[i];
    }

    cout << "Ingrese el multiplicador: ";
    cin >> multiplicador;

    for (int i = 0; i < CANTIDAD_NUMEROS; i++){
        vectorResultante[i] = vectorIngresado[i] * multiplicador;
        cout << "El resultado #" << i + 1 << " es: " << vectorResultante[i] << "\n";
    }

    return 0;
}
