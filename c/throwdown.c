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

    while(1){
        
        int secretNumber = (rand() % 3) + 1;

        int user_input;

        printf("Rock(1) Paper(2) Scissors(3)\n"); scanf("%d", &user_input);

        if (user_input >= 4 || user_input <= 0){
            printf("Invalid input!\n");
            continue;
        }
        if(user_input == 1 && secretNumber == 1 || user_input == 2 && secretNumber == 2 || user_input == 3 && secretNumber == 3){
            printf("Draw!:\n");
        }
        else if(user_input == 1 && secretNumber == 3){
            printf("You won!\n");
        }
        else if(user_input == 2 && secretNumber == 1){
            printf("You won!\n");
        }
        else if(user_input == 3 && secretNumber == 2){
            printf("You won!\n");
        }
        else{
            printf("You lost!\n");
        }
    }

    return 0;
}
