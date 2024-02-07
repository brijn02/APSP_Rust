import os
import matplotlib.pyplot as plt
from scipy.optimize import curve_fit
import numpy as np

def read_file(file_path):
    with open(file_path, 'r') as file:
        lines = file.readlines()
        n_value = int(os.path.basename(file_path).split('_')[1].split('.')[0])
        data = [(n_value, entry[0], entry[1]) for entry in [tuple(map(float, line.split())) for line in lines]]
    return data

def read_all_files(folder_path):
    all_data = {i + 1: [] for i in range(6)}

    files = [f for f in os.listdir(folder_path) if f.startswith('dense_') and f.endswith('.txt')]

    for file_name in files:
        try:
            file_path = os.path.join(folder_path, file_name)
            data = read_file(file_path)

            for i, entry in enumerate(data):
                all_data[i + 1].append(entry)
        except Exception as e:
            print(f"Error processing file {file_name}: {e}")

    # Sort each list in data[1, ..., 6] based on the first entry
    for i in range(1, 7):
        all_data[i] = sorted(all_data[i], key=lambda x: x[0])

    return all_data

## Plot all method results in one graph
def plot_all(all_data, method_info, log_scale=False):

    plt.figure(figsize=(10, 6))
    plt.title("Time Complexity for all Methods")

    for method_number, method_data in all_data.items():
        n_values = [entry[0] for entry in method_data]
        y_values = [entry[1] for entry in method_data]
        method_name = method_info[method_number]['name']
        method_color = method_info[method_number]['color']
        plt.plot(n_values, y_values, 'o-', label=method_name, color=method_color)

    plt.xlabel('#Nodes - n')
    plt.ylabel('Time [ms]')
    plt.legend()

    if log_scale:
        plt.xscale('log')
        plt.yscale('log')

    plt.savefig("plot_all.pdf")
    plt.show()

def linear_function_log(x, m, b):
    return m * np.log(x) + b

def fit_dij(x, m, b):
    return (m*np.log(x*(np.log(x)**(1/3)))) + b

