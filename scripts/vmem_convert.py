#!/usr/bin/env python3
import json
import sys

key_map = {
        'vram': '8000',
        'oam': 'fe00',
        'joyp': 'ff00',
        'sb': 'ff01',
        'div': 'ff04',
        'tima': 'ff05',
        'tma': 'ff06',
        'tac': 'ff07',
        'if': 'ff0f',
        'ie': 'ffff',
        'lcdc': 'ff40',
        'stat': 'ff41',
        'scy': 'ff42',
        'scx': 'ff43',
        'ly': 'ff44',
        'lyc': 'ff45',
        'wy': 'ff4a',
        'wx': 'ff4b',
        'dma': 'ff46',
        'bgp': 'ff47',
        'obp0': 'ff48',
        'obp1': 'ff49',
        'boot_rom_disable': 'ff50',
        'sc': 'ff02',
        'nr52': 'ff26',
        'nr51': 'ff25',
        'nr50': 'ff24',
        'nr12': 'ff12',
        'nr22': 'ff17',
        'nr42': 'ff21',
        'nr14': 'ff14',
        'nr24': 'ff19',
        'nr44': 'ff23',
        'nr10': 'ff10',
        'nr30': 'ff1a',
        'nr11': 'ff11',
        'nr13': 'ff13',
        'nr21': 'ff16',
        'nr23': 'ff18',
        'wave_pattern_ram_start': 'ff30',
        'wave_pattern_ram_end': 'ff3f',
        'nr31': 'ff1b',
        'nr32': 'ff1c',
        'nr33': 'ff1d',
        'nr34': 'ff1e',
        'nr41': 'ff20',
        'nr43': 'ff22',
}

def parse_dict(data):
    for key in data.keys():
        if key in ['vblank_interrupt_enabled', 'stat_interrupt_enabled']:
            continue

        d = data[key]
        if isinstance(d, list):
            key_str = key_map[key]
            str_data = ["{:x}".format(i) for i in data[key]]
            print('{}: {}'.format(key_str, ' '.join(str_data)))
        elif isinstance(d, dict):
            parse_dict(d)
        elif isinstance(d, int):
            key_str = key_map[key]
            print('{}: {:x}'.format(key_str, d))
        else:
            print('No idea what to do with', key, d)
            print(type(d))

with open(sys.argv[1]) as f:
    data = json.load(f)

parse_dict(data)
