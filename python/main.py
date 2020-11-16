# importing the required module
import matplotlib.pyplot as plt
import matplotlib
import json
import os

def load_run_results():
    run_results = []
    for generation_file in os.scandir("../simulations/current/"):
        if generation_file.is_file():
            run_results.append(json.load(open("../simulations/current/" + generation_file.name)))

    return run_results

def graph_average_fitness_by_generation(run_results):
    x = []
    y = []
    for gen_number, generation in enumerate(run_results):
        total_fitness = 0

        for bot in generation:
            total_fitness += bot["fitness"]

        average_fitness_for_generation = total_fitness / len(generation)
        y.append(average_fitness_for_generation)
        x.append(gen_number + 1)

    # plotting the points
    plt.plot(x, y)

    # naming the x axis
    plt.xlabel('Generation')
    # naming the y axis
    plt.ylabel('Average Fitness')

    # giving a title to my graph
    plt.title('Average Fitness')

    # function to show the plot
    plt.show()



run_results = load_run_results()
print(len(run_results))

# graph average fitness
graph_average_fitness_by_generation(run_results)
