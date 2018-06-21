#!/usr/bin/env python3
import fileinput
import subprocess
import sys
import numpy as np
import matplotlib.pyplot as plt
import matplotlib.animation as animation

emu = subprocess.Popen(['cargo', 'run', '--example', 'display'], stdout=subprocess.PIPE);

frames = 0

def get_frame():
    global frames
    screen_buffer = np.full((144, 160, 3), np.uint8(0))

    while True:
        line = emu.stdout.readline().decode("utf-8")
        # line = sys.stdin.readline()
        if line == "":
            emu.terminate()
            # print("End of file")
            return screen_buffer 

        if line.startswith("LINE "):
            stash = line
            line = line.split()

            ly = int(line[1])
            if ly == 144:
                frames += 1
                print("Returning frame", frames)
                return screen_buffer

            if ly < 144:
                data = line[2]
                if len(data) != 160:
                    print(data)
                    print(len(data))
                    print(stash)
                assert len(data) == 160

                for i, pixel in enumerate(data):
                    # 0-9 scaled to 0-252
                    pixel = np.uint8(int(pixel) * 255)
                    screen_buffer[ly][i] = (pixel, pixel, pixel)
        else:
            print(line, end='')

fig = plt.figure()
im = plt.imshow(get_frame(), animated=True)

def updatefig(*args):
    im.set_array(get_frame())
    return im,

ani = animation.FuncAnimation(fig, updatefig, interval=1, blit=True)
plt.show()
emu.terminate()
