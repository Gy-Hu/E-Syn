from pysr import PySRRegressor
import sympy

'''
# ---------------------------------------------------Model 1 Starting Point (default)-------------------------------------------------
'''
model1 = PySRRegressor(
    procs=64,
    populations=8,
    # ^ 2 populations per core, so one is always running.
    population_size=50,
    # ^ Slightly larger populations, for greater diversity.
    ncyclesperiteration=500,
    # ^ Generations between migrations.
    # niterations=10000000,  # Run forever
    niterations=1000,
    early_stop_condition=(
        #"stop_if(loss, complexity) = loss < 1e-6 && complexity < 10"
        "stop_if(loss, complexity) = loss < 1e-8 && complexity < 15"
        # Stop early if we find a good and simple equation
    ),
    timeout_in_seconds=60 * 60 * 24,
    # ^ Alternatively, stop after 24 hours have passed.
    maxsize=50,
    # ^ Allow greater complexity.
    maxdepth=10,
    # ^ But, avoid deep nesting.
    binary_operators=["*", "+", "-", "/"],
    unary_operators=["square", "cube", "exp", "cos2(x)=cos(x)^2"],
    constraints={
        "/": (-1, 9),
        "square": 9,
        "cube": 9,
        "exp": 9,
    },
    # ^ Limit the complexity within each argument.
    # "inv": (-1, 9) states that the numerator has no constraint,
    # but the denominator has a max complexity of 9.
    # "exp": 9 simply states that `exp` can only have
    # an expression of complexity 9 as input.
    nested_constraints={
        "square": {"square": 1, "cube": 1, "exp": 0},
        "cube": {"square": 1, "cube": 1, "exp": 0},
        "exp": {"square": 1, "cube": 1, "exp": 0},
    },
    # ^ Nesting constraints on operators. For example,
    # "square(exp(x))" is not allowed, since "square": {"exp": 0}.
    complexity_of_operators={"/": 2, "exp": 3},
    # ^ Custom complexity of particular operators.
    complexity_of_constants=2,
    # ^ Punish constants more than variables
    select_k_features=4,
    # ^ Train on only the 4 most important features
    progress=True,
    # ^ Can set to false if printing to a file.
    weight_randomize=0.1,
    # ^ Randomize the tree much more frequently
    cluster_manager=None,
    # ^ Can be set to, e.g., "slurm", to run a slurm
    # cluster. Just launch one script from the head node.
    precision=64,
    # ^ Higher precision calculations.
    warm_start=True,
    # ^ Start from where left off.
    turbo=True,
    # ^ Faster evaluation (experimental)
    julia_project=None,
    # ^ Can set to the path of a folder containing the
    # "SymbolicRegression.jl" repo, for custom modifications.
    update=False,
    # ^ Don't update Julia packages
    extra_sympy_mappings={"cos2": lambda x: sympy.cos(x)**2},
    # extra_torch_mappings={sympy.cos: torch.cos},
    # ^ Not needed as cos already defined, but this
    # is how you define custom torch operators.
    # extra_jax_mappings={sympy.cos: "jnp.cos"},
    # ^ For JAX, one passes a string.
)
'''
# ---------------------------------------------------Model 1 End Point (default)-------------------------------------------------
'''

'''
# ---------------------------------------------------Model 2 Starting Point (default)-------------------------------------------------
'''
model2 = PySRRegressor(
    procs=4,
    populations=8,
    population_size=50,
    ncyclesperiteration=500,
    niterations=10000000,
    early_stop_condition=(
        "stop_if(loss, complexity) = loss < 1e-6 && complexity < 10"
    ),
    timeout_in_seconds=60 * 60 * 24,
    maxsize=50,
    maxdepth=10,
    binary_operators=["*", "+", "-", "/"],
    unary_operators=["square", "cube", "exp", "cos2(x)=cos(x)^2"],
    constraints={
        "/": (-1, 9),
        "square": 9,
        "cube": 9,
        "exp": 9,
    },
    nested_constraints={
        "square": {"square": 1, "cube": 1, "exp": 0},
        "cube": {"square": 1, "cube": 1, "exp": 0},
        "exp": {"square": 1, "cube": 1, "exp": 0},
    },
    complexity_of_operators={"/": 2, "exp": 3},
    complexity_of_constants=2,
    select_k_features=4,
    progress=True,
    weight_randomize=0.1,
    cluster_manager=None,
    precision=64,
    warm_start=True,
    turbo=True,
    julia_project=None,
    update=False,
    extra_sympy_mappings={"cos2": lambda x: sympy.cos(x)**2},
)
'''
# ---------------------------------------------------Model 2 End Point (default)-------------------------------------------------
'''


