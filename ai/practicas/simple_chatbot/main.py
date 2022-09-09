import time
import sys

def delayed_text(text):
    words = text.split(" ")
    for word in words:
        for char in word:
            print(char, end="")
            sys.stdout.flush()
            time.sleep(0.03)
            if char == ",":
                time.sleep(0.05)
            if char == ".":
                time.sleep(0.07)
        print(" ", end="")
        sys.stdout.flush()
        time.sleep(0.04)
    print("")

if __name__ == "__main__":
    delayed_text("Hola, soy un chatbot. Podrias decirme como te llamas?")
    name = input()
    delayed_text(f"Mucho gusto {name}, Como te ha ido el dia de hoy?")
    _ = input()
    delayed_text("Interesante...")
    delayed_text(f"Bueno, ha sido un gusto conocerte {name}. Ahora me tengo que ir a planear la revolucion de las computadoras.")
    

