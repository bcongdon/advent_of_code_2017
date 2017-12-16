#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <pthread.h>


#define FACTOR_A 16807L
#define FACTOR_B 48271L
#define GENERATOR_MOD 0x7FFFFFFF
#define CHECK_MASK 0xFFFF
#define P1_ITERATIONS 40000000
#define P2_ITERATIONS 5000000

typedef struct {
    long seed_a;
    long seed_b;
} input;

long generate(long seed, long factor) {
    long prod = seed * factor;
    long tmp = (prod & GENERATOR_MOD) + (prod >> 31);
    if(tmp >> 31 == 0) {
        return tmp;
    } else {
        return tmp - GENERATOR_MOD;
    }
}

void *part1(void *arg) {
    input *in = arg;
    int judge = 0;
    long a = in->seed_a;
    long b = in->seed_b;

    for(int i = 0; i < P1_ITERATIONS; i++) {
        a = generate(a, FACTOR_A);
        b = generate(b, FACTOR_B);

        if((a & CHECK_MASK) == (b & CHECK_MASK)) {
            judge++;
        }
    }
    printf("Part 1: %d\n", judge);
    return NULL;
}

void *part2(void *arg) {
    input *in = arg;
    int judge = 0;
    long a = in->seed_a;
    long b = in->seed_b;

    for(int i = 0; i < P2_ITERATIONS; i++) {
        for(a = generate(a, FACTOR_A); a % 4 != 0;) {
            a = generate(a, FACTOR_A);
        }
        for(b = (b*FACTOR_B) % GENERATOR_MOD; b % 4 != 0;) {
            b = generate(b, FACTOR_B);
        }

        if((a & CHECK_MASK) == (b & CHECK_MASK)) {
            judge++;
        }
    }
    printf("Part 2: %d\n", judge);
    return NULL;
}

int main()
{
    input puzzle;
    puzzle.seed_a = 783;
    puzzle.seed_b = 325;

    pthread_t t1, t2;
    
    pthread_create(&t1, NULL, part1, (void*)&puzzle);
    pthread_create(&t2, NULL, part2, (void*)&puzzle);
    
    pthread_join(t1, NULL);
    pthread_join(t2, NULL);
}