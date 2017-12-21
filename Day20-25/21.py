import numpy as np
start_config = np.array([list(i) for i in ['.#.', '..#', '###']])


def to_cfg_string(chunk):
    return '/'.join([''.join(r) for r in chunk])


def from_cfg_string(cfg_str):
    return np.array([list(i) for i in cfg_str.split('/')])


def find_new_config(chunk, rules):
    for _ in range(4):
        if to_cfg_string(chunk) in rules:
            return from_cfg_string(rules[to_cfg_string(chunk)])
        if to_cfg_string(np.flip(chunk, 0)) in rules:
            return from_cfg_string(rules[to_cfg_string(np.flip(chunk, 0))])
        chunk = np.rot90(chunk)


def iterate_production(config, rules):
    chunk_size = 0
    if len(config) % 2 == 0:
        chunk_size = 2
    else:
        chunk_size = 3
    new_cfg = np.empty(((chunk_size+1)*len(config)//chunk_size, (chunk_size+1)*len(config)//chunk_size), dtype=str)
    for x in range(0, len(config), chunk_size):
        for y in range(0, len(config), chunk_size):
            cx = (x//chunk_size)*(chunk_size+1)
            cy = (y//chunk_size)*(chunk_size+1)
            cfg = find_new_config(config[x:x+chunk_size, y:y+chunk_size], rules)
            new_cfg[cx:cx+chunk_size+1, cy:cy+chunk_size+1] = cfg
    return new_cfg


def count_lights(config):
    return(list(config.flatten()).count('#'))

if __name__ == '__main__':
    with open('21.txt') as f:
        lines = f.readlines()
    rules = {k: v for k, v in [l.strip().split(" => ") for l in lines]}

    config = start_config
    for i in range(5):
        config = iterate_production(config, rules)
    print("Part 1: {}".format(count_lights(config)))

    for i in range(18-5):
        config = iterate_production(config, rules)
    print("Part 2: {}".format(count_lights(config)))
