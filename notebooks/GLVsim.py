import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

class GLV:
    def __init__(self, n):
        """
        A class for simulating generalized lotka-volterra.
        """
        self.n = n
        self.x = np.zeros(n)
        self.r = np.zeros(n)
        self.A = np.zeros((n, n))
    
    def set_state(self, state):
        self.x = state.copy()
        return state
    
    def set_rates(self, rates):
        self.r = rates.copy()
        return rates

    def set_matrix(self, matrix):
        self.A = matrix.copy()
        return matrix

    def step(self, dt):
        dxdt = self.x * (self.r + self.A.dot(self.x))
        self.x += dxdt * dt
    
    def simulate(self, end_time, dt):
        times = np.arange(0, end_time, dt)
        history = np.zeros((times.shape[0], self.n))

        for ti, time in enumerate(times):
            history[ti] = self.x
            self.step(dt)
        
        return times, history
