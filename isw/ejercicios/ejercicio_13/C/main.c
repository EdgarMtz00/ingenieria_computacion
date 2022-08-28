#include <stdio.h>
#include <stdlib.h>

const int CANTIDAD_NUMEROS = 10;

int main()
{
    int vector[CANTIDAD_NUMEROS];
    int vectorResultante[CANTIDAD_NUMEROS];
    int multiplicador;

    printf("Ingrese el primer numero: ");
    scanf("%d", &vector[0]);

    printf("Ingrese el segundo numero: ");
    scanf("%d", &vector[1]);

    printf("Ingrese el tercer numero: ");
    scanf("%d", &vector[2]);

    printf("Ingrese el cuarto numero: ");
    scanf("%d", &vector[3]);

    printf("Ingrese el quinto numero: ");
    scanf("%d", &vector[4]);

    printf("Ingrese el sexto numero: ");
    scanf("%d", &vector[5]);

    printf("Ingrese el septimo numero: ");
    scanf("%d", &vector[6]);

    printf("Ingrese el octavo numero: ");
    scanf("%d", &vector[7]);

    printf("Ingrese el noveno numero: ");
    scanf("%d", &vector[8]);

    printf("Ingrese el decimo numero: ");
    scanf("%d", &vector[9]);

    printf("Ingrese el multiplicador: ");
    scanf("%d", &multiplicador);

    vectorResultante[0] = vector[0] * multiplicador;
    vectorResultante[1] = vector[1] * multiplicador;
    vectorResultante[2] = vector[2] * multiplicador;
    vectorResultante[3] = vector[3] * multiplicador;
    vectorResultante[4] = vector[4] * multiplicador;
    vectorResultante[5] = vector[5] * multiplicador;
    vectorResultante[6] = vector[6] * multiplicador;
    vectorResultante[7] = vector[7] * multiplicador;
    vectorResultante[8] = vector[8] * multiplicador;
    vectorResultante[9] = vector[9] * multiplicador;

    printf("El primer resultado es: %d\n", vectorResultante[0]);
    printf("El segundo resultado es: %d\n", vectorResultante[1]);
    printf("El tercer resultado es: %d\n", vectorResultante[2]);
    printf("El cuarto resultado es: %d\n", vectorResultante[3]);
    printf("El quinto resultado es: %d\n", vectorResultante[4]);
    printf("El sexto resultado es: %d\n", vectorResultante[5]);
    printf("El septimo resultado es: %d\n", vectorResultante[6]);
    printf("El octavo resultado es: %d\n", vectorResultante[7]);
    printf("El noveno resultado es: %d\n", vectorResultante[8]);
    printf("El decimo resultado es: %d\n", vectorResultante[9]);
    return 0;
}
