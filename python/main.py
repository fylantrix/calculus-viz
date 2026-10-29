import numpy as np
import matplotlib.pyplot as plt
from matplotlib.widgets import Slider, RadioButtons
from enum import Enum


class Function(Enum):
    Sin = 1
    Cos = 2
    Square = 3
    Cube = 4
    Exponential = 5

    def label(self) -> str:
        return {
            Function.Sin: "sin(x)",
            Function.Cos: "cos(x)",
            Function.Square: "x²",
            Function.Cube: "x³",
            Function.Exponential: "eˣ",
        }[self]

    def eval(self, x):
        match self:
            case Function.Sin:
                return np.sin(x)
            case Function.Cos:
                return np.cos(x)
            case Function.Square:
                return x**2
            case Function.Cube:
                return x**3
            case Function.Exponential:
                return np.exp(x)

    def derivative(self, x):
        match self:
            case Function.Sin:
                return np.cos(x)
            case Function.Cos:
                return -np.sin(x)
            case Function.Square:
                return 2 * x
            case Function.Cube:
                return 3 * x**2
            case Function.Exponential:
                return np.exp(x)


x = np.linspace(-5, 5, 300)
dx_initial = 1.0
selected_function = Function.Sin

fig, ax = plt.subplots()
fig.subplots_adjust(left=0.25, bottom=0.25)

base_line, = ax.plot(
    x, selected_function.eval(x), label=selected_function.label()
)
difference_line, = ax.plot(
    x,
    (selected_function.eval(x + dx_initial) - selected_function.eval(x))
    / dx_initial,
    label="Forward difference",
)

ax.set_xlim(-5, 5)
ax.grid(True)
ax.legend()

# function selector
radio_ax = fig.add_axes((0.03, 0.35, 0.16, 0.4))
radio = RadioButtons(
    radio_ax,
    [func.label() for func in Function],
)

# dx slider
slider_ax = fig.add_axes((0.25, 0.1, 0.65, 0.04))
slider = Slider(slider_ax, "dx", 0, 2, valinit=dx_initial)


def update(_=None):
    func = selected_function
    dx = slider.val

    base_line.set_ydata(func.eval(x))

    if dx == 0:
        # the limit of the difference quotient as dx approaches zero
        difference_line.set_ydata(func.derivative(x))
    else:
        difference_line.set_ydata(
            (func.eval(x + dx) - func.eval(x)) / dx
        )

    base_line.set_label(func.label())
    ax.relim()
    ax.autoscale_view(scalex=False, scaley=True)
    ax.legend()
    fig.canvas.draw_idle()


def select_function(label):
    global selected_function
    selected_function = next(
        func for func in Function if func.label() == label
    )
    update()


slider.on_changed(update)
radio.on_clicked(select_function)

plt.show()
