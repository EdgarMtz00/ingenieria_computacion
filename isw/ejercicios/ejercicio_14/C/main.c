#include <stdio.h>
#include <stdlib.h>

struct Alumno {
    char nombre[15];
    int edad;
    float estatura;
};

int main()
{
    struct Alumno alumno;

    printf("Ingrese el nombre del alumno: ");
    scanf("%s", &alumno.nombre);

    printf("Ingrese la edad del alumno: ");
    scanf("%d", &alumno.edad);

    printf("Ingrese la estatura del alumno: ");
    scanf("%f", &alumno.estatura);

    printf("El nombre del alumno es: %s\n", alumno.nombre);
    printf("La edad del alumno es: %d\n", alumno.edad);
    printf("La estatura del alumno es: %f\n", alumno.estatura);
    return 0;
}
