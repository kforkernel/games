/*
Copyright (c) 2026 @kforkernel
All rights reserved.

Licensed under the kforkernel Use License (KUL) v1.0.

See LICENSE.md for terms and conditions.
*/

#include <stdio.h>
#include <stdlib.h>
#include <time.h>

int main(void){

    srand(time(NULL));
    int secretNumber = (rand() % 100) + 1;
    
    printf("Welcome to my guessing game!\n");

    // printf("The num is: %i\n", secretNumber);

    int counter = 0;

    while(1){
        if(counter >= 5){
            printf("You lost!\n");
            break;
        }

        int user_guess;
        printf("[%d/10] Enter guess: ", counter + 1); scanf("%d", &user_guess);

        if (user_guess < secretNumber){
            printf("Grater than\n");
            counter++;
        }
        else if (user_guess > secretNumber){
            printf("Less than:\n");
            counter++;
        }
        else{
            printf("Congratulations! You guessed correct!\n");
            break;
        }
    }
    return 0;
}
