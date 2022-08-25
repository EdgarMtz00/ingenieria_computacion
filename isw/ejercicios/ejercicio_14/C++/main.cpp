#include <iostream>

using namespace std;

struct Alumno {
    char nombre[15];
    int edad;
    float estatura;
};

int main()
{
    struct Alumno alumno;

    cout << "Ingrese el nombre del alumno: ";
    cin >> alumno.nombre;

    cout << "Ingrese la edad del alumno: ";
    cin >> alumno.edad;

    cout << "Ingrese la estatura del alumno: ";
    cin >> alumno.estatura;

    cout << "El nombre del alumno es: " << alumno.nombre << "\n";
    cout << "La edad del alumno es: " << alumno.edad << "\n";
    cout << "La estatura del alumno es: " << alumno.estatura << "\n";

    return 0;
}
