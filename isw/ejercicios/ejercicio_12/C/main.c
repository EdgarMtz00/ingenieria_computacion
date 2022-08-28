/*
* Edgar
* Ejercicio 12
* Tiempo = 00:16
*/

#include <stdio.h>
#include <stdlib.h>

const int MAXIMO_ASISTENCIAS = 34;

int main()
{
    int numFaltas = 0;
    printf("Ingrese la cantidad de faltas del alumno\n");
    scanf("%d", &numFaltas);

    float porcentajeAsistencia = 100.0f - (numFaltas * 100.0f / MAXIMO_ASISTENCIAS);
    printf("Porcentaje de asistencia = %f\%\n", porcentajeAsistencia);
    return 0;
}
