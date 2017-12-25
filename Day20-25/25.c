#include <stdio.h>
#include <string.h>

#define TAPE_SIZE 15000
#define NUM_STEPS 12994925

typedef struct {
    int new_val;
    int movement;
    char next_state;
} transition;


transition turing_machine[6][2] = {
    {{1, +1, 'b'}, {0, -1, 'f'}}, // a
    {{0, +1, 'c'}, {0, +1, 'd'}}, // b
    {{1, -1, 'd'}, {1, +1, 'e'}}, // c
    {{0, -1, 'e'}, {0, -1, 'd'}}, // d
    {{0, +1, 'a'}, {1, +1, 'c'}}, // e
    {{1, -1, 'a'}, {1, +1, 'a'}}, // f
};

int runTM(int steps) {
    int tape[TAPE_SIZE] = {0};
    int loc = TAPE_SIZE/2;
    char state = 'a';
    int num_ones = 0;

    for(int i = 0; i < steps; i++) {
        int val = tape[loc];
        transition t = turing_machine[state - 'a'][val];

        if(t.new_val == 1 && val == 0) {
            num_ones++;
        } else if(t.new_val == 0 && val == 1) {
            num_ones--;
        }

        tape[loc] = t.new_val;
        loc += t.movement;
        state = t.next_state;
    }
    return num_ones;
} 

int main(){
    printf("Part 1: %d\n", runTM(12994925));
    printf("Part 2: ⭐\n");
}