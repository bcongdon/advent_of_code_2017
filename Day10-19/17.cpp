#include <vector>
#include <iostream>

#define PUZZLE_INPUT 354

int runSpinlock(int offset, int iterations) {
    std::vector<int> spinlock;
    spinlock.push_back(0);
    int loc = 0;

    for(int i = 1; i <= iterations; i++) {
        loc = (loc+offset)%i + 1;
        std::vector<int>::iterator it = spinlock.begin();
        spinlock.insert(it+loc, i);
    }
    return spinlock[loc+1];
}

int simSpinlock(int offset, int iterations) {
    int loc = 0;
    uint target_val = 0;

    for(int i = 1; i <= iterations; i++) {
        loc = (loc+offset)%i + 1;
        if(loc == 1) {
            target_val = i;
        }
    }
    return target_val;
}

int main(){
    int p1 = runSpinlock(PUZZLE_INPUT, 2017);
    printf("Part 1: %d\n", p1);

    int p2 = simSpinlock(PUZZLE_INPUT, 50000000);
    printf("Part 2: %d\n", p2);
}