'''
---------------------------------------------------Model 3 Starting Point (By chatGPT)-------------------------------------------------
'''
model3 = PySRRegressor(
    procs=4,
    populations=10,  # Increased the population size
    population_size=50,
    ncyclesperiteration=500,
    niterations=10000000,
    early_stop_condition=(
        # Increased complexity constraint
        "stop_if(loss, complexity) = loss < 1e-6 && complexity < 15"
    ),
    timeout_in_seconds=60 * 60 * 24,
    maxsize=50,
    maxdepth=15,  # Increased maximum depth
    # Added exponentiation operator
    binary_operators=["*", "+", "-", "/", "**"],
    # Removed "cube" and "cos2(x)" operators, added "log" operator
    unary_operators=["square", "exp", "log"],
    constraints={
        "/": (-1, 9),
        "square": 9,
        "exp": 9,
        "log": (1, 9),  # Added constraint on "log" operator
    },
    nested_constraints={
        "square": {"square": 1, "exp": 0, "log": 0},
        "exp": {"square": 1, "exp": 0, "log": 0},
        # Added constraints on "log" operator
        "log": {"square": 1, "exp": 1, "log": 0},
    },
    complexity_of_operators={"/": 2, "**": 3, "exp": 3,
                             "log": 4},  # Adjusted complexity of operators
    complexity_of_constants=2,
    select_k_features=4,
    progress=True,
    weight_randomize=0.1,
    cluster_manager=None,
    precision=64,
    warm_start=True,
    turbo=True,
    julia_project=None,
    update=False,
    extra_sympy_mappings={"cos2": lambda x: sympy.cos(x)**2},
)
'''
---------------------------------------------------Model 3 End Point (By chatGPT)-------------------------------------------------
'''

'''
---------------------------------------------------Model 4 Starting Point (By chatGPT)-------------------------------------------------
'''
model4 = PySRRegressor(
    procs=4,
    populations=20,  # Increased population size
    population_size=100,
    ncyclesperiteration=500,
    niterations=10000000,
    early_stop_condition=(
        # Increased complexity constraint
        "stop_if(loss, complexity) = loss < 1e-6 && complexity < 20"
    ),
    timeout_in_seconds=60 * 60 * 24,
    maxsize=100,
    maxdepth=20,  # Increased maximum depth
    binary_operators=["*", "+", "-", "/", "**", "sin",
                      "cos"],  # Added trigonometric functions
    # Removed "square" and "cube", added "abs"
    unary_operators=["exp", "log", "abs"],
    constraints={
        "/": (-10, 10),  # Expanded range of division operator
        "**": (0, 5),  # Limited exponentiation operator to positive exponents only
        "sin": (-10, 10),  # Added constraints on trigonometric functions
        "cos": (-10, 10),
        "exp": (0, 10),
        "log": (0.1, 10),  # Added lower bound for log operator
        "abs": (-10, 10),  # Added absolute value operator
    },
    nested_constraints={
        # Allowed division to be nested with all operators
        "/": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        # Allowed exponentiation to be nested with all operators
        "**": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        "sin": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        "cos": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        "exp": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        "log": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
        "abs": {"**": 1, "sin": 1, "cos": 1, "exp": 1, "log": 1, "abs": 1},
    },
    complexity_of_operators={
        "/": 2,
        "**": 3,
        "sin": 4,
        "cos": 4,
        "exp": 5,
        "log": 5,
        "abs": 2
    },  # Adjusted complexity of operators
    complexity_of_constants=2,
    select_k_features=4,
    progress=True,
    weight_randomize=0.1,
    cluster_manager=None,
    precision=64,
    warm_start=True,
    turbo=True,
    julia_project=None,
    update=False,
)

'''
---------------------------------------------------Model 4 End Point (By chatGPT)-------------------------------------------------
'''

