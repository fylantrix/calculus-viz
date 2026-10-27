import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider

x = np.linspace(-5, 5, 300)

fig, ax = plt.subplots()
fig.subplots_adjust(bottom=0.25)

dx = 1

base, = ax.plot(x, np.sin(x))
line, = ax.plot(x, np.sin(x+dx) - np.sin(x))
ax.set_xlim(-5, 5)
ax.set_ylim(-1.2, 1.2)
ax.grid(True)

slider_ax = fig.add_axes((0.2, 0.1, 0.65, 0.04))
slider = Slider(slider_ax, "dx", -0.0, 2.0, valinit=dx)

def update(value):
    line.set_ydata((np.sin(x + value) - np.sin(x)) / value)
    fig.canvas.draw_idle()

slider.on_changed(update)
plt.show()

