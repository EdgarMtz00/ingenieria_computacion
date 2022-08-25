#include <iostream>

using namespace std;

const int CANTIDAD_NUMEROS = 10;

int main()
{
    int vectorIngresado[CANTIDAD_NUMEROS];
    int vectorResultante[CANTIDAD_NUMEROS];
    int multiplicador;

    cout << "Ingrese el primer numero: ";
    cin >> vectorIngresado[0];

    cout << "Ingrese el segundo numero: ";
    cin >> vectorIngresado[1];

    cout << "Ingrese el tercer numero: ";
    cin >> vectorIngresado[2];

    cout << "Ingrese el cuarto numero: ";
    cin >> vectorIngresado[3];

    cout << "Ingrese el quinto numero: ";
    cin >> vectorIngresado[4];

    cout << "Ingrese el sexto numero: ";
    cin >> vectorIngresado[5];

    cout << "Ingrese el septimo numero: ";
    cin >> vectorIngresado[6];

    cout << "Ingrese el octavo numero: ";
    cin >> vectorIngresado[7];

    cout << "Ingrese el noveno numero: ";
    cin >> vectorIngresado[8];

    cout << "Ingrese el decimo numero: ";
    cin >> vectorIngresado[9];

    cout << "Ingrese el multiplicador: ";
    cin >> multiplicador;

    vectorResultante[0] = vectorIngresado[0] * multiplicador;
    vectorResultante[1] = vectorIngresado[1] * multiplicador;
    vectorResultante[2] = vectorIngresado[2] * multiplicador;
    vectorResultante[3] = vectorIngresado[3] * multiplicador;
    vectorResultante[4] = vectorIngresado[4] * multiplicador;
    vectorResultante[5] = vectorIngresado[5] * multiplicador;
    vectorResultante[6] = vectorIngresado[6] * multiplicador;
    vectorResultante[7] = vectorIngresado[7] * multiplicador;
    vectorResultante[8] = vectorIngresado[8] * multiplicador;
    vectorResultante[9] = vectorIngresado[9] * multiplicador;

    cout << "El primer resultado es: " << vectorResultante[0] << "\n";
    cout << "El segundo resultado es: " << vectorResultante[1] << "\n";
    cout << "El tercer resultado es: " << vectorResultante[2] << "\n";
    cout << "El cuarto resultado es: " << vectorResultante[3] << "\n";
    cout << "El quinto resultado es: " << vectorResultante[4] << "\n";
    cout << "El sexto resultado es: " << vectorResultante[5] << "\n";
    cout << "El septimo resultado es: " << vectorResultante[6] << "\n";
    cout << "El octavo resultado es: " << vectorResultante[7] << "\n";
    cout << "El noveno resultado es: " << vectorResultante[8] << "\n";
    cout << "El decimo resultado es: " << vectorResultante[9] << "\n";
    return 0;
    return 0;
}
