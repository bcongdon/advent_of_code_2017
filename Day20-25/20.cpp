#include <string>
#include <math.h>
#include <iostream>
#include <boost/regex.hpp>
#include <boost/format.hpp>
#include <fstream>
#include <unordered_map>

#define SIMULATED_ROUNDS 1000
#define COLLISION_ROUNDS 40

using namespace std;

class Particle {
private:
    long x, y, z;
    long vx, vy, vz;
    long ax, ay, az;
public:
    bool annihilated;

    Particle(string line) {
        boost::regex re("[p,v,a,=,<,>,\\s]+");
        boost::sregex_token_iterator
            p(line.begin(), line.end(), re, -1);
        boost::sregex_token_iterator end;

        vector<int> values;
        while(p != end) {
            if(p->str().length() == 0) {
                p++;
                continue;
            }

            values.push_back(atoi((p++)->str().c_str()));
        }

        x = values[0];
        y = values[1];
        z = values[2];
        vx = values[3];
        vy = values[4];
        vz = values[5];
        ax = values[6];
        ay = values[7];
        az = values[8];
        annihilated = false;
    }

    void move() {
        vx += ax;
        vy += ay;
        vz += az;
        x += vx;
        y += vy;
        z += vz;
    }

    float distance() {
        float dist = pow(x, 2);
        dist += pow(y, 2);
        dist += pow(z, 2);
        return dist;
    }

    void simulateMovement(float time) {
        x += int(vx*time) + int(0.5*ax*time*time);
        y += int(vy*time) + int(0.5*ay*time*time);
        z += int(vz*time) + int(0.5*az*time*time);
    }

    string location() {
        return boost::str(boost::format("%d,%d,%d") % x % y % z);
    }

    friend ostream& operator<<(ostream &os, const Particle& p);
};

std::ostream& operator<<(std::ostream &strm, const Particle &p) {
    return strm << boost::format("<%d,%d,%d; %d,%d,%d; %d,%d,%d>") % p.x % p.y % p.z % p.vx % p.vy % p.vz % p.ax % p.ay % p.az;
}

int closestParticle(vector<Particle> particles) {
    vector<float> distances;
    for(int i = 0; i < particles.size(); i++) {
        particles[i].simulateMovement(SIMULATED_ROUNDS);
        distances.push_back(particles[i].distance());
    }

    int minParticle = 0;
    for(int i = 0; i < distances.size(); i++) {
        if(distances[i] < distances[minParticle])
            minParticle = i;
    }
    return minParticle;
}

int numUncollidedParticles(vector<Particle> particles) {
    for(int i = 0; i < COLLISION_ROUNDS; i++) {
        unordered_map<string, int> locations;

        for(int j = 0; j < particles.size(); j++) {
            if(particles[j].annihilated)
                continue;
            
            particles[j].move();
            
            string loc = particles[j].location();
            if(locations.find(loc) != locations.end()) {
                particles[j].annihilated = true;
                particles[locations[loc]].annihilated = true;
            } else {
                locations[loc] = j;
            }
        }
    }
    int nonAnnihilated = 0;
    for(int i = 0; i < particles.size(); i++) {
        if(!particles[i].annihilated) nonAnnihilated++;
    }
    return nonAnnihilated;
}

int main() {
    vector<Particle> particles;

    ifstream infile("20.txt");
    string line;
    while (getline(infile, line)) {
        particles.push_back(Particle(line));
    }
    cout << "Part 1: " << closestParticle(particles) << "\n";
    cout << "Part 2: " << numUncollidedParticles(particles) << "\n";
}