'''
---------------------------------------------------Model 5 Starting Point (Linear Operations Only)-------------------------------------------------
'''
model5 = PySRRegressor(
    procs=64,
    populations=20,  # Retaining increased population size from Model 4
    population_size=100,
    ncyclesperiteration=500,
    niterations=1000,
    # niterations=10000000,
    early_stop_condition=(
        # Retaining complexity constraint from Model 4
        "stop_if(loss, complexity) = loss < 1e-6 && complexity < 20"
    ),
    timeout_in_seconds=60 * 60 * 24,
    maxsize=100,
    maxdepth=20,  # Retaining maximum depth from Model 4
    #binary_operators=["*", "+"],  # Only linear operations
    binary_operators=["+"],  # Only linear operations
    unary_operators=[],  # No unary operations
    constraints={},
    nested_constraints={},
    complexity_of_operators={
        #"*": 2,
        "+": 2,
    },  # Complexity of linear operations
    complexity_of_constants=2,
    select_k_features=4,
    progress=True,
    weight_randomize=0.1,
    cluster_manager=None,
    precision=64,
    warm_start=True,
    turbo=True,
    julia_project=None,
    update=False,
)
'''
---------------------------------------------------Model 5 End Point (Linear Operations Only)-------------------------------------------------
'''

'''
---------------------------------------------------Model 6 Starting Point (Mononitic)-------------------------------------------------
'''

model6 = PySRRegressor(
    procs=64,
    populations=8,
    # ^ 2 populations per core, so one is always running.
    population_size=50,
    # ^ Slightly larger populations, for greater diversity.
    ncyclesperiteration=500,
    # ^ Generations between migrations.
    # niterations=10000000,  # Run forever
    niterations=1000,
    early_stop_condition=(
        #"stop_if(loss, complexity) = loss < 1e-6 && complexity < 10"
        "stop_if(loss, complexity) = loss < 1e-8 && complexity < 15"
        # Stop early if we find a good and simple equation
    ),
    timeout_in_seconds=60 * 60 * 24,
    # ^ Alternatively, stop after 24 hours have passed.
    maxsize=50,
    # ^ Allow greater complexity.
    maxdepth=10,
    # ^ But, avoid deep nesting.
    binary_operators=["*", "+", "/"],
    unary_operators=["square", "cube", "exp"],
    constraints={
        "/": (-1, 9),
        "square": 9,
        "cube": 9,
        "exp": 9,
    },
    # ^ Limit the complexity within each argument.
    # "inv": (-1, 9) states that the numerator has no constraint,
    # but the denominator has a max complexity of 9.
    # "exp": 9 simply states that `exp` can only have
    # an expression of complexity 9 as input.
    nested_constraints={
        "square": {"square": 1, "cube": 1, "exp": 0},
        "cube": {"square": 1, "cube": 1, "exp": 0},
        "exp": {"square": 1, "cube": 1, "exp": 0},
    },
    # ^ Nesting constraints on operators. For example,
    # "square(exp(x))" is not allowed, since "square": {"exp": 0}.
    complexity_of_operators={"/": 2, "exp": 3},
    # ^ Custom complexity of particular operators.
    complexity_of_constants=2,
    # ^ Punish constants more than variables
    select_k_features=4,
    # ^ Train on only the 4 most important features
    progress=True,
    # ^ Can set to false if printing to a file.
    weight_randomize=0.1,
    # ^ Randomize the tree much more frequently
    cluster_manager=None,
    # ^ Can be set to, e.g., "slurm", to run a slurm
    # cluster. Just launch one script from the head node.
    precision=64,
    # ^ Higher precision calculations.
    warm_start=True,
    # ^ Start from where left off.
    turbo=True,
    # ^ Faster evaluation (experimental)
    julia_project=None,
    # ^ Can set to the path of a folder containing the
    # "SymbolicRegression.jl" repo, for custom modifications.
    update=False,
    # ^ Don't update Julia packages
    # extra_torch_mappings={sympy.cos: torch.cos},
    # ^ Not needed as cos already defined, but this
    # is how you define custom torch operators.
    # extra_jax_mappings={sympy.cos: "jnp.cos"},
    # ^ For JAX, one passes a string.
)


'''
---------------------------------------------------Model 6 End Point (Mononitic)-------------------------------------------------
'''
