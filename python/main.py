import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider

x = np.linspace(-5, 5, 300)
slope = 1.0

fig, ax = plt.subplots()
fig.subplots_adjust(bottom=0.25)

line, = ax.plot(x, pow(x, slope))
ax.set_xlim(-5, 5)
ax.set_ylim(-10, 10)
ax.grid(True)

slider_ax = fig.add_axes((0.2, 0.1, 0.65, 0.04))
slider = Slider(slider_ax, "Slope", -2.0, 2.0, valinit=slope)

def update(value):
    line.set_ydata(value * x)
    fig.canvas.draw_idle()

slider.on_changed(update)
plt.show()

