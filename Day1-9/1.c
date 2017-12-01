#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct result {
    int part1;
    int part2;
} result;

char *getInput(char *fname) {
    FILE *fp = fopen(fname, "r");
    char *input = calloc(4096, sizeof(char));
    fgets(input, 4096, fp);
    return input;
}

result captcha(char *input) {
    int part1 = 0;
    int part2 = 0;
    int n = strlen(input) - 1;
    
    for(int i = 0; i < n; i++) {
        if(input[i] == input[(i+1)%n]) {
            part1 += input[i] - '0';
        }
        if(input[i] == input[(i + n/2)%n]) {
            part2 += input[i] - '0';
        }
    }

    struct result res;
    res.part1 = part1;
    res.part2 = part2;
    return res;
}

int main() {
    char *input = getInput("1.txt");
    result res = captcha(input);
    printf("Part 1: %d\n", res.part1);
    printf("Part 2: %d\n", res.part2);
}