## Plot function for Base Floyd-Warshall under the rust libary petgraph 
def plot_BaseFW(method_data, method_name, method_color, log_scale=False, ax=None):

    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Base Floyd-Warshall - petgraph")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:

        # Fit the linear function to the logarithm of y-values
        popt, pcov = curve_fit(linear_function_log, n_values, np.log(y_values), sigma=np.log(errors))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the linear fit on the original scale
        fitted_values = np.exp(linear_function_log(n_values, m, b))
        ax.plot(n_values, fitted_values, 'k--', label=f'Linear Fit: y = m*x + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(3 * np.log(n_values))/250000, 'k-', label='Theoretical Bound - O(n³)')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

## Plot function for the Floyd-Warshall Algorithm
def plot_FW(method_data, method_name, method_color, log_scale=False, ax=None):
    
    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Floyd-Warshall")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:
        # Fit the linear function to the logarithm of y-values
        popt, pcov = curve_fit(linear_function_log, n_values, np.log(y_values), sigma=np.log(errors))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the linear fit on the original scale
        fitted_values = np.exp(linear_function_log(n_values, m, b))
        ax.plot(n_values, fitted_values, 'k--', label=f'Linear Fit: y = m*x + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(3 * np.log(n_values))/250000, 'k-', label='Theoretical Bound - O(n³)')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

## Plot function for the dijkstra applied to all Nodes
def plot_dij(method_data, method_name, method_color, log_scale=False, show=False, ax=None):
    
    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Dijkstra on each Node")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:
        # Fit the function to the logarithm of y-values
        popt, pcov = curve_fit(fit_dij, n_values, np.log(y_values), sigma=np.log(errors))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the fit on the original scale
        fitted_values = np.exp(fit_dij(n_values, m, b))
        ax.plot(n_values, fitted_values, 'k--', label=f'Fit: y = (x^m) * log(x) + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(np.log(((n_values)**3) * (np.log(n_values))))/1000000, 'k-', label='Theoretical Bound - O(x³log(x))')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

## Plot function for the dijkstra applied to all Nodes in parallel
def plot_Pdij(method_data, method_name, method_color, log_scale=False, show=False, ax=None):
    
    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Dijkstra on each Node in Parallel")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:
        # Fit the function to the logarithm of y-values
        popt, pcov = curve_fit(fit_dij, n_values, np.log(y_values), sigma=np.log(errors))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the fit on the original scale
        fitted_values = np.exp(fit_dij(n_values, m, b))
        ax.plot(n_values, fitted_values, 'k--', label=f'Fit: y = (x^m) * log(x) + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(np.log(((n_values)**3) * (np.log(n_values))))/7000000, 'k-', label='Theoretical Bound - O(x³log(x))')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

## Plot function for the Blocked Floyd Warshall 
def plot_BlockedFW(method_data, method_name, method_color, log_scale=False, ax=None):
    
    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Blocked Floyd-Warshall")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:
        # Fit the linear function to the logarithm of y-values
        popt, pcov = curve_fit(linear_function_log, n_values, np.log(y_values), sigma=np.log(errors))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the linear fit on the original scale
        fitted_values = np.exp(linear_function_log(n_values, m, b))
        ax.plot(n_values, fitted_values, 'k--', label=f'Linear Fit: y = m*x + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(3 * np.log(n_values))/250000, 'k-', label='Theoretical Bound - O(n³)')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

## Plot function for the Blocked Floyd Warshall in parallel
def plot_PFW(method_data, method_name, method_color, log_scale=False, ax=None):
    
    if ax is None:
        fig, ax = plt.subplots() 

    ax.set_title("Floyd-Warshall Parallel")

    n_values = np.array([entry[0] for entry in method_data])
    y_values = np.array([entry[1] for entry in method_data])
    errors = np.array([entry[2] for entry in method_data]) 

    ax.plot(n_values, y_values, 'o', label=method_name, color=method_color)

    if log_scale:
        # Fit the linear function to the logarithm of y-values
        popt, pcov = curve_fit(linear_function_log, n_values[3:], np.log(y_values[3:]), sigma=np.log(errors[3:]))
        m, b = popt

        std_errors = np.sqrt(np.diag(pcov))
        std_m, std_b = std_errors

        # Plot the linear fit on the original scale
        fitted_values = np.exp(linear_function_log(n_values[3:], m, b))
        ax.plot(n_values[3:], fitted_values, 'k--', label=f'Linear Fit: y = m*x + b; (m = {m:.2f} ± {std_m:.2f}, b = {b:.2f} ± {std_b:.2f})')

    # Plot the theoretical bound
    ax.plot(n_values, np.exp(3 * np.log(n_values))/1000000, 'k-', label='Theoretical Bound - O(n³)')

    ax.set_xlabel('#Nodes - n')
    ax.set_ylabel('Time [ms]')

    ax.legend()
    ax.legend(loc='upper left')

    if log_scale:
        ax.set_yscale('log')
        ax.set_xscale('log')

    return m, std_m, b, std_b

def print_all_dense_results(all_data):
    for i in range(len(all_data[1])):
        print("\multicolumn{1}{|c|}{", "{}".format(all_data[1][i][0]) ,"} & $", "{:.2f}".format(all_data[1][i][1]), " \pm ","{:.2f}".format(all_data[1][i][2]), "$ & $"
                                                                                    , "{:.2f}".format(all_data[2][i][1]), " \pm ","{:.2f}".format(all_data[2][i][2]), "$ & $"
                                                                                    , "{:.2f}".format(all_data[3][i][1]), " \pm ","{:.2f}".format(all_data[3][i][2]), "$ & $"
                                                                                    , "{:.2f}".format(all_data[4][i][1]), " \pm ","{:.2f}".format(all_data[4][i][2]), "$ & $"
                                                                                    , "{:.2f}".format(all_data[5][i][1]), " \pm ","{:.2f}".format(all_data[5][i][2]), "$ & $"
                                                                                    , "{:.2f}".format(all_data[6][i][1]), " \pm ","{:.2f}".format(all_data[6][i][2]), "$ \\\\ \hline")

def main():
    import os
    import matplotlib.pyplot as plt
    
    folder_path = 'dense_results'
    all_data = read_all_files(folder_path)
    method_info = {
        1: {'name': 'Base Floyd-Warshall', 'color': 'Green'},
        2: {'name': 'Floyd-Warshall', 'color': 'DarkBlue'},
        3: {'name': 'Dijkstra', 'color': 'Red'},
        4: {'name': 'Dijkstra Parallel', 'color': 'Orange'},
        5: {'name': 'Floyd-Warshall Block', 'color': 'mediumpurple'},
        6: {'name': 'Floyd-Warshall Parallel', 'color': 'steelblue'}
    }

    plot_all(all_data, method_info, True)

    show = False

    ############################

    fig, axs = plt.subplots(1, 2, figsize=(20, 10), sharex=True, sharey=True)

    m_BFW, em_BFW, b_BFW, eb_BFW = plot_BaseFW(all_data[1], method_info[1]["name"], method_info[1]["color"], True, ax=axs[0])
    m_FW, em_FW, b_FW, eb_FW = plot_FW(all_data[2], method_info[2]["name"], method_info[2]["color"], True, ax=axs[1])

    # Optional: Adjust layout
    plt.tight_layout()

    plt.savefig('plot_BFW_FW.pdf')

    if show:
        plt.show()

    ############################

    fig, axs = plt.subplots(1, 2, figsize=(20, 10), sharex=True, sharey=True)

    m_dij, em_dij, b_dij, eb_dij = plot_dij(all_data[3], method_info[3]["name"], method_info[3]["color"], True, ax=axs[0])
    m_Pdij, em_Pdij, b_Pdij, eb_Pdij = plot_Pdij(all_data[4], method_info[4]["name"], method_info[4]["color"], True, ax=axs[1])

    # Optional: Adjust layout
    plt.tight_layout()

    plt.savefig('plot_DIJ_PDIJ.pdf')

    if show:
        plt.show()

    ############################

    fig, axs = plt.subplots(1, 2, figsize=(20, 10), sharex=True, sharey=True)

    m_BlockedFW, em_BlockedFW, b_BlockedFW, eb_BlockedFW  = plot_BlockedFW(all_data[5], method_info[5]["name"], method_info[5]["color"], True, ax=axs[0])
    m_PFW, em_PFW, b_PFW, eb_PFW = plot_PFW(all_data[6], method_info[6]["name"], method_info[6]["color"], True, ax=axs[1])

    # Optional: Adjust layout
    plt.tight_layout()

    plt.savefig('plot_BlockedFW_PFW.pdf')

    if show:
        plt.show()


    print_table1 = False

    if print_table1:
        print("\\begin{table}[!hbt]")
        print("\\centering")
        print("\\resizebox{\\textwidth}{!}{%")
        print("\\begin{tabular}{c|c|c|c|c|c|c|}")
        print("\cline{2-7}")
        print(" & Base FW & FW & Dijkstra & Dijkstra Parallel & Blocked FW & Parallel FW \\\\ \hline \hline")
        print("\multicolumn{1}{|c|}{fitted function}  & $t = mV + b$ & $t = mV + b$ & $t = V^m \log(V) + b$ & $t = V^m \log(V) + b$ & $t = mV + b$ & $t = mV + b$ \\\\ \hline \hline")
        print("\multicolumn{1}{|c|}{$m$} & $", "{:.2f}".format(m_BFW), " \pm ","{:.2f}".format(em_BFW), "$ & $"
                                            , "{:.2f}".format(m_FW), " \pm ","{:.2f}".format(em_FW), "$ & $"
                                            , "{:.2f}".format(m_dij), " \pm ","{:.2f}".format(em_dij), "$ & $"
                                            , "{:.2f}".format(m_Pdij), " \pm ","{:.2f}".format(em_Pdij), "$ & $"
                                            , "{:.2f}".format(m_BlockedFW), " \pm ","{:.2f}".format(em_BlockedFW), "$ & $"
                                            , "{:.2f}".format(m_PFW), " \pm ","{:.2f}".format(eb_PFW), "$ \\\\ \hline")
        print("\multicolumn{1}{|c|}{$b$} & $", "{:.2f}".format(b_BFW), " \pm ","{:.2f}".format(eb_BFW), "$ & $"
                                            , "{:.2f}".format(b_FW), " \pm ","{:.2f}".format(eb_FW), "$ & $"
                                            , "{:.2f}".format(b_dij), " \pm ","{:.2f}".format(eb_dij), "$ & $"
                                            , "{:.2f}".format(b_Pdij), " \pm ","{:.2f}".format(eb_Pdij), "$ & $"
                                            , "{:.2f}".format(b_BlockedFW), " \pm ","{:.2f}".format(eb_BlockedFW), "$ & $"
                                            , "{:.2f}".format(b_PFW), " \pm ","{:.2f}".format(eb_PFW), "$ \\\\ \hline")
        print("\\end{tabular}%")
        print("}")
        print("\\caption{Dense Graph - Fit results for each implementation.}")
        print("\\label{tab:dense_fit_results}")
        print("\\end{table}")


    print_table2 = False

    if print_table2:
        print("\\begin{table}[!hbt]")
        print("\\centering")
        print("\\resizebox{\\textwidth}{!}{%")
        print("\\begin{tabular}{c|c|c|c|c|c|c|}")
        print("\cline{2-7}")
        print(" & Base FW & FW & Dijkstra & Dijkstra Parallel & Blocked FW & Parallel FW \\\\ \hline \hline")
        print_all_dense_results(all_data)
        print("\\end{tabular}%")
        print("}")
        print("\\caption{Dense Graph - Times for each implementation.}")
        print("\\label{tab:dense_times}")
        print("\\end{table}")


main()