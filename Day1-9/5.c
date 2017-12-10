#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_LINES 2048

int part1(int *jumps, int len) {
    int c = 0;
    int steps = 0;
    while(c >= 0 && c < len) {
        int j = jumps[c];
        jumps[c]++;
        steps++;
        c += j;
    }
    return steps;
}

int part2(int *jumps, int len) {
    int c = 0;
    int steps = 0;
    while(c >= 0 && c < len) {
        int j = jumps[c];
        if(j >= 3) {
            jumps[c]--;
        }
        else {
            jumps[c]++;
        }
        steps++;
        c += j;
    }
    return steps;
}

int getInput(char *fname, int** arrOut) {
    FILE *fp = fopen(fname, "r");
    char *line = NULL;
    size_t len = 0;
    int *arr = malloc(sizeof(int) * MAX_LINES);
    int i = 0;
    while (getline(&line, &len, fp) != -1) {
       arr[i++] = atoi(line);
    }
    free(line);
    *arrOut = arr;
    return i;
}

int main() {
    int *input = NULL;
    int len = getInput("5.txt", &input);

    int *p1 = malloc(sizeof(int) * len);
    memcpy(p1, input, sizeof(int) * len);
    int p1_result = part1(p1, len);
    printf("Part 1: %d\n", p1_result);
    
    free(p1);
    p1 = NULL;

    int *p2 = malloc(sizeof(int) * len);
    memcpy(p2, input, sizeof(int) * len);
    int p2_result = part2(p2, len);
    printf("Part 2: %d\n", p2_result);
    
    free(p2);
    p2 = NULL;

    free(input);
    input = NULL;
}