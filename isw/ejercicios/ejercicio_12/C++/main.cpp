#include <iostream>

using namespace std;

const int MAXIMO_ASISTENCIAS = 34;

int main()
{
    int numFaltas = 0;
    cout << "Ingrese la cantidad de faltas del alumno: ";
    cin >> numFaltas;

    float porcentajeAsistencia = 100.0f - (numFaltas * 100.0f / MAXIMO_ASISTENCIAS);
    cout << "Porcentaje de asistencia = " << porcentajeAsistencia << "%\n";
    return 0;
}
