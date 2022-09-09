import java.util.Scanner;
public class Main{
    static int CANTIDAD_NUMEROS = 10;
    static Scanner scanner = new Scanner(System.in);

	public static void main(String args[]){
        int[] vectorIngresado = new int[CANTIDAD_NUMEROS];
        int[] vectorResultante = new int[CANTIDAD_NUMEROS];
        int multiplicador;

        for (int i = 0; i < CANTIDAD_NUMEROS; i++){
            System.out.printf("Ingrese el elemento #%d: ", i+ 1);
            vectorIngresado[i] = scanner.nextInt();
        }

        System.out.printf("Ingrese el multiplicador: ");
        multiplicador = scanner.nextInt();

        for (int i = 0; i < CANTIDAD_NUMEROS; i++){
            vectorResultante[i] = vectorIngresado[i] * multiplicador;
            System.out.printf("El resultado #%d es: %d\n", i + 1, vectorResultante[i]);
        }
	}
}
