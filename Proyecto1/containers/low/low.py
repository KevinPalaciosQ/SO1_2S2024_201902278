import time

def show_time():
    while True:
        current_time = time.strftime("%H:%M:%S")
        print(f"Current Time: {current_time}", end='\r')
        time.sleep(1)  # Actualiza cada segundo

if __name__ == '__main__':
    show_time()
