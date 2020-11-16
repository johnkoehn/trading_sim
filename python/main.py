# importing the required module
import matplotlib.pyplot as plt
import matplotlib
import json
import os
import graphing

run_results = graphing.load_run_results()
print(len(run_results))

# average_fitness_by_generation_figure = plt.figure()
# highest_fitness_by_generation_figure = plt.figure()


# Display highest fitness by generation

# Display average fitness

# Get best performaning bot

# Display wins/losses

# Display trades?

# graph average fitness
graphing.graph_average_fitness_by_generation(run_results)
graphing.graph_highest_fitness_by_generation(run_results)

plt.show